extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

const INTEGER_TYPES: [&str; 12] = [
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
];

#[proc_macro_derive(Mathy)]
pub fn derive_math_functions(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let mut integer_fields = Vec::new();
    if let Data::Struct(s) = input.data {
        if let Fields::Named(namen) = s.fields {
            for field in namen.named {
                for field_type in field.clone().ty.into_token_stream() {
                    if INTEGER_TYPES.contains(&field_type.to_string().as_str()) {
                        integer_fields.push(field.clone().ident.unwrap());
                    }
                }
            }
        }
    }

    let result = if integer_fields.len() > 0 {
        let first = integer_fields.remove(0);
        quote! {
            impl #name {
                pub fn sum(&self) -> isize {
                    let mut result = self.#first as isize;
                    #(
                        result += self.#integer_fields as isize;
                    )*
                    result
                }

                pub fn max(&self) -> isize {
                    let mut result = self.#first as isize;
                    #(
                        let mut field_max = self.#integer_fields as isize;
                        if field_max > result {
                            result = field_max
                        }
                    )*
                    result
                }

                pub fn min(&self) -> isize {
                    let mut result = self.#first as isize;
                    #(
                        let mut field_max = self.#integer_fields as isize;
                        if field_max < result {
                            result = field_max
                        }
                    )*
                    result
                }

                pub fn sub(&self) -> isize {
                    let mut result = self.#first as isize;
                    #(
                        result -= self.#integer_fields as isize;
                    )*
                    result
                }

                pub fn mul(&self) -> isize {
                    let mut result = self.#first as isize;
                    #(
                        result *= self.#integer_fields as isize;
                    )*
                    result
                }

                pub fn div(&self) -> isize {
                    let mut result = self.#first as isize;
                    #(
                        result /= self.#integer_fields as isize;
                    )*
                    result
                }
            }
        }
    } else {
        quote! {}
    };

    TokenStream::from(result)
}
