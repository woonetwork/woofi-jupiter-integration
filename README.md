# woofi-jupiter-integration
Solana Jupiter integration for WOOFi Swap.

This module implements the `Amm` trait defined [here](https://github.com/jup-ag/rust-amm-implementation).

To test, simply run:

```
cargo test -- --nocapture


// If test in devnet
//cargo test --features "devnet" -- --nocapture
```

This will print out a quote for selling 0.01 SOL against the Woofi devnet SOL/USDC market. Sample output:
```
Getting quote for selling 0.01 SOL
result.out_amount:2347347
result.in_amount:10000000
result.fee_amount:72599
result.fee_mint:4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU
Getting quote for buying SOL using 200 USDC
result.out_amount:801670727
result.in_amount:200000000
result.fee_amount:6000000
result.fee_mint:4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU
test test_jupiter_quote ... ok
```


