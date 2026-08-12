import { writable } from 'svelte/store';

export type WorkspaceFileKind = 'markdown' | 'mermaid';

export type WorkspaceFile = {
  id: string;
  name: string;
  path: string;
  kind: WorkspaceFileKind;
  content: string;
  dirty: boolean;
  lastOpenedAt: string;
};

export type WorkspaceState = {
  openFile: WorkspaceFile | null;
  recentFiles: WorkspaceFile[];
  sidebarCollapsed: boolean;
  statusMessage: string;
  isOffline: boolean;
};

const initialState: WorkspaceState = {
  openFile: null,
  recentFiles: [],
  sidebarCollapsed: false,
  statusMessage: 'Ready — local workspace mode',
  isOffline: true
};

function createWorkspaceStore() {
  const { subscribe, set, update } = writable<WorkspaceState>(initialState);

  return {
    subscribe,
    toggleSidebar: () => update((state) => ({ ...state, sidebarCollapsed: !state.sidebarCollapsed })),
    setStatus: (statusMessage: string) => update((state) => ({ ...state, statusMessage })),
    openLocalFile: (file: Omit<WorkspaceFile, 'id' | 'dirty' | 'lastOpenedAt'>) =>
      update((state) => {
        const openFile: WorkspaceFile = {
          ...file,
          id: `${file.kind}:${file.path}`,
          dirty: false,
          lastOpenedAt: new Date().toISOString()
        };

        return {
          ...state,
          openFile,
          recentFiles: [openFile, ...state.recentFiles.filter((recent) => recent.path !== file.path)].slice(0, 8),
          statusMessage: `Opened ${file.name}`
        };
      }),
    updateOpenFileContent: (content: string) =>
      update((state) => ({
        ...state,
        openFile: state.openFile ? { ...state.openFile, content, dirty: true } : null,
        statusMessage: state.openFile ? `Editing ${state.openFile.name}` : state.statusMessage
      })),
    reset: () => set(initialState)
  };
}

export const workspace = createWorkspaceStore();

export function inferWorkspaceFileKind(name: string): WorkspaceFileKind {
  return name.toLowerCase().endsWith('.mmd') || name.toLowerCase().endsWith('.mermaid') ? 'mermaid' : 'markdown';
}
