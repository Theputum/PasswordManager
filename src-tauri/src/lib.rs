// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn key(key: &str) -> String {
    format!("The text you inputted was: {}", key)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
