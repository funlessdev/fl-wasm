//! imports/exports for WebAssembly functions
#![cfg(target_arch = "wasm32")]

#[link(wasm_import_module = "fl_imps")]
extern "C" {
    fn __http_request(
        response_ptr: *const u8, //where the response starts (reads from response_ptr to response_ptr + response_len)
        response_len_ptr: *const u32, // where the response length is stored (we assume the length to be a 32 bit unsigned int)
        status_ptr: *const u16,
        method: u8, // no need for a pointer here, as the value is a simple int from 0 to 3
        uri_ptr: *const u8,
        uri_len: usize,
        header_ptr: *const u8,
        header_len: usize,
        body_ptr: *const u8,
        body_len: usize,
    );
}

enum HTTPMethod {
    GET = 0,
    POST = 1,
    PUT = 2,
    DELETE = 3,
}
pub struct FLRequest {
    method: HTTPMethod,
    body: String,
    headers: Vec<(String, String)>,
    uri: String,
}

pub struct FLResponse {
    pub body: String,
    pub status: u16,
}

impl FLRequest {
    pub fn new() -> FLRequest {
        FLRequest {
            method: HTTPMethod::GET,
            body: String::from(""),
            headers: vec![],
            uri: String::from(""),
        }
    }

    pub fn with_method(mut self, method: String) -> FLRequest {
        self.method = match method.as_str() {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            "PUT" => HTTPMethod::PUT,
            "DELETE" => HTTPMethod::DELETE,
            _ => self.method,
        };
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> FLRequest {
        let couple = (key, value);
        self.headers.push(couple);
        self
    }

    pub fn with_uri(mut self, uri: String) -> FLRequest {
        self.uri = uri;
        self
    }

    pub fn with_body(mut self, body: String) -> FLRequest {
        self.body = body;
        self
    }

    /*
        Send the fully built HTTP request.
        Having a separate function for this allows us to build the request in a clean way,
        simply converting it to raw pointers when it's ready.
    */
    pub fn send(self) -> FLResponse {
        use std::slice;
        use std::str;

        let buf: Vec<u8> = Vec::new();
        let length: u32 = 0;
        let status: u16 = 0;
        let response_ptr = buf.as_ptr();

        let concat_headers = self
            .headers
            .into_iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .reduce(|tot, e| format!("{}\n{}", tot, e))
            .unwrap_or(String::new());

        let response = unsafe {
            __http_request(
                response_ptr,
                &length,
                &status,
                self.method as u8,
                self.uri.as_ptr(),
                self.uri.len(),
                concat_headers.as_ptr(),
                concat_headers.len(),
                self.body.as_ptr(),
                self.body.len(),
            );

            let size = usize::try_from(length).unwrap_or(0);

            let body_slice = slice::from_raw_parts(response_ptr, size);
            let body = str::from_utf8(body_slice).unwrap_or("").to_string();
            FLResponse { status, body }
        };
        response
    }
}
