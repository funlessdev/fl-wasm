// re-export macros
pub use funless_macros::FLFunction;

// re-export error
pub use crate::error::FLError;

pub type FLResult = Result<serde_json::Value, FLError>;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use crate::function_wasm::{console_log, get_input_data, insert_response, insert_error};
    } else {
        // this code is non-functional, since functions only run in wasm32,
        // but it is needed to fix compile errors when building for other targets
        pub fn console_log(_s: &str) {}
    }
}
