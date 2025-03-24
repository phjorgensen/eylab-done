use std::sync::Mutex;
use task::ListCrud;
use tauri::{Manager, State};
use uuid::Uuid;
mod task;

#[derive(Default)]
struct AppData {
    task_list: task::TaskList,
}

#[tauri::command]
fn get_tasks(state: State<'_, Mutex<AppData>>) -> Vec<task::Task> {
    let state = state.lock().unwrap();
    return state.task_list.tasks.clone();
}

#[tauri::command]
fn add_task(state: State<'_, Mutex<AppData>>, new_task: String) -> bool {
    let mut state = state.lock().unwrap();
    state.task_list.tasks.push(task::create_task(new_task));
    return true;
}

#[tauri::command]
fn complete_task(state: State<'_, Mutex<AppData>>, task_id: String, completed_by: String) -> bool {
    let mut state = state.lock().unwrap();
    return true;
}

#[tauri::command]
fn delete_task(state: State<'_, Mutex<AppData>>, task_id: String) -> bool {
    let mut state = state.lock().unwrap();

    match Uuid::parse_str(&task_id) {
        Ok(uuid) => {
            state.task_list.delete(uuid);
            true
        }
        Err(_e) => false,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data = Mutex::new(AppData {
        task_list: task::create_task_list(vec![
            task::create_task(String::from("Walk Eyla")),
            task::create_task(String::from("Give Eyla dinner")),
            task::create_task(String::from("Walk Eyla again!")),
        ]),
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_tasks,
            add_task,
            complete_task,
            delete_task
        ])
        .setup(|app| {
            app.manage(data);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
