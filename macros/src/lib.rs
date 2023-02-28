use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;

#[proc_macro_error]
#[proc_macro_derive(FLFunction)]
pub fn derive_function(_input: TokenStream) -> TokenStream {
    quote!(
        // version of the runtime api
        pub const VERSION: i32 = 1;

        #[no_mangle]
        pub extern "C" fn __runtime_version() -> i32 {
            VERSION
        }

        #[no_mangle]
        #[cfg(target_arch = "wasm32")]
        pub extern "C" fn __invoke(req_len: i32) -> i32 {
            // get input data
            let input_data = get_input_data(req_len);

            // parse input data
            let json = serde_json::from_slice(&input_data);

            if json.is_err() {
                insert_error("input JSON not well-formatted");
                return 1;
            }
            // unwrap is safe, since we checked for errors above
            let json = json.unwrap();

            // call the user-defined function
            let res = fl_main(json);

            // handle the result
            match res {
                Ok(data) => {
                    let resp = data.to_string();
                    insert_response(&resp);
                    0
                }
                Err(e) => {
                    let errmsg = e.to_string();
                    insert_error(&errmsg);
                    1
                }
            }
        }
    )
    .into()
}
