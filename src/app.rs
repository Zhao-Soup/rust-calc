use leptos::task::spawn_local;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        js_namespace = ["window", "__TAURI__", "core"],
        catch
    )]
    async fn invoke(
        cmd: &str,
        args: wasm_bindgen::JsValue
    ) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>; //changed to match exception cases like /0
}

#[derive(serde::Serialize)]
struct CalculatorArgs {
    a: f64,
    b: f64,
}

#[component]
pub fn App() -> impl IntoView {
    let (a, set_a) = signal(String::new());
    let (b, set_b) = signal(String::new());
    let (result, set_result) = signal(String::new());
    let (operation, set_operation) = signal(String::from("add"));

    let calculate = move |_| {
        let a = a.get();
        let b = b.get();
        let operation = operation.get();

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&CalculatorArgs {
                a: a.parse().unwrap_or(0.0),
                b: b.parse().unwrap_or(0.0),
            })
            .unwrap();

            let result = invoke(&operation, args).await;

            match result{
                Ok(value) => set_result.set(value.as_f64().unwrap().to_string()),
                Err(error) => set_result.set(error.as_string().unwrap()),
            }
        });
    };

    view! {
        <main class="container">
            <h1>"Rust Calculator"</h1>

            <input
                type="number"
                placeholder="First number"
                on:input=move |ev| set_a.set(event_target_value(&ev))
            />

            <input
                type="number"
                placeholder="Second number"
                on:input=move |ev| set_b.set(event_target_value(&ev))
            />

            <select on:change = move |ev| set_operation.set(event_target_value(&ev))>
                <option value = "add">"+"</option>
                <option value = "subtract">"-"</option>
                <option value = "multiply">"*"</option>
                <option value = "divide">"/"</option>
            </select>
                
            <button on:click=calculate>
                "Calculate"
            </button>
            
            <p>
                "Result: " {move || result.get()}
            </p>
        </main>
    }
}