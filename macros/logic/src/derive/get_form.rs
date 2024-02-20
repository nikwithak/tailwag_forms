use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field};
use tailwag_utils::macro_utils::attribute_parsing::GetAttribute;

pub fn derive_struct(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;

    // Panic with error message if we get a non-struct
    let Data::Struct(data) = data else {
        panic!("Only Structs are supported")
    };

    match &data.fields {
        syn::Fields::Named(fields) => {
            let field_names = fields.named.iter().map(build_form_field);

            fn build_form_field(field: &Field) -> TokenStream {
                let name =
                    field.ident.as_ref().expect("This macro requires named fields").to_string();

                let required = field.get_attribute("required").is_some();

                // TODO: Set label with attribute
                // let label = field
                //     .get_attribute("label")
                //     .map_or(name.to_string(), |label| label.parse_args());
                let label = &name;

                let mut tokens = quote!(FormField::text(#name).label(#label));
                if required {
                    tokens = quote!(#tokens.is_required(true));
                }
                tokens
            }

            let parse_args_impl_tokens = quote!(
                impl tailwag::forms::GetForm for #ident
                {
                    fn get_form() -> tailwag::forms::Form {
                        type Form = tailwag::forms::Form;
                        type FormField = tailwag::forms::FormField;
                        tailwag::forms::Form {
                            button_name: "Submit".to_string(),
                            fields: vec![
                                #(#field_names),*
                            ],
                        }
                    }
                }
            );

            parse_args_impl_tokens
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
