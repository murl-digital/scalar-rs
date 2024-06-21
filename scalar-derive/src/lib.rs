use convert_case::Casing;
use darling::{util::Flag, Error, FromDeriveInput, FromField, Result};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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
    name: Option<String>,
    placeholder: Option<String>
}

#[proc_macro_derive(Document, attributes(document, field))]
pub fn derive_document(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let document = match Document::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
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
        let name = field.name.unwrap_or(field.ident.map(|i| i.to_string().to_case(convert_case::Case::Title)).expect("this shouldn't be a tuple struct!!!!"));
        let placeholder = match field.placeholder {
            Some(str) => quote! { Some(#str) },
            None => quote! { None }
        };
        Ok(quote! {
            #ty::to_editor_field(None, #name, #placeholder)
        })
    }).collect::<core::result::Result<Vec<_>, darling::Error>>() {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()) }
    };

    let output = quote! {
        use ::scalar::{EditorField, editor_field::ToEditorField};
            impl Document for #ident {
                #doc_identifier

                #doc_title
    
                fn schema() -> Vec<EditorField> {
                    vec![
                        #(#fields),*
                    ]
                }
            }
    };
    output.into()
}
