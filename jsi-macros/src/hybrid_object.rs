use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{ImplItem, ItemImpl, LitStr};

pub fn expand_hybrid_object(name_lit: LitStr, input: ItemImpl) -> TokenStream {
    let name = name_lit.value();

    let self_ty = input.self_ty.clone();

    // Collect methods annotated with #[hybrid_method]
    let mut method_regs: Vec<TokenStream> = Vec::new();
    for it in &input.items {
        if let ImplItem::Fn(m) = it {
            let has_attr = m.attrs.iter().any(|a| a.path().is_ident("hybrid_method"));
            if !has_attr { continue; }

            let method_ident = m.sig.ident.clone();
            let method_name_str = method_ident.to_string();

            // Expect signature: fn(&self, rt: &mut RuntimeHandle, ...args)
            let mut arg_idents: Vec<syn::Ident> = Vec::new();
            let mut arg_types: Vec<syn::Type> = Vec::new();
            for (i, arg) in m.sig.inputs.iter().enumerate() {
                if i == 0 { continue; } // &self
                if i == 1 { continue; } // &mut RuntimeHandle
                if let syn::FnArg::Typed(pat) = arg {
                    let ident = if let syn::Pat::Ident(pi) = &*pat.pat {
                        pi.ident.clone()
                    } else {
                        format_ident!("arg{}", i - 2)
                    };
                    arg_idents.push(ident);
                    arg_types.push((*pat.ty).clone());
                }
            }

            let arg_convert = arg_idents.iter().zip(arg_types.iter()).map(|(id, ty)| {
                quote! {
                    let #id: #ty = match __args_iter.next() {
                        Some(val) => match ::jsi::FromValue::from_value(&val, rt) {
                            Some(v) => v,
                            None => ::anyhow::bail!("argument conversion failed: {}", stringify!(#id)),
                        },
                        None => ::anyhow::bail!("not enough arguments"),
                    };
                }
            });

            let arg_len = syn::LitInt::new(&format!("{}", arg_idents.len()), Span::call_site());
            let reg = quote! {
                {
                    let fn_val = ::jsi::JsiFn::from_host_fn(
                        &::jsi::PropName::new(#method_name_str, rt),
                        #arg_len,
                        Box::new(move |this, args, rt| {
                            let this_obj: ::jsi::JsiObject = match ::jsi::FromValue::from_value(&this, rt) {
                                Some(o) => o,
                                None => ::anyhow::bail!("'this' is not an object"),
                            };
                            let state: ::std::sync::Arc<#self_ty> = match this_obj.get_native_state::<#self_ty>(rt) {
                                Some(s) => s,
                                None => ::anyhow::bail!("missing native state for hybrid object"),
                            };

                            let mut __args_iter = args.into_iter();
                            #(#arg_convert)*

                            let result = #self_ty::#method_ident(&*state, rt, #(#arg_idents),*);
                            Ok(::jsi::IntoValue::into_value(result, rt))
                        }),
                        rt,
                    );
                    let val = fn_val.into_value(rt);
                    obj.set(::jsi::PropName::new(#method_name_str, rt), &val, rt);
                }
            };
            method_regs.push(reg);
        }
    }

    let expanded = quote! {
        #input

        impl ::jsi::HybridObject for #self_ty {
            fn hybrid_object_name(&self) -> &'static str { #name }

            fn load_methods<'rt>(&self, obj: &mut ::jsi::JsiObject<'rt>, rt: &mut ::jsi::RuntimeHandle<'rt>) {
                #(#method_regs)*
            }
        }
    };

    expanded
}
