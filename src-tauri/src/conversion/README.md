# Conversion backend layer

This module defines the backend contract for import/export jobs. Markdown remains the canonical editable format: imports normalize source files to Markdown, while exports render from Markdown into the requested target.

## Supported job types

- PDF to Markdown
- DOCX to Markdown
- XLSX to Markdown tables
- HTML to Markdown
- TXT to Markdown
- Markdown to PDF
- Markdown to DOCX
- Markdown to HTML
- Mermaid to SVG, PNG, or PDF

## Integration plan

1. Prefer Pandoc for DOCX/HTML/Markdown export paths where it is installed or bundled.
2. Use Rust-native implementations for TXT normalization, Markdown to HTML, job orchestration, capability detection, and status reporting.
3. Add MarkItDown as an optional import fallback for mixed office/PDF documents when packaging Python is acceptable.
4. Keep Docling behind a research/experimental capability flag for complex layout extraction.
5. Use a Mermaid renderer adapter for SVG/PNG/PDF so diagram rendering is isolated from document conversion.

## Frontend API

Tauri commands in `api.rs` expose enqueue, list, status, and cancel operations. Job statuses include queued, running with progress, completed with output paths and warnings, failed with a recoverable flag, and cancelled.
