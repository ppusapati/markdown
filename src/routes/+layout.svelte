<script lang="ts">
  import BreadcrumbNav from '$lib/components/BreadcrumbNav.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import { openFilePlaceholder, openFolderPlaceholder } from '$lib/tauri';
  import { workspace } from '$lib/stores/workspace';

  async function openFolder() {
    workspace.setStatus(await openFolderPlaceholder());
  }

  async function openFile() {
    workspace.setStatus(await openFilePlaceholder());
  }
</script>

<div class="app-shell">
  <header class="topbar">
    <div>
      <p class="eyebrow">Markdown Workspace</p>
      <BreadcrumbNav />
    </div>
    <div class="actions">
      <button type="button" on:click={openFolder}>Open Folder</button>
      <button type="button" on:click={openFile}>Open File</button>
    </div>
  </header>

  <div class="workspace-frame">
    <Sidebar />
    <main>
      <slot />
    </main>
  </div>

  <StatusBar />
</div>

<style>
  :global(body) {
    margin: 0;
    background: #eef2f7;
    color: #1d2939;
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  }

  :global(button) {
    font: inherit;
  }

  .app-shell {
    display: grid;
    grid-template-rows: auto 1fr auto;
    height: 100vh;
  }

  .topbar {
    align-items: center;
    background: white;
    border-bottom: 1px solid #d9dee8;
    display: flex;
    justify-content: space-between;
    padding: 0.85rem 1rem;
  }

  .eyebrow {
    color: #4f7cff;
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    margin: 0 0 0.2rem;
    text-transform: uppercase;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .actions button {
    background: #1d2939;
    border: 0;
    border-radius: 0.6rem;
    color: white;
    cursor: pointer;
    padding: 0.55rem 0.85rem;
  }

  .workspace-frame {
    display: flex;
    min-height: 0;
  }

  main {
    flex: 1;
    min-width: 0;
    overflow: auto;
  }
</style>
