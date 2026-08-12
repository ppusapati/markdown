import MarkdownIt from 'markdown-it';
import anchor from 'markdown-it-anchor';
import footnote from 'markdown-it-footnote';
import taskLists from 'markdown-it-task-lists';
import matter from 'gray-matter';
import hljs from 'highlight.js';

export type TocItem = { level: number; title: string; slug: string };
export type RenderedMarkdown = {
  body: string;
  frontmatter: Record<string, unknown>;
  frontmatterRaw: string;
  toc: TocItem[];
};

const slugify = (value: string) =>
  value
    .toLowerCase()
    .trim()
    .replace(/[^a-z0-9\s-]/g, '')
    .replace(/\s+/g, '-')
    .replace(/-+/g, '-');

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  highlight(code, language) {
    const validLanguage = language && hljs.getLanguage(language) ? language : 'plaintext';
    const highlighted = hljs.highlight(code, { language: validLanguage }).value;
    return `<pre class="hljs"><code class="language-${validLanguage}">${highlighted}</code></pre>`;
  }
})
  .enable('table')
  .use(footnote)
  .use(taskLists, { enabled: true, label: true, labelAfter: true })
  .use(anchor, { slugify, permalink: anchor.permalink.headerLink() });

export function renderMarkdown(source: string): RenderedMarkdown {
  const parsed = matter(source);
  const env: { headings?: TocItem[] } = {};
  const tokens = md.parse(parsed.content, env);
  const toc: TocItem[] = [];

  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (token.type === 'heading_open') {
      const inline = tokens[index + 1];
      const title = inline?.content ?? '';
      const level = Number(token.tag.replace('h', ''));
      const slug = token.attrGet('id') ?? slugify(title);
      toc.push({ level, title, slug });
    }
  }

  return {
    body: md.renderer.render(tokens, md.options, env),
    frontmatter: parsed.data,
    frontmatterRaw: parsed.matter ?? '',
    toc
  };
}
