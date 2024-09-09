package rpcclient

type SettlementClient struct {
	Client *RPCClient
}

func NewSettlementClient(address string) (*SettlementClient, error) {
	client, err := Connect(address)
	if err != nil {
		return nil, err
	}
	return &SettlementClient{client}, nil
}
