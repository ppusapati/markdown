<script lang="ts">
  import 'highlight.js/styles/github.css';
  import { renderMarkdown } from '$lib/markdown/render';

  type Props = { source: string };
  let { source }: Props = $props();
  let rendered = $derived(renderMarkdown(source));
  let hasFrontmatter = $derived(Object.keys(rendered.frontmatter).length > 0 || rendered.frontmatterRaw.length > 0);
</script>

<aside class="toc" aria-label="Table of contents">
  <strong>Table of contents</strong>
  {#if rendered.toc.length}
    <ol>
      {#each rendered.toc as item}
        <li style={`--level: ${item.level}`}><a href={`#${item.slug}`}>{item.title}</a></li>
      {/each}
    </ol>
  {:else}
    <p>No headings yet.</p>
  {/if}
</aside>

<section class="preview" aria-label="Rendered Markdown preview">
  {#if hasFrontmatter}
    <details class="frontmatter" open>
      <summary>Frontmatter</summary>
      <pre>{JSON.stringify(rendered.frontmatter, null, 2)}</pre>
    </details>
  {/if}
  <article>{@html rendered.body}</article>
</section>

<style>
  :global(.preview h1, .preview h2, .preview h3) { scroll-margin-top: 1rem; }
  .toc, .preview { background: white; border: 1px solid #d9e1f2; border-radius: 16px; padding: 1rem; overflow: auto; }
  .toc { max-height: 14rem; }
  .toc ol { list-style: none; padding: 0; margin: .75rem 0 0; }
  .toc li { margin-left: calc((var(--level) - 1) * .85rem); line-height: 1.8; }
  .toc a { color: #315cba; text-decoration: none; }
  .frontmatter { margin-bottom: 1rem; border: 1px dashed #b8c4dd; border-radius: 12px; padding: .75rem; background: #f8faff; }
  .frontmatter pre { white-space: pre-wrap; margin-bottom: 0; }
  .preview article { line-height: 1.65; }
  .preview :global(table) { border-collapse: collapse; width: 100%; margin: 1rem 0; }
  .preview :global(th), .preview :global(td) { border: 1px solid #cfd8ea; padding: .45rem .6rem; }
  .preview :global(blockquote) { border-left: 4px solid #8da2d1; margin-left: 0; padding-left: 1rem; color: #526071; }
  .preview :global(pre) { border-radius: 12px; padding: 1rem; overflow: auto; }
</style>
