use std::collections::HashMap;
use std::iter::FromIterator;
use proc_macro::TokenStream;


use rocket::http::Method;
use syn::{FunctionRetTy, Ident, Ty};
use syn::{parse_expr, parse_item};
use quote::Tokens;


use ::{PARAM_PREFIX, USER_FN_PREFIX};
use utils::*;
use errors::*;
use errors::ErrorKind::*;

macro_rules! method_decorator {
    ($name:ident) => (
        pub fn $name(args: TokenStream, item: TokenStream) -> Result<TokenStream>  {
            let args = parse_expr(&args.to_string()).map_err(|_| RouteTooLessParam(0))?;
            let item = parse_item(&item.to_string())?;
            let router_def = RouterDef::new(None, args, item)?;
            
            debug!("Route params: {:?}", router_def);
            Ok(generic_route_decorator(router_def)?)
        }
    );
    ($name:ident, $method:ident) => (
        pub fn $name(args: TokenStream, item: TokenStream) -> Result<TokenStream>  {
            let args = parse_expr(&args.to_string()).map_err(|_| RouteTooLessParam(0))?;
            let item = parse_item(&item.to_string())?;
            let router_def = RouterDef::new(Some(Method::$method), args, item)?;

            debug!("Route params: {:?}", router_def);
            Ok(generic_route_decorator(router_def)?)
        }
    )
}

method_decorator!(route_decorator);
method_decorator!(get_decorator, Get);
method_decorator!(put_decorator, Put);
method_decorator!(post_decorator, Post);
method_decorator!(delete_decorator, Delete);
method_decorator!(head_decorator, Head);
method_decorator!(patch_decorator, Patch);
// TODO: Allow this once Diesel incompatibility is fixed.
// method_decorator!(options_decorator, Options);


