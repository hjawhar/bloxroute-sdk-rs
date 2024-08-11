pub mod mock;

#[cfg(test)]
mod tests {
    use crate::mock::{BLOCK_DATA, TX_TYPE_1559_1, TX_TYPE_1559_2};
    use bloxroute_sdk::models::{
        block::BloxrouteBlock, transaction::BloxrouteTransaction, BloxrouteGeneric,
    };

    #[tokio::test]
    async fn destruct_blocks() {
        let response_json: BloxrouteGeneric<BloxrouteBlock> =
            serde_json::from_str(BLOCK_DATA).unwrap();
        assert_eq!(
            response_json.params.result.header.miner,
            "0x388c818ca8b9251b393131c08a736a67ccb19297"
        );
    }

    #[tokio::test]
    async fn destruct_transactions_1() {
        let response_json: BloxrouteGeneric<BloxrouteTransaction> =
            serde_json::from_str(TX_TYPE_1559_1).unwrap();
        assert_eq!(
            response_json.params.result.txContents.unwrap().s.unwrap(),
            "0x5ca022fabdbfbda632c037cd22ed0e63627b7de4b8268aed8d196929687fe07e"
        );
    }

    #[tokio::test]
    async fn destruct_transactions_2() {
        let response_json: BloxrouteGeneric<BloxrouteTransaction> =
            serde_json::from_str(TX_TYPE_1559_2).unwrap();
        assert_eq!(
            response_json.params.result.txContents.unwrap().r.unwrap(),
            "0x259a397807739bc3a0531ceb51b6af58b8302417f5531c8bf4d02b9d16c2d3a9"
        );
    }
}
