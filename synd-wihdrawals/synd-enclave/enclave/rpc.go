package enclave

import (
	"context"

	"github.com/ethereum/go-ethereum/common/hexutil"
)

const Namespace = "enclave"

type RPC interface {
	SignerPublicKey(ctx context.Context) (hexutil.Bytes, error)
	SignerAttestation(ctx context.Context) (hexutil.Bytes, error)
	DecryptionPublicKey(ctx context.Context) (hexutil.Bytes, error)
	DecryptionAttestation(ctx context.Context) (hexutil.Bytes, error)
	EncryptedSignerKey(ctx context.Context, attestation hexutil.Bytes) (hexutil.Bytes, error)
	SetSignerKey(ctx context.Context, encrypted hexutil.Bytes) error
	VerifySequencingChain(ctx context.Context, input TEEInput) (TEEOutput, error)
	VerifyAppchain(ctx context.Context, input TEEInput) (TEEOutput, error)
}
