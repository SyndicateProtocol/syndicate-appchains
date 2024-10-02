package types

type BatcherTransaction struct {
	BlockHash        string `json:"blockHash"`
	BlockNumber      string `json:"blockNumber"`
	From             string `json:"from"`
	To               string `json:"to"`
	Input            string `json:"input"`
	Hash             string `json:"hash"`
	Gas              string `json:"gas"`
	GasPrice         string `json:"gasPrice"`
	Nonce            string `json:"nonce"`
	TransactionIndex string `json:"transactionIndex"`
	Value            string `json:"value"`
	Type             string `json:"type"`
	V                string `json:"v"`
	R                string `json:"r"`
	S                string `json:"s"`
}

func NewBatcherTransaction(blockHash, blockNumber, from, to, input string) *BatcherTransaction {
	return &BatcherTransaction{
		BlockHash:        blockHash,
		BlockNumber:      blockNumber,
		From:             from,
		To:               to,
		Input:            input,
		Hash:             "0x0000000000000000000000000000000000000000000000000000000000000000",
		Gas:              "0x0",
		GasPrice:         "0x0",
		Nonce:            "0x0",
		TransactionIndex: "0x0",
		Value:            "0x0",
		Type:             "0x0",
		V:                "0x0",
		R:                "0x0",
		S:                "0x0",
	}
}
