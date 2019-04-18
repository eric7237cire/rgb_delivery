use ::{quote, derive_field_str};
use serde_derive_internals::{ast, attr};
use ::{collapse_list_bracket, type_to_ts_string};

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
                    derive_struct_variant(&variant_name,  &variant.fields).to_string()
                }
                ast::Style::Newtype => derive_new_type_variant(&variant_name, variant_idx, &variant.fields).to_string(),
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

fn derive_new_type_variant<'a>(
    variant_name: &str, _variant_idx: usize,
    fields: &Vec<ast::Field<'a>>,
) -> String {
    /*
    let contents = fields.into_iter().enumerate()
        .map(|(field_idx, field)| derive_field_str(variant_idx, field_idx, field))
        .collect::<Vec<_>>().join(", ");*/

    let (ty, _is_opt) = type_to_ts_string(&fields[0].ty);
    //export type TileRoad = {type: "TileRoad"} & Road
    //Use intersection types to combine the 2
    format!("export type {} = {{type: \"{0}\"}} & {}", variant_name, ty)
}

fn derive_struct_variant<'a>(
    variant_name: &str,

    fields: &Vec<ast::Field<'a>>,
) -> String {
    let contents = fields.into_iter()
        .filter_map(|field| {
            if field.attrs.skip_serializing()  {
                None
            } else {
                Some(derive_field_str( field))
            }
        })
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
