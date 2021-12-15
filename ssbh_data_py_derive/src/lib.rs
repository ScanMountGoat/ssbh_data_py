extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use syn::{parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, Ident};

fn get_pyi_field_type(attrs: &[Attribute]) -> Option<String> {
    if let Ok(syn::Meta::List(l)) = attrs.iter().find(|a| a.path.is_ident("pyi"))?.parse_meta() {
        for nested in l.nested {
            // There may be multiple attributes, so just find the first matching attribute.
            // ex: #[pyi(python_type = "list[float]")] or #[pyi(python_type("list[float]"))]
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(v)) = nested {
                match v.path.get_ident().unwrap().to_string().as_str() {
                    "python_type" => match v.lit {
                        syn::Lit::Str(s) => return Some(s.value()),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }

    None
}

fn get_has_pyi_methods(attrs: &[Attribute]) -> Option<bool> {
    if let Ok(syn::Meta::List(l)) = attrs.iter().find(|a| a.path.is_ident("pyi"))?.parse_meta() {
        for nested in l.nested {
            // There may be multiple attributes, so just find the first matching attribute.
            // ex: #[pyi(has_methods = true)]
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(v)) = nested {
                match v.path.get_ident().unwrap().to_string().as_str() {
                    "has_methods" => match v.lit {
                        syn::Lit::Bool(s) => return Some(s.value()),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }

    None
}

#[proc_macro_derive(Pyi, attributes(pyi))]
pub fn pyi_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields: Vec<_> = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named.iter().collect(),
        _ => panic!("Unsupported type"),
    };

    // Assume that Rust fields match Python and are not renamed by PyO3.
    let field_names: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    // Use the attribute as an override for the type string if present.
    let field_py_types: Vec<_> = fields
        .iter()
        .map(|f| {
            get_pyi_field_type(&f.attrs)
                .map(|ty| {
                    quote! {
                        #ty
                    }
                })
                .unwrap_or({
                    let field_type = &f.ty;

                    quote! {
                        <#field_type>::py_type_string()
                    }
                })
        })
        .collect();


    // TODO: There's probably a nicer way to do this using an attribute macro.
    // The macro would generate the PyiMethods implementation from the function signatures.
    let has_methods = get_has_pyi_methods(&input.attrs).unwrap_or(false);
    let impl_pyi_methods = if has_methods { quote! {}} else {quote! {impl crate::PyiMethods for #name { }}};

    let class_name = name.to_string();

    // Generate a python class string to use for type stubs (.pyi) files.
    let expanded = quote! {
        impl crate::PyiClass for #name {
            fn pyi_class() -> String {
                let mut result = format!("class {}:\n", #class_name);
                #(
                    result += &format!("    {}: {}\n", #field_names, #field_py_types);
                )*
                result
            }
        }

        #impl_pyi_methods

        impl crate::PyTypeString for #name {
            fn py_type_string() -> String {
                #class_name.to_string()
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(MapPy, attributes(map))]
pub fn map_py_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // ex: #[map(ssbh_data::modl_data::ModlData)]
    let map_type: syn::Path = input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("map"))
        .map(|a| a.parse_args().unwrap())
        .expect("Must specify a map type");

    let name = &input.ident;

    // Assume both structs have identical field names.
    // This could be improved via skip and rename attributes in the future.
    let map_data = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let named_fields: Vec<_> = fields.named.iter().map(|field| &field.ident).collect();
            quote! {
                #(
                    #named_fields: self.#named_fields.map_py(py)?
                ),*
            }
        }
        _ => panic!("Unsupported type"),
    };

    generate_map_py(name, &map_type, &map_data).into()
}

fn generate_map_py(name: &Ident, map_type: &syn::Path, map_data: &TokenStream2) -> TokenStream2 {
    quote! {
        // Map from the implementing type to the map type.
        impl crate::MapPy<#map_type> for #name {
            fn map_py(
                &self,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<#map_type> {
                Ok(
                    #map_type {
                        #map_data
                    }
                )
            }
        }

        // Map from the map type to the implementing type.
        impl crate::MapPy<#name> for #map_type {
            fn map_py(
                &self,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<#name> {
                Ok(
                    #name {
                        #map_data
                    }
                )
            }
        }

        // Define the Rust <-> Python conversion to support the Vec <-> PyList conversion.
        impl crate::MapPy<pyo3::PyObject> for #map_type {
            fn map_py(
                &self,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
                let x: #name = self.map_py(py)?;
                Ok(x.into_py(py))
            }
        }

        impl crate::MapPy<#map_type> for pyo3::PyObject {
            fn map_py(&self, py: pyo3::Python) -> pyo3::prelude::PyResult<#map_type> {
                let x: #name = self.extract(py)?;
                x.map_py(py)
            }
        }
    }
}
