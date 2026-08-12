<script lang="ts">
  import { workspace } from '$lib/stores/workspace';
</script>

<footer class="status-bar">
  <span>{$workspace.statusMessage}</span>
  <span>{$workspace.isOffline ? 'Offline-first' : 'Connected'}</span>
  <span>{$workspace.openFile ? `${$workspace.openFile.kind} · ${$workspace.openFile.dirty ? 'Unsaved' : 'Saved'}` : 'No file open'}</span>
</footer>

<style>
  .status-bar {
    align-items: center;
    background: #101828;
    color: #f9fafb;
    display: flex;
    font-size: 0.82rem;
    gap: 1rem;
    justify-content: space-between;
    min-height: 2rem;
    padding: 0 1rem;
  }
</style>
