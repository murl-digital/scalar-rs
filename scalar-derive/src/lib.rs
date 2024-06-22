use convert_case::Casing;
use darling::{Error, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[derive(FromDeriveInput)]
#[darling(attributes(document), supports(struct_named))]
struct Document {
    identifier: Option<String>,
    title: Option<String>
}

#[derive(FromField)]
#[darling(attributes(field))]
struct FieldInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    title: Option<String>,
    placeholder: Option<String>,
    default: Option<syn::Lit>
}

#[proc_macro_derive(Document, attributes(document, field))]
pub fn derive_document(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let document = match Document::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };
    let struct_fields = match input.data {
        Data::Struct(st) => {
            st.fields
        },
        _ => unreachable!()
    };
    let ident = input.ident;

    let doc_identifier = match document.identifier {
        Some(ident) => quote! {
            fn identifier() -> &'static str {
                #ident
            } 
        },
        None => {
            let ident = ident.to_string().to_case(convert_case::Case::Snake);
            quote! {
                fn identifier() -> &'static str {
                    #ident
                }
            }
        }
    };

    let doc_title = match document.title {
        Some(title) => quote! {
            fn title() -> &'static str {
                #title
            }
        },
        None => {
            let title = ident.to_string().to_case(convert_case::Case::Title);
            quote! {
                fn title() -> &'static str {
                    #title
                }
            }
        }
    };

    let fields = match struct_fields.iter().map(|field| {
        let field = FieldInfo::from_field(field)?;
        let ty = field.ty;
        let ident = field.ident.map(|i| i.to_string()).expect("this shouldn't be a tuple struct!!!!");
        let title = field.title.unwrap_or(ident.to_case(convert_case::Case::Title));
        let placeholder = match field.placeholder {
            Some(str) => quote! { Some(#str) },
            None => quote! { None }
        };
        let default = match field.default {
            Some(lit) => quote! { Some(#lit) },
            None => quote! { None::<#ty> }
        };
        Ok(quote! {
            #ty::to_editor_field(#default, #ident, #title, #placeholder)
        })
    }).collect::<core::result::Result<Vec<_>, darling::Error>>() {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()) }
    };

    let output = quote! {
        impl Document for #ident {
            #doc_identifier

            #doc_title

            fn fields() -> Vec<::scalar::EditorField> {
                use ::scalar::editor_field::ToEditorField;
                vec![
                    #(#fields),*
                ]
            }
        }
    };
    output.into()
}
