extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Field, FieldsNamed, Ident, Token};

#[proc_macro_derive(WithDeps)]
pub fn with_depth(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let fields = extract_fields(&input);
    let field_names = fields.iter().map(|f| {
        let ident = &f.ident;
        return ident;
    });
    let field_names_clone = field_names.clone();
    let field_with_types = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        return quote! {
            #ident: #ty
        };
    });
    let methods = fields.iter().map(|f| {
        let ident = &f.ident.clone().unwrap();
        let method_name = Ident::new(&format!("set_{}", ident), ident.span());
        let ty = &f.ty;
        return quote! {
            pub fn #method_name(&mut self,#ident:#ty){
                self.#ident = #ident;
            }
        };
    });
    let result = quote! {
        impl #name {
            pub fn new(#(#field_with_types),*) -> Self {
                Self {
                    #(#field_names_clone),*
                }
            }
            #(#methods)*
        };
    };
    result.into()
}

fn extract_fields(input: &DeriveInput) -> &Punctuated<Field, Token![,]> {
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(FieldsNamed { named: fields, .. }),
        ..
    }) = &input.data
    {
        fields
    } else {
        panic!("#[derive(WithDeps)] can only be used on structs with named fields");
    };
    fields
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
