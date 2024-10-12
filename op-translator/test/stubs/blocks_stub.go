package stubs

import "github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"

var MetabasedBlock0xE7309D = types.Block{
	"hash":   "0x07003531c4a10122c5016a9e0f08e939b832cbc386c1ba9924defbe39a1a09ca",
	"number": "0xe7309d",
	"transactions": []string{
		"0x6ddc2c0a29cb87f250394740b6c18c281a7d939882db0ac7a476a07316fe6595",
		"0x2b73f561b26f0c1363e9f3793728dc12f93166ec6307fdbcbacf0d814d66f30c",
		"0x9670e384a05d68bdef658d27d84e418b60952c7cb5928b500859c543df92f822",
		"0x903687407ca69d00adbac0f8a29b503df4cbb6a6609b2e6875ae4024996b3e7a",
		"0xc5eec37b7d3b9002afd24af8799f572a5f522640a703fcc9431c7e8a217eea4d",
		"0x3dcea9dde44bfe75b08e0d9b3f447a8e0fe1bc716d9aeb49c0ba3d48cf2817b5",
		"0x1e58ed61eb030c3e68bea3fd98dfbcf0929ec8f52c81ddf1060b6f132acc19ca",
		"0x16ea244eea5470cf1fba52c6f6b6036647070afd3c4afac398a8c8fcf3de87f8",
		"0xbe3a0ae883a9498992e4fde4643faf1a1231e0115578aaccfff02e669c72091a",
		"0xc7a84ff35c8dfa9fd584849237bfd196599a5e8a446883268998bac40eeafa62",
		"0xd317432b1c017f207d6f9db2592a22543bca283c44054e19ff36ae9cae5a9724",
		"0xcae3c01d862cf742eba2a2c52cd399e422d67209a6e9ed39f34f431c37070a74",
		"0xb05a63b7a28d99df7aec5add968040e37135927130044d3b054726a3b02033a8",
		"0x18eb8e3806e8eb51bda25e8ee30fdeef739e18ef4395bfb18d7ff82b272c9a72",
		"0x95e9fa55097d5d7add85fa0ea12c6e36a357a583146f4730a2e92b3ea3477ede",
		"0xf8911dd428c80c4c735c5f17edbdc5361403c2646c23f862d8a7478a591bd0a1",
		"0xfc5f2e2e83d1efbfda141b10904d4d3d679b43cdf723a52a44f4fda3a54cd5c5",
		"0x37e0c25ccc157913d92d3dbeae60f068c1c285c2d4952bda0e03dfb55022bfac",
		"0xd5634946446842806ecdc504dc8a38d14db21d9fc225eea56b3136d4089c03f4",
	},
}

var ExpectedBlock0xE7309D = types.Block{
	"hash":   "0x5214c19f0635af3e8c98ea12e3748d2cd8c20f933aa46b5de778f8a1ea6075c4",
	"number": "0xe730a8",
	"transactions": append(TransactionsBlock0xe730a8Full,
		map[string]any{
			"blockNumber":          "0xe730a8",
			"hash":                 "0xc317b5a50b257acf42703cfbe6ed1d0086fd882c197d61bea4f1f287287a8195",
			"from":                 "0x0000000000000000000000000000000000000123",
			"to":                   "0x0000000000000000000000000000000000000123",
			"input":                "0x0012b10845ba7ec0740be0cee0cb9f30800000000000587801004800b7ff00f845a0000000000000000000000000000000000000000000000000000000000000000080a0000000000000000000000000000000000000000000000000000000000000000080c0010000ffffacae043e01",
			"nonce":                "0x0",
			"gasPrice":             "0x0",
			"r":                    "0xc913bee429cdb4e87bb93f7764c3cf27056b762b0a0ddfe0f18c1b9655d25491",
			"s":                    "0x3c0ee2abc9492fd47ff479db361db374e2d7a7311aa5253f2a4d8648bf224ebf",
			"v":                    "0x0",
			"value":                "0x0",
			"gas":                  "0x0",
			"transactionIndex":     "0x0",
			"type":                 "0x2",
			"maxFeePerGas":         "0x0",
			"maxPriorityFeePerGas": "0x0",
		}),
}
