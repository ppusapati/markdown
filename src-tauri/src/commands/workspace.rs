use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct WorkspaceState {
    root: Mutex<Option<PathBuf>>,
}

impl WorkspaceState {
    pub fn set_root(&self, root: PathBuf) -> Result<(), CommandError> {
        *self.root.lock().map_err(|_| CommandError::StatePoisoned)? = Some(root);
        Ok(())
    }

    pub fn root(&self) -> Result<PathBuf, CommandError> {
        self.root
            .lock()
            .map_err(|_| CommandError::StatePoisoned)?
            .clone()
            .ok_or(CommandError::WorkspaceNotOpen)
    }
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("workspace state is unavailable")]
    StatePoisoned,
    #[error("workspace has not been opened")]
    WorkspaceNotOpen,
    #[error("path is outside the active workspace")]
    OutsideWorkspace,
    #[error("absolute paths are not allowed inside the active workspace")]
    AbsolutePath,
    #[error("parent directory traversal is not allowed")]
    ParentTraversal,
    #[error("invalid path: {0}")]
    InvalidPath(String),
    #[error("I/O error: {0}")]
    Io(String),
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<std::io::Error> for CommandError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub root_path: String,
    pub name: String,
}

#[tauri::command]
pub fn open_folder(
    path: String,
    state: tauri::State<'_, WorkspaceState>,
) -> Result<WorkspaceInfo, CommandError> {
    let root = PathBuf::from(path).canonicalize()?;

    if !root.is_dir() {
        return Err(CommandError::InvalidPath(
            "workspace root must be a directory".into(),
        ));
    }

    state.set_root(root.clone())?;
    Ok(workspace_info(root))
}

#[tauri::command]
pub fn get_workspace(
    state: tauri::State<'_, WorkspaceState>,
) -> Result<Option<WorkspaceInfo>, CommandError> {
    let guard = state.root.lock().map_err(|_| CommandError::StatePoisoned)?;
    Ok(guard.clone().map(workspace_info))
}

pub fn workspace_info(root: PathBuf) -> WorkspaceInfo {
    let name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Workspace")
        .to_string();

    WorkspaceInfo {
        root_path: root.to_string_lossy().to_string(),
        name,
    }
}
