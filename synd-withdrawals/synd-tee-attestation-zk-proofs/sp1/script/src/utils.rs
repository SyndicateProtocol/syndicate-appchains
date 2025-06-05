use std::fs;
use x509_cert::der::{DecodePem, Encode};

/// Reads and decodes the attestation document and root certificate.
///
/// # Arguments
///
/// * `att_doc_path` - Path to the hex-encoded attestation document.
/// * `root_cert_path` - Path to the PEM-encoded root certificate.
///
/// # Returns
///
/// A tuple containing the CBOR-encoded attestation document and the DER-encoded root certificate.
///
/// # Panics
///
/// Panics if file reading or decoding fails.
pub fn read_and_decode_attestation_docs(
    att_doc_path: &str,
    root_cert_path: &str,
) -> (Vec<u8>, Vec<u8>) {
    println!("reading attestation document from: {}", att_doc_path);
    println!("reading root certificate from: {}", root_cert_path);

    let hex_doc = fs::read(att_doc_path).expect("Failed to read attestation document");
    let pem_root_cert = fs::read(root_cert_path).expect("Failed to read root certificate");

    // TODO SEQ-999: The expected doc is currently hex encoded, need to decide if we want the input
    // to be hex or bin
    let cbor_encoded_attestation_document =
        hex::decode(hex_doc).expect("Failed to decode attestation document from hex");

    let der_encoded_root_cert = x509_cert::Certificate::from_pem(&pem_root_cert)
        .expect("Failed to parse PEM root certificate")
        .to_der()
        .expect("Failed to convert root certificate to DER");

    (cbor_encoded_attestation_document, der_encoded_root_cert)
}
