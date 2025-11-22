extern crate alloc;
use crate::p384::{ecdsa, ecdsa::signature::Verifier as _};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// https://tools.ietf.org/html/rfc8152#section-4.2
/// https://docs.aws.amazon.com/enclaves/latest/user/verify-root.html#validation-process
#[derive(Deserialize)]
pub struct CoseSign1<'a> {
    #[serde(borrow)]
    pub protected: &'a [u8],

    #[allow(dead_code)]
    pub unprotected: serde::de::IgnoredAny,

    // the attestation document
    #[serde(borrow)]
    pub payload: &'a [u8],

    // the COSE signature (using the public key from the attestation document's certificate)
    #[serde(borrow)]
    pub signature: &'a [u8],
}

impl<'a> CoseSign1<'a> {
    pub fn from_bytes(bytes: &'a mut [u8]) -> Result<Self, &'static str> {
        let tagged_cose_sign1: serde_cbor::tags::Tagged<Self> =
            serde_cbor::de::from_mut_slice(bytes).map_err(|_| "Deserialization error")?;

        match tagged_cose_sign1.tag {
            None | Some(18) => (), /* CBOR tag 18 is for COSE_Sign1 - https://datatracker.ietf.org/doc/html/rfc8152#section-2 */
            Some(_) => return Err("Tag error: Expected CBOR Tag 18 or no tag for CoseSign1"),
        }

        Ok(tagged_cose_sign1.value)
    }

    pub fn signed_message_bytes(&self) -> Vec<u8> {
        serde_cbor::to_vec(&CoseSig("Signature1", self.protected, Default::default(), self.payload))
            .unwrap()
    }

    pub fn verify_signature(&self, pub_key: &ecdsa::VerifyingKey) -> Result<(), &'static str> {
        let signature = ecdsa::Signature::from_bytes(self.signature.into())
            .map_err(|_| "Signature deserialization error")?;
        pub_key
            .verify(&self.signed_message_bytes(), &signature)
            .map_err(|_| "Signature verification failed")
    }
}

