use crate::cose::CoseSign1;
use alloy::{primitives::Address, sol};
use const_oid::db::rfc5912::ECDSA_WITH_SHA_384;
use p384::{
    ecdsa::{signature::Verifier as P384Verifier, Signature as P384Signature},
    pkcs8::DecodePublicKey,
};
// use openssl::{error::ErrorStack, hash::MessageDigest, pkey::PKey, sign::Verifier};
use serde::Deserialize;
use std::collections::HashMap;
use x509_cert::{
    certificate::CertificateInner,
    der::{Decode, Encode},
};

// SP1 specific
sol! {
  /// The public values encoded as a struct that can be easily deserialized inside Solidity.
  /// must match the definition in `synd-contracts/src/withdrawal/AttestationDocVerifier.sol`
  struct PublicValuesStruct {
      bytes32 root_cert_hash;
      uint64 validity_window_start;
      uint64 validity_window_end;
      // https://docs.aws.amazon.com/enclaves/latest/user/set-up-attestation.html#where
      // each pcr is 48 bytes, that is subsequently keccak256'd
      bytes32 pcr_0;
      bytes32 pcr_1;
      bytes32 pcr_2;
      bytes32 pcr_8;
      address tee_signing_key;
  }
}

#[derive(Debug)]
pub enum VerificationError {
    InvalidCoseSign1Signature(String),
    DocumentParseError(String),
    CoseSign1ParseError(String),
    CaBundleParseError(String),
    CertificateParseError(String),
    InvalidRootCert,
    ValidityError(String),
    InvalidCertSignatureAlgorithm(String),
    MandatoryFieldsMissing,
    BadDigest,
    BadTimestamp,
    BadPCRsLen,
    BadPCRIndex,
    BadPCRValue,
    BadCABundleLen,
    BadCABundleItemLen,
    BadPublicKeyLen,
    BadUserDataLen,
    BadNonceLen,
    PublicKeyMissing,
}

const MAX_PCR_COUNT: usize = 32;
const MAX_CABUNDLE_COUNT: usize = 4;

/// https://docs.aws.amazon.com/enclaves/latest/user/verify-root.html#doc-def
#[derive(Deserialize)]
struct AwsNitroAttestationDocument<'a> {
    #[serde(borrow)]
    module_id: &'a str,

    #[serde(borrow)]
    digest: &'a str,

    timestamp: u64,

    #[serde(borrow)]
    pcrs: HashMap<u8, &'a [u8]>, // SHA384 hash is 384 bits = 48 bytes

    #[serde(borrow)]
    certificate: &'a [u8], // encoded as DER

    #[serde(borrow)]
    cabundle: [&'a [u8]; MAX_CABUNDLE_COUNT], // encoded as DER

    //--- optional fields---
    #[serde(borrow)]
    public_key: Option<&'a [u8]>,

    #[serde(borrow)]
    user_data: Option<&'a [u8]>,

    #[serde(borrow)]
    nonce: Option<&'a [u8]>,
}

