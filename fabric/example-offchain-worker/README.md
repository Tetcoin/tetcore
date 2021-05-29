<!-- markdown-link-check-disable -->
# Offchain Worker Example Module

The Offchain Worker Example: A simple noble demonstrating
concepts, APIs and structures common to most offchain workers.

Run `cargo doc --package noble-example-offchain-worker --open` to view this module's
documentation.

- [`noble_example_offchain_worker::Trait`](./trait.Trait.html)
- [`Call`](./enum.Call.html)
- [`Module`](./struct.Module.html)


## Overview

In this example we are going to build a very simplistic, naive and definitely NOT
production-ready oracle for BTC/USD price.
Offchain Worker (OCW) will be triggered after every block, fetch the current price
and prepare either signed or unsigned transaction to feed the result back on chain.
The on-chain logic will simply aggregate the results and store last `64` values to compute
the average price.
Additional logic in OCW is put in place to prevent spamming the network with both signed
and unsigned transactions, and custom `UnsignedValidator` makes sure that there is only
one unsigned transaction floating in the network.

License: Unlicense