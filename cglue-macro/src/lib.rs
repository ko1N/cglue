extern crate proc_macro;

mod gen;
mod util;

use gen::generics::{GenericCastType, GenericType};
use gen::trait_groups::*;
use proc_macro::TokenStream;
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::*;

#[proc_macro_attribute]
pub fn int_result(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn no_int_result(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn wrap_with(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn wrap_with_obj(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn wrap_with_group(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn return_wrap(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn cglue_trait_group(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as TraitGroup);
    args.create_group().into()
}

#[proc_macro]
pub fn cglue_impl_group(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as TraitGroupImpl);
    args.implement_group().into()
}

#[proc_macro]
pub fn group_obj(args: TokenStream) -> TokenStream {
    let crate_path = crate::util::crate_path();

    let GenericCastType {
        ident,
        target:
            GenericType {
                path,
                target,
                generics,
                ..
            },
    } = parse_macro_input!(args as GenericCastType);

    let gen = quote! {
        #crate_path::trait_group::Opaquable::into_opaque(#path #target :: <#generics>::from(#ident))
    };

    gen.into()
}

#[proc_macro]
pub fn cast(args: TokenStream) -> TokenStream {
    let cast = parse_macro_input!(args as TraitCastGroup);
    cast.cast_group(CastType::Cast).into()
}

#[proc_macro]
pub fn as_ref(args: TokenStream) -> TokenStream {
    let cast = parse_macro_input!(args as TraitCastGroup);
    cast.cast_group(CastType::AsRef).into()
}

#[proc_macro]
pub fn as_mut(args: TokenStream) -> TokenStream {
    let cast = parse_macro_input!(args as TraitCastGroup);
    cast.cast_group(CastType::AsMut).into()
}

#[proc_macro]
pub fn into(args: TokenStream) -> TokenStream {
    let cast = parse_macro_input!(args as TraitCastGroup);
    cast.cast_group(CastType::Into).into()
}

#[proc_macro]
pub fn check(args: TokenStream) -> TokenStream {
    let cast = parse_macro_input!(args as TraitCastGroup);
    cast.cast_group(CastType::OnlyCheck).into()
}

#[proc_macro]
pub fn trait_obj(args: TokenStream) -> TokenStream {
    let crate_path = crate::util::crate_path();

    let GenericCastType {
        ident,
        target:
            GenericType {
                path,
                target,
                generics,
                ..
            },
    } = parse_macro_input!(args as GenericCastType);

    let target = format_ident!("CGlueBase{}", target.to_token_stream().to_string());

    let gen = quote! {
        #crate_path::trait_group::Opaquable::into_opaque(#path #target :: <#generics>::from(#ident))
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn cglue_trait(_args: TokenStream, input: TokenStream) -> TokenStream {
    let tr = parse_macro_input!(input as ItemTrait);

    let trait_def = gen::traits::gen_trait(&tr);

    let gen = quote! {
        #tr
        #trait_def
    };

    gen.into()
}
