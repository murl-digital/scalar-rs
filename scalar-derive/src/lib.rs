use convert_case::Casing;
use darling::{util::Flag, FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Ident, PathArguments, Type};

#[derive(FromDeriveInput)]
#[darling(attributes(document), supports(struct_named))]
struct Document {
    identifier: Option<String>,
    title: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(supports(struct_newtype, struct_named))]
struct ToEditorField {
    ident: syn::Ident,
    data: darling::ast::Data<(), FieldInfo>,
}

#[derive(FromDeriveInput)]
#[darling(supports(enum_unit, enum_named))]
struct Enum {
    data: darling::ast::Data<EnumVariant, FieldInfo>,
}

#[derive(FromVariant)]
struct EnumVariant {
    ident: Ident,
    fields: darling::ast::Fields<FieldInfo>,
}

#[derive(FromField, Clone)]
#[darling(attributes(field))]
struct FieldInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    title: Option<String>,
    placeholder: Option<String>,
    default: Option<syn::Lit>,
    validate: Flag,
}

/// Sets up an enum for use in a Document. This macro does a couple of things:
/// 1. It derives serde's Serialize and Deserialize traits. Make sure you have serde installed!
/// 2. Sets up said serialization and deserialization to work the way the editor expects.
/// 3. Derives ToEditorField for the schema
#[proc_macro_attribute]
pub fn doc_enum(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize, ::scalar::Enum)]
        #[serde(tag = "type")]
        #input
    };
    output.into()
}

#[proc_macro_derive(EditorField, attributes(field))]
pub fn struct_to_editor_field(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let struct_info = match ToEditorField::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let ident = struct_info.ident.to_owned();
    let fields = struct_info
        .data
        .take_struct()
        .expect("a compiler error should've been returned, this has to be a struct");

    match fields.style {
        darling::ast::Style::Tuple => {
            let field = fields
                .fields
                .first()
                .expect("there should always be at least one field");
            let field_ty = &field.ty;
            quote! {
                impl ::scalar::editor_field::ToEditorField<#field_ty> for #ident  {
                    fn to_editor_field(
                        default: Option<impl Into<#field_ty>>,
                        name: &'static str,
                        title: &'static str,
                        placeholder: Option<&'static str>,
                        validator: Option<&'static str>,
                    ) -> ::scalar::EditorField
                    where
                        Self: std::marker::Sized,
                    {
                        use ::scalar::editor_field::ToEditorField;
                        <#field_ty>::to_editor_field(default, name, title, placeholder, validator)
                    }
                }

                impl From<#ident> for #field_ty {
                    fn from(val: #ident) -> Self {
                        val.0
                    }
                }
            }
            .into()
        }
        darling::ast::Style::Struct => {
            let fields = fields
                .iter()
                .map(|f| field_to_info_call(f.to_owned()))
                .collect::<Vec<_>>();

            quote! {
                impl ::scalar::editor_field::ToEditorField<#ident> for #ident {
                    fn to_editor_field(
                        default: Option<impl Into<#ident>>,
                        name: &'static str,
                        title: &'static str,
                        placeholder: Option<&'static str>,
                        validator: Option<&'static str>,
                    ) -> ::scalar::EditorField
                    where
                        Self: std::marker::Sized,
                    {
                        use ::scalar::editor_field::ToEditorField;
                        ::scalar::EditorField {
                            name,
                            title,
                            placeholder,
                            required: true,
                            validator,
                            field_type: ::scalar::EditorType::Struct {
                                fields: vec![#(#fields),*]
                            }
                        }
                    }
                }
            }
            .into()
        }
        darling::ast::Style::Unit => unreachable!("it's impossible for this to be a unit struct"),
    }
}

