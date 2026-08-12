import { invoke } from '@tauri-apps/api/core';

export type SearchHit = {
  path: string;
  title: string;
  snippet: string;
  score: number;
  modified?: string;
};

export class WorkspaceSearch {
  constructor(private workspaceRoot: string) {}

  buildIndex() {
    return invoke<number>('build_search_index', { request: { workspaceRoot: this.workspaceRoot } });
  }

  updateIndex(changedPaths: string[]) {
    return invoke<number>('update_search_index', { request: { workspaceRoot: this.workspaceRoot, changedPaths } });
  }

  query(query: string, limit = 20) {
    return invoke<SearchHit[]>('query_search_index', { request: { workspaceRoot: this.workspaceRoot, query, limit } });
  }
}
