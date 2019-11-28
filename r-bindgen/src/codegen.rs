use proc_quote::quote;

use crate::parse::Func;

pub(crate) fn codegen(func: Func) -> proc_macro2::TokenStream {
    let (param_pat, param_type): (Vec<_>, Vec<_>) = func.params.into_iter().unzip();
    let param_name = param_pat.iter().map(|pat| &pat.ident);
    let param_name2 = param_name.clone();

    let ident = func.ident;
    let output = func.output;
    let ret_type = match &output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };
    let body = func.body;

    quote! {
        #[no_mangle]
        pub extern fn #ident ( #(#param_name: r_interop::SEXP),*) -> r_interop::SEXP {
            fn wrapped( #(#param_pat: #param_type),* ) #output {
                #body
            }
            let res = wrapped(#(<#param_type as r_interop::FromSEXP>::from_sexp(#param_name2) ),*);
            <#ret_type as r_interop::IntoSEXP>::into_sexp(res)
        }
    }
}
