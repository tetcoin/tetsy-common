// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Tests for tetsy-scale-info feature of tetsy-primitive-types.

use tetsy_primitive_types::{H256, U256};
use tetsy_scale_info::{build::Fields, Path, Type, TypeInfo};

#[test]
fn u256_scale_info() {
	let r#type =
		Type::builder().path(Path::new("U256", "tetsy_primitive_types")).composite(Fields::unnamed().field_of::<[u64; 4]>());

	assert_eq!(U256::type_info(), r#type.into());
}

#[test]
fn h256_scale_info() {
	let r#type =
		Type::builder().path(Path::new("H256", "tetsy_primitive_types")).composite(Fields::unnamed().field_of::<[u8; 32]>());

	assert_eq!(H256::type_info(), r#type.into());
}
