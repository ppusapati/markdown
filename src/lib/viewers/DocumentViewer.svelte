<script lang="ts">
  import PdfViewer from './PdfViewer.svelte';
  import DocxViewer from './DocxViewer.svelte';
  import XlsxViewer from './XlsxViewer.svelte';
  import { getViewableFileType, type ViewerFile } from './viewerTypes';

  export let file: ViewerFile;

  $: fileType = getViewableFileType(file.name);
</script>

{#if fileType === 'pdf'}
  <PdfViewer data={file.data} fileName={file.name} />
{:else if fileType === 'docx'}
  <DocxViewer data={file.data} fileName={file.name} />
{:else if fileType === 'xlsx'}
  <XlsxViewer data={file.data} fileName={file.name} />
{:else}
  <p role="status">No read-only viewer is available for {file.name}.</p>
{/if}
