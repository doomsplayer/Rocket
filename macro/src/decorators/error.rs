use proc_macro::TokenStream;
use syn::{ExprKind, FnArg, FunctionRetTy, Ident, Ty};
use syn::{parse_expr, parse_item};

use ::USER_FN_PREFIX;
use errors::*;
use errors::ErrorKind::*;
use utils::*;

const ERR_PARAM: &'static str = "_error";
const REQ_PARAM: &'static str = "_request";

pub fn error_decorator(args: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let expr = parse_expr(&args.to_string()).map_err(|_| ErrorParamNumNotCorrect)?;
    let code = match expr.node {
        ExprKind::Paren(expr) => {
            match expr.node {
                ExprKind::Lit(lit) => lit.as_isize()? as u16,
                _ => bail!(ErrorCodeNotInteger),
            }
        }
        _ => bail!(ErrorParamNumNotCorrect),
    };

    let mut func = parse_item(&item.to_string()).unwrap();

    let fn_name = func.ident.to_string();
    let user_fn_ident = Ident::new(USER_FN_PREFIX.to_string() + &fn_name);
    func.ident = user_fn_ident.clone();
    let fn_ident = Ident::new(fn_name);
    let fn_vis = func.vis.clone();
    let fn_generics = func.fn_generics()?;


    let fn_decl = func.fn_decl()?;
    if fn_decl.inputs.len() > 2 {
        bail!(ErrorHandleTooMuchParam);
    }
    let fn_ret_ty = match &fn_decl.output {
        &FunctionRetTy::Ty(ref ty) => ty.clone(),
        _ => Ty::Tup(vec![]),
    };

    let err_param_ident = Ident::new(ERR_PARAM);
    let req_param_ident = Ident::new(REQ_PARAM);

    let mut input_name_idents = vec![];
    let mut input_tys = vec![];
    let mut input_idx = vec![];
    for (idx, fn_arg) in fn_decl.inputs.iter().enumerate() {
        let (ident, ty) = match fn_arg {
            &FnArg::Captured(_, ref ty) => {
                (match ty {
                     &Ty::Rptr(..) => req_param_ident.clone(),
                     &Ty::Path(..) => err_param_ident.clone(),
                     _ => bail!(ErrorHandleUnexpectedParam),
                 },
                 ty.clone())
            }
            &FnArg::Ignored(ref ty) => {
                (match ty {
                     &Ty::Rptr(..) => req_param_ident.clone(),
                     &Ty::Path(..) => err_param_ident.clone(),
                     _ => bail!(ErrorHandleUnexpectedParam),
                 },
                 ty.clone())
            }
            _ => bail!(ErrorHandleContainSelf),
        };
        input_name_idents.push(ident);
        input_tys.push(ty);
        input_idx.push(Ident::new(idx));
    }

    let input_tys = input_tys.as_slice();
    let input_idx = input_idx.as_slice();

    let out = quote! {

        #func

        #[allow(non_camel_case_types)]
        #fn_vis struct #fn_ident;
     
        impl ::rocket::StaticCatchInfo for #fn_ident {
            fn code(&self) -> u16 { #code }
            fn handler(&self) -> ::rocket::handler::ErrorHandler {
                fn imp<'_a>(#err_param_ident: ::rocket::Error,
                                        #req_param_ident: &'_a ::rocket::Request) -> ::rocket::response::Result<'_a> {

                    let user_response = #user_fn_ident(#(#input_name_idents),*);
                    let response = ::rocket::response::Responder::respond(user_response)?;
                    let status = ::rocket::http::Status::raw(#code);
                    ::rocket::response::Response::build().status(status).merge(response).ok()
                }
                imp
            }

        }

        impl #fn_generics ::std::ops::FnOnce<(#(#input_tys),*)> for #fn_ident {
            type Output = #fn_ret_ty;
            extern "rust-call" fn call_once(self, args: (#(#input_tys),*)) -> Self::Output {
                #user_fn_ident(#(args.#input_idx),*)
            }
        }

        impl #fn_generics ::std::ops::FnMut<(#(#input_tys),*)> for #fn_ident {
            extern "rust-call" fn call_mut(&mut self, args: (#(#input_tys),*)) -> Self::Output {
                #user_fn_ident(#(args.#input_idx),*)
            }
        }

        impl #fn_generics ::std::ops::Fn<(#(#input_tys),*)> for #fn_ident {
            extern "rust-call" fn call(&self, args: (#(#input_tys),*)) -> Self::Output {
                #user_fn_ident(#(args.#input_idx),*)
            }
        }
    };
    Ok(out.parse().unwrap())
}