#[proc_macro_derive(Enum)]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let enum_info = match Enum::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let ident = input.ident;

    let variants: Vec<proc_macro2::TokenStream> = match enum_info.data {
        darling::ast::Data::Enum(variants) => variants
            .iter()
            .map(|v| {
                let ident = v.ident.to_string();
                let fields: Vec<proc_macro2::TokenStream> = v
                    .fields
                    .iter()
                    .map(|field| field_to_info_call(field.to_owned()))
                    .collect();

                let fields_tokens = match fields.len() {
                    0 => quote! { None },
                    _ => quote! { Some(vec![#(#fields),*]) },
                };

                quote! {
                    ::scalar::editor_type::EnumVariant {
                        variant_name: #ident,
                        fields: #fields_tokens
                    }
                }
            })
            .collect(),
        darling::ast::Data::Struct(_) => unreachable!(),
    };

    let output = quote! {
        impl ::scalar::editor_field::ToEditorField<#ident> for #ident where Self: ::serde::Serialize {
            fn to_editor_field(default: Option<impl Into<Self>>, name: &'static str, title: &'static str, placeholder: Option<&'static str>, validator: Option<&'static str>) -> ::scalar::EditorField where Self: std::marker::Sized {
                ::scalar::EditorField { name, title, placeholder, required: true, validator, field_type: ::scalar::EditorType::Enum {
                    default: default.map(Into::into).map(|v| ::scalar::convert(v)),
                    variants: vec![#(#variants),*]
                } }
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Document, attributes(document, field))]
pub fn derive_document(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let document = match Document::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let struct_fields = match input.data {
        Data::Struct(st) => st.fields,
        _ => unreachable!(),
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

    let struct_fields = match struct_fields
        .iter()
        .map(FieldInfo::from_field)
        .collect::<Result<Vec<FieldInfo>, darling::Error>>()
    {
        Ok(f) => f,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let fields = struct_fields
        .iter()
        .map(|f| field_to_info_call(f.to_owned()))
        .collect::<Vec<_>>();

    let validators = struct_fields.iter().filter(|&f| f.validate.is_present()).map(|f| {
        let ty = &f.ty;

        quote! {
            map.insert(stringify!(#ty).to_string(), ::scalar::validations::create_validator_function(model, <#ty>::validate));
        }
    }).collect::<Vec<_>>();

    let output = quote! {
        #[automatically_derived]
        impl Document for #ident {
            #doc_identifier

            #doc_title

            fn fields() -> Vec<::scalar::EditorField> {
                use ::scalar::editor_field::ToEditorField;
                vec![
                    #(#fields),*
                ]
            }

            fn validators(model: scalar::validations::DataModel) -> ::std::collections::HashMap<String, scalar::validations::ValidatorFunction> {
                use ::scalar::validations::Validator;
                use ::std::collections::HashMap;
                let mut map = HashMap::new();
                #(#validators)*
                map
            }
        }
    };
    output.into()
}

fn field_to_info_call(field: FieldInfo) -> proc_macro2::TokenStream {
    let ty = field.ty;

    let ident = field
        .ident
        .map(|i| i.to_string())
        .expect("this shouldn't be a tuple struct!!!!");
    let title = field
        .title
        .unwrap_or(ident.to_case(convert_case::Case::Title));
    let placeholder = match field.placeholder {
        Some(str) => quote! { Some(#str) },
        None => quote! { None },
    };

    let validator = match field.validate.is_present() {
        true => quote! { Some(stringify!(#ty)) },
        false => quote! { None },
    };

    let default = match field.default {
        Some(lit) => quote! { Some(#lit) },
        None => {
            let actual_ty = match ty {
                Type::Path(ref path) => {
                    if let PathArguments::AngleBracketed(generic) =
                        &path.path.segments.last().unwrap().arguments
                    {
                        generic.args.to_token_stream()
                    } else {
                        ty.to_token_stream()
                    }
                }
                _ => ty.to_token_stream(),
            };

            quote! { None::<#ty> }
        }
    };
    quote! {
        <#ty>::to_editor_field(#default, #ident, #title, #placeholder, #validator)
    }
}
