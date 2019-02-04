#![recursion_limit="128"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(FromStr, attributes(from_str))]
pub fn enum_str_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_enum_str_derive(&ast)
}

struct Variant<'a> {
    pub name: &'a syn::Ident,
    pub literal: String,
}

fn impl_enum_str_derive(ast: &syn::DeriveInput) -> TokenStream {
    let mut variants = Vec::new();
    if let syn::Data::Enum(ref enum_data) = ast.data {
        for variant in &enum_data.variants {
            let mut literal = None;

            for attr in &variant.attrs {
                if let Some(syn::Meta::NameValue(name_value)) = attr.interpret_meta() {
                    if let syn::Lit::Str(lit_str) = name_value.lit {
                        literal = Some(lit_str.value());
                    }
                }
            }

            match literal {
                Some(l) => variants.push(Variant { name: &variant.ident, literal: l}),
                None => variants.push(Variant { name: &variant.ident, literal: variant.ident.to_string() }),
            }
        }
    }

    let name = &ast.ident;
    let mut variant_tts = Vec::new();
    for var in &variants {
        let var_name = &var.name;
        let var_literal = &var.literal;
        variant_tts.push(quote! { #var_literal => Ok(#name::#var_name) });
    }

    let gen = quote! {
        impl ::std::str::FromStr for #name {
            type Err = ParseEnumVariantError;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    #(#variant_tts),*,
                    _ => Err(Self::Err::new(stringify!(#name))),
                }
            }
        }
    };
    gen.into()
}