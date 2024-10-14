// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod game_data;
mod sudoku;

use game_data::DATA;
use sudoku::Sudoku;

use std::sync::Mutex;
use tauri::State;

struct GameState {
    sudoku: Mutex<Sudoku>,
}

#[tauri::command]
fn new_game(game_state: State<GameState>, index: usize) -> Sudoku {
    if index < DATA.len() {
        *game_state.sudoku.lock().unwrap() = DATA[index - 1];
    }
    *game_state.sudoku.lock().unwrap()
}

#[tauri::command]
fn evaluate(game_state: State<GameState>) -> bool {
    game_state.sudoku.lock().unwrap().is_completed()
}

#[tauri::command]
fn input_sync(game_state: State<GameState>, index: (usize, usize), value: u32) {
    game_state.sudoku.lock().unwrap().values[index.0][index.1] = value;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(GameState {
            sudoku: Mutex::new(Sudoku::new()),
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![new_game, evaluate, input_sync])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
