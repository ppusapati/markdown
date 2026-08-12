<script lang="ts">
  import { workspace } from '$lib/stores/workspace';

  $: crumbs = $workspace.openFile?.path.replace('local://', '').split('/').filter(Boolean) ?? ['Markdown Workspace'];
</script>

<nav aria-label="Breadcrumb" class="breadcrumbs">
  {#each crumbs as crumb, index}
    <span class:active={index === crumbs.length - 1}>{crumb}</span>
    {#if index < crumbs.length - 1}<span class="separator">/</span>{/if}
  {/each}
</nav>

<style>
  .breadcrumbs {
    align-items: center;
    color: #667085;
    display: flex;
    font-size: 0.9rem;
    gap: 0.4rem;
    min-width: 0;
  }

  .active {
    color: #1d2939;
    font-weight: 600;
  }

  .separator {
    color: #98a2b3;
  }
</style>
