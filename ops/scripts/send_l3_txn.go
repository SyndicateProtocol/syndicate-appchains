package main

import (
	"context"
	"encoding/hex"
	"fmt"
	"log"
	"math/big"
	"os"
	"strconv"
	"strings"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/joho/godotenv"
)

func main() {
	// Load environment variables from .env file
	if err := godotenv.Load(); err != nil {
		log.Fatalf("Error loading .env file: %v", err)
	}

	// Get environment variables
	sequencingRPCUrl := os.Getenv("SEQUENCING_RPC_URL")
	sequencingContractAddress := common.HexToAddress(os.Getenv("SEQUENCING_CONTRACT_ADDRESS"))
	l3ChainIDString := os.Getenv("L3_CHAIN_ID")
	sequencingChainIDString := os.Getenv("SEQUENCING_CHAIN_ID")
	l3PrivateKey := os.Getenv("PRIVATE_KEY_L3")
	stratosPrivateKey := os.Getenv("PRIVATE_KEY_STRATOS")

	// Define the RPC URL and connect to the client
	client, err := ethclient.Dial(sequencingRPCUrl)
	if err != nil {
		log.Fatalf("Failed to connect to the Ethereum client: %v", err)
	}

	// Step 1: Define and sign the raw transaction to be passed as input data
	// Parse chain IDs from environment variables
	l3ChainID, err := strconv.ParseInt(l3ChainIDString, 10, 64)
	if err != nil {
		log.Fatalf("Failed to parse l3ChainIDString: %v", err)
	}
	sequencingChainID, err := strconv.ParseInt(sequencingChainIDString, 10, 64)
	if err != nil {
		log.Fatalf("Failed to parse sequencingChainIDString: %v", err)
	}

	// L3 Transaction parameters
	l3ChainIDBig := big.NewInt(l3ChainID)
	toAddress := common.HexToAddress("0x52A4380F691E71ff0015352AB1a450a1dfb689b9")

	// Create a new L3 raw transaction
	rawTx := types.NewTx(&types.DynamicFeeTx{
		ChainID:   l3ChainIDBig,
		Nonce:     uint64(3), //nolint:mnd // test value
		To:        &toAddress,
		Value:     big.NewInt(1e18), //nolint:mnd // test value
		Gas:       uint64(50000),    //nolint:mnd // test value
		Data:      []byte{},
		GasFeeCap: big.NewInt(10e9), //nolint:mnd // test value // maxFeePerGas
		GasTipCap: big.NewInt(10e9), //nolint:mnd // test value // maxPriorityFeePerGas
	})

	// Load the private key for signing from environment variable
	privateKeyL3, err := crypto.HexToECDSA(l3PrivateKey[2:])
	if err != nil {
		log.Fatalf("Invalid private key: %v", err)
	}

	// Sign the L3 raw transaction
	signedRawTx, err := types.SignTx(rawTx, types.NewLondonSigner(l3ChainIDBig), privateKeyL3)
	if err != nil {
		log.Fatalf("Failed to sign raw transaction: %v", err)
	}

	// Encode the signed L3 transaction as input for the sequencing contract
	signedTxData, err := signedRawTx.MarshalBinary()
	if err != nil {
		log.Fatalf("Failed to encode signed transaction: %v", err)
	}

	fmt.Println("signedTxData", hex.EncodeToString(signedTxData))

	// ABI encode the function call with the signed transaction as input
	contractABI, err := abi.JSON(strings.NewReader(`[{"inputs":[{"internalType":"bytes","name":"transaction","type":"bytes"}],"name":"processTransaction","outputs":[],"stateMutability":"nonpayable","type":"function"}]`))
	if err != nil {
		log.Fatalf("Failed to parse ABI: %v", err)
	}

	// Pack the `processTransaction` function call with the signed transaction data
	inputData, err := contractABI.Pack("processTransaction", signedTxData)
	if err != nil {
		log.Fatalf("Failed to pack function call: %v", err)
	}

	// Step 3: Send the L2 transaction to the sequencing contract
	privateKeyStratos, err := crypto.HexToECDSA(stratosPrivateKey[2:])
	if err != nil {
		log.Fatalf("Invalid private key: %v", err)
	}
	auth, err := bind.NewKeyedTransactorWithChainID(privateKeyStratos, big.NewInt(sequencingChainID))
	if err != nil {
		log.Fatalf("Failed to create transactor: %v", err)
	}

	// Send the transaction with the packed data to the sequencing contract
	gasPriceL2 := big.NewInt(10e9) //nolint:mnd // test value
	gasLimitL2 := uint64(5000000)  //nolint:mnd // test value
	nonceL2 := uint64(1)
	tx := types.NewTransaction(nonceL2, sequencingContractAddress, big.NewInt(0), gasLimitL2, gasPriceL2, inputData)
	signedTx, err := auth.Signer(auth.From, tx)
	log.Printf("Hex encoded signed tx data: %s", hex.EncodeToString(signedTxData))
	if err != nil {
		log.Fatalf("Failed to sign transaction: %v", err)
	}

	err = client.SendTransaction(context.Background(), signedTx)
	if err != nil {
		log.Fatalf("Failed to send transaction: %v", err)
	}

	fmt.Printf("Transaction sent to contract! Hash: %s\n", signedTx.Hash().Hex())
}
