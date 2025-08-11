package wavmio

import (
	"context"
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto/kzg4844"
	"github.com/offchainlabs/nitro/arbutil"
	"github.com/offchainlabs/nitro/daprovider/das/dastree"
	"github.com/offchainlabs/nitro/daprovider/das/dasutil"
	"github.com/offchainlabs/nitro/eigenda"
)

type PreimageDASReader struct {
	Wavm *Wavm
}

func (dasReader *PreimageDASReader) GetByHash(ctx context.Context, hash common.Hash) ([]byte, error) {
	oracle := func(hash common.Hash) ([]byte, error) {
		return dasReader.Wavm.ResolveTypedPreimage(arbutil.Keccak256PreimageType, hash)
	}
	return dastree.Content(hash, oracle)
}

func (dasReader *PreimageDASReader) GetKeysetByHash(ctx context.Context, hash common.Hash) ([]byte, error) {
	return dasReader.GetByHash(ctx, hash)
}

func (dasReader *PreimageDASReader) ExpirationPolicy(ctx context.Context) (dasutil.ExpirationPolicy, error) {
	return dasutil.DiscardImmediately, nil
}

type BlobPreimageReader struct {
	Wavm *Wavm
}

func (r *BlobPreimageReader) GetBlobs(
	ctx context.Context,
	batchBlockHash common.Hash,
	versionedHashes []common.Hash,
) ([]kzg4844.Blob, error) {
	var blobs []kzg4844.Blob
	for _, h := range versionedHashes {
		var blob kzg4844.Blob
		preimage, err := r.Wavm.ResolveTypedPreimage(arbutil.EthVersionedHashPreimageType, h)
		if err != nil {
			return nil, err
		}
		if len(preimage) != len(blob) {
			return nil, fmt.Errorf("for blob %v got back preimage of length %v but expected blob length %v", h, len(preimage), len(blob))
		}
		copy(blob[:], preimage)
		blobs = append(blobs, blob)
	}
	return blobs, nil
}

func (r *BlobPreimageReader) Initialize(ctx context.Context) error {
	return nil
}

type EigenDAPreimageReader struct {
	Wavm *Wavm
}

// QueryBlob returns the blob for the given cert from the preimage oracle using the hash of the
// certificate kzg commitment for identifying the preimage.
func (dasReader *EigenDAPreimageReader) QueryBlob(ctx context.Context, cert *eigenda.EigenDAV1Cert, domain string) ([]byte, error) {
	hash, err := cert.PreimageHash()
	if err != nil {
		return nil, err
	}

	preimage, err := dasReader.Wavm.ResolveTypedPreimage(arbutil.EigenDaPreimageType, *hash)
	if err != nil {
		return nil, err
	}

	decodedBlob, err := eigenda.GenericDecodeBlob(preimage)
	if err != nil {
		return nil, err
	}

	return decodedBlob, nil
}
