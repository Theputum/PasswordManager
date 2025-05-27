// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Mutex;
use tauri::State;

struct EntryList(Mutex<Vec<String>>);

#[tauri::command]
fn key(key: String, list: State<'_, EntryList>) -> Vec<String> {
    let mut entries = list.0.lock().unwrap();
    entries.push(format!("The text you inputted was: {}", key));
    entries.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(EntryList(Mutex::new(Vec::new())))
        .invoke_handler(tauri::generate_handler![key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
