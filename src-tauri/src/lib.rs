mod commands;

use commands::workspace::WorkspaceState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(WorkspaceState::default())
        .invoke_handler(tauri::generate_handler![
            commands::workspace::open_folder,
            commands::workspace::get_workspace,
            commands::file_ops::list_directory,
            commands::file_ops::read_file,
            commands::file_ops::write_file,
            commands::file_ops::create_folder,
            commands::file_ops::rename,
            commands::file_ops::delete,
            commands::file_ops::duplicate,
            commands::file_ops::move_entry,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Tauri application");
}
