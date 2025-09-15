#!/bin/bash
OLD=https://verifier.direct.us-east-2.aws.testnet.syndicate.io
NEW=https://verifier.direct.us-east-2.aws.testnet.syndicate.io
cast rpc --rpc-url $NEW enclave_setSignerKey $(cast rpc --rpc-url $OLD enclave_encryptedSignerKey $(cast rpc --rpc-url $NEW enclave_decryptionAttestation))