/// Represents the COSE_Sign1 Sig_structure to be serialized.
/// https://tools.ietf.org/html/rfc8152#section-4.4
/// For COSE_Sign1, this is an array of 4 elements:
/// [
///   context: tstr,
///   body_protected: bstr,
///   external_aad: bstr,
///   payload: bstr
/// ]
#[derive(Serialize)]
struct CoseSig<'a>(
    &'a str,
    #[serde(with = "serde_bytes")] &'a [u8],
    #[serde(with = "serde_bytes")] &'a [u8],
    #[serde(with = "serde_bytes")] &'a [u8],
);

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    // Public domain work: Pride and Prejudice by Jane Austen, taken from https://www.gutenberg.org/files/1342/1342.txt
    const TEXT: &[u8] = b"It is a truth universally acknowledged, that a single man in possession of a good fortune, must be in want of a wife.";

    #[test]
    fn test_cose_sign1() {
        // test case taken from https://github.com/awslabs/aws-nitro-enclaves-cose/blob/6064f826d551a9db0bd42e9cf928feaf272e8d17/src/sign.rs#L738

        // This output was validated against COSE-C implementation
        let bytes = &mut [
            0x84, /* Protected: {1: -35} */
            0x44, 0xA1, 0x01, 0x38, 0x22, /* Unprotected: {4: '11'} */
            0xA1, 0x04, 0x42, 0x31, 0x31, /* payload: */
            0x58, 0x75, 0x49, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x72, 0x75, 0x74,
            0x68, 0x20, 0x75, 0x6E, 0x69, 0x76, 0x65, 0x72, 0x73, 0x61, 0x6C, 0x6C, 0x79, 0x20,
            0x61, 0x63, 0x6B, 0x6E, 0x6F, 0x77, 0x6C, 0x65, 0x64, 0x67, 0x65, 0x64, 0x2C, 0x20,
            0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6E, 0x67, 0x6C, 0x65, 0x20,
            0x6D, 0x61, 0x6E, 0x20, 0x69, 0x6E, 0x20, 0x70, 0x6F, 0x73, 0x73, 0x65, 0x73, 0x73,
            0x69, 0x6F, 0x6E, 0x20, 0x6F, 0x66, 0x20, 0x61, 0x20, 0x67, 0x6F, 0x6F, 0x64, 0x20,
            0x66, 0x6F, 0x72, 0x74, 0x75, 0x6E, 0x65, 0x2C, 0x20, 0x6D, 0x75, 0x73, 0x74, 0x20,
            0x62, 0x65, 0x20, 0x69, 0x6E, 0x20, 0x77, 0x61, 0x6E, 0x74, 0x20, 0x6F, 0x66, 0x20,
            0x61, 0x20, 0x77, 0x69, 0x66, 0x65, 0x2E, /* signature - length 48 x 2 */
            0x58, 0x60, /* R: */
            0xCD, 0x42, 0xD2, 0x76, 0x32, 0xD5, 0x41, 0x4E, 0x4B, 0x54, 0x5C, 0x95, 0xFD, 0xE6,
            0xE3, 0x50, 0x5B, 0x93, 0x58, 0x0F, 0x4B, 0x77, 0x31, 0xD1, 0x4A, 0x86, 0x52, 0x31,
            0x75, 0x26, 0x6C, 0xDE, 0xB2, 0x4A, 0xFF, 0x2D, 0xE3, 0x36, 0x4E, 0x9C, 0xEE, 0xE9,
            0xF9, 0xF7, 0x95, 0xA0, 0x15, 0x15, /* S: */
            0x5B, 0xC7, 0x12, 0xAA, 0x28, 0x63, 0xE2, 0xAA, 0xF6, 0x07, 0x8A, 0x81, 0x90, 0x93,
            0xFD, 0xFC, 0x70, 0x59, 0xA3, 0xF1, 0x46, 0x7F, 0x64, 0xEC, 0x7E, 0x22, 0x1F, 0xD1,
            0x63, 0xD8, 0x0B, 0x3B, 0x55, 0x26, 0x25, 0xCF, 0x37, 0x9D, 0x1C, 0xBB, 0x9E, 0x51,
            0x38, 0xCC, 0xD0, 0x7A, 0x19, 0x31,
        ];
        let cose_doc = CoseSign1::from_bytes(bytes).unwrap();

        assert_eq!(cose_doc.payload, TEXT);
        assert_eq!(cose_doc.protected, &[0xA1, 0x01, 0x38, 0x22]);
    }

    #[test]
    fn test_sig_structure_bytes_generation() {
        let cose_sign1_obj = CoseSign1 {
            protected: &[0xA1, 0x01, 0x38, 0x22],
            unprotected: serde::de::IgnoredAny,
            payload: TEXT,
            signature: &[], // Signature not used for this test
        };

        let sig_structure = cose_sign1_obj.signed_message_bytes();

        // Expected Sig_structure from RFC8152 C.2.1 (Sign1 example)
        // (
        //   "Signature1",       // context
        //   h'A1013822',        // protected headers {1: -35} as bstr. CBOR(map) = A1013822
        //   h'',               // external_aad (empty bstr)
        //   <payload_bytes>   // payload as bstr = TEXT
        // )
        // SigStructure is: 84 6A 5369676E617475726531 44 A1013822 40 5875 <TEXT> ...

        let mut expected_bytes = vec![0x84]; // Array of 4 items

        expected_bytes.push(0x6A); // tstr len 10

        // 1. context "Signature1"
        expected_bytes.extend_from_slice(b"Signature1");
        // 2. protected_map_bytes: bstr( A1 01 38 22 )
        expected_bytes.push(0x44); // bstr len 4
        expected_bytes.extend_from_slice(&[0xA1, 0x01, 0x38, 0x22]);
        // 3. external_aad: empty bstr
        expected_bytes.push(0x40);
        // 4. payload TEXT
        expected_bytes.push(0x58); // bstr len 117 (0x75)
        expected_bytes.push(0x75);
        expected_bytes.extend_from_slice(TEXT);

        assert_eq!(sig_structure, expected_bytes);
    }
}