impl AwsNitroAttestationDocument<'_> {
    fn parse_document(input: &mut [u8]) -> Result<AwsNitroAttestationDocument, VerificationError> {
        let doc: AwsNitroAttestationDocument = serde_cbor::de::from_mut_slice(input)
            .map_err(|e| VerificationError::DocumentParseError(e.to_string()))?;

        if doc.module_id.is_empty() ||
            doc.digest.is_empty() ||
            doc.timestamp == 0 ||
            doc.pcrs.is_empty() ||
            doc.certificate.is_empty() ||
            doc.cabundle.is_empty()
        {
            return Err(VerificationError::MandatoryFieldsMissing);
        }

        if doc.digest != "SHA384" {
            return Err(VerificationError::BadDigest);
        }

        if doc.timestamp < 1 {
            return Err(VerificationError::BadTimestamp);
        }

        if doc.pcrs.is_empty() || doc.pcrs.len() > MAX_PCR_COUNT {
            return Err(VerificationError::BadPCRsLen);
        }

        for (key, value) in &doc.pcrs {
            if *key > 31 {
                // u8 key cannot be < 0
                return Err(VerificationError::BadPCRIndex);
            }

            if value.is_empty() || !(value.len() == 32 || value.len() == 48 || value.len() == 64) {
                return Err(VerificationError::BadPCRValue);
            }
        }

        if doc.cabundle.is_empty() {
            return Err(VerificationError::BadCABundleLen);
        }

        for item in &doc.cabundle {
            if item.is_empty() || item.len() > 1024 {
                return Err(VerificationError::BadCABundleItemLen);
            }
        }

        if let Some(public_key) = doc.public_key {
            if public_key.len() > 1024 {
                return Err(VerificationError::BadPublicKeyLen);
            }
        }

        if let Some(user_data) = doc.user_data {
            if user_data.len() > 1024 {
                return Err(VerificationError::BadUserDataLen);
            }
        }

        if let Some(nonce) = doc.nonce {
            if nonce.len() > 1024 {
                return Err(VerificationError::BadNonceLen);
            }
        }
        Ok(doc)
    }

    fn verify_cert_chain(
        &self,
        trusted_root_cert_der: &[u8],
    ) -> Result<(CertificateInner, u64, u64), VerificationError> {
        // assert the cert chain starts with the trusted root cert
        if self.cabundle.first().is_none_or(|&cert| cert != trusted_root_cert_der) {
            return Err(VerificationError::InvalidRootCert);
        }

        let mut validity_start = u64::MIN;
        let mut validity_end = u64::MAX;

        let mut certs = self
            .cabundle
            .iter()
            .map(|cert_data| {
                x509_cert::Certificate::from_der(cert_data)
                    .map_err(|e| VerificationError::CaBundleParseError(e.to_string()))
            })
            .collect::<Result<Vec<x509_cert::Certificate>, VerificationError>>()?;

        let cert = x509_cert::Certificate::from_der(self.certificate)
            .map_err(|e| VerificationError::CertificateParseError(e.to_string()))?;

        certs.push(cert.clone());

        for i in 1..certs.len() {
            let cert = &certs[i];

            let validity = cert.tbs_certificate.validity;
            let not_before = validity.not_before.to_unix_duration().as_secs();
            let not_after = validity.not_after.to_unix_duration().as_secs();

            if not_before > validity_start {
                validity_start = not_before;
            }
            if not_after < validity_end {
                validity_end = not_after;
            }

            let parent_cert = &certs[i - 1];
            verify_x509_parent(cert, parent_cert)?;
        }

        if validity_start >= validity_end {
            return Err(VerificationError::ValidityError(format!(
                "Validity window is invalid: validity_start: {validity_start}, validity_end: {validity_end}"
            )));
        }

        Ok((cert, validity_start, validity_end))
    }
}

/// Verify that the parent certificate is valid and that the certificate is signed by the parent.
/// NOTE: only supports ECDSA with SHA-384.
pub fn verify_x509_parent(
    cert: &CertificateInner,
    parent_cert: &CertificateInner,
) -> Result<(), VerificationError> {
    if cert.signature_algorithm.oid != ECDSA_WITH_SHA_384 {
        return Err(VerificationError::InvalidCertSignatureAlgorithm(
            cert.signature_algorithm.oid.to_string(),
        ));
    }

    let parent_spki_der =
        parent_cert.tbs_certificate.subject_public_key_info.to_der().map_err(|e| {
            VerificationError::CertificateParseError(format!(
                "Failed to DER-encode parent's SubjectPublicKeyInfo: {e}"
            ))
        })?;

    let parent_verifying_key = p384::ecdsa::VerifyingKey::from_public_key_der(&parent_spki_der)
        .map_err(|e| {
            VerificationError::CertificateParseError(format!(
                "Failed to parse parent's public key from SubjectPublicKeyInfo: {e}"
            ))
        })?;

    let msg_to_verify = cert.tbs_certificate.to_der().map_err(|e| {
        VerificationError::CertificateParseError(format!(
            "Failed to DER-encode TBS certificate: {e}"
        ))
    })?;

    let signature_bytes = cert.signature.as_bytes().ok_or_else(|| {
        VerificationError::CertificateParseError("Signature is not a valid BIT STRING".to_string())
    })?;

    let signature = P384Signature::from_der(signature_bytes).map_err(|e| {
        VerificationError::CertificateParseError(format!("Failed to parse signature: {e}"))
    })?;

    parent_verifying_key.verify(&msg_to_verify, &signature).map_err(|e| {
        VerificationError::CertificateParseError(format!("Signature verification failed: {e}"))
    })?;

    Ok(())
}

pub struct ValidationResult {
    pub tee_signing_key: Address,
    pub validity_window_start: u64,
    pub validity_window_end: u64,
    pub pcr_0: Vec<u8>,
    pub pcr_1: Vec<u8>,
    pub pcr_2: Vec<u8>,
    pub pcr_8: Vec<u8>,
}

