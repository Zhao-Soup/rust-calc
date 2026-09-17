// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn add(a:f64,b:f64)-> f64{
    a+b
}

#[tauri::command]
fn subtract(a:f64,b:f64)-> f64{
    a-b
}

#[tauri::command]
fn multiply(a:f64, b:f64)-> f64{
    a*b
}

#[tauri::command]
fn divide(a:f64, b:f64)-> f64{
    a/b
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add, subtract,multiply,divide])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
