<script lang="ts">
  import { workspace, type WorkspaceFileKind } from '$lib/stores/workspace';

  type DropTemplate = {
    kind: WorkspaceFileKind;
    label: string;
    extension: string;
    description: string;
  };

  const templates: DropTemplate[] = [
    {
      kind: 'markdown',
      label: 'Markdown document',
      extension: '.md',
      description: 'Drag in notes, docs, and README files.'
    },
    {
      kind: 'mermaid',
      label: 'Mermaid diagram',
      extension: '.mmd',
      description: 'Drop flowcharts, sequence diagrams, and graphs.'
    }
  ];

  function handleDrop(event: DragEvent, kind: WorkspaceFileKind) {
    event.preventDefault();
    const file = event.dataTransfer?.files?.[0];
    const name = file?.name ?? (kind === 'markdown' ? 'untitled.md' : 'diagram.mmd');

    workspace.openLocalFile({
      name,
      path: `local://${name}`,
      kind,
      content: kind === 'markdown' ? '# Untitled\n\nStart writing locally.' : 'graph TD;\n  A[Start] --> B[Workspace];'
    });
  }
</script>

<aside class:collapsed={$workspace.sidebarCollapsed} aria-label="Workspace sidebar">
  <button class="collapse" type="button" on:click={workspace.toggleSidebar}>
    {$workspace.sidebarCollapsed ? '»' : '«'}
  </button>

  {#if !$workspace.sidebarCollapsed}
    <h2>Workspace</h2>
    <p class="hint">Local-only project area</p>

    <div class="drop-list">
      {#each templates as template}
        <section
          class="drop-card {template.kind}"
          on:dragover|preventDefault
          on:drop={(event) => handleDrop(event, template.kind)}
        >
          <strong>{template.label}</strong>
          <span>{template.extension}</span>
          <p>{template.description}</p>
        </section>
      {/each}
    </div>

    <h3>Recent</h3>
    {#if $workspace.recentFiles.length === 0}
      <p class="empty">No local files opened yet.</p>
    {:else}
      <ul>
        {#each $workspace.recentFiles as file}
          <li>{file.name}</li>
        {/each}
      </ul>
    {/if}
  {/if}
</aside>

<style>
  aside {
    position: relative;
    width: 18rem;
    min-width: 18rem;
    border-right: 1px solid #d9dee8;
    background: #f6f8fb;
    padding: 1rem;
    transition: width 160ms ease, min-width 160ms ease;
  }

  aside.collapsed {
    width: 3.25rem;
    min-width: 3.25rem;
    padding: 0.75rem;
  }

  .collapse {
    float: right;
    border: 1px solid #ccd4e0;
    border-radius: 0.5rem;
    background: white;
    cursor: pointer;
  }

  h2,
  h3 {
    margin: 0 0 0.5rem;
  }

  .hint,
  .empty {
    color: #667085;
    font-size: 0.9rem;
  }

  .drop-list {
    display: grid;
    gap: 0.75rem;
    margin: 1rem 0 1.5rem;
  }

  .drop-card {
    border: 1px dashed #9aa8bd;
    border-radius: 0.85rem;
    background: white;
    padding: 1rem;
  }

  .drop-card.markdown {
    border-color: #4f7cff;
  }

  .drop-card.mermaid {
    border-color: #9b5cff;
  }

  .drop-card span {
    display: inline-block;
    margin-left: 0.5rem;
    color: #667085;
    font-size: 0.8rem;
  }

  ul {
    margin: 0;
    padding-left: 1.25rem;
  }
</style>
