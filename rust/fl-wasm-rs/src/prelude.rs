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
        pub struct FLRequest {
            pub body: String
        }
        pub struct FLResponse {
            pub body: String,
            pub status: u16
        }
        impl<'a> FLRequest {
            pub fn new() -> FLRequest {
                FLRequest { body: String::from("") }
            }
            pub fn with_method(self, _method: String) -> FLRequest {
                self
            }

            pub fn with_header(self, _key: String, _value: String) -> FLRequest {
                self
            }

            pub fn with_uri(self, _uri: String) -> FLRequest {
                self
            }

            pub fn with_body(self, _body: String) -> FLRequest {
                self
            }

            pub fn send(self) -> FLResponse {
                FLResponse { body: String::from(""), status: 0 }
            }
        }
    }
}
