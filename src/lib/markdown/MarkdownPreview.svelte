<script lang="ts">
  import MermaidPreview from '../mermaid/MermaidPreview.svelte';
  import { splitMermaidFences } from './mermaidFences';

  export let markdown = '';
  $: segments = splitMermaidFences(markdown);
</script>

<div class="markdown-preview">
  {#each segments as segment}
    {#if segment.kind === 'mermaid'}
      <MermaidPreview source={segment.content} />
    {:else}
      <pre class="markdown-text">{segment.content}</pre>
    {/if}
  {/each}
</div>
