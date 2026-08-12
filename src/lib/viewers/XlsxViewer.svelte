<script lang="ts">
  import * as XLSX from 'xlsx';
  import { onMount } from 'svelte';

  export let data: ArrayBuffer | Uint8Array | Blob;
  export let fileName = 'workbook.xlsx';

  let workbook: XLSX.WorkBook | null = null;
  let activeSheet = '';
  let rows: unknown[][] = [];
  let searchQuery = '';
  let error = '';
  let loading = true;

  async function toArrayBuffer(input: typeof data): Promise<ArrayBuffer | Uint8Array> {
    if (input instanceof Blob) return input.arrayBuffer();
    return input;
  }

  async function loadWorkbook() {
    loading = true;
    try {
      workbook = XLSX.read(await toArrayBuffer(data), { type: 'array' });
      activeSheet = workbook.SheetNames[0] ?? '';
      selectSheet(activeSheet);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Unable to load Excel workbook.';
    } finally {
      loading = false;
    }
  }

  function selectSheet(sheetName: string) {
    activeSheet = sheetName;
    const sheet = workbook?.Sheets[sheetName];
    rows = sheet ? XLSX.utils.sheet_to_json<unknown[]>(sheet, { header: 1, blankrows: false }) : [];
  }

  $: filteredRows = searchQuery.trim()
    ? rows.filter((row) => row.some((cell) => String(cell ?? '').toLowerCase().includes(searchQuery.trim().toLowerCase())))
    : rows;

  onMount(loadWorkbook);
</script>

<section class="viewer xlsx-viewer" aria-label={`Excel viewer for ${fileName}`}>
  <header class="toolbar">
    <strong>{fileName}</strong>
    <input type="search" bind:value={searchQuery} placeholder="Search active sheet" aria-label="Search active sheet" />
  </header>
  {#if loading}<p>Loading workbook…</p>{/if}
  {#if error}<p class="error">{error}</p>{/if}
  <div class="layout">
    <nav aria-label="Workbook sheets">
      <h2>Sheets</h2>
      {#each workbook?.SheetNames ?? [] as sheetName}
        <button type="button" class:active={sheetName === activeSheet} on:click={() => selectSheet(sheetName)}>{sheetName}</button>
      {/each}
    </nav>
    <div class="table-wrap">
      <table>
        <tbody>
          {#each filteredRows as row}
            <tr>{#each row as cell}<td>{cell ?? ''}</td>{/each}</tr>
          {:else}
            <tr><td>No rows to display.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</section>

<style>
  .toolbar, .layout { display: flex; gap: 1rem; align-items: flex-start; }
  .toolbar { align-items: center; margin-bottom: 1rem; }
  nav { min-width: 12rem; max-height: 70vh; overflow: auto; }
  nav button { display: block; margin-bottom: 0.5rem; }
  nav button.active { font-weight: 700; }
  .table-wrap { max-height: 75vh; max-width: 100%; overflow: auto; }
  table { border-collapse: collapse; }
  td { border: 1px solid #ddd; padding: 0.35rem 0.5rem; white-space: nowrap; }
  .error { color: #b00020; }
</style>
