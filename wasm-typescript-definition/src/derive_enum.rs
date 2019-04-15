use quote;
use serde_derive_internals::{ast, attr};
use collapse_list_bracket;
use collapse_list_brace;
use type_to_ts;
use super::{derive_element, derive_field};

pub fn derive_enum<'a>(
    variants: Vec<ast::Variant<'a>>,
    _attr_container: &attr::Container,
) -> quote::Tokens {
    let tokens = variants.into_iter().enumerate()
        .map(|(variant_idx, variant)| {
            let variant_name = variant.attrs.name().serialize_name();
            match variant.style {
                ast::Style::Struct => {
                    derive_struct_variant(&variant_name, variant_idx, &variant.fields)
                }
                ast::Style::Newtype => derive_newtype_variant(&variant_name, variant_idx, &variant.fields[0]),
                ast::Style::Tuple => derive_tuple_variant(&variant_name, variant_idx, &variant.fields),
                ast::Style::Unit => derive_unit_variant(&variant_name),
            }
        })
        .fold(quote!{}, |mut agg, tokens| { agg.append_all(tokens); agg });
    
    tokens
}

fn derive_unit_variant<'a>(variant_name: &str) -> quote::Tokens {
    quote!{
        | { "tag": #variant_name, }
    }
}

fn derive_newtype_variant<'a>(
    variant_name: &str, _variant_idx: usize, 
    field: &ast::Field<'a>
) -> quote::Tokens {
    let (ty, _is_opt) = type_to_ts(&field.ty);
    quote!{
        | { "tag": #variant_name, "fields": #ty, }
    }
}

fn derive_struct_variant<'a>(
    variant_name: &str,
    variant_idx: usize,
    fields: &Vec<ast::Field<'a>>,
) -> quote::Tokens {
    let contents = collapse_list_brace(fields.into_iter().enumerate()
        .map(|(field_idx, field)| derive_field(variant_idx, field_idx, field))
        .collect::<Vec<_>>());
    quote!{
        | { "tag": #variant_name, "fields": #contents, }
    }
}

fn derive_tuple_variant<'a>(
    variant_name: &str,
    _variant_idx: usize,
    fields: &Vec<ast::Field<'a>>,
) -> quote::Tokens {
    let contents = collapse_list_bracket(fields.into_iter().enumerate()
        .map(|(element_idx, field)| derive_element(0, element_idx, &field))
        .collect::<Vec<_>>());
    quote!{
        | {"tag": #variant_name, "fields": #contents, }
    }
}
