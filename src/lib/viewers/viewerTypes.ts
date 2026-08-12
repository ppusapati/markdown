export type ViewableFileType = 'pdf' | 'docx' | 'xlsx' | 'unsupported';

export interface ViewerFile {
  name: string;
  data: ArrayBuffer | Blob | Uint8Array | string;
}

export function getViewableFileType(fileName: string): ViewableFileType {
  const normalized = fileName.toLowerCase();

  if (normalized.endsWith('.pdf')) return 'pdf';
  if (normalized.endsWith('.docx')) return 'docx';
  if (normalized.endsWith('.xlsx')) return 'xlsx';

  return 'unsupported';
}
