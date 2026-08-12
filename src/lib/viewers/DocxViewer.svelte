<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { renderAsync } from 'docx-preview';

  export let data: ArrayBuffer | Blob | Uint8Array;
  export let fileName = 'document.docx';

  let container: HTMLDivElement;
  let searchQuery = '';
  let headings: { id: string; text: string }[] = [];
  let matches: HTMLElement[] = [];
  let error = '';
  let loading = true;

  async function toBlob(input: typeof data): Promise<Blob> {
    if (input instanceof Blob) return input;
    return new Blob([input], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
  }

  async function loadDocx() {
    loading = true;
    try {
      container.innerHTML = '';
      await renderAsync(await toBlob(data), container, undefined, { inWrapper: true, ignoreFonts: false });
      await tick();
      headings = Array.from(container.querySelectorAll('h1,h2,h3,h4,h5,h6')).map((heading, index) => {
        const id = `docx-heading-${index + 1}`;
        heading.id = id;
        return { id, text: heading.textContent?.trim() || `Heading ${index + 1}` };
      });
      updateMatches();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Unable to load Word document.';
    } finally {
      loading = false;
    }
  }

  function updateMatches() {
    const query = searchQuery.trim().toLowerCase();
    matches = query ? Array.from(container?.querySelectorAll('p, h1, h2, h3, h4, h5, h6, li') ?? [])
      .filter((element): element is HTMLElement => element instanceof HTMLElement && (element.textContent ?? '').toLowerCase().includes(query)) : [];
  }

  function scrollTo(id: string) {
    document.getElementById(id)?.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }

  onMount(loadDocx);
  $: if (searchQuery !== undefined) updateMatches();
</script>

<section class="viewer docx-viewer" aria-label={`Word viewer for ${fileName}`}>
  <header class="toolbar">
    <strong>{fileName}</strong>
    <input type="search" bind:value={searchQuery} placeholder="Search Word document" aria-label="Search Word document" />
    {#if matches.length}<button type="button" on:click={() => matches[0].scrollIntoView({ behavior: 'smooth' })}>{matches.length} match{matches.length === 1 ? '' : 'es'}</button>{/if}
  </header>
  {#if loading}<p>Loading Word document…</p>{/if}
  {#if error}<p class="error">{error}</p>{/if}
  <div class="layout">
    <nav aria-label="Document headings">
      <h2>Headings</h2>
      {#each headings as heading}
        <button type="button" on:click={() => scrollTo(heading.id)}>{heading.text}</button>
      {:else}
        <p>No headings found.</p>
      {/each}
    </nav>
    <article class="document" bind:this={container}></article>
  </div>
</section>

<style>
  .toolbar, .layout { display: flex; gap: 1rem; align-items: flex-start; }
  .toolbar { align-items: center; margin-bottom: 1rem; }
  nav { min-width: 14rem; max-height: 70vh; overflow: auto; }
  nav button { display: block; margin-bottom: 0.5rem; text-align: left; }
  .document { max-height: 75vh; overflow: auto; padding: 1rem; border: 1px solid #ddd; }
  .error { color: #b00020; }
</style>
