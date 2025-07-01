// for testing purposes only
package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave/wavmio"
)

func main() {
	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	data := wavmio.ValidationInput{}

	if err := json.Unmarshal(input, &data); err != nil {
		panic(err)
	}

	result, err := enclave.Verify(data, nil)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%s %s\n", result.BlockHash, result.SendRoot)
}
