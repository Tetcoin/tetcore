// This file is part of Tetcore.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

use crate::noble::Def;
use fabric_support_procedural_tools::clean_type_string;
use quote::ToTokens;

struct ConstDef {
	/// Name of the associated type.
	pub ident: syn::Ident,
	/// The type in Get, e.g. `u32` in `type Foo: Get<u32>;`, but `Self` is replaced by `T`
	pub type_: syn::Type,
	/// The doc associated
	pub doc: Vec<syn::Lit>,
	/// default_byte implementation
	pub default_byte_impl: proc_macro2::TokenStream,
}

/// * Impl fn module_constant_metadata for noble.
pub fn expand_constants(def: &mut Def) -> proc_macro2::TokenStream {
	let fabric_support = &def.fabric_support;
	let type_impl_gen = &def.type_impl_generics(proc_macro2::Span::call_site());
	let type_decl_gen = &def.type_decl_generics(proc_macro2::Span::call_site());
	let type_use_gen = &def.type_use_generics(proc_macro2::Span::call_site());
	let noble_ident = &def.noble_struct.noble;

	let mut where_clauses = vec![&def.config.where_clause];
	where_clauses.extend(def.extra_constants.iter().map(|d| &d.where_clause));
	let completed_where_clause = super::merge_where_clauses(&where_clauses);

	let config_consts = def.config.consts_metadata.iter().map(|const_| {
		let ident = &const_.ident;
		let const_type = &const_.type_;

		ConstDef {
			ident: const_.ident.clone(),
			type_: const_.type_.clone(),
			doc: const_.doc.clone(),
			default_byte_impl: quote::quote!(
				let value = <T::#ident as #fabric_support::traits::Get<#const_type>>::get();
				#fabric_support::codec::Encode::encode(&value)
			),
		}
	});

	let extra_consts = def.extra_constants.iter().flat_map(|d| &d.extra_constants).map(|const_| {
		let ident = &const_.ident;

		ConstDef {
			ident: const_.ident.clone(),
			type_: const_.type_.clone(),
			doc: const_.doc.clone(),
			default_byte_impl: quote::quote!(
				let value = <Noble<#type_use_gen>>::#ident();
				#fabric_support::codec::Encode::encode(&value)
			),
		}
	});

	let consts = config_consts.chain(extra_consts)
		.map(|const_| {
			let const_type = &const_.type_;
			let const_type_str = clean_type_string(&const_type.to_token_stream().to_string());
			let ident = &const_.ident;
			let ident_str = format!("{}", ident);
			let doc = const_.doc.clone().into_iter();
			let default_byte_impl = &const_.default_byte_impl;
			let default_byte_getter = syn::Ident::new(
				&format!("{}DefaultByteGetter", ident),
				ident.span()
			);

			quote::quote!({
				#[allow(non_upper_case_types)]
				#[allow(non_camel_case_types)]
				struct #default_byte_getter<#type_decl_gen>(
					#fabric_support::tetcore_std::marker::PhantomData<(#type_use_gen)>
				);

				impl<#type_impl_gen> #fabric_support::dispatch::DefaultByte for
					#default_byte_getter<#type_use_gen>
					#completed_where_clause
				{
					fn default_byte(&self) -> #fabric_support::tetcore_std::vec::Vec<u8> {
						#default_byte_impl
					}
				}

				unsafe impl<#type_impl_gen> Send for #default_byte_getter<#type_use_gen>
					#completed_where_clause
				{}
				unsafe impl<#type_impl_gen> Sync for #default_byte_getter<#type_use_gen>
					#completed_where_clause
				{}

				#fabric_support::dispatch::ModuleConstantMetadata {
					name: #fabric_support::dispatch::DecodeDifferent::Encode(#ident_str),
					ty: #fabric_support::dispatch::DecodeDifferent::Encode(#const_type_str),
					value: #fabric_support::dispatch::DecodeDifferent::Encode(
						#fabric_support::dispatch::DefaultByteGetter(
							&#default_byte_getter::<#type_use_gen>(
								#fabric_support::tetcore_std::marker::PhantomData
							)
						)
					),
					documentation: #fabric_support::dispatch::DecodeDifferent::Encode(
						&[ #( #doc ),* ]
					),
				}
			})
		});

	quote::quote!(
		impl<#type_impl_gen> #noble_ident<#type_use_gen> #completed_where_clause{

			#[doc(hidden)]
			pub fn module_constants_metadata()
				-> &'static [#fabric_support::dispatch::ModuleConstantMetadata]
			{
				&[ #( #consts ),* ]
			}
		}
	)
}
