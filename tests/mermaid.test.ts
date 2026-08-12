import { describe, expect, it } from 'vitest';
import { splitMermaidFences } from '../src/lib/markdown/mermaidFences';
import { getMermaidDiagramType, isSupportedMermaidDiagram, validateMermaid } from '../src/lib/mermaid/validation';

describe('Mermaid support', () => {
  it('splits Mermaid code fences from Markdown', () => {
    const segments = splitMermaidFences('# Demo\n```mermaid\nflowchart TD\nA-->B\n```\nDone');
    expect(segments.map((segment) => segment.kind)).toEqual(['markdown', 'mermaid', 'markdown']);
    expect(segments[1]).toMatchObject({ content: 'flowchart TD\nA-->B' });
  });

  it('detects supported core diagram types', () => {
    for (const source of ['flowchart TD', 'sequenceDiagram', 'classDiagram', 'stateDiagram-v2', 'erDiagram', 'journey', 'gantt', 'pie title Pets', 'mindmap', 'timeline', 'gitGraph', 'architecture-beta']) {
      expect(isSupportedMermaidDiagram(source), source).toBe(true);
      expect(getMermaidDiagramType(source)).toBeTruthy();
    }
  });

  it('returns validation feedback for invalid source', async () => {
    await expect(validateMermaid('notADiagram')).resolves.toMatchObject({ ok: false });
  });
});
