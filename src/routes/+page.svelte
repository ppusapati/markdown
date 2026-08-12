<script lang="ts">
  import { workspace } from '$lib/stores/workspace';
</script>

<section class="editor-stage">
  {#if $workspace.openFile}
    <div class="document-card">
      <div class="document-heading">
        <div>
          <p class="label">{$workspace.openFile.kind}</p>
          <h1>{$workspace.openFile.name}</h1>
        </div>
        <span>{$workspace.openFile.dirty ? 'Unsaved local changes' : 'Local draft'}</span>
      </div>
      <textarea
        aria-label="Editor"
        value={$workspace.openFile.content}
        on:input={(event) => workspace.updateOpenFileContent(event.currentTarget.value)}
      />
    </div>
  {:else}
    <div class="empty-state">
      <p class="label">Offline-first</p>
      <h1>Open or drop a Markdown or Mermaid file to begin.</h1>
      <p>Your first workspace shell is local-only, with Tauri command hooks ready for native file access.</p>
    </div>
  {/if}
</section>

<style>
  .editor-stage {
    min-height: 100%;
    padding: 2rem;
  }

  .empty-state,
  .document-card {
    background: white;
    border: 1px solid #d9dee8;
    border-radius: 1rem;
    box-shadow: 0 20px 45px rgba(16, 24, 40, 0.08);
    margin: 0 auto;
    max-width: 56rem;
    padding: 2rem;
  }

  .label {
    color: #4f7cff;
    font-size: 0.8rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    margin: 0;
    text-transform: uppercase;
  }

  h1 {
    margin: 0.25rem 0 1rem;
  }

  .document-heading {
    align-items: start;
    display: flex;
    justify-content: space-between;
    gap: 1rem;
  }

  .document-heading span {
    color: #667085;
    font-size: 0.9rem;
  }

  textarea {
    border: 1px solid #ccd4e0;
    border-radius: 0.75rem;
    box-sizing: border-box;
    font: 0.95rem/1.6 ui-monospace, SFMono-Regular, Menlo, monospace;
    min-height: 24rem;
    padding: 1rem;
    resize: vertical;
    width: 100%;
  }
</style>
