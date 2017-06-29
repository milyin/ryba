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
//    println!("{}",gen);
    gen.parse().unwrap()
}

//fn context(name: String, typename: String)  -> syn::Field 
fn change_type(x: &mut syn::Field)
{
    if let syn::Ty::Path(_, syn::Path { global: _, segments: ref mut p } ) = x.ty { 
        if p.len() > 0 {
            p[0].ident = syn::Ident::new("ContextField");
            if let syn::PathParameters::AngleBracketed(ref mut angle_bracketed_data) = p[0].parameters {
                angle_bracketed_data.lifetimes = vec![];
            }
        } 
    }
}

fn get_inner_type(x: &syn::Ty) -> Option<&syn::Ty>
{
    if let syn::Ty::Path(_, syn::Path { global: _, segments: ref p } ) = *x { 
        if p.len() > 0 {
            if let syn::PathParameters::AngleBracketed(ref angle_bracketed_data) = p[0].parameters {
                if angle_bracketed_data.types.len() > 0 {
                    return Some(&angle_bracketed_data.types[0])
                }
            }
        } 
    }
    None
}

fn impl_to_context(ast: &mut syn::MacroInput) -> quote::Tokens {
    let form_name = &ast.ident;
    let generics = &ast.generics;
    if let syn::Body::Struct(syn::VariantData::Struct(ref mut s)) = ast.body {
        let ctx_name = syn::Ident::new(form_name.to_string()+"Context");
        for field in s.iter_mut() { change_type(field) };
        let ctx_fields : Vec<&syn::Field> = s.iter().collect();
        let type_names : Vec<syn::Ty> = s.iter().map(|ref x| get_inner_type(&x.ty).unwrap().clone()).collect();
        let field_names1 : Vec<syn::Ident> = s.iter().map(|ref x| x.ident.clone().unwrap()).collect();
        let field_names2 = field_names1.clone();
        let field_names3 = field_names1.clone();
        let field_names4 = field_names1.clone();
        let field_names5 = field_names1.clone();
        return quote! {
            impl #generics #form_name #generics {
                pub fn context(&self) -> #ctx_name {
                    #ctx_name {
                       #( #field_names1: (&self.#field_names2).into() ),*
                    }
                }
                pub fn values(&self) -> Option<( #(&#type_names),*  )> {
                    if let ( #(&Ok(ref #field_names3)),* ) = ( #(&self.#field_names4.value),* ) {
                        Some(( #(#field_names5),*))
                    } else {
                        None
                    }
                }
            }
            #[derive(Serialize, Default)]
            pub struct #ctx_name {
                #( #ctx_fields ),*
            }
        }
    }
    quote! {}
}