use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{Data, DeriveInput, Field};
use tailwag_utils::macro_utils::attribute_parsing::GetAttribute;

// fn build_get_insert_statement(input: &DeriveInput) -> TokenStream {
//     let input_table_definition =
//         crate::util::database_table_definition::build_table_definition(input);

//     let insert_maps = input_table_definition.columns.iter().map(|(_, column)| {
//         let column_name = format_ident!("{}", column.column_name.as_str());
//         let column_name_as_string = column.column_name.as_str();

//         type E = tailwag_orm::data_definition::table::DatabaseColumnType;
//         let format_string = match &column.column_type {
//             E::Boolean | E::Int | E::Float => quote!("{}"), // No surrounding quotes - I need to refactor how this works *ANYWAY* (i.e. prep statements)
//             E::String | E::Timestamp | E::Uuid => quote!(
//                 "'{}'"
//             ),
//             E::Json =>  quote!(
//                 "'{}'"
//             ),
//             tailwag_orm::data_definition::table::DatabaseColumnType::OneToMany(_) => todo!(),
//             tailwag_orm::data_definition::table::DatabaseColumnType::ManyToMany(_) => todo!(),
//             tailwag_orm::data_definition::table::DatabaseColumnType::OneToOne(_) => todo!(),
//         };

//         let insert_statement = if column.is_nullable() {
//             quote!(
//                 if let Some(#column_name) = &self.#column_name {
//                     insert_map.insert(
//                         tailwag::orm::data_definition::table::Identifier::new(#column_name_as_string).expect("Invalid column identifier found - this should not happen. Panicking."),
//                         format!(#format_string, &#column_name.to_string()),
//                     );
//                 }
//             )
//         } else {
//             quote!(
//                 insert_map.insert(
//                     tailwag::orm::data_definition::table::Identifier::new(#column_name_as_string.to_string()).expect("Invalid column identifier found - this should not happen. Panicking."),
//                     // TODO: Can't support differently named column/struct_attr names right now
//                     // TODO: Only supports string-like types, apparetly?
//                     format!(#format_string, &self.#column_name.to_string()),
//                 );
//             )
//         };
//         insert_statement
//     });

//     let tokens = quote!(
//         fn get_insert_statement(&self) -> tailwag::orm::object_management::insert::InsertStatement {
//             let mut insert_map = std::collections::HashMap::new();

//             #(#insert_maps)*

//             let insert = tailwag::orm::object_management::insert::InsertStatement::new(
//                 <Self as tailwag::orm::data_manager::GetTableDefinition>::get_table_definition().clone(),
//                 insert_map,
//             );
//             insert
//         }
//     );

//     tokens
// }

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

            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
