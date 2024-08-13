# Bloxroute Rust SDK

## Prerequisites

Before using the bloXroute Rust SDK, make sure you have the following prerequisites:

- bloXroute credentials: you need to sign up for an [account][account] and the ["Authorization"][authorization] header from the Account Portal.
- Rust programming language: you should have Rust installed on your system. The bloXroute Rust SDK is compatible with Rust versions 1.80 and above.

[account]: https://portal.bloxroute.com/register
[authorization]: https://docs.bloXroute.com/apis/authorization-headers

## Supported services

- EVM
    - Streams
        - [X] newTxs
        - [X] pendingTxs
        - [X] newBlocks
        - [X] bdnBlocks
        - [ ] transactionStatus
        - [ ] txReceipts
- Solana
    - Trader API v2
        - General 
            - [ ] Get Account Balance
            - [ ] Get Rate Limit
            - [ ] Get Transaction Status
            - [ ] Get Recent Priority Fee
            - [X] Stream Priority Fee
            - [X] Stream Bundle Tip
            - [X] Submit Signed Transaction
            - [X] Submit Signed Transaction Batch
        - Openbook
            - [X] Get Markets
            - [X] Get Orderbooks
            - [X] Get Depth
            - [ ] Get Tickers
            - [ ] Get Open Orders
            - [ ] Get Unsettled
            - [ ] Create Order Transaction
            - [ ] Create Replace Transaction
            - [ ] Create Cancel Order Transaction
            - [ ] Create Settle Transaction
            - [ ] Stream Orderbooks
            - [ ] Stream Tickers
        - Jupiter
            - [ ] Get Quotes
            - [ ] Create Swap Transaction
            - [ ] Create Swap with Instructions
            - [ ] Create Route Swap
        - Raydium
            - [X] Get Quotes
            - [X] Get Pools
            - [X] Get Pool Reserves
            - [X] Create Swap Transaction
            - [X] Create Route Swap
            - [ ] Stream Pool Reserves
            - [ ] Stream Swaps
            - [ ] Stream New Raydium Pools
        - Zeta Markets
            - [ ] Stream Zeta Transactions
            - [ ] Create Zero Cross Margin Account

## Implementation:
<p>Example using Tokio can be found: <a href="https://github.com/hjawhar/bloxroute-sdk-rs/tree/master/examples/tokio">here</a></p>
<p>You can run EVM examples using the following command:</p>

```
endpoint=wss://germany.eth.blxrbdn.com/ws auth_header=YOUR_AUTH_HEADER cargo run --example evm
```

<p>You can run Solana examples using the following command:</p>

```
endpoint=wss://uk.solana.dex.blxrbdn.com/ws auth_header=YOUR_AUTH_HEADER cargo run --example solana
```

```
endpoint=https://uk.solana.dex.blxrbdn.com auth_header=YOUR_AUTH_HEADER cargo run --example solana
```


## Note
<p>
The SDK is still experimental and in progress - some breaking changes might occur while still in development!
</p>