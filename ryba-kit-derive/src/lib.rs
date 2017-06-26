extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(FieldForm)]
pub fn field_form(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_field_form(&ast);
    gen.parse().unwrap()
}

fn impl_field_form(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    if let syn::Body::Struct(syn::VariantData::Struct(ref s)) = ast.body {
        let field_names1 : Vec<syn::Ident> = s.iter().map(|ref x| x.ident.clone().unwrap()).collect();
        let field_names2 : Vec<syn::Ident> = s.iter().map(|ref x| x.ident.clone().unwrap()).collect();
        return quote! {
            impl FieldForm for #name {
                fn is_ok(&self) -> bool {
                    #(
                        self.#field_names1.is_ok()
                    ) && *
                }
                fn has_msg(&self) -> bool {
                    #(
                        self.#field_names2.has_msg()
                    ) || *
                }            
           }
        }
    }
    quote! {}
}