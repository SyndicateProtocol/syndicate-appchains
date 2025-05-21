package enclave

import (
	"context"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/rpc"
)

type Client struct {
	*rpc.Client
}

var _ RPC = (*Client)(nil)

func (c *Client) callContext(ctx context.Context, result interface{}, method string, args ...interface{}) error {
	return c.CallContext(ctx, result, Namespace+"_"+method, args...)
}

func (c *Client) SignerPublicKey(ctx context.Context) (hexutil.Bytes, error) {
	var result hexutil.Bytes
	return result, c.callContext(ctx, &result, "signerPublicKey")
}

func (c *Client) SignerAttestation(ctx context.Context) (hexutil.Bytes, error) {
	var result hexutil.Bytes
	return result, c.callContext(ctx, &result, "signerAttestation")
}

func (c *Client) DecryptionPublicKey(ctx context.Context) (hexutil.Bytes, error) {
	var result hexutil.Bytes
	return result, c.callContext(ctx, &result, "decryptionPublicKey")
}

func (c *Client) DecryptionAttestation(ctx context.Context) (hexutil.Bytes, error) {
	var result hexutil.Bytes
	return result, c.callContext(ctx, &result, "decryptionAttestation")
}

func (c *Client) EncryptedSignerKey(ctx context.Context, attestation hexutil.Bytes) (hexutil.Bytes, error) {
	var result hexutil.Bytes
	return result, c.callContext(ctx, &result, "encryptedSignerKey", attestation)
}

func (c *Client) SetSignerKey(ctx context.Context, encrypted hexutil.Bytes) error {
	return c.callContext(ctx, nil, "setSignerKey", encrypted)
}

func (c *Client) VerifySequencingChain(ctx context.Context, input TEEInput) (TEEOutput, error) {
	var result TEEOutput
	return result, c.callContext(ctx, &result, "verifySequencingChain", input)
}

func (c *Client) VerifyAppchain(ctx context.Context, input TEEInput) (TEEOutput, error) {
	var result TEEOutput
	return result, c.callContext(ctx, &result, "verifyAppchain", input)
}
