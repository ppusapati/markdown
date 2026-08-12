import { invoke } from '@tauri-apps/api/core';

export type EntryKind = 'file' | 'directory';

export interface WorkspaceInfo {
  rootPath: string;
  name: string;
}

export interface DirectoryEntry {
  name: string;
  path: string;
  kind: EntryKind;
  size: number;
}

export function openFolder(path: string): Promise<WorkspaceInfo> {
  return invoke('open_folder', { path });
}

export function getWorkspace(): Promise<WorkspaceInfo | null> {
  return invoke('get_workspace');
}

export function listDirectory(path = ''): Promise<DirectoryEntry[]> {
  return invoke('list_directory', { path });
}

export function readFile(path: string): Promise<string> {
  return invoke('read_file', { path });
}

export function writeFile(path: string, contents: string): Promise<void> {
  return invoke('write_file', { path, contents });
}

export function createFolder(path: string): Promise<void> {
  return invoke('create_folder', { path });
}

export function renameEntry(path: string, newName: string): Promise<string> {
  return invoke('rename', { path, newName });
}

export function deleteEntry(path: string): Promise<void> {
  return invoke('delete', { path });
}

export function duplicateEntry(path: string, destinationPath: string): Promise<void> {
  return invoke('duplicate', { path, destinationPath });
}

export function moveEntry(path: string, destinationPath: string): Promise<void> {
  return invoke('move_entry', { path, destinationPath });
}
