import mermaid from 'mermaid';
import { SUPPORTED_MERMAID_DIAGRAMS, type MermaidValidationResult } from './types';

const LEADING_DIRECTIVE = /^\s*%%\{[\s\S]*?\}%%\s*/;
const DETECTOR = /^\s*(?<type>flowchart|graph|sequenceDiagram|classDiagram|stateDiagram(?:-v2)?|erDiagram|journey|gantt|pie|mindmap|timeline|gitGraph|architecture-beta)\b/i;

mermaid.initialize({ startOnLoad: false, securityLevel: 'strict' });

export function getMermaidDiagramType(source: string): string | undefined {
  const normalized = source.replace(LEADING_DIRECTIVE, '');
  return normalized.match(DETECTOR)?.groups?.type;
}

export function isSupportedMermaidDiagram(source: string): boolean {
  const type = getMermaidDiagramType(source);
  return Boolean(type && SUPPORTED_MERMAID_DIAGRAMS.some((supported) => supported.toLowerCase() === type.toLowerCase()));
}

export async function validateMermaid(source: string): Promise<MermaidValidationResult> {
  const trimmed = source.trim();
  if (!trimmed) {
    return { ok: false, message: 'Mermaid diagram is empty.' };
  }

  const diagramType = getMermaidDiagramType(trimmed);
  if (!diagramType) {
    return { ok: false, message: `Unsupported Mermaid diagram type. Supported types: ${SUPPORTED_MERMAID_DIAGRAMS.join(', ')}.` };
  }

  try {
    await mermaid.parse(trimmed, { suppressErrors: false });
    return { ok: true, diagramType };
  } catch (error) {
    return normalizeMermaidError(error);
  }
}

function normalizeMermaidError(error: unknown): MermaidValidationResult {
  const message = error instanceof Error ? error.message : String(error);
  const location = /line\s+(\d+)(?:.*column\s+(\d+))?/i.exec(message);
  return {
    ok: false,
    message: message.replace(/\s+/g, ' ').trim() || 'Invalid Mermaid syntax.',
    line: location?.[1] ? Number(location[1]) : undefined,
    column: location?.[2] ? Number(location[2]) : undefined
  };
}
