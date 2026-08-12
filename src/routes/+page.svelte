<script lang="ts">
  const workspaceSections = ['Explorer', 'Editor', 'Preview'];
</script>

<svelte:head>
  <title>Markdown Workspace</title>
  <meta
    name="description"
    content="A cross-platform desktop workspace for writing and previewing Markdown."
  />
</svelte:head>

<main class="workspace-shell" aria-label="Markdown Workspace shell placeholder">
  <aside class="sidebar" aria-label="Workspace navigation">
    <p class="eyebrow">Markdown Workspace</p>
    <h1>Workspace Shell</h1>
    <p class="subtitle">Open folders, compose Markdown, and preview documents in one desktop workspace.</p>
  </aside>

  <section class="panels" aria-label="Planned workspace panels">
    {#each workspaceSections as section}
      <article class="panel">
        <span>{section}</span>
        <p>{section} placeholder ready for later implementation phases.</p>
      </article>
    {/each}
  </section>
</main>
