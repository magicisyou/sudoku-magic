// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod game_data;
mod sudoku;

use game_data::DATA;
use sudoku::Sudoku;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    sync::Mutex,
};
use tauri::State;

struct GameState {
    sudoku: Mutex<Sudoku>,
}

#[tauri::command]
async fn new_game(
    game_state: State<'_, GameState>,
    index: usize,
    app_data_dir: String,
) -> Result<Sudoku, ()> {
    if let Some(sudoku) = load_from_file(index, &app_data_dir).await {
        *game_state.sudoku.lock().unwrap() = sudoku;
    } else if index < DATA.len() {
        *game_state.sudoku.lock().unwrap() = Sudoku::from_array(DATA[index - 1]);
    }
    Ok(*game_state.sudoku.lock().unwrap())
}

#[tauri::command]
async fn evaluate(
    game_state: State<'_, GameState>,
    sudoku: Sudoku,
    index: usize,
    app_data_dir: String,
) -> Result<Sudoku, ()> {
    *game_state.sudoku.lock().unwrap() = sudoku;
    game_state.sudoku.lock().unwrap().evaluate();
    let _ = save_to_file(&sudoku, index, &app_data_dir).await;
    Ok(*game_state.sudoku.lock().unwrap())
}

async fn save_to_file(
    sudoku: &Sudoku,
    index: usize,
    app_data_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("1");
    let app_data_dir = Path::new(&app_data_dir);
    fs::create_dir_all(&app_data_dir)?;
    let data_path = app_data_dir.join(format!("data{index}.toml"));
    println!("2");

    let toml_str = serde_json::to_string(&sudoku)?;
    println!("4");
    let mut file = File::create(data_path)?;
    println!("5");
    file.write_all(toml_str.as_bytes())?;
    println!("3");

    Ok(())
}

async fn load_from_file(index: usize, app_data_dir: &str) -> Option<Sudoku> {
    let app_data_dir = Path::new(app_data_dir);
    let data_path = app_data_dir.join(format!("data{index}.toml"));

    if let Ok(sudoku_string) = fs::read_to_string(data_path) {
        if let Ok(sudoku) = serde_json::from_str(&sudoku_string) {
            return Some(sudoku);
        }
    }

    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(GameState {
            sudoku: Mutex::new(Sudoku::new()),
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![new_game, evaluate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
