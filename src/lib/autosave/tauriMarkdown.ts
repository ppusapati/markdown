import { invoke } from '@tauri-apps/api/core';

export type AutosaveStatus = 'idle' | 'saving' | 'saved' | 'error';

export async function readMarkdownFile(path: string): Promise<string> {
  return invoke<string>('read_markdown_file', { path });
}

export async function writeMarkdownFile(path: string, contents: string): Promise<void> {
  await invoke('write_markdown_file', { path, contents });
}

export function createAutosave(path: () => string | undefined, delay = 800, onStatus?: (status: AutosaveStatus, error?: Error) => void) {
  let timer: ReturnType<typeof setTimeout> | undefined;
  let status: AutosaveStatus = 'idle';
  let error: Error | undefined;

  async function save(contents: string) {
    const filePath = path();
    if (!filePath) return;
    status = 'saving';
    error = undefined;
    onStatus?.(status);
    try {
      await writeMarkdownFile(filePath, contents);
      status = 'saved';
      onStatus?.(status);
    } catch (caught) {
      status = 'error';
      error = caught instanceof Error ? caught : new Error(String(caught));
      onStatus?.(status, error);
    }
  }

  return {
    get status() { return status; },
    get error() { return error; },
    schedule(contents: string) {
      if (timer) clearTimeout(timer);
      timer = setTimeout(() => void save(contents), delay);
    },
    flush(contents: string) {
      if (timer) clearTimeout(timer);
      return save(contents);
    }
  };
}
