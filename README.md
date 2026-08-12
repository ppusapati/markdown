# Markdown Workspace

Markdown Workspace is a Tauri v2 desktop application with a SvelteKit frontend. This initial scaffold provides the cross-platform shell for future Markdown editing, file workspace, and preview functionality.

## Development

Install dependencies and start the Tauri development workflow:

```sh
npm install
npm run tauri dev
```

Build the web frontend independently:

```sh
npm run build
```

The Tauri backend lives in `src-tauri/` and is configured for Windows, macOS, and Linux desktop builds.
