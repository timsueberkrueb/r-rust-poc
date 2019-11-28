use syn::spanned::Spanned;

pub(crate) fn parse(input: proc_macro::TokenStream) -> Result<Func, syn::Error> {
    let func: syn::ItemFn = syn::parse(input.into())?;

    // Some sanity validations
    if let syn::Visibility::Public(_) = &func.vis {
        if !func.sig.generics.params.is_empty() {
            return Err(syn::Error::new_spanned(
                func.sig.generics.params,
                "Generic functions are not supported",
            ));
        }
    } else {
        return Err(syn::Error::new_spanned(func, "Function must be `pub`"));
    }

    let params: Vec<(syn::PatIdent, syn::Type)> = func
        .sig
        .inputs
        .into_iter()
        .map(|arg| {
            if let syn::FnArg::Typed(arg) = arg {
                if let syn::Pat::Ident(ident) = &*arg.pat {
                    Ok((ident.clone(), *arg.ty))
                } else {
                    Err(syn::Error::new(
                        arg.span(),
                        "Function parameters must be identifiers",
                    ))
                }
            } else {
                Err(syn::Error::new(arg.span(), "Self parameter not supported"))
            }
        })
        .collect::<Result<_, _>>()?;

    let func = Func {
        ident: func.sig.ident,
        params,
        output: func.sig.output,
        body: func.block,
    };

    Ok(func)
}

pub(crate) struct Func {
    pub(crate) ident: syn::Ident,
    pub(crate) params: Vec<(syn::PatIdent, syn::Type)>,
    pub(crate) output: syn::ReturnType,
    pub(crate) body: Box<syn::Block>,
}
