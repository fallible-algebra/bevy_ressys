extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::{punctuated::Punctuated, token::Comma, *};

/// Given a path to a logging macro and a system that gives a result type,
/// transform the system into one that does not return a result type and instead
/// reports `Err`s to the given macro i.e.
/// 
/// ```
/// #[res_system(bevy::log::warn)]
/// fn this_system_warns(/* args */) -> Result<(), String> {
///     Err("This is a warning".to_owned())
/// }
/// ```
/// 
/// Becomes
/// 
/// ```
/// fn this_system_warns(/* args */) {
///     fn inner_fn_for_res_system(/* args */) {
///         Err("This is a warning".to_owned())
///     }
///     if let Err(err) = inner_fn_for_res_system(/* args */) {
///         bevy::log::warn!(err)
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn res_system(attr: TokenStream, item: TokenStream) -> TokenStream {
    // We're making an attribute for a system function, so parse as ItemFn.
    let mut inner_ressys: ItemFn = parse_macro_input!(item as ItemFn);
    let mut outer_ressys = inner_ressys.clone();
    let inner_ident = Ident::new("inner_fn_for_res_system", Span::call_site());
    // Transform the typed arguments into just its patterns
    let args = inner_ressys.sig.inputs.clone();
    let args2: Punctuated<Expr, Comma> = args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat) = arg {
                let mut pattern = pat.pat.clone();
                // This is a hack and a half. should be replaced with a more 
                // reasonable solution but not handling mutable variables is
                // a case I entirely forgot about. They all get passed by
                // value to the system anyway, at least they should be.
                if let Pat::Ident(ident) = pattern.as_mut() {
                    ident.mutability = None;
                }
                Expr::Verbatim(quote! {#pattern})
            } else {
                Expr::Verbatim(quote! {compile_error!("Ran into Reciever type in macro")})
            }
        })
        .collect();
    // rename the inner result system
    inner_ressys.sig.ident = inner_ident.clone();
    // Set the return type of the outer result system to the default.
    outer_ressys.sig.output = ReturnType::Default;
    for arg in outer_ressys.sig.inputs.iter_mut() {
        if let FnArg::Typed(PatType{ pat, ..}) = arg {
            if let Pat::Ident(ident) = pat.as_mut() {
                if ident.by_ref.is_none() && ident.mutability.is_some() {
                    ident.mutability = None;
                }
            }
        }
    }
    // Parse the attributes for the macro as an expression path i.e. `bevy::log::warn`
    let attr2 = parse_macro_input!(attr as ExprPath);
    let outer_block = quote! {
        {
            #inner_ressys
            if let Err(err) = #inner_ident(#args2) {
                #attr2!(err)
            }
        }
    };
    outer_ressys.block = Box::new(parse2(outer_block).unwrap());
    outer_ressys.into_token_stream().into()
}

/// Minor alteration to [res_system] that passes the arguments to the a simple format string.
/// i.e. in [res_system] arguments are passed as `macro!(err)` but in this macro it's passed as
/// `macro!("{}", err)`.
#[proc_macro_attribute]
pub fn ressys_fmt(attr: TokenStream, item: TokenStream) -> TokenStream {
    // We're making an attribute for a system function, so parse as ItemFn.
    let mut inner_ressys: ItemFn = parse_macro_input!(item as ItemFn);
    let mut outer_ressys = inner_ressys.clone();
    let inner_ident = Ident::new("inner_fn_for_res_system", Span::call_site());
    // Transform the typed arguments into just its patterns
    let args = inner_ressys.sig.inputs.clone();
    let args2: Punctuated<Expr, Comma> = args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat) = arg {
                let mut pattern = pat.pat.clone();
                // This is a hack and a half. should be replaced with a more 
                // reasonable solution but not handling mutable variables is
                // a case I entirely forgot about. They all get passed by
                // value to the system anyway, at least they should be.
                if let Pat::Ident(ident) = pattern.as_mut() {
                    ident.mutability = None;
                }
                Expr::Verbatim(quote! {#pattern})
            } else {
                Expr::Verbatim(quote! {compile_error!("Ran into Reciever type in macro")})
            }
        })
        .collect();
    // rename the inner result system
    inner_ressys.sig.ident = inner_ident.clone();
    // Set the return type of the outer result system to the default.
    outer_ressys.sig.output = ReturnType::Default;
    for arg in outer_ressys.sig.inputs.iter_mut() {
        if let FnArg::Typed(PatType{ pat, ..}) = arg {
            if let Pat::Ident(ident) = pat.as_mut() {
                if ident.by_ref.is_none() && ident.mutability.is_some() {
                    ident.mutability = None;
                }
            }
        }
    }
    // Parse the attributes for the macro as an expression path i.e. `bevy::log::warn`
    let attr2 = parse_macro_input!(attr as ExprPath);
    let outer_block = quote! {
        {
            #inner_ressys
            if let Err(err) = #inner_ident(#args2) {
                #attr2!("{}", err)
            }
        }
    };
    outer_ressys.block = Box::new(parse2(outer_block).unwrap());
    outer_ressys.into_token_stream().into()}