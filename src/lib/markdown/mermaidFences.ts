import { MERMAID_LANGUAGE_IDS } from '../mermaid/types';

export type MarkdownSegment =
  | { kind: 'markdown'; content: string }
  | { kind: 'mermaid'; content: string; info: string };

const FENCE = /(^|\n)(`{3,}|~{3,})([^\n]*)\n([\s\S]*?)(?:\n\2(?=\n|$))/g;

export function splitMermaidFences(markdown: string): MarkdownSegment[] {
  const segments: MarkdownSegment[] = [];
  let cursor = 0;
  let match: RegExpExecArray | null;

  while ((match = FENCE.exec(markdown))) {
    const fenceStart = match.index + match[1].length;
    const info = match[3].trim();
    const language = info.split(/\s+/)[0]?.toLowerCase();

    if (!MERMAID_LANGUAGE_IDS.has(language)) continue;

    if (fenceStart > cursor) {
      segments.push({ kind: 'markdown', content: markdown.slice(cursor, fenceStart) });
    }

    segments.push({ kind: 'mermaid', content: match[4], info });
    cursor = FENCE.lastIndex;
  }

  if (cursor < markdown.length) {
    segments.push({ kind: 'markdown', content: markdown.slice(cursor) });
  }

  return segments.length ? segments : [{ kind: 'markdown', content: markdown }];
}
