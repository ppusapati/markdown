<script lang="ts">
  import { onMount, tick } from 'svelte';
  import * as pdfjsLib from 'pdfjs-dist';
  import workerUrl from 'pdfjs-dist/build/pdf.worker.mjs?url';

  export let data: ArrayBuffer | Uint8Array | Blob | string;
  export let fileName = 'document.pdf';

  let canvasContainer: HTMLDivElement;
  let thumbnailContainer: HTMLDivElement;
  let pdf: pdfjsLib.PDFDocumentProxy | null = null;
  let pageCanvases: HTMLCanvasElement[] = [];
  let pageText: string[] = [];
  let currentPage = 1;
  let zoom = 1;
  let searchQuery = '';
  let matches: number[] = [];
  let error = '';
  let loading = true;

  pdfjsLib.GlobalWorkerOptions.workerSrc = workerUrl;

  async function toPdfData(input: typeof data): Promise<ArrayBuffer | Uint8Array | string> {
    if (input instanceof Blob) return input.arrayBuffer();
    return input;
  }

  async function loadPdf() {
    loading = true;
    error = '';
    try {
      const source = await toPdfData(data);
      pdf = await pdfjsLib.getDocument(source).promise;
      await renderAllPages();
      await buildSearchIndex();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Unable to load PDF.';
    } finally {
      loading = false;
    }
  }

  async function renderPage(pageNumber: number, scale: number, target: HTMLElement, className: string) {
    if (!pdf) return;
    const page = await pdf.getPage(pageNumber);
    const viewport = page.getViewport({ scale });
    const canvas = document.createElement('canvas');
    canvas.className = className;
    canvas.width = viewport.width;
    canvas.height = viewport.height;
    canvas.dataset.page = String(pageNumber);
    const context = canvas.getContext('2d');
    if (!context) return;
    await page.render({ canvasContext: context, viewport }).promise;
    target.appendChild(canvas);
    return canvas;
  }

  async function renderAllPages() {
    if (!pdf || !canvasContainer || !thumbnailContainer) return;
    canvasContainer.innerHTML = '';
    thumbnailContainer.innerHTML = '';
    pageCanvases = [];
    for (let pageNumber = 1; pageNumber <= pdf.numPages; pageNumber += 1) {
      const canvas = await renderPage(pageNumber, zoom, canvasContainer, 'pdf-page');
      if (canvas) pageCanvases = [...pageCanvases, canvas];
      const button = document.createElement('button');
      button.type = 'button';
      button.className = 'thumbnail-button';
      button.ariaLabel = `Go to page ${pageNumber}`;
      button.onclick = () => scrollToPage(pageNumber);
      thumbnailContainer.appendChild(button);
      await renderPage(pageNumber, 0.18, button, 'pdf-thumbnail');
    }
  }

  async function buildSearchIndex() {
    if (!pdf) return;
    pageText = [];
    for (let pageNumber = 1; pageNumber <= pdf.numPages; pageNumber += 1) {
      const page = await pdf.getPage(pageNumber);
      const textContent = await page.getTextContent();
      pageText = [...pageText, textContent.items.map((item) => 'str' in item ? item.str : '').join(' ')];
    }
    updateMatches();
  }

  function updateMatches() {
    const query = searchQuery.trim().toLowerCase();
    matches = query ? pageText.flatMap((text, index) => text.toLowerCase().includes(query) ? [index + 1] : []) : [];
  }

  function scrollToPage(pageNumber: number) {
    currentPage = pageNumber;
    pageCanvases[pageNumber - 1]?.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }

  async function setZoom(nextZoom: number) {
    zoom = Math.min(3, Math.max(0.5, nextZoom));
    await tick();
    await renderAllPages();
    scrollToPage(currentPage);
  }

  onMount(loadPdf);

  $: if (searchQuery !== undefined) updateMatches();
</script>

<section class="viewer pdf-viewer" aria-label={`PDF viewer for ${fileName}`}>
  <header class="toolbar">
    <strong>{fileName}</strong>
    <button type="button" on:click={() => setZoom(zoom - 0.25)}>-</button>
    <span>{Math.round(zoom * 100)}%</span>
    <button type="button" on:click={() => setZoom(zoom + 0.25)}>+</button>
    <input type="search" bind:value={searchQuery} placeholder="Search PDF" aria-label="Search PDF" />
    {#if matches.length}
      <button type="button" on:click={() => scrollToPage(matches[0])}>{matches.length} page match{matches.length === 1 ? '' : 'es'}</button>
    {/if}
  </header>

  {#if loading}<p>Loading PDF…</p>{/if}
  {#if error}<p class="error">{error}</p>{/if}

  <div class="layout">
    <aside class="thumbnails" bind:this={thumbnailContainer} aria-label="PDF thumbnails"></aside>
    <main class="pages" bind:this={canvasContainer}></main>
  </div>
</section>

<style>
  .toolbar, .layout { display: flex; gap: 0.75rem; align-items: center; }
  .layout { align-items: flex-start; }
  .thumbnails { width: 9rem; max-height: 70vh; overflow: auto; }
  .pages { max-height: 75vh; overflow: auto; }
  :global(.pdf-page) { display: block; margin: 0 0 1rem; box-shadow: 0 0.25rem 1rem #0002; }
  :global(.thumbnail-button) { display: block; margin-bottom: 0.5rem; padding: 0.25rem; cursor: pointer; }
  .error { color: #b00020; }
</style>
