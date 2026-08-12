import { SearchHit, WorkspaceSearch } from './search';

export function mountWorkspaceSearch(container: HTMLElement, workspaceRoot: string, onOpen: (path: string) => void) {
  const search = new WorkspaceSearch(workspaceRoot);
  container.innerHTML = `
    <section class="workspace-search" aria-label="Workspace search">
      <input class="workspace-search__input" type="search" placeholder="Search files, headings, tags, Mermaid…" />
      <button class="workspace-search__reindex" type="button">Index</button>
      <ol class="workspace-search__results"></ol>
    </section>`;
  const input = container.querySelector<HTMLInputElement>('.workspace-search__input')!;
  const button = container.querySelector<HTMLButtonElement>('.workspace-search__reindex')!;
  const results = container.querySelector<HTMLOListElement>('.workspace-search__results')!;

  button.addEventListener('click', async () => { await search.buildIndex(); input.dispatchEvent(new Event('input')); });
  input.addEventListener('input', debounce(async () => {
    const query = input.value.trim();
    results.replaceChildren();
    if (!query) return;
    renderResults(results, await search.query(query), onOpen);
  }, 150));
}

function renderResults(container: HTMLOListElement, hits: SearchHit[], onOpen: (path: string) => void) {
  for (const hit of hits) {
    const item = document.createElement('li');
    const button = document.createElement('button');
    button.type = 'button';
    button.className = 'workspace-search__hit';
    button.innerHTML = `<strong></strong><span></span><p></p>`;
    button.querySelector('strong')!.textContent = hit.title || hit.path;
    button.querySelector('span')!.textContent = hit.path;
    button.querySelector('p')!.textContent = hit.snippet;
    button.addEventListener('click', () => onOpen(hit.path));
    item.append(button);
    container.append(item);
  }
}

function debounce<T extends (...args: unknown[]) => void>(fn: T, wait: number): T {
  let timer: ReturnType<typeof setTimeout> | undefined;
  return ((...args: unknown[]) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), wait);
  }) as T;
}
