use fl_wasm_rs::prelude::*;
use serde::Deserialize;

#[derive(Default, FLFunction)]
struct Fn;

#[derive(Deserialize)]
struct Req {
    method: String,
    body: String,
    url: String,
}

fn fl_main(input: serde_json::Value) -> FLResult {
    match serde_json::from_value::<Req>(input) {
        Ok(r) => {
            let req: FLRequest = FLRequest::new()
                .with_method(&r.method)
                .with_body(&r.body)
                .with_uri(&r.url)
                .with_header("Content-Type", "application/json");

            let resp = req.send();
            let status = resp.status;
            let body = resp.body;

            let json: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(
                &format!(r#"{{"payload": {}, "status": "{}"}}"#, body, status),
            );

            json.map_err(|e| FLError::ExecError(e.to_string()))
        }
        Err(e) => Err(FLError::ExecError(format!("Invalid input: {}", e))),
    }
}
