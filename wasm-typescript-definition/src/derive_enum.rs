use ::{quote, derive_field_str};
use serde_derive_internals::{ast, attr};
use collapse_list_bracket;

use type_to_ts;
use super::{derive_element};

pub fn derive_enum(
    variants: Vec<ast::Variant>,
    _attr_container: &attr::Container,
) -> String {

    //Create a string union type for all the tags (discriminator column)

    let quoted_types = variants.iter().map( |variant| {
        let variant_name = variant.attrs.name().serialize_name();
        format!( r#""{}""#, variant_name)
    }).collect::<Vec<String>>().join(" | ");

    let types = variants.iter().map( |variant| {
        let variant_name = variant.attrs.name().serialize_name();
        variant_name
    }).collect::<Vec<String>>().join(" | ");

    let tokens = variants.into_iter().enumerate()
        .map(|(variant_idx, variant)| {
            let variant_name = variant.attrs.name().serialize_name();
            match variant.style {
                ast::Style::Struct => {
                    derive_struct_variant(&variant_name, variant_idx, &variant.fields).to_string()
                }
                ast::Style::Newtype => derive_newtype_variant(&variant_name, variant_idx, &variant.fields[0]).to_string(),
                ast::Style::Tuple => derive_tuple_variant(&variant_name, variant_idx, &variant.fields).to_string(),
                ast::Style::Unit => derive_unit_variant(&variant_name),
            }
        }).collect::<Vec<String>>().join("\n");

    format!("export type {enum_name} = {1}\n{0}\nexport type {enum_name}_type = {2}\n", tokens,types,quoted_types,
            enum_name=_attr_container.name().serialize_name() )

}

fn derive_unit_variant<'a>(variant_name: &str) -> String {
    format!("export type {} = {{type: \"{0}\" }}", variant_name)
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
) -> String {
    let contents = fields.into_iter().enumerate()
        .map(|(field_idx, field)| derive_field_str(variant_idx, field_idx, field))
        .collect::<Vec<_>>().join(", ");

    format!("export type {0} = {{type: \"{0}\", {1}}};", variant_name, contents)
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
