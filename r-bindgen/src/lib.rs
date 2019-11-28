extern crate proc_macro;

mod codegen;
mod parse;

#[proc_macro_attribute]
pub fn r_bindgen(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match parse::parse(input) {
        Err(err) => err.to_compile_error().into(),
        Ok(res) => codegen::codegen(res).into(),
    }
}
