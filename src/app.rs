use leptos::task::spawn_local;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue;
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

    let calculate = move |_| {
        let a = a.get();
        let b = b.get();

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&CalculatorArgs {
                a: a.parse().unwrap_or(0.0),
                b: b.parse().unwrap_or(0.0),
            })
            .unwrap();

            let result = invoke("add", args).await;
            set_result.set(result.as_f64().unwrap().to_string());
        });
    };

    let subtract = move |_| {
        let a = a.get();
        let b = b.get();

        spawn_local(async move{
            let args = serde_wasm_bindgen::to_value(&CalculatorArgs{
                a: a.parse().unwrap_or(0.0),
                b: b.parse().unwrap_or(0.0),
            })
            .unwrap();

            let result = invoke("subtract", args).await;
            set_result.set(result.as_f64().unwrap().to_string());
        });
    };

    let multiply = move |_| {
        let a = a.get();
        let b = b.get();

        spawn_local(async move{
            let args = serde_wasm_bindgen::to_value(&CalculatorArgs{
                a: a.parse().unwrap_or(0.0),
                b: b.parse().unwrap_or(0.0),
            })
            .unwrap();
            
            let result = invoke("multiply", args).await;
            set_result.set(result.as_f64().unwrap().to_string());
        });
    };

    let divide = move |_| {
        let a = a.get();
        let b = b.get();

        spawn_local(async move{
            let args = serde_wasm_bindgen::to_value(&CalculatorArgs{
                a: a.parse().unwrap_or(0.0),
                b: b.parse().unwrap_or(0.0),
            })
            .unwrap();

            let result = invoke("divide", args).await;
            set_result.set(result.as_f64().unwrap().to_string());
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

            <button on:click=calculate>
                "Add"
            </button>

            <button on:click=subtract>
                "Subtract"
            </button>
            
            <button on:click=multiply>
                "multiply"
            </button>

            <button on:click=divide>
                "divide"
            </button>

            <p>
                "Result: " {move || result.get()}
            </p>
        </main>
    }
}