extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate serde_derive_internals;
extern crate syn;
extern crate serde;

use serde_derive_internals::{ast, Ctxt};
use syn::{DeriveInput, Lit, Expr};

mod derive_enum;
mod derive_struct;


#[cfg(feature = "bytes")]
extern crate serde_bytes;



#[proc_macro_derive(TypescriptDefinition)]
pub fn derive_typescript_definition(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //eprintln!(".........[input] {}", input);
    let input: DeriveInput = syn::parse(input).unwrap();

    let cx = Ctxt::new();
    let container = ast::Container::from_ast(&cx, &input);

    let typescript = match container.data {
        ast::Data::Enum(variants) => {
            derive_enum::derive_enum(variants, &container.attrs).to_string()
        }
        ast::Data::Struct(style, fields) => {
            derive_struct::derive_struct(style, fields, &container.attrs)
        }
    };

    let typescript_string = typescript.to_string();
    let typescript_ident = syn::Ident::from(format!("{}___typescript_definition", container.ident));
    let export_ident = syn::Ident::from(format!("TS_EXPORT_{}", container.ident.to_string().to_uppercase()));

    eprintln!("....[typescript] {:?}", typescript_string);
    //eprintln!("........[export_ident] {:?}", export_ident);
   //eprintln!("............[container] {:?}", container);
    // eprintln!();
    // eprintln!();

    let mut expanded = quote!{

        #[wasm_bindgen(typescript_custom_section)]
        const #export_ident : &'static str = #typescript_string;

    };

    if cfg!(any(debug_assertions, feature = "test-export")) {
        expanded.append_all(quote!{
            fn #typescript_ident ( ) -> &'static str {
                #typescript_string
            }
        });
    }

    cx.check().unwrap();

    expanded.into()
}

fn collapse_list_bracket(body: Vec<quote::Tokens>) -> quote::Tokens {
    if body.len() == 1 {
        body[0].clone()
    } else {
        let tokens = body.into_iter().fold(quote!{}, |mut agg, tokens| { agg.append_all(quote!{ #tokens , }); agg });
        quote!{ [ #tokens ] }
    }
}

fn parse_non_primitive(s: &str) -> (String, bool) {

    if s.starts_with("Vec < Option <") {
        let rest = &s[15..s.len()-4];

        (format!("Array<{}>", rest), false)
    }
    else if s.starts_with("Option <") {
        let rest = &s[9..s.len() - 2];
        (format!("{}", rest), true)
    }
    else if s.starts_with("Vec <") {
        let rest = &s[5..s.len() - 1];
        (format!("Array<{}>", rest), false)
    }
    else {
        (format!("{}",s), false)
    }

}


fn type_to_ts_string(ty: &syn::Type) -> (String, bool) {
   // println!("Type: ??? {:?}", ty);
    use syn::Type::*;
    let mut is_optional = false;

    let q_tokens = match ty {

        Path(inner) => {
            //let ty_string = format!("{:?}", inner.path);
            let result = quote!{ #inner };
            match result.to_string().as_ref() {
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" =>
                    "number".to_string(),
                "String" | "&str" | "&'static str" =>
                    "string".to_string(),
                "bool" => "boolean".to_string(),
                something_else => {
                    let (q, is_opt) = parse_non_primitive(something_else);
                    is_optional = is_opt;
                    q
                },
            }
        },
        Array(array) => {
            let array_len = match array.len {
                Expr::Lit(ref inner) => {
                    match inner.lit {
                        Lit::Int(ref inner_inner) => {
                            inner_inner.value()
                        }
                        _ => {
                            //Error
                            4
                        }
                    }
                },
                _ => {
                    //Error
                    2
                }
            };

            let array_type : String = type_to_ts_string(array.elem.as_ref()).0;

            let repeated_tuple_strings = (0..array_len).map(|_|
                        array_type.clone())
                        .collect::<Vec<String>>();

            format!("[{}] ",
                    repeated_tuple_strings
                        .join(", "))
        },

        _  => format!("any no match ty 3").to_string()
    };

        (q_tokens, is_optional)
}

fn type_to_ts(ty: &syn::Type) -> (quote::Tokens, bool) {
   // println!("Type: ??? {:?}", ty);
    use syn::Type::*;
    let is_optional = false;

    let q_tokens = match ty {
        Slice(..) => quote!{ any },
        Array(..) => quote!{ any },
        Ptr(..) => quote!{ any },
        Reference(..) => quote!{ any },
        BareFn(..) => quote!{ any },
        Never(..) => quote!{ any },
        Tuple(..) => quote!{ any },
        Path(inner) => {
            //let ty_string = format!("{:?}", inner.path);
            let result = quote!{ #inner };
            match result.to_string().as_ref() {
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" =>
                    quote! { number },
                "String" | "&str" | "&'static str" =>
                    quote! { string },
                "bool" => quote!{ boolean },
                _ => quote!{any},
            }
        }
        TraitObject(..) => quote!{ trait any },
        ImplTrait(..) => quote!{ impl any },
        Paren(..) => quote!{ any },
        Group(..) => quote!{ any },
        Infer(..) => quote!{ any },
        Macro(..) => quote!{ any },
        Verbatim(..) => quote!{ any },
    };

        (q_tokens, is_optional)
}

fn derive_field_str(_variant_idx: usize, _field_idx: usize, field: &ast::Field) -> String {
    let field_name = field.attrs.name().serialize_name();
    let (ty, is_opt) = type_to_ts_string(&field.ty);

    format! ("{}{}: {}", field_name, if is_opt { "?" } else { ""}, ty)


}


fn derive_element(_variant_idx: usize, _element_idx: usize, field: &ast::Field) -> quote::Tokens {
    let (ty, _is_opt) = type_to_ts(&field.ty);
    quote!{
        #ty
    }
}
