extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};

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

    let expanded = generate_map_py(name, &map_type, &map_data);
    TokenStream::from(expanded)
}

fn generate_map_py(name: &Ident, map_type: &syn::Path, map_data: &TokenStream2) -> TokenStream2 {
    let expanded = quote! {
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
    };
    expanded
}