/// https://github.com/aws/aws-nitro-enclaves-nsm-api/blob/main/docs/attestation_process.md#32-syntactical-validation
///
/// - Decode the CBOR object and map it to a COSE_Sign1 structure;
/// - Extract the Attestation Document from the COSE_Sign1 structure;
/// - Verify the certificates chain;
/// - Ensure that the Signed Attestation Document was correctly signed.
///
/// returns the pub key generated by the TEE and the validity window for the attestation document's
/// certificate
pub fn verify_aws_nitro_attestation(
    input: &[u8],
    trusted_root_cert_der: &[u8],
) -> Result<ValidationResult, VerificationError> {
    let mut input = input.to_vec();
    let cose_sign1 = CoseSign1::from_bytes(&mut input)
        .map_err(|e| VerificationError::CoseSign1ParseError(e.to_string()))?;

    let mut payload_data = cose_sign1.payload.to_vec();
    let doc = AwsNitroAttestationDocument::parse_document(&mut payload_data)?;
    let (cert, validity_window_start, validity_window_end) =
        doc.verify_cert_chain(trusted_root_cert_der)?;

    let spki_der = cert.tbs_certificate.subject_public_key_info.to_der().map_err(|e| {
        VerificationError::CertificateParseError(format!(
            "Failed to DER-encode SubjectPublicKeyInfo: {e}"
        ))
    })?;

    let pub_key = p384::ecdsa::VerifyingKey::from_public_key_der(&spki_der).map_err(|e| {
        VerificationError::CertificateParseError(format!(
            "Failed to parse public key from SubjectPublicKeyInfo: {e}"
        ))
    })?;

    cose_sign1
        .verify_signature(&pub_key)
        .map_err(|e| VerificationError::InvalidCoseSign1Signature(e.to_string()))?;

    let pub_key = doc.public_key.ok_or(VerificationError::PublicKeyMissing)?;

    // pub key comes with a recovery byte suffix https://github.com/ethereum/go-ethereum/blob/c87b856c1a7daff56b46be70cdb7092adc519b7c/crypto/crypto.go#L40
    assert_eq!(pub_key.len(), 65);

    Ok(ValidationResult {
        // exclude the leading 0x04 byte prefix
        tee_signing_key: Address::from_raw_public_key(&pub_key[..64]),
        validity_window_start,
        validity_window_end,
        pcr_0: doc.pcrs.get(&0).unwrap().to_vec(),
        pcr_1: doc.pcrs.get(&1).unwrap().to_vec(),
        pcr_2: doc.pcrs.get(&2).unwrap().to_vec(),
        pcr_8: doc.pcrs.get(&8).unwrap().to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::hex;
    use x509_cert::der::DecodePem;

    fn der_from_pem(pem: &[u8]) -> Vec<u8> {
        let cert = x509_cert::Certificate::from_pem(pem).unwrap();
        cert.to_der().unwrap()
    }

    #[test]
    fn test_verify_aws_nitro_attestation_emtpy_pub_key() {
        let doc_cbor = include_bytes!("testdata/att_doc_sample.bin");
        let trusted_root_cert_der = der_from_pem(include_bytes!("testdata/aws_nitro_root.pem"));

        let res = verify_aws_nitro_attestation(doc_cbor, &trusted_root_cert_der);
        //all validation passes, but it attests to no pub key
        assert!(matches!(res, Err(VerificationError::PublicKeyMissing)));
    }

    #[test]
    fn test_verify_aws_nitro_attestation_with_pub_key() {
        let doc_hex = include_bytes!("testdata/att_doc_sample_2.hex");
        let doc_cbor = hex::decode(doc_hex).unwrap();
        let trusted_root_cert_der = der_from_pem(include_bytes!("testdata/aws_nitro_root.pem"));

        let res = verify_aws_nitro_attestation(&doc_cbor, &trusted_root_cert_der).unwrap();
        assert_eq!(
            res.tee_signing_key,
            alloy::primitives::Address::from_raw_public_key(&hex::decode("040697cfa9437ccd8db7b2f2ff47dee17a5269b0e8600b6a8334339f28dddae716edcc41ebf70dec757d0ee9fa55448bd01b98fd7cf1676ad82f7b60e04b72cb").unwrap())
        );
        assert_eq!(res.validity_window_start, 1748509950);
        assert_eq!(res.validity_window_end, 1748520753);
    }
}
