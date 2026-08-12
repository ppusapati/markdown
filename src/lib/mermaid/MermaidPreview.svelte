<script lang="ts">
  import { onDestroy, tick } from 'svelte';
  import mermaid from 'mermaid';
  import { validateMermaid } from './validation';

  export let source = '';
  export let id = `mermaid-${Math.random().toString(36).slice(2)}`;

  let container: HTMLDivElement;
  let svg = '';
  let error = '';
  let renderToken = 0;

  mermaid.initialize({ startOnLoad: false, securityLevel: 'strict', theme: 'default' });

  $: render(source);

  async function render(value: string) {
    const token = ++renderToken;
    svg = '';
    error = '';

    const validation = await validateMermaid(value);
    if (token !== renderToken) return;
    if (!validation.ok) {
      error = validation.message;
      return;
    }

    await tick();
    try {
      const result = await mermaid.render(`${id}-${token}`, value, container);
      if (token === renderToken) svg = result.svg;
    } catch (err) {
      if (token === renderToken) error = err instanceof Error ? err.message : String(err);
    }
  }

  onDestroy(() => {
    renderToken += 1;
  });
</script>

<div class="mermaid-preview" bind:this={container} data-testid="mermaid-preview">
  {#if error}
    <pre class="mermaid-error" role="alert">{error}</pre>
  {:else if svg}
    {@html svg}
  {:else}
    <span class="mermaid-loading">Rendering diagram…</span>
  {/if}
</div>

<style>
  .mermaid-preview { overflow: auto; max-width: 100%; }
  .mermaid-error { color: #b42318; background: #fff1f0; border: 1px solid #ffccc7; padding: 0.75rem; white-space: pre-wrap; }
  .mermaid-loading { color: #667085; }
</style>
