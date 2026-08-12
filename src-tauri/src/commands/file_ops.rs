use super::workspace::{CommandError, WorkspaceState};
use serde::Serialize;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryEntry {
    pub name: String,
    pub path: String,
    pub kind: EntryKind,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EntryKind {
    File,
    Directory,
}

#[tauri::command]
pub fn list_directory(
    path: String,
    state: tauri::State<'_, WorkspaceState>,
) -> Result<Vec<DirectoryEntry>, CommandError> {
    let root = state.root()?;
    let directory = resolve_existing(&root, &path)?;

    if !directory.is_dir() {
        return Err(CommandError::InvalidPath("path must be a directory".into()));
    }

    let mut entries = fs::read_dir(directory)?
        .map(|entry| {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let path = entry.path();
            let kind = if metadata.is_dir() {
                EntryKind::Directory
            } else {
                EntryKind::File
            };
            let name = entry.file_name().to_string_lossy().to_string();
            let relative_path = relative_workspace_path(&root, &path)?;

            Ok(DirectoryEntry {
                name,
                path: relative_path,
                kind,
                size: metadata.len(),
            })
        })
        .collect::<Result<Vec<_>, CommandError>>()?;

    entries.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
    Ok(entries)
}

#[tauri::command]
pub fn read_file(
    path: String,
    state: tauri::State<'_, WorkspaceState>,
) -> Result<String, CommandError> {
    let root = state.root()?;
    let file = resolve_existing(&root, &path)?;

    if !file.is_file() {
        return Err(CommandError::InvalidPath("path must be a file".into()));
    }

    Ok(fs::read_to_string(file)?)
}

#[tauri::command]
pub fn write_file(
    path: String,
    contents: String,
    state: tauri::State<'_, WorkspaceState>,
) -> Result<(), CommandError> {
    let root = state.root()?;
    let file = resolve_for_write(&root, &path)?;

    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(file, contents)?;
    Ok(())
}

#[tauri::command]
pub fn create_folder(
    path: String,
    state: tauri::State<'_, WorkspaceState>,
) -> Result<(), CommandError> {
    let root = state.root()?;
    let directory = resolve_for_write(&root, &path)?;
    fs::create_dir_all(directory)?;
    Ok(())
}

#[tauri::command]
pub fn rename(
    path: String,
    new_name: String,
    state: tauri::State<'_, WorkspaceState>,
) -> Result<String, CommandError> {
    validate_entry_name(&new_name)?;
    let root = state.root()?;
    let source = resolve_existing(&root, &path)?;
    let destination = source
        .parent()
        .ok_or_else(|| CommandError::InvalidPath("cannot rename workspace root".into()))?
        .join(new_name);
    ensure_inside_workspace(&root, &destination)?;

    fs::rename(&source, &destination)?;
    relative_workspace_path(&root, &destination)
}

#[tauri::command]
pub fn delete(path: String, state: tauri::State<'_, WorkspaceState>) -> Result<(), CommandError> {
    let root = state.root()?;
    let target = resolve_existing(&root, &path)?;

    if target == root {
        return Err(CommandError::InvalidPath(
            "cannot delete workspace root".into(),
        ));
    }

    if target.is_dir() {
        fs::remove_dir_all(target)?;
    } else {
        fs::remove_file(target)?;
    }

    Ok(())
}

#[tauri::command]
pub fn duplicate(
    path: String,
    destination_path: String,
    state: tauri::State<'_, WorkspaceState>,
) -> Result<(), CommandError> {
    let root = state.root()?;
    let source = resolve_existing(&root, &path)?;
    let destination = resolve_for_write(&root, &destination_path)?;

    ensure_not_inside_source(&source, &destination)?;

    if source.is_dir() {
        copy_directory(&source, &destination)?;
    } else {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, destination)?;
    }

    Ok(())
}

#[tauri::command]
pub fn move_entry(
    path: String,
    destination_path: String,
    state: tauri::State<'_, WorkspaceState>,
) -> Result<(), CommandError> {
    let root = state.root()?;
    let source = resolve_existing(&root, &path)?;
    let destination = resolve_for_write(&root, &destination_path)?;

    if source == root {
        return Err(CommandError::InvalidPath(
            "cannot move workspace root".into(),
        ));
    }

    ensure_not_inside_source(&source, &destination)?;

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::rename(source, destination)?;
    Ok(())
}

fn resolve_existing(root: &Path, workspace_path: &str) -> Result<PathBuf, CommandError> {
    let target = root
        .join(validate_relative_path(workspace_path)?)
        .canonicalize()?;
    ensure_inside_workspace(root, &target)?;
    Ok(target)
}

fn resolve_for_write(root: &Path, workspace_path: &str) -> Result<PathBuf, CommandError> {
    let relative = validate_relative_path(workspace_path)?;
    let target = root.join(relative);
    let parent = target
        .parent()
        .ok_or_else(|| CommandError::InvalidPath("path has no parent".into()))?;
    let canonical_parent = parent.canonicalize()?;
    ensure_inside_workspace(root, &canonical_parent)?;
    Ok(canonical_parent.join(
        target
            .file_name()
            .ok_or_else(|| CommandError::InvalidPath("path has no file name".into()))?,
    ))
}

fn validate_relative_path(path: &str) -> Result<PathBuf, CommandError> {
    let path = Path::new(path);

    if path.is_absolute() {
        return Err(CommandError::AbsolutePath);
    }

    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => clean.push(part),
            Component::CurDir => {}
            Component::ParentDir => return Err(CommandError::ParentTraversal),
            Component::RootDir | Component::Prefix(_) => return Err(CommandError::AbsolutePath),
        }
    }

    Ok(clean)
}

fn validate_entry_name(name: &str) -> Result<(), CommandError> {
    if name.trim().is_empty() || Path::new(name).components().count() != 1 {
        return Err(CommandError::InvalidPath(
            "name must be a single path segment".into(),
        ));
    }

    Ok(())
}

fn ensure_inside_workspace(root: &Path, target: &Path) -> Result<(), CommandError> {
    let root = root.canonicalize()?;
    if target.starts_with(root) {
        Ok(())
    } else {
        Err(CommandError::OutsideWorkspace)
    }
}

fn ensure_not_inside_source(source: &Path, destination: &Path) -> Result<(), CommandError> {
    if source.is_dir() && destination.starts_with(source) {
        return Err(CommandError::InvalidPath(
            "destination cannot be inside the source directory".into(),
        ));
    }

    Ok(())
}

fn relative_workspace_path(root: &Path, target: &Path) -> Result<String, CommandError> {
    let target = target.canonicalize()?;
    ensure_inside_workspace(root, &target)?;
    Ok(target
        .strip_prefix(root.canonicalize()?)
        .map_err(|_| CommandError::OutsideWorkspace)?
        .to_string_lossy()
        .replace('\\', "/"))
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), CommandError> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let destination_entry = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &destination_entry)?;
        } else {
            fs::copy(entry.path(), destination_entry)?;
        }
    }

    Ok(())
}
