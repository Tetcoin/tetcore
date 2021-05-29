# System Module

The System module provides low-level access to core types and cross-cutting utilities.
It acts as the base layer for other nobles to interact with the Tetcore framework components.

- [`system::Trait`](https://docs.rs/fabric-system/latest/fabric_system/trait.Trait.html)

## Overview

The System module defines the core data types used in a Tetcore runtime.
It also provides several utility functions (see [`Module`](https://docs.rs/fabric-system/latest/fabric_system/struct.Module.html)) for other FABRIC nobles.

In addition, it manages the storage items for extrinsics data, indexes, event records, and digest items,
among other things that support the execution of the current block.

It also handles low-level tasks like depositing logs, basic set up and take down of
temporary storage entries, and access to previous block hashes.

## Interface

### Dispatchable Functions

The System module does not implement any dispatchable functions.

### Public Functions

See the [`Module`](https://docs.rs/fabric-system/latest/fabric_system/struct.Module.html) struct for details of publicly available functions.

### Signed Extensions

The System module defines the following extensions:

  - [`CheckWeight`]: Checks the weight and length of the block and ensure that it does not
    exceed the limits.
  - [`CheckNonce`]: Checks the nonce of the transaction. Contains a single payload of type
    `T::Index`.
  - [`CheckEra`]: Checks the era of the transaction. Contains a single payload of type `Era`.
  - [`CheckGenesis`]: Checks the provided genesis hash of the transaction. Must be a part of the
    signed payload of the transaction.
  - [`CheckSpecVersion`]: Checks that the runtime version is the same as the one used to sign the
    transaction.
  - [`CheckTxVersion`]: Checks that the transaction version is the same as the one used to sign the
    transaction.

Lookup the runtime aggregator file (e.g. `node/runtime`) to see the full list of signed
extensions included in a chain.

## Usage

### Prerequisites

Import the System module and derive your module's configuration trait from the system trait.

### Example - Get extrinsic count and parent hash for the current block

```rust
use fabric_support::{decl_module, dispatch};
use fabric_system::{self as system, ensure_signed};

pub trait Config: system::Config {}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 0]
		pub fn system_module_example(origin) -> dispatch::DispatchResult {
			let _sender = ensure_signed(origin)?;
			let _extrinsic_count = <system::Module<T>>::extrinsic_count();
			let _parent_hash = <system::Module<T>>::parent_hash();
			Ok(())
		}
	}
}
```

License: Apache-2.0