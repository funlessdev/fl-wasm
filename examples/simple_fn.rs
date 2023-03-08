use fl_wasm_rs::prelude::*;
use serde::Deserialize;

#[derive(Default, FLFunction)]
struct MyFn;

#[derive(Default, Deserialize)]
struct Person {
    name: String,
}

fn fl_main(input: serde_json::Value) -> FLResult {
    let parsed_input = serde_json::from_value::<Person>(input);

    match parsed_input {
        Ok(person) => {
            let out = format!("Hello {}!", person.name);
            let json: Result<serde_json::Value, serde_json::Error> =
                serde_json::from_str(&format!(r#"{{"payload": "{}" }}"#, &out));
            json.map_err(|_| FLError::ExecError("Failed to parse JSON".to_string()))
        }
        Err(e) => Err(FLError::ExecError(format!("Failed to parse input: {}", e))),
    }
}
