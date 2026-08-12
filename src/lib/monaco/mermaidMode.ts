import type * as Monaco from 'monaco-editor';
import { validateMermaid } from '../mermaid/validation';

export const MERMAID_MONACO_LANGUAGE_ID = 'mermaid';

export function registerMermaidMonacoMode(monaco: typeof Monaco): void {
  if (!monaco.languages.getLanguages().some((language) => language.id === MERMAID_MONACO_LANGUAGE_ID)) {
    monaco.languages.register({ id: MERMAID_MONACO_LANGUAGE_ID, extensions: ['.mmd', '.mermaid'], aliases: ['Mermaid', 'mermaid', 'mmd'] });
  }

  monaco.languages.setMonarchTokensProvider(MERMAID_MONACO_LANGUAGE_ID, {
    tokenizer: {
      root: [
        [/%%.*/, 'comment'],
        [/\b(flowchart|graph|sequenceDiagram|classDiagram|stateDiagram(?:-v2)?|erDiagram|journey|gantt|pie|mindmap|timeline|gitGraph|architecture-beta)\b/, 'keyword'],
        [/-->|---|==>|-.->|\|/, 'operator'],
        [/"[^"]*"/, 'string'],
        [/\b\d+(?:\.\d+)?\b/, 'number']
      ]
    }
  });

  monaco.languages.setLanguageConfiguration(MERMAID_MONACO_LANGUAGE_ID, {
    comments: { lineComment: '%%' },
    brackets: [['{', '}'], ['[', ']'], ['(', ')']],
    autoClosingPairs: [{ open: '"', close: '"' }, { open: '[', close: ']' }, { open: '(', close: ')' }, { open: '{', close: '}' }]
  });
}

export function wireMermaidValidation(monaco: typeof Monaco, editor: Monaco.editor.IStandaloneCodeEditor): () => void {
  const model = editor.getModel();
  if (!model || model.getLanguageId() !== MERMAID_MONACO_LANGUAGE_ID) return () => undefined;

  let disposed = false;
  let revision = 0;

  async function validate() {
    const current = ++revision;
    const result = await validateMermaid(model!.getValue());
    if (disposed || current !== revision) return;

    monaco.editor.setModelMarkers(model!, 'mermaid', result.ok ? [] : [{
      severity: monaco.MarkerSeverity.Error,
      message: result.message,
      startLineNumber: result.line ?? 1,
      startColumn: result.column ?? 1,
      endLineNumber: result.line ?? 1,
      endColumn: (result.column ?? 1) + 1
    }]);
  }

  const subscription = model.onDidChangeContent(validate);
  void validate();

  return () => {
    disposed = true;
    subscription.dispose();
    monaco.editor.setModelMarkers(model, 'mermaid', []);
  };
}
