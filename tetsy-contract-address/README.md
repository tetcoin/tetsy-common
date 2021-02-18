# Contract address

Provides a function to create a vapory contract address.

## Examples

Create a vapory address from sender and nonce.

```rust
use tetsy_contract_address::{
	Address, U256, TetsyContractAddress
};
use std::str::FromStr;

let sender = Address::from_str("0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6").unwrap();
let tetsy_contract_address = TetsyContractAddress::from_sender_and_nonce(&sender, &U256::zero());
```
