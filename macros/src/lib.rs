use syn::parse_macro_input;
use tailwag_forms_macro_logic as logic;

#[proc_macro_derive(GetForm)]
pub fn derive_get_form(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = logic::derive::get_form::derive_struct(&input);
    impl_trait_tokens.into()
}
