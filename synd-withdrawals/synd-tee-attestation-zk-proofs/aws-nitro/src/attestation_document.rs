use crate::{
    cose::CoseSign1,
    keccak256,
    p384::{
        ecdsa::{self, signature::Verifier as _},
        elliptic_curve::PublicKey,
        NistP384,
    },
};
use alloy_primitives::{Address, B256};
use serde::Deserialize;
use x509_cert::{
    certificate::CertificateInner,
    der::{self, referenced::OwnedToRef as _, Decode as _, Encode as _},
    spki::{self, SubjectPublicKeyInfoOwned},
};

fn from_public_key_info(
    info: &SubjectPublicKeyInfoOwned,
) -> Result<ecdsa::VerifyingKey, spki::Error> {
    let key: PublicKey<NistP384> = PublicKey::try_from(info.owned_to_ref())?;
    Ok(ecdsa::VerifyingKey::from_affine(*key.as_affine()).unwrap())
}

#[derive(Debug)]
pub enum VerificationError {
    DocumentParseError(serde_cbor::Error),
    CoseSign1ParseError(&'static str),
    CertificateParseError(spki::Error),
    CertificateSignatureError(ecdsa::Error),
    InvalidRootCert,
    InvalidSignature,
    MandatoryFieldsMissing,
    BadDigest,
    BadPCRValue,
    BadCABundleItemLen,
    BadPublicKeyLen,
    PublicKeyMissing,
}

const MAX_CABUNDLE_COUNT: usize = 4;

// order matters here - CBOR uses the int offset as the deserialization key
#[derive(Deserialize)]
struct Pcrs<'a> {
    pcr_0: &'a [u8],
    pcr_1: &'a [u8],
    pcr_2: &'a [u8],
}

/// https://docs.aws.amazon.com/enclaves/latest/user/verify-root.html#doc-def
#[derive(Deserialize)]
struct AwsNitroAttestationDocument<'a> {
    timestamp: u64,

    #[serde(borrow)]
    digest: &'a str,

    #[serde(borrow)]
    pcrs: Pcrs<'a>,

    #[serde(borrow)]
    certificate: &'a [u8], // encoded as DER

    #[serde(borrow)]
    cabundle: [&'a [u8]; MAX_CABUNDLE_COUNT], // encoded as DER

    //--- optional fields---
    #[serde(borrow)]
    public_key: Option<&'a [u8]>,
}

impl AwsNitroAttestationDocument<'_> {
    fn parse_document(
        input: &mut [u8],
    ) -> Result<AwsNitroAttestationDocument<'_>, VerificationError> {
        let doc: AwsNitroAttestationDocument =
            serde_cbor::de::from_mut_slice(input).map_err(VerificationError::DocumentParseError)?;

        if doc.digest.is_empty() ||
            doc.timestamp == 0 ||
            doc.certificate.is_empty() ||
            doc.cabundle.is_empty()
        {
            return Err(VerificationError::MandatoryFieldsMissing);
        }

        if doc.digest != "SHA384" {
            return Err(VerificationError::BadDigest);
        }

        for item in &doc.cabundle {
            if item.is_empty() || item.len() > 1024 {
                return Err(VerificationError::BadCABundleItemLen);
            }
        }

        Ok(doc)
    }

    fn verify_cert_chain(&self) -> Result<(CertificateInner, u64), VerificationError> {
        let mut validity_end = u64::MAX;

        let end_idx = self.cabundle.len();
        let mut parent_cert = None;
        for i in 0..=end_idx {
            let child_cert = x509_cert::Certificate::from_der(if i == end_idx {
                self.certificate
            } else {
                self.cabundle[i]
            })
            .map_err(|e| VerificationError::CertificateParseError(e.into()))?;

            let not_after =
                child_cert.tbs_certificate.validity.not_after.to_unix_duration().as_secs();

            if not_after < validity_end {
                validity_end = not_after;
            }

            if let Some(parent_cert) = parent_cert {
                verify_x509_parent(&child_cert, &parent_cert)?;
            }

            parent_cert = Some(child_cert);
        }
        Ok((parent_cert.unwrap(), validity_end))
    }
}

/// Verify that the parent certificate is valid and that the certificate is signed by the parent.
/// NOTE: only supports ECDSA with SHA-384.
pub fn verify_x509_parent(
    cert: &CertificateInner,
    parent_cert: &CertificateInner,
) -> Result<(), VerificationError> {
    if cert.signature_algorithm.oid != der::oid::db::rfc5912::ECDSA_WITH_SHA_384 {
        return Err(VerificationError::CertificateParseError(
            der::Error::from(der::ErrorKind::OidUnknown { oid: cert.signature_algorithm.oid })
                .into(),
        ));
    }

    let parent_verifying_key =
        from_public_key_info(&parent_cert.tbs_certificate.subject_public_key_info)
            .map_err(VerificationError::CertificateParseError)?;

    let msg_to_verify = cert
        .tbs_certificate
        .to_der()
        .map_err(|e| VerificationError::CertificateParseError(e.into()))?;

    let signature_bytes =
        cert.signature.as_bytes().ok_or_else(|| VerificationError::InvalidSignature)?;

    let signature = ecdsa::Signature::from_der(signature_bytes)
        .map_err(VerificationError::CertificateSignatureError)?;

    parent_verifying_key
        .verify(&msg_to_verify, &signature)
        .map_err(VerificationError::CertificateSignatureError)?;

    Ok(())
}

