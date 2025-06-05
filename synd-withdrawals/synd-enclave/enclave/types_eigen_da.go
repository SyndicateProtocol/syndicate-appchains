package enclave

type G1Point struct {
	X string `json:"X"` // uint256 -> string for safe JSON handling
	Y string `json:"Y"`
}

type VersionedBlobParams struct {
	MaxNumOperators uint32 `json:"maxNumOperators"`
	NumChunks       uint32 `json:"numChunks"`
	CodingRate      uint8  `json:"codingRate"`
}

type SecurityThresholds struct {
	ConfirmationThreshold uint8 `json:"confirmationThreshold"`
	AdversaryThreshold    uint8 `json:"adversaryThreshold"`
}

type QuorumBlobParam struct {
	QuorumNumber                    uint8  `json:"quorumNumber"`
	AdversaryThresholdPercentage    uint8  `json:"adversaryThresholdPercentage"`
	ConfirmationThresholdPercentage uint8  `json:"confirmationThresholdPercentage"`
	ChunkLength                     uint32 `json:"chunkLength"`
}

type BlobHeader struct {
	Commitment       G1Point           `json:"commitment"`
	DataLength       uint32            `json:"dataLength"`
	QuorumBlobParams []QuorumBlobParam `json:"quorumBlobParams"`
}

type ReducedBatchHeader struct {
	BlobHeadersRoot      [32]byte `json:"blobHeadersRoot"`
	ReferenceBlockNumber uint32   `json:"referenceBlockNumber"`
}

type BatchHeader struct {
	BlobHeadersRoot       [32]byte `json:"blobHeadersRoot"`
	QuorumNumbers         []byte   `json:"quorumNumbers"`
	SignedStakeForQuorums []byte   `json:"signedStakeForQuorums"`
	ReferenceBlockNumber  uint32   `json:"referenceBlockNumber"`
}

type BatchMetadata struct {
	BatchHeader             BatchHeader `json:"batchHeader"`
	SignatoryRecordHash     [32]byte    `json:"signatoryRecordHash"`
	ConfirmationBlockNumber uint32      `json:"confirmationBlockNumber"`
}

type BlobVerificationProof struct {
	BatchID        uint32        `json:"batchId"`
	BlobIndex      uint32        `json:"blobIndex"`
	BatchMetadata  BatchMetadata `json:"batchMetadata"`
	InclusionProof []byte        `json:"inclusionProof"`
	QuorumIndices  []byte        `json:"quorumIndices"`
}

type EigenDACert struct {
	BlobVerificationProof BlobVerificationProof `json:"blobVerificationProof"`
	BlobHeader            BlobHeader            `json:"blobHeader"`
}