// FIXME: Compilation fails when parameters have the same name as the function!
fn generic_route_decorator(router_def: RouterDef) -> Result<TokenStream> {
    let mut func = router_def.item.clone();
    let fn_vis = func.vis.clone();
    let fn_name = router_def.fn_name();
    let user_fn_ident = Ident::new(USER_FN_PREFIX.to_string() + &fn_name);
    func.ident = user_fn_ident.clone();
    let fn_ident = Ident::new(fn_name);
    let mut impl_generics = func.fn_generics()?.clone();
    let fn_decl = func.fn_decl()?;
    let fn_ret_ty = match &fn_decl.output {
        &FunctionRetTy::Ty(ref ty) => ty.clone(),
        _ => Ty::Tup(vec![]),
    };

    let (input_names, input_tys, input_idx) = fn_decl.arg_triple()?;

    let mut param_stack = HashMap::from_iter(input_names.iter()
        .cloned()
        .zip(input_tys.iter().cloned()));

    let data_statement = generate_data_statement(&mut param_stack, &router_def)?;
    let query_statement = generate_query_statement(&mut param_stack, &router_def)?;
    let param_statements = generate_param_statements(&mut param_stack, &router_def)?;

    assert_eq!(param_stack.len(), 0);

    let user_defined_lifetimes = impl_generics.lifetimes
        .iter()
        .map(|l| l.lifetime.ident.to_string())
        .collect();
    let mut lifetime_pool = LifetimePool::new(&user_defined_lifetimes);

    let impl_tys: Vec<_> = input_tys.iter()
        .map(|ty| ty.coalesce_lifetime_recursive(&mut lifetime_pool))
        .collect();
    impl_generics.lifetimes.extend(lifetime_pool.used_lifetime_def());

    let input_tys = input_tys.as_slice();
    let impl_tys = impl_tys.as_slice();
    let input_idx = input_idx.as_slice();
    let input_name_idents: Vec<_> =
        input_names.into_iter().map(|name| Ident::new(name)).collect();

    let path = router_def.path;
    let method = MethodWrapper(router_def.method);
    let optional_content_type = router_def.format
        .map(|f| {
            let c = ContentTypeWrapper(f);
            quote! { Some(#c) }
        })
        .unwrap_or(quote! { None });
    let optional_rank =
        router_def.rank.map(|r| quote! { Some(#r) }).unwrap_or(quote! { None });

    let tokens = quote! {
        
        #func

        #[allow(non_camel_case_types)]
        #fn_vis struct #fn_ident;

        impl ::rocket::StaticRouteInfo for #fn_ident {
            fn method(&self) -> ::rocket::http::Method {
                #method
            }
            fn path(&self) -> &'static str {
                #path
            }
            fn format(&self) -> Option<::rocket::http::ContentType> {
                #optional_content_type
            }
            fn handler(&self) -> ::rocket::handler::Handler {
                fn #fn_ident<'_b>(_req: &'_b ::rocket::Request, _data: ::rocket::Data)    
                        -> ::rocket::handler::Outcome<'_b> {
                    #param_statements
                    #query_statement
                    #data_statement
                    let responder = #user_fn_ident(#(#input_name_idents),*);
                    ::rocket::handler::Outcome::of(responder)
                }
                #fn_ident
            }
            fn rank(&self) -> Option<isize> {
                #optional_rank
            }
        }

        impl #impl_generics ::std::ops::FnOnce<(#(#impl_tys),*)> for #fn_ident {
            type Output = #fn_ret_ty;
            extern "rust-call" fn call_once(self, args: (#(#input_tys),*)) -> Self::Output {
                #user_fn_ident(#(args.#input_idx),*)
            }
        }

        impl #impl_generics ::std::ops::FnMut<(#(#impl_tys),*)> for #fn_ident {
            extern "rust-call" fn call_mut(&mut self, args: (#(#input_tys),*)) -> Self::Output {
                #user_fn_ident(#(args.#input_idx),*)
            }
        }

        impl #impl_generics ::std::ops::Fn<(#(#impl_tys),*)> for #fn_ident {
            extern "rust-call" fn call(&self, args: (#(#input_tys),*)) -> Self::Output {
                #user_fn_ident(#(args.#input_idx),*)
            }
        }
    };
    tokens.parse().map_err(|e| LexError(e).into())
}

fn generate_query_statement(param_stack: &mut HashMap<String, Ty>,
                            router_def: &RouterDef)
                            -> Result<Option<Tokens>> {
    if let Some(ref query_param_name) = router_def.query_param {

        let modified_query_param_name = PARAM_PREFIX.to_string() + &query_param_name;
        let ty = param_stack.remove(&modified_query_param_name)
            .ok_or(RouteParamNotFoundInFnInput(query_param_name.clone()))?;
        let param_ident = Ident::new(modified_query_param_name);
        let ty = ty.strip_lifetime_recursive().0;

        let query_string_expr = quote!(match _req.uri().query() {
            Some(query) => query,
            None => return ::rocket::Outcome::Forward(_data),
        });

        Ok(Some(quote! {
            let #param_ident: #ty =
                match ::rocket::request::FromForm::from_form_string(#query_string_expr) {
                    Ok(v) => v,
                    Err(_) => return ::rocket::Outcome::Forward(_data)
                };
        }))

    } else {
        Ok(None)
    }
}

fn generate_data_statement(param_stack: &mut HashMap<String, Ty>,
                           router_def: &RouterDef)
                           -> Result<Option<Tokens>> {
    if let Some(ref data_param_name) = router_def.data_param {

        let modified_data_param_name = PARAM_PREFIX.to_string() + &data_param_name;

        let ty = param_stack.remove(&modified_data_param_name)
            .ok_or(RouteParamNotFoundInFnInput(data_param_name.clone()))?;
        let ty = ty.strip_lifetime_recursive().0;
        let param_ident = Ident::new(modified_data_param_name);

        Ok(Some(quote! {
            let #param_ident: #ty =
                match ::rocket::data::FromData::from_data(_req, _data) {
                    ::rocket::Outcome::Success(d) => d,
                    ::rocket::Outcome::Forward(d) =>
                        return ::rocket::Outcome::Forward(d),
                    ::rocket::Outcome::Failure((code, _)) => {
                        return ::rocket::Outcome::Failure(code);
                    }
                };
        }))
    } else {
        Ok(None)
    }
}

// TODO: Add some kind of logging facility in Rocket to get be able to log
// an error/debug message if parsing a parameter fails.
fn generate_param_statements(param_stack: &mut HashMap<String, Ty>,
                             router_def: &RouterDef)
                             -> Result<Tokens> {
    let mut fn_param_statements = vec![];

    // Generate a statement for every declared paramter in the path.
    for (i, param) in ParamIter::new(&router_def.path).enumerate() {
        let param_name = param.name().to_string();
        let modified_param_name = PARAM_PREFIX.to_string() + &param_name;
        let ty = param_stack.remove(&modified_param_name)
            .ok_or(RouteParamNotFoundInFnInput(param_name.clone()))?;
        let ty = ty.strip_lifetime_recursive().0;

        let expr = match param {
            Param::Single(_) => {
                quote! {
                        match _req.get_param_str(#i) {
                            Some(s) => <#ty as ::rocket::request::FromParam>::from_param(s),
                            None => return ::rocket::Outcome::Forward(_data)
                        }
                    }
            }
            Param::Many(_) => {
                quote! {
                    match _req.get_raw_segments(#i) {
                        Some(s) => <#ty as ::rocket::request::FromSegments>::from_segments(s),
                        None => return ::rocket::Outcome::Forward(_data)
                    }
                }
            }
        };

        let param_ident = Ident::new(modified_param_name);
        let original_param_ident = Ident::new(param_name);
        fn_param_statements.push(quote! {
            let #param_ident: #ty = match #expr {
                Ok(v) => v,
                Err(e) => {
                    println!("    => Failed to parse '{}': {:?}",
                                stringify!(#original_param_ident), e);
                    return ::rocket::Outcome::Forward(_data)
                }
            };
        })
    }

    // Generate the code for `from_request` parameters.
    for (modified_arg_name, arg_ty) in param_stack.drain() {
        let arg_ty = arg_ty.strip_lifetime_recursive().0;
        let arg_ident = Ident::new(modified_arg_name.clone());

        fn_param_statements.push(quote! {
            let #arg_ident: #arg_ty = match
                    ::rocket::request::FromRequest::from_request(_req) {
                        ::rocket::outcome::Outcome::Success(v) => v,
                        ::rocket::outcome::Outcome::Forward(_) =>
                            return ::rocket::Outcome::forward(_data),
                        ::rocket::outcome::Outcome::Failure((code, _)) => {
                            return ::rocket::Outcome::Failure(code)
                },
            };
        })
    }

    Ok(quote! {
        #(#fn_param_statements);*
    })
}
