use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        js_namespace = ["window", "__TAURI__", "core"],
        catch
    )]
    async fn invoke(
        cmd: &str,
        args: wasm_bindgen::JsValue,
    ) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>;
}

#[derive(serde::Serialize)]
struct CalculatorArgs {
    a: f64,
    b: f64,
}

#[component]
pub fn App() -> impl IntoView {
    let (display, set_display) = signal(String::from("0"));
    let (first_operand, set_first_operand) = signal::<Option<f64>>(None);
    let (operation, set_operation) = signal::<Option<String>>(None);
    let (start_new, set_start_new) = signal(true);

    let append_digit = move |digit: char| {
        if start_new.get() || display.get() == "0" {
            set_display.set(digit.to_string());
            set_start_new.set(false);
        } else {
            set_display.update(|value| value.push(digit));
        }
    };

    let decimal = move |_| {
        if start_new.get() {
            set_display.set("0.".to_string());
            set_start_new.set(false);
        } else if !display.get().contains('.') {
            set_display.update(|value| value.push('.'));
        }
    };

    let clear = move |_| {
        set_display.set("0".to_string());
        set_first_operand.set(None);
        set_operation.set(None);
        set_start_new.set(true);
    };

    let toggle_sign = move |_| {
        if let Ok(value) = display.get().parse::<f64>() {
            if value == 0.0 {
                set_display.set("0".to_string());
            } else {
                set_display.set((-value).to_string());
            }
        }
    };

    let percent = move |_| {
        if let Ok(value) = display.get().parse::<f64>() {
            set_display.set((value / 100.0).to_string());
        }
    };

    let choose_operation = move |op: &'static str| {
        if let Ok(value) = display.get().parse::<f64>() {
            set_first_operand.set(Some(value));
            set_operation.set(Some(op.to_string()));
            set_start_new.set(true);
        }
    };

    let calculate = move |_| {
        let first = first_operand.get();
        let selected_operation = operation.get();
        let current_display = display.get();
        let new_input = start_new.get();

        let (a, operation) = match (first, selected_operation) {
            (Some(a), Some(operation)) => (a, operation),
            _ => return,
        };

        let b = if new_input {
            a
        } else {
            match current_display.parse::<f64>() {
                Ok(value) => value,
                Err(_) => {
                    set_display.set("Invalid number".to_string());
                    return;
                }
            }
        };

        spawn_local(async move {
            let args = match serde_wasm_bindgen::to_value(&CalculatorArgs { a, b }) {
                Ok(value) => value,
                Err(_) => {
                    set_display.set("Invalid arguments".to_string());
                    return;
                }
            };

            let result = invoke(&operation, args).await;

            match result {
                Ok(value) => match value.as_f64() {
                    Some(number) => {
                        set_display.set(number.to_string());
                        set_first_operand.set(None);
                        set_operation.set(None);
                        set_start_new.set(true);
                    }
                    None => {
                        set_display.set("Invalid result".to_string());
                    }
                },
                Err(error) => {
                    set_display.set(
                        error
                            .as_string()
                            .unwrap_or_else(|| "Calculation error".to_string()),
                    );
                    set_first_operand.set(None);
                    set_operation.set(None);
                    set_start_new.set(true);
                }
            }
        });
    };

    view! {
        <main class="calculator">
            <div class="calculator-title">
                "The Ultimate Handheld Digital Numeric Processing Machine"
            </div>

            <div class="display">
                {move || display.get()}
            </div>

            <div class="keypad">
                <button class="function" on:click=clear>
                    "AC"
                </button>

                <button class="function" on:click=toggle_sign>
                    "+/-"
                </button>

                <button class="function" on:click=percent>
                    "%"
                </button>

                <button class="operator" on:click=move |_| choose_operation("divide")>
                    "/"
                </button>

                <button on:click=move |_| append_digit('7')>
                    "7"
                </button>

                <button on:click=move |_| append_digit('8')>
                    "8"
                </button>

                <button on:click=move |_| append_digit('9')>
                    "9"
                </button>

                <button class="operator" on:click=move |_| choose_operation("multiply")>
                    "*"
                </button>

                <button on:click=move |_| append_digit('4')>
                    "4"
                </button>

                <button on:click=move |_| append_digit('5')>
                    "5"
                </button>

                <button on:click=move |_| append_digit('6')>
                    "6"
                </button>

                <button class="operator" on:click=move |_| choose_operation("subtract")>
                    "-"
                </button>

                <button on:click=move |_| append_digit('1')>
                    "1"
                </button>

                <button on:click=move |_| append_digit('2')>
                    "2"
                </button>

                <button on:click=move |_| append_digit('3')>
                    "3"
                </button>

                <button class="operator" on:click=move |_| choose_operation("add")>
                    "+"
                </button>

                <button class="zero" on:click=move |_| append_digit('0')>
                    "0"
                </button>

                <button on:click=decimal>
                    "."
                </button>

                <button class="equals" on:click=calculate>
                    "="
                </button>
            </div>
        </main>
    }
}