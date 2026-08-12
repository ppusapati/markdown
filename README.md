# Markdown Desktop Workspace

Markdown Desktop Workspace is a local-first desktop application for reading, organizing, previewing, searching, and converting Markdown-centered knowledge files. The product brings a focused Markdown authoring and viewing experience together with practical support for adjacent document formats, especially diagrams, PDFs, Office files, and common text assets.

## Product vision

The vision is to provide a fast, dependable desktop workspace for people who keep project knowledge in files. Instead of pushing users into a cloud-first content system, the application treats the local filesystem as the source of truth while adding polished navigation, rich previews, Markdown rendering, diagram support, search, and export workflows.

The product should feel lightweight enough for daily notes, technical documentation, and project references, while still being extensible enough to become a broader document workspace over time.

## Target users

- **Developers and technical writers** who maintain Markdown documentation, Mermaid diagrams, README files, changelogs, and architecture notes.
- **Product and project teams** who need a local workspace for specs, meeting notes, PDFs, spreadsheets, and reference files.
- **Students, researchers, and analysts** who collect notes and source documents in folders and need fast search and preview.
- **Power users of local files** who prefer transparent storage, portable documents, and no required cloud account.

## Core principles

1. **Local-first by default**: files remain on the user's device and in user-selected folders.
2. **Markdown-centered, multi-format aware**: Markdown is the primary authoring format, with strong viewing support for related document types.
3. **Fast and predictable**: indexing, previewing, and navigation should feel responsive on ordinary project folders.
4. **Safe file handling**: rendering and conversion should avoid surprising writes, destructive edits, or hidden file mutations.
5. **Progressive capability**: begin with reliable viewing and organization, then add editing, conversion, and deeper productivity features in phases.
6. **Cross-platform desktop quality**: the application should behave like a native desktop app across supported operating systems.

## Supported file types

Initial releases should prioritize viewing, organizing, indexing, and conversion workflows for common knowledge-work formats:

- Markdown: `.md`, `.markdown`, `.mdown`, `.mkd`
- Mermaid diagrams embedded in Markdown or stored as diagram text where supported
- Plain text and code-adjacent files: `.txt`, `.log`, `.json`, `.yaml`, `.yml`, `.toml`, `.csv`
- PDFs: `.pdf`
- Images used in documentation: `.png`, `.jpg`, `.jpeg`, `.gif`, `.webp`, `.svg`
- Office-style documents for viewing and conversion workflows: `.docx`, `.xlsx`, `.pptx`
- Export targets: HTML, PDF, and selected document formats as the conversion layer matures

## MVP phases

- **Phase 1 — Foundation and read-only workspace**: establish the Tauri shell, local folder access, file tree, Markdown preview, Mermaid rendering, basic viewers, and app storage.
- **Phase 2 — Editing and conversion workflows**: add Markdown editing, safer write flows, export support, document conversion plumbing, and improved viewer integrations.
- **Phase 3 — Search and workspace intelligence**: add indexing, full-text search, metadata extraction, recent files, tags or collections, and workspace-level navigation improvements.
- **Phase 4 — Polish, extensibility, and advanced formats**: refine cross-platform packaging, performance, settings, plugin-ready boundaries, and deeper support for complex formats.

## Architecture

### Desktop shell: Tauri v2

The application is built as a Tauri v2 desktop app to combine a lightweight native shell with a web-based interface. Tauri provides window management, secure filesystem access, native dialogs, application packaging, and a controlled bridge between the frontend and backend.

### Rust backend

The Rust backend owns local system integration and performance-sensitive work, including:

- Filesystem traversal, folder watching, and safe read/write operations.
- Search indexing and metadata extraction.
- Conversion command orchestration and sandboxed process boundaries where appropriate.
- Storage access for application state, recent workspaces, preferences, and cache metadata.
- Tauri commands exposed to the frontend through a narrow, typed API.

### SvelteKit frontend

The frontend uses SvelteKit for the desktop UI. It is responsible for the workspace layout, file tree, tabs or panes, Markdown editor and preview surfaces, viewer routing, search UI, settings, and conversion flows. The frontend should keep UI state responsive while delegating privileged filesystem and indexing work to Rust commands.

### Markdown and Mermaid handling

Markdown rendering should support common Markdown extensions needed for technical documentation, including tables, task lists, fenced code blocks, links, and images. Mermaid diagrams should render from fenced Mermaid blocks in Markdown and should fail gracefully with clear error states when diagram syntax is invalid.

### Viewers

Viewer components should be selected by file type. Markdown receives first-class preview and editing experiences; PDFs and images receive dedicated viewers; text and structured data files receive readable previews; Office files start with preview or conversion-based workflows rather than full native editing.

### Conversion

Conversion is handled as a backend-mediated workflow. The frontend initiates conversion requests, while Rust validates paths, manages temporary outputs, invokes conversion libraries or external tools, tracks status, and returns output locations. Early conversion targets should focus on Markdown to HTML/PDF and Office-to-preview-friendly formats.

### Search

Search should begin with filename and lightweight content search, then evolve into a persistent local index. The backend should manage indexing schedules, incremental updates, file watching, metadata extraction, and query execution. The frontend should provide fast filters, result previews, and navigation into source files.

### Storage

Storage should remain transparent and local. User documents stay in their original folders. Application data should be limited to preferences, recent workspaces, indexes, thumbnail or render caches, and conversion artifacts. The storage layer should be versioned so future migrations are explicit and reversible where practical.

## Phased roadmap

### Phase 1: Foundation and read-only workspace

- Scaffold the Tauri v2 application with a SvelteKit frontend and Rust backend.
- Open a local folder as a workspace and display a navigable file tree.
- Preview Markdown with links, images, tables, task lists, code blocks, and Mermaid diagrams.
- Add basic viewers for PDFs, images, plain text, and structured text files.
- Store recent workspaces and basic application preferences locally.
- Establish safe path handling and a typed command boundary between frontend and backend.

### Phase 2: Editing and conversion workflows

- Add Markdown editing with preview synchronization.
- Support explicit save flows, dirty-state indicators, and basic file create/rename/delete operations.
- Add Markdown export to HTML and PDF.
- Introduce conversion workflows for selected Office and document formats.
- Improve viewer fallbacks and conversion error reporting.
- Add settings for editor behavior, rendering options, and conversion preferences.

### Phase 3: Search and workspace intelligence

- Implement persistent local indexing for supported text-based files.
- Add full-text search with filters by file type, path, and recent activity.
- Extract metadata such as headings, links, tags, and document titles where possible.
- Add recent files, pinned files, collections, or lightweight workspace organization features.
- Improve large-workspace performance with incremental indexing and file watching.

### Phase 4: Polish, extensibility, and advanced formats

- Harden packaging, auto-update strategy, signing expectations, and platform-specific integrations.
- Improve accessibility, keyboard navigation, theming, and responsive layout behavior.
- Optimize rendering, indexing, and conversion for larger workspaces.
- Define extension points for additional viewers, converters, and metadata extractors.
- Expand support for complex document formats while maintaining clear capability boundaries.

## Out of scope for initial releases

The following capabilities are intentionally excluded from the first releases so the product can focus on a reliable local Markdown workspace:

- **Full Office editing**: Word, Excel, and PowerPoint files may be viewed or converted, but native-grade Office editing is not part of the initial scope.
- **Real-time collaboration**: simultaneous multi-user editing, comments, presence, and conflict resolution are deferred.
- **Cloud sync**: hosted accounts, managed cloud storage, and proprietary synchronization are out of scope; users may still place workspaces inside third-party sync folders at their own discretion.
- **PowerPoint authoring**: the product may preview or convert presentation files, but creating and editing PowerPoint decks is not planned for the MVP.
- **Mobile applications**: initial work is focused on the desktop experience.
- **End-to-end project management**: tasks, calendars, issue tracking, and workflow automation are outside the initial product boundary.
