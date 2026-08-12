export const MERMAID_LANGUAGE_IDS = new Set(['mermaid', 'mmd']);

export const SUPPORTED_MERMAID_DIAGRAMS = [
  'flowchart',
  'graph',
  'sequenceDiagram',
  'classDiagram',
  'stateDiagram',
  'stateDiagram-v2',
  'erDiagram',
  'journey',
  'gantt',
  'pie',
  'mindmap',
  'timeline',
  'gitGraph',
  'architecture-beta'
] as const;

export type MermaidValidationResult =
  | { ok: true; diagramType?: string }
  | { ok: false; message: string; line?: number; column?: number };
