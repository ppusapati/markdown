<script lang="ts">
  import MarkdownEditor from '$lib/editor/MarkdownEditor.svelte';
  import MarkdownPreview from '$lib/preview/MarkdownPreview.svelte';
  import { createAutosave, type AutosaveStatus } from '$lib/autosave/tauriMarkdown';

  const starter = `---
title: First Markdown workflow
tags:
  - editor
  - preview
---

# First Markdown workflow

Use the split view to edit Markdown on the left and preview it on the right.

## Checklist

- [x] Tables
- [x] Task lists
- [x] Footnotes
- [x] Code highlighting

## Table

| Feature | Status |
| --- | --- |
| Monaco editor | Ready |
| markdown-it preview | Ready |

## Code

\`\`\`ts
const greeting: string = 'Hello Markdown';
console.log(greeting);
\`\`\`

Here is a footnote.[^1]

[^1]: Footnotes render in the live preview.
`;

  let markdown = $state(starter);
  let filePath = $state('');
  let autosaveStatus = $state<AutosaveStatus>('idle');
  let autosaveError = $state('');
  const autosave = createAutosave(
    () => filePath || undefined,
    800,
    (status, error) => {
      autosaveStatus = status;
      autosaveError = error?.message ?? '';
    }
  );

  function handleChange(next: string) {
    markdown = next;
    autosave.schedule(next);
  }

  async function saveNow() {
    await autosave.flush(markdown);
  }
</script>

<svelte:head><title>Markdown editor</title></svelte:head>

<main class="workspace">
  <header class="toolbar">
    <div>
      <p class="eyebrow">Markdown</p>
      <h1>Split-view editor</h1>
    </div>
    <label>
      Tauri file path
      <input bind:value={filePath} placeholder="/path/to/document.md" />
    </label>
    <button onclick={saveNow} disabled={!filePath}>Save now</button>
    <span class={`status ${autosaveStatus}`}>{autosaveStatus}</span>
  </header>

  {#if autosaveError}<p class="error">Autosave failed: {autosaveError}</p>{/if}

  <section class="split-view">
    <MarkdownEditor value={markdown} onChange={handleChange} />
    <div class="preview-column">
      <MarkdownPreview source={markdown} />
    </div>
  </section>
</main>

<style>
  .workspace { min-height: 100vh; padding: 1.5rem; display: grid; grid-template-rows: auto 1fr; gap: 1rem; }
  .toolbar { display: flex; align-items: end; gap: 1rem; flex-wrap: wrap; background: white; border: 1px solid #d9e1f2; border-radius: 18px; padding: 1rem; }
  .eyebrow { margin: 0; color: #5870a6; text-transform: uppercase; letter-spacing: .08em; font-size: .75rem; font-weight: 700; }
  h1 { margin: .15rem 0 0; font-size: clamp(1.5rem, 3vw, 2rem); }
  label { display: grid; gap: .25rem; color: #526071; min-width: min(28rem, 100%); }
  input { border: 1px solid #c5d0e6; border-radius: 10px; padding: .65rem .8rem; }
  button { border: 0; border-radius: 10px; padding: .75rem 1rem; background: #315cba; color: white; cursor: pointer; }
  button:disabled { opacity: .45; cursor: not-allowed; }
  .status { padding: .45rem .7rem; border-radius: 999px; background: #eef3ff; color: #315cba; }
  .status.error, .error { color: #a32222; }
  .split-view { min-height: 72vh; display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1fr); gap: 1rem; }
  .preview-column { display: grid; grid-template-rows: auto minmax(0, 1fr); gap: 1rem; min-height: 0; }
  @media (max-width: 900px) { .split-view { grid-template-columns: 1fr; } }
</style>
