// Copyright 2021-2022, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE.md

package enclave

import (
	"bytes"
	"encoding/hex"
	"errors"
	"fmt"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave/wavmio"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/ethdb"
	"github.com/ethereum/go-ethereum/ethdb/memorydb"

	"github.com/offchainlabs/nitro/arbutil"
)

type PreimageDb struct {
	wavm  *wavmio.Wavm
	memDb *memorydb.Database
}

func (db *PreimageDb) Has(key []byte) (bool, error) {
	return false, errors.New("preimage DB doesn't support Has")
}

func (db *PreimageDb) DeleteRange(start, end []byte) error {
	return errors.New("preimage DB doesn't support DeleteRange")
}

func (db *PreimageDb) Get(key []byte) ([]byte, error) {
	res, err := db.memDb.Get(key)
	if err == nil {
		return res, nil
	}
	var hash [32]byte
	if len(key) == 32 {
		copy(hash[:], key)
	} else if len(key) == len(rawdb.CodePrefix)+32 && bytes.HasPrefix(key, rawdb.CodePrefix) {
		// Retrieving code
		copy(hash[:], key[len(rawdb.CodePrefix):])
	} else {
		return nil, fmt.Errorf("preimage DB attempted to access non-hash key %v", hex.EncodeToString(key))
	}
	return db.wavm.ResolveTypedPreimage(arbutil.Keccak256PreimageType, hash)
}

func (db *PreimageDb) Put(key []byte, value []byte) error {
	return errors.New("preimage DB doesn't support Put")
}

func (db *PreimageDb) Delete(key []byte) error {
	return errors.New("preimage DB doesn't support Delete")
}

func (db *PreimageDb) NewBatch() ethdb.Batch {
	return db.memDb.NewBatch()
}

func (db *PreimageDb) NewBatchWithSize(size int) ethdb.Batch {
	return db.memDb.NewBatchWithSize(size)
}

func (db *PreimageDb) NewIterator(prefix []byte, start []byte) ethdb.Iterator {
	panic("recording KV doesn't support NewIterator")
}

func (db *PreimageDb) Stat() (string, error) {
	return "", errors.New("preimage DB doesn't support Stat")
}

func (db *PreimageDb) Compact(start []byte, limit []byte) error {
	return nil
}

func (db *PreimageDb) Close() error {
	return db.memDb.Close()
}
