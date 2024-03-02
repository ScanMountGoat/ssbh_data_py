extern crate proc_macro;

use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use syn::{parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, Ident};

fn find_string_literal_attr(attrs: &[Attribute], outer: &str, inner: &str) -> Option<String> {
    if let Ok(syn::Meta::List(l)) = attrs.iter().find(|a| a.path.is_ident(outer))?.parse_meta() {
        for nested in l.nested {
            // There may be multiple attributes, so just find the first matching attribute.
            // ex: #[outer(inner = "list[float]")] or #[outer(inner("list[float]"))]
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(v)) = nested {
                if v.path.get_ident().unwrap().to_string().as_str() == inner {
                    if let syn::Lit::Str(s) = v.lit {
                        return Some(s.value());
                    }
                }
            }
        }
    }

    None
}

// TODO: This won't be required once types like ModlData can have derived init methods.
fn get_has_pyi_methods(attrs: &[Attribute]) -> Option<bool> {
    if let Ok(syn::Meta::List(l)) = attrs.iter().find(|a| a.path.is_ident("pyi"))?.parse_meta() {
        for nested in l.nested {
            // There may be multiple attributes, so just find the first matching attribute.
            // ex: #[pyi(has_methods = true)]
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(v)) = nested {
                if v.path.get_ident().unwrap().to_string().as_str() == "has_methods" {
                    if let syn::Lit::Bool(s) = v.lit {
                        return Some(s.value());
                    }
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

    // We need extra indentation here for the methods within a class.
    let formatted_fields = format_fields(&fields, 8, true);
    let has_methods = get_has_pyi_methods(&input.attrs).unwrap_or(false);
    let impl_pyi_methods = if has_methods {
        quote! {}
    } else {
        quote! {
            impl crate::PyiMethods for #name {
                fn pyi_methods() -> String {
                    format!("    def __init__(\n        self,\n{}\n    ) -> None: ...", &[#(#formatted_fields),*].join(",\n"))
                }
            }
        }
    };

    let class_name = name.to_string();

    // Generate a python class string to use for type stubs (.pyi) files.
    let formatted_fields = format_fields(&fields, 4, false);
    let expanded = quote! {
        impl crate::PyiClass for #name {
            fn pyi_class() -> String {
                format!("class {}:\n{}", #class_name, &[#(#formatted_fields),*].join("\n"))
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

fn format_fields(
    fields: &[&syn::Field],
    indent: usize,
    include_defaults: bool,
) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|f| {
            // Assume that Rust fields match Python and are not renamed by PyO3.
            let py_name = f.ident.as_ref().unwrap().to_string();

            // Use the attribute as an override for the type string if present.
            let py_type = find_string_literal_attr(&f.attrs, "pyi", "python_type")
                .map(|ty| {
                    quote! {
                        #ty
                    }
                })
                .unwrap_or({
                    let field_type = &f.ty;

                    quote! {
                        <#field_type as crate::PyTypeString>::py_type_string()
                    }
                });

            // We only want the default values for method parameters.
            match (
                find_string_literal_attr(&f.attrs, "pyi", "default"),
                include_defaults,
            ) {
                (Some(default), true) => {
                    quote! {
                        format!("{}{}: {} = {}", " ".repeat(#indent), #py_name, #py_type, #default)
                    }
                }
                _ => {
                    quote! {
                        format!("{}{}: {}", " ".repeat(#indent), #py_name, #py_type)
                    }
                }
            }
        })
        .collect()
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
                    #named_fields: self.#named_fields.map_py(py, use_numpy)?
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
                use_numpy: bool
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
                use_numpy: bool
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
                use_numpy: bool
            ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
                let x: #name = self.map_py(py, use_numpy)?;
                Ok(x.into_py(py))
            }
        }

        impl crate::MapPy<#map_type> for pyo3::PyObject {
            fn map_py(&self, py: pyo3::Python, use_numpy: bool) -> pyo3::prelude::PyResult<#map_type> {
                let x: #name = self.extract(py)?;
                x.map_py(py, use_numpy)
            }
        }
    }
}

#[proc_macro_derive(PyRepr, attributes(pyrepr))]
pub fn py_repr_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // ex: #[pyrepr("ssbh_data_py.matl_data")]
    let module: syn::LitStr = input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("pyrepr"))
        .map(|a| a.parse_args().unwrap())
        .expect("Must specify the module with a pyrepr attribute");

    let name = &input.ident;

    // For the repr, assume there is a constructor with all fields.
    // We can simply call the py_repr function on all fields in order.
    // The trait allows use to handle nested types automatically.
    let field_reprs: Vec<_> = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields
            .named
            .iter()
            .map(|field| &field.ident)
            .map(|i| {
                quote! {
                    self.#i.py_repr()
                }
            })
            .collect(),
        _ => panic!("Unsupported type"),
    };

    let format_string = format!(
        "{}.{}({})",
        module.value(),
        name,
        vec!["{}"; field_reprs.len()].join(", ")
    );
    let result: TokenStream2 = quote! {
        impl crate::PyRepr for #name {
            fn py_repr(&self) -> String {
                format!(#format_string, #(#field_reprs),*)
            }
        }
    };
    result.into()
}

#[proc_macro_derive(PyInit, attributes(pyinit))]
pub fn py_init_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields: Vec<_> = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named.iter().collect(),
        _ => panic!("Unsupported type"),
    };

    // PyO3 treats parameters of type Option<T> as optional parameters in Python.
    // This allows Class(), Class(param=value), and Class(param=None) in Python.
    // This avoids complications trying to generate the #[args(...)] attribute.
    let field_params: Vec<_> = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            // Make the parameter Option<T> to support defaults.
            let default = find_string_literal_attr(&f.attrs, "pyinit", "default");
            let field_type = &f.ty;
            let field_type = if default.is_some() {
                quote! {Option<#field_type>}
            } else {
                quote! {#field_type}
            };
            quote! {
                #field_name: #field_type
            }
        })
        .collect();

    // The field is initialized as "field: field.unwrap_or(default)" or "field".
    // TODO: This code is shared with above.
    let field_names: Vec<_> = fields
        .iter()
        .map(|f| {
            let name = &f.ident;

            // HACK: Use string literals to avoid parsing the default value tokens.
            // This allows using Rust syntax like "Default::default()".
            let default = find_string_literal_attr(&f.attrs, "pyinit", "default")
                .map(|s| TokenStream2::from_str(&s).unwrap());

            default
                .map(|default| quote! { #name: #name.unwrap_or(#default) })
                .unwrap_or(quote! {#name})
        })
        .collect();

    let expanded = quote! {
        #[allow(clippy::too_many_arguments)]
        #[pymethods]
        impl #name {
            #[new]
            fn new(
                py: Python,
                #(#field_params),*
            ) -> PyResult<Self> {
                Ok(Self {
                    #(#field_names),*
                })
            }

            fn __repr__(&self) -> String {
                self.py_repr()
            }
        }
    };

    expanded.into()
}
