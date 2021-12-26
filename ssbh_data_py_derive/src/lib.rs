extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, FnArg, Ident, ItemFn, Pat,
};

fn get_pyi_field_type(attrs: &[Attribute]) -> Option<String> {
    if let Ok(syn::Meta::List(l)) = attrs.iter().find(|a| a.path.is_ident("pyi"))?.parse_meta() {
        for nested in l.nested {
            // There may be multiple attributes, so just find the first matching attribute.
            // ex: #[pyi(python_type = "list[float]")] or #[pyi(python_type("list[float]"))]
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(v)) = nested {
                if v.path.get_ident().unwrap().to_string().as_str() == "python_type" {
                    if let syn::Lit::Str(s) = v.lit {
                        return Some(s.value());
                    }
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

    let formatted_fields: Vec<_> = fields
        .iter()
        .map(|f| {
            // Assume that Rust fields match Python and are not renamed by PyO3.
            let py_name = f.ident.as_ref().unwrap().to_string();

            // Use the attribute as an override for the type string if present.
            let py_type = get_pyi_field_type(&f.attrs)
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

            quote! {
                format!("    {}: {}", #py_name, #py_type)
            }
        })
        .collect();

    // TODO: There's probably a nicer way to do this using an attribute macro.
    // The macro would generate the PyiMethods implementation from the function signatures.
    let has_methods = get_has_pyi_methods(&input.attrs).unwrap_or(false);
    let impl_pyi_methods = if has_methods {
        quote! {}
    } else {
        quote! {impl crate::PyiMethods for #name { }}
    };

    let class_name = name.to_string();

    // Generate a python class string to use for type stubs (.pyi) files.
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
        .expect("Must specify a the module with a pyrepr attribute");

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

        #[pyproto]
        impl pyo3::PyObjectProtocol for #name {
            fn __repr__(&self) -> String {
                self.py_repr()
            }
        }
    };
    result.into()
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    // TODO: Can we just append something after the constructor to implement a PyRepr trait?
    println!("attr: \"{}\"", attr);
    println!("item: \"{}\"", item);

    let new_item = item.clone();
    let input = parse_macro_input!(new_item as ItemFn);
    println!(
        "{:?}",
        // Get the names and types of the function signatures.
        // TODO: Use this to build the pyi implementation?
        input
            .sig
            .inputs
            .iter()
            .map(|i| {
                if let FnArg::Typed(typed) = i {
                    if let Pat::Ident(arg_ident) = typed.pat.as_ref() {
                        Some(arg_ident.ident.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    );

    // TODO: Can we generate the implementations here?
    item
}