pub struct ValidationResult {
    pub root_cert_hash: B256,
    pub tee_signing_key: Address,
    pub validity_window_end: u64,
    pub pcr_0: [u8; 48],
    pub pcr_1: [u8; 48],
    pub pcr_2: [u8; 48],
}

impl ValidationResult {
    pub fn data_hash(&self) -> B256 {
        let mut buffer = [0; 176];
        buffer[0..32].copy_from_slice(self.root_cert_hash.as_slice());
        buffer[32..80].copy_from_slice(&self.pcr_0);
        buffer[80..128].copy_from_slice(&self.pcr_1);
        buffer[128..176].copy_from_slice(&self.pcr_2);
        keccak256(&buffer).into()
    }
    pub fn public_data(&self) -> [u8; 60] {
        let mut buffer = [0; 60];
        buffer[0..32].copy_from_slice(self.data_hash().as_slice());
        buffer[32..40].copy_from_slice(&self.validity_window_end.to_be_bytes());
        buffer[40..60].copy_from_slice(self.tee_signing_key.as_slice());
        buffer
    }
    pub fn calldata(&self) -> [u8; 64] {
        let mut buffer = [0; 64];
        buffer[24..32].copy_from_slice(&self.validity_window_end.to_be_bytes());
        buffer[44..64].copy_from_slice(self.tee_signing_key.as_slice());
        buffer
    }
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
pub fn verify_aws_nitro_attestation(input: &[u8]) -> Result<ValidationResult, VerificationError> {
    let mut input = input.to_vec();
    let cose_sign1 =
        CoseSign1::from_bytes(&mut input).map_err(VerificationError::CoseSign1ParseError)?;

    let mut payload_data = cose_sign1.payload.to_vec();
    let doc = AwsNitroAttestationDocument::parse_document(&mut payload_data)?;
    let (cert, validity_window_end) = doc.verify_cert_chain()?;

    let pub_key = from_public_key_info(&cert.tbs_certificate.subject_public_key_info)
        .map_err(VerificationError::CertificateParseError)?;

    cose_sign1.verify_signature(&pub_key).map_err(VerificationError::CoseSign1ParseError)?;

    let pub_key = doc.public_key.ok_or(VerificationError::PublicKeyMissing)?;

    // pub key comes with a recovery byte suffix https://github.com/ethereum/go-ethereum/blob/c87b856c1a7daff56b46be70cdb7092adc519b7c/crypto/crypto.go#L40
    if pub_key.len() != 65 || pub_key[0] != 0x04 {
        return Err(VerificationError::BadPublicKeyLen);
    }

    Ok(ValidationResult {
        root_cert_hash: keccak256(doc.cabundle.first().unwrap_or(&doc.certificate)).into(),
        // exclude the leading 0x04 byte prefix
        tee_signing_key: Address::from_raw_public_key(&pub_key[1..]),
        validity_window_end,
        pcr_0: doc.pcrs.pcr_0.try_into().map_err(|_| VerificationError::BadPCRValue)?,
        pcr_1: doc.pcrs.pcr_1.try_into().map_err(|_| VerificationError::BadPCRValue)?,
        pcr_2: doc.pcrs.pcr_2.try_into().map_err(|_| VerificationError::BadPCRValue)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_aws_nitro_attestation_empty_pub_key() {
        let doc_cbor = include_bytes!("testdata/att_doc_sample.bin");
        let res = verify_aws_nitro_attestation(doc_cbor);
        //all validation passes, but it attests to no pub key
        assert!(matches!(res, Err(VerificationError::PublicKeyMissing)));
    }

    #[test]
    fn test_verify_aws_nitro_attestation_with_pub_key() {
        let doc_hex = include_bytes!("testdata/att_doc_sample_2.hex");
        let doc_cbor = hex::decode(doc_hex).unwrap();

        let res = verify_aws_nitro_attestation(&doc_cbor).unwrap();
        let pub_key = &hex::decode("040697cfa9437ccd8db7b2f2ff47dee17a5269b0e8600b6a8334339f28dddae716edcc41ebf70dec757d0ee9fa55448bd01b98fd7cf1676ad82f7b60e04b72cb36").unwrap();
        assert_eq!(res.tee_signing_key, Address::from_raw_public_key(&pub_key[1..]));
        assert_eq!(res.validity_window_end, 1748520753);
    }
}
