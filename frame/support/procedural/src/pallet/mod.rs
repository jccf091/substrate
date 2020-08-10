// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Implementation for pallet attribute macro.
//!
//! General workflow:
//! 1 - parse all pallet attributes:
//!   This step remove all attributes `#[pallet::*]` from the ItemMod and build the `Def` struct
//!   which holds the ItemMod without `#[pallet::*]` and information given by those attributes
//! 2 - expand from the parsed information
//!   This step will modify the ItemMod by adding some derive attributes or phantom data variants
//!   to user defineds types. And also crate new types and implement block.

mod parse;
mod expand;

use proc_macro2::Span;
pub use parse::Def;

pub fn pallet(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream
) -> proc_macro::TokenStream {
	let attr = syn::parse::<syn::Ident>(attr)
		.map_err(|e| {
			let error_msg = "Invalid pallet macro call: expect `#[frame_support::pallet($IDENT)]`";
			let mut err = syn::Error::new(Span::call_site(), error_msg);
			err.combine(e);
			err
		});

	let attr = match attr {
		Ok(attr) => attr,
		Err(err) => return err.to_compile_error().into(),
	};

	let item = syn::parse_macro_input!(item as syn::ItemMod);
	match parse::Def::try_from(attr, item) {
		Ok(def) => expand::expand(def).into(),
		Err(e) => e.to_compile_error().into(),
	}
}

// TODO TODO: genesisConfig: and proof of concept is done!!!
// TODO TODO: (and maybe CallDef just for ppl)
