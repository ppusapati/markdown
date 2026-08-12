type TauriInvoke = <T>(command: string, args?: Record<string, unknown>) => Promise<T>;

async function getTauriInvoke(): Promise<TauriInvoke | null> {
  if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) {
    return null;
  }

  const tauri = await import('@tauri-apps/api/core');
  return tauri.invoke;
}

export async function openFolderPlaceholder(): Promise<string> {
  const invoke = await getTauriInvoke();

  if (!invoke) {
    return 'Open folder placeholder: Tauri command unavailable in browser preview.';
  }

  return invoke<string>('open_folder');
}

export async function openFilePlaceholder(): Promise<string> {
  const invoke = await getTauriInvoke();

  if (!invoke) {
    return 'Open file placeholder: Tauri command unavailable in browser preview.';
  }

  return invoke<string>('open_file');
}
