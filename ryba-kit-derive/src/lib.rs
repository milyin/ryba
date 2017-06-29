extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ToContext)]
pub fn to_context(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let mut ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_to_context(&mut ast);
    println!("{}",gen);
    gen.parse().unwrap()
}

//fn context(name: String, typename: String)  -> syn::Field 
fn change_type(x: &mut syn::Field)
{
    println!();
    println!("before {:?}",x);
    if let syn::Ty::Path(_, syn::Path { global: _, segments: ref mut p } ) = x.ty { 
        if p.len() > 0 {
            p[0].ident = syn::Ident::new("ContextField");
            if let syn::PathParameters::AngleBracketed(ref mut angle_bracketed_data) = p[0].parameters {
                angle_bracketed_data.lifetimes = vec![];
            }
        } 
    }
    println!("after {:?}",x);
}

fn impl_to_context(ast: &mut syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    if let syn::Body::Struct(syn::VariantData::Struct(ref mut s)) = ast.body {
        let ctx_name = syn::Ident::new(name.to_string()+"Context");
        for field in s.iter_mut() { change_type(field) };
        let ctx_fields : Vec<&syn::Field> = s.iter().collect();
//        let field_names1 : Vec<syn::Ident> = s.iter().map(|ref x| x.ident.clone().unwrap()).collect();
//        let field_names2 : Vec<syn::Ident> = s.iter().map(|ref x| x.ident.clone().unwrap()).collect();
        return quote! {
            struct #ctx_name {
                #( #ctx_fields ),*
            }
/*            struct #context_name {
                #(                   
                     #ctx_field_names
                ) , *
            }*/
/*            impl FieldForm for #name {
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
           } */
        }
    }
    quote! {}
}