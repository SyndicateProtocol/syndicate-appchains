package rpc

type SequencingClient struct {
	Client *RPCClient
}

func NewSequencingClient(address string) (*SequencingClient, error) {
	client, err := Connect(address)
	if err != nil {
		return nil, err
	}
	return &SequencingClient{client}, nil
}
