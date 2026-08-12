<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import * as monaco from 'monaco-editor';
  import 'monaco-editor/esm/vs/basic-languages/markdown/markdown.contribution';

  type Props = {
    value: string;
    onChange?: (value: string) => void;
  };

  let { value = '', onChange }: Props = $props();
  let container: HTMLDivElement;
  let editor: monaco.editor.IStandaloneCodeEditor;

  onMount(() => {
    editor = monaco.editor.create(container, {
      value,
      language: 'markdown',
      automaticLayout: true,
      minimap: { enabled: false },
      wordWrap: 'on',
      lineNumbers: 'on',
      theme: 'vs',
      scrollBeyondLastLine: false
    });

    const modelListener = editor.onDidChangeModelContent(() => {
      onChange?.(editor.getValue());
    });

    return () => modelListener.dispose();
  });

  $effect(() => {
    if (editor && value !== editor.getValue()) {
      const position = editor.getPosition();
      editor.setValue(value);
      if (position) editor.setPosition(position);
    }
  });

  onDestroy(() => editor?.dispose());
</script>

<div class="editor-shell" bind:this={container} aria-label="Markdown source editor"></div>

<style>
  .editor-shell {
    min-height: 100%;
    height: 100%;
    border-radius: 16px;
    overflow: hidden;
    border: 1px solid #d9e1f2;
  }
</style>
