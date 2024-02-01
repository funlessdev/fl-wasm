// re-export macros
pub use fl_wasm_rs_macros::FLFunction;

// re-export error
pub use crate::error::FLError;

pub type FLResult = Result<serde_json::Value, FLError>;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use crate::function_wasm::{console_log, get_input_data, insert_response, insert_error};
        pub use crate::http::{FLRequest, FLResponse};
    } else {
        // this code is non-functional, since functions only run in wasm32,
        // but it is needed to fix compile errors when building for other targets
        pub fn console_log(_s: &str) {}
        pub struct FLRequest<'a> {
            pub body: &'a str
        }
        pub struct FLResponse<'a> {
            pub body: &'a str,
            pub status: u16
        }
        impl<'a> FLRequest<'a> {
            pub fn new() -> FLRequest<'a> {
                FLRequest { body: "" }
            }
            pub fn with_method(self, _method: &'a str) -> FLRequest<'a> {
                self
            }

            pub fn with_header(self, _key: &'a str, _value: &'a str) -> FLRequest<'a> {
                self
            }

            pub fn with_uri(self, _uri: &'a str) -> FLRequest<'a> {
                self
            }

            pub fn with_body(self, _body: &'a str) -> FLRequest<'a> {
                self
            }

            pub fn send(self) -> FLResponse<'a> {
                FLResponse { body: "", status: 0 }
            }
        }
    }
}
