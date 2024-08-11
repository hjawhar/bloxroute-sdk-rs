# Bloxroute Rust SDK

## Prerequisites

Before using the bloXroute Rust SDK, make sure you have the following prerequisites:

- bloXroute credentials: you need to sign up for an [account][account] and the ["Authorization"][authorization] header from the Account Portal.
- Rust programming language: you should have Rust installed on your system. The bloXroute Rust SDK is compatible with Rust versions 1.80 and above.

[account]: https://portal.bloxroute.com/register
[authorization]: https://docs.bloXroute.com/apis/authorization-headers

## Supported services

- Streams
    - [X] newTxs
    - [X] pendingTxs
    - [X] newBlocks
    - [X] bdnBlocks
    - [ ] transactionStatus
    - [ ] txReceipts

## Implementation:
<p>Example using Tokio can be found: <a href="https://github.com/hjawhar/bloxroute-sdk-rs/tree/master/examples/tokio">here</a>
</p>


## Note
<p>
The SDK is still experimental and in progress
</p>