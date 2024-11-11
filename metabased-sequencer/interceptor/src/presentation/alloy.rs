/// Combines `n` [filler]s into one [filler].
///
/// It achieves this by nesting [`JoinFill`] providers into a binary tree structure. In this tree,
/// every node with 2 children is a [`JoinFill`] provider while every leaf node is a provider from
/// the given arguments.
///
/// [filler]: alloy::providers::fillers::TxFiller
/// [`JoinFill`]: alloy::providers::fillers::JoinFill
macro_rules! join_fill {
    ($x:expr, $($y:tt)+) => {
        alloy::providers::fillers::JoinFill::new($x, join_fill!($($y)+))
    };
    ($x:expr $(,)?) => {
        $x
    };
}

#[cfg(test)]
mod tests {
    use alloy::network::Ethereum;
    use alloy::providers::fillers::{FillerControlFlow, TxFiller};
    use alloy::providers::{Network, SendableTx};
    use alloy::rpc::types::TransactionRequest;
    use alloy::transports::TransportResult;

    #[derive(Clone, Copy, Debug)]
    pub struct DummyGasFiller;

    impl TxFiller<Ethereum> for DummyGasFiller {
        type Fillable = ();

        fn status(&self, _tx: &<Ethereum as Network>::TransactionRequest) -> FillerControlFlow {
            FillerControlFlow::Finished
        }

        fn fill_sync(&self, tx: &mut SendableTx<Ethereum>) {
            if let SendableTx::Builder(tx) = tx {
                tx.gas.replace(1);
            }
        }

        async fn prepare<P, T>(
            &self,
            _provider: &P,
            _tx: &<Ethereum as Network>::TransactionRequest,
        ) -> TransportResult<Self::Fillable> {
            Ok(())
        }

        async fn fill(
            &self,
            _to_fill: Self::Fillable,
            tx: SendableTx<Ethereum>,
        ) -> TransportResult<SendableTx<Ethereum>> {
            Ok(tx)
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct DummyChainIdFiller;

    impl TxFiller<Ethereum> for DummyChainIdFiller {
        type Fillable = ();

        fn status(&self, _tx: &<Ethereum as Network>::TransactionRequest) -> FillerControlFlow {
            FillerControlFlow::Finished
        }

        fn fill_sync(&self, tx: &mut SendableTx<Ethereum>) {
            if let SendableTx::Builder(tx) = tx {
                tx.chain_id.replace(2);
            }
        }

        async fn prepare<P, T>(
            &self,
            _provider: &P,
            _tx: &<Ethereum as Network>::TransactionRequest,
        ) -> TransportResult<Self::Fillable> {
            Ok(())
        }

        async fn fill(
            &self,
            _to_fill: Self::Fillable,
            tx: SendableTx<Ethereum>,
        ) -> TransportResult<SendableTx<Ethereum>> {
            Ok(tx)
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct DummyNonceFiller;

    impl TxFiller<Ethereum> for DummyNonceFiller {
        type Fillable = ();

        fn status(&self, _tx: &<Ethereum as Network>::TransactionRequest) -> FillerControlFlow {
            FillerControlFlow::Finished
        }

        fn fill_sync(&self, tx: &mut SendableTx<Ethereum>) {
            if let SendableTx::Builder(tx) = tx {
                tx.nonce.replace(3);
            }
        }

        async fn prepare<P, T>(
            &self,
            _provider: &P,
            _tx: &<Ethereum as Network>::TransactionRequest,
        ) -> TransportResult<Self::Fillable> {
            Ok(())
        }

        async fn fill(
            &self,
            _to_fill: Self::Fillable,
            tx: SendableTx<Ethereum>,
        ) -> TransportResult<SendableTx<Ethereum>> {
            Ok(tx)
        }
    }

    #[test]
    fn test_join_fill_creates_filler_that_applies_all_given_fillers_on_transaction() {
        let tx_request = TransactionRequest::default();
        let mut actual_tx = SendableTx::Builder(tx_request);

        let filler = join_fill!(DummyChainIdFiller, DummyGasFiller, DummyNonceFiller);
        filler.fill_sync(&mut actual_tx);

        let actual_tx = actual_tx.as_builder().unwrap();
        let expected_tx = &TransactionRequest {
            nonce: Some(3),
            chain_id: Some(2),
            gas: Some(1),
            ..Default::default()
        };

        assert_eq!(actual_tx, expected_tx);
    }
}
