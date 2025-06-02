// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Serialize, Deserialize};
use std::{fs, sync::Mutex};
use tauri::{AppHandle, Manager, State};

#[derive(Serialize, Deserialize, Clone)]
struct EntryList {
    entries: Vec<String>,
}

impl EntryList {
    fn new() -> Self {
        EntryList { entries: vec![] }
    }
}

struct AppState(Mutex<EntryList>);

#[tauri::command]
fn key(key: String, app: AppHandle, state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut list = state.0.lock().unwrap();
    list.entries.push(format!("The text you inputted was: {}", key));
    
    // Save to disk
    if let Err(e) = save_to_file(&app, &list.entries) {
        return Err(format!("Failed to save: {}", e));
    }
    
    Ok(list.entries.clone())
}

#[tauri::command]
fn load_entries(app: AppHandle, state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let entries = load_from_file(&app).unwrap_or_default();
    let mut list = state.0.lock().unwrap();
    list.entries = entries.clone();
    Ok(entries)
}

fn save_to_file(app: &AppHandle, entries: &Vec<String>) -> std::io::Result<()> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, format!("App data dir not found: {}", e)))?;
    
    fs::create_dir_all(&app_data_dir)?;
    let file_path = app_data_dir.join("entries.json");
    let json = serde_json::to_string(entries)?;
    fs::write(file_path, json)?;
    Ok(())
}

fn load_from_file(app: &AppHandle) -> Option<Vec<String>> {
    let app_data_dir = app.path().app_data_dir().ok()?;
    let file_path = app_data_dir.join("entries.json");
    let content = fs::read_to_string(file_path).ok()?;
    serde_json::from_str(&content).ok()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState(Mutex::new(EntryList::new())))
        .invoke_handler(tauri::generate_handler![key, load_entries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
