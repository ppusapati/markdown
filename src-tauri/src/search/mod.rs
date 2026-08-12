use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use ignore::WalkBuilder;
use once_cell::sync::OnceCell;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
    time::SystemTime,
};
use tantivy::{
    collector::TopDocs, doc, query::QueryParser, schema::*, Index, IndexReader, ReloadPolicy,
    TantivyDocument,
};

static SEARCH_SERVICE: OnceCell<Mutex<SearchService>> = OnceCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub path: PathBuf,
    pub title: String,
    pub snippet: String,
    pub score: f32,
    pub modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildSearchIndexRequest {
    pub workspace_root: PathBuf,
    pub index_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSearchIndexRequest {
    pub workspace_root: PathBuf,
    pub changed_paths: Vec<PathBuf>,
    pub index_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySearchIndexRequest {
    pub workspace_root: PathBuf,
    pub query: String,
    pub limit: Option<usize>,
    pub index_dir: Option<PathBuf>,
}

#[derive(Clone)]
struct SearchFields {
    path: Field,
    title: Field,
    filename: Field,
    body: Field,
    headings: Field,
    tags: Field,
    mermaid: Field,
    modified: Field,
}

impl SearchFields {
    fn from_schema(schema: &Schema) -> Result<Self> {
        Ok(Self {
            path: schema.get_field("path")?,
            title: schema.get_field("title")?,
            filename: schema.get_field("filename")?,
            body: schema.get_field("body")?,
            headings: schema.get_field("headings")?,
            tags: schema.get_field("tags")?,
            mermaid: schema.get_field("mermaid")?,
            modified: schema.get_field("modified")?,
        })
    }
}

pub struct SearchService {
    index: Option<Index>,
    reader: Option<IndexReader>,
    workspace_root: Option<PathBuf>,
    index_dir: Option<PathBuf>,
}

impl Default for SearchService {
    fn default() -> Self {
        Self {
            index: None,
            reader: None,
            workspace_root: None,
            index_dir: None,
        }
    }
}

impl SearchService {
    pub fn build_index(
        &mut self,
        workspace_root: &Path,
        index_dir: Option<&Path>,
    ) -> Result<usize> {
        let workspace_root = workspace_root
            .canonicalize()
            .context("workspace root does not exist")?;
        let index_dir = index_path(&workspace_root, index_dir);
        if index_dir.exists() {
            fs::remove_dir_all(&index_dir)?;
        }
        fs::create_dir_all(&index_dir)?;

        let index = Index::create_in_dir(&index_dir, search_schema())?;
        let count = write_markdown_documents(&index, &workspace_root, None)?;
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        self.index = Some(index);
        self.reader = Some(reader);
        self.workspace_root = Some(workspace_root);
        self.index_dir = Some(index_dir);
        Ok(count)
    }

    pub fn update_index(
        &mut self,
        workspace_root: &Path,
        paths: &[PathBuf],
        index_dir: Option<&Path>,
    ) -> Result<usize> {
        let index = self.ensure_index(workspace_root, index_dir)?;
        let workspace_root = workspace_root.canonicalize()?;
        let count = write_markdown_documents(&index, &workspace_root, Some(paths))?;
        if let Some(reader) = &self.reader {
            reader.reload()?;
        }
        Ok(count)
    }

    pub fn query(
        &mut self,
        workspace_root: &Path,
        query: &str,
        limit: usize,
        index_dir: Option<&Path>,
    ) -> Result<Vec<SearchHit>> {
        let index = self.ensure_index(workspace_root, index_dir)?;
        let reader = self
            .reader
            .as_ref()
            .context("search reader not initialized")?;
        let searcher = reader.searcher();
        let schema = index.schema();
        let fields = SearchFields::from_schema(&schema)?;
        let parser = QueryParser::for_index(
            &index,
            vec![
                fields.title,
                fields.filename,
                fields.headings,
                fields.tags,
                fields.body,
                fields.mermaid,
            ],
        );
        let parsed = parser
            .parse_query(query)
            .or_else(|_| parser.parse_query(&format!("{}*", query)))?;
        let hits = searcher.search(&parsed, &TopDocs::with_limit(limit))?;
        hits.into_iter()
            .filter_map(|(score, addr)| {
                let doc: TantivyDocument = searcher.doc(addr).ok()?;
                let path = doc.get_first(fields.path)?.as_str()?.into();
                let title = doc
                    .get_first(fields.title)
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let body = doc
                    .get_first(fields.body)
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let modified = doc
                    .get_first(fields.modified)
                    .and_then(|v| v.as_datetime())
                    .map(|d| DateTime::<Utc>::from_timestamp(d.into_timestamp_secs(), 0))
                    .flatten();
                Some(SearchHit {
                    path,
                    title,
                    snippet: make_snippet(body, query),
                    score,
                    modified,
                })
            })
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    fn ensure_index(&mut self, workspace_root: &Path, index_dir: Option<&Path>) -> Result<Index> {
        let workspace_root = workspace_root
            .canonicalize()
            .context("workspace root does not exist")?;
        let index_dir = index_path(&workspace_root, index_dir);
        let needs_open = self.index_dir.as_ref() != Some(&index_dir)
            || self.workspace_root.as_ref() != Some(&workspace_root);
        if needs_open {
            if !index_dir.join("meta.json").exists() {
                self.build_index(&workspace_root, Some(&index_dir))?;
            }
            let index = Index::open_in_dir(&index_dir)?;
            let reader = index
                .reader_builder()
                .reload_policy(ReloadPolicy::OnCommitWithDelay)
                .try_into()?;
            self.index = Some(index);
            self.reader = Some(reader);
            self.workspace_root = Some(workspace_root);
            self.index_dir = Some(index_dir);
        }
        Ok(self.index.as_ref().expect("index initialized").clone())
    }
}

fn search_schema() -> Schema {
    let mut builder = Schema::builder();
    builder.add_text_field("path", STRING | STORED);
    builder.add_text_field("title", TEXT | STORED);
    builder.add_text_field("filename", TEXT | STORED);
    builder.add_text_field("body", TEXT | STORED);
    builder.add_text_field("headings", TEXT | STORED);
    builder.add_text_field("tags", TEXT | STORED);
    builder.add_text_field("mermaid", TEXT | STORED);
    builder.add_date_field("modified", STORED | FAST);
    builder.build()
}

fn write_markdown_documents(index: &Index, root: &Path, only: Option<&[PathBuf]>) -> Result<usize> {
    let schema = index.schema();
    let fields = SearchFields::from_schema(&schema)?;
    let mut writer = index.writer(50_000_000)?;
    let files = markdown_files(root, only)?;
    let mut count = 0;
    for file in files {
        let relative = file
            .strip_prefix(root)
            .unwrap_or(&file)
            .to_string_lossy()
            .replace('\\', "/");
        writer.delete_term(Term::from_field_text(fields.path, &relative));
        if !file.exists() {
            continue;
        }
        let content = fs::read_to_string(&file).unwrap_or_default();
        let parsed = parse_markdown(&content);
        let modified = fs::metadata(&file)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        writer.add_document(doc!(
            fields.path => relative,
            fields.title => parsed.title,
            fields.filename => file.file_name().unwrap_or_default().to_string_lossy().to_string(),
            fields.body => content,
            fields.headings => parsed.headings.join("\n"),
            fields.tags => parsed.tags.join(" "),
            fields.mermaid => parsed.mermaid_blocks.join("\n"),
            fields.modified => tantivy::DateTime::from_timestamp_secs(modified.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs() as i64)
        ))?;
        count += 1;
    }
    writer.commit()?;
    Ok(count)
}

fn markdown_files(root: &Path, only: Option<&[PathBuf]>) -> Result<Vec<PathBuf>> {
    if let Some(paths) = only {
        return Ok(paths
            .iter()
            .map(|p| {
                if p.is_absolute() {
                    p.clone()
                } else {
                    root.join(p)
                }
            })
            .filter(|p| p.extension().is_some_and(|e| e == "md" || e == "markdown") || !p.exists())
            .collect());
    }
    Ok(WalkBuilder::new(root)
        .hidden(false)
        .filter_entry(|e| e.file_name() != ".markdown-search")
        .build()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "md" || e == "markdown"))
        .collect())
}

struct ParsedMarkdown {
    title: String,
    headings: Vec<String>,
    tags: Vec<String>,
    mermaid_blocks: Vec<String>,
}

fn parse_markdown(content: &str) -> ParsedMarkdown {
    let tag_re = Regex::new(r"(?m)(?:^|\s)#([A-Za-z0-9_/-]+)").unwrap();
    let mut headings = Vec::new();
    let mut mermaid_blocks = Vec::new();
    let mut in_mermaid = false;
    let mut current = Vec::new();
    for line in content.lines() {
        if line.trim_start().starts_with("```mermaid") {
            in_mermaid = true;
            current.clear();
            continue;
        }
        if in_mermaid && line.trim_start().starts_with("```") {
            in_mermaid = false;
            mermaid_blocks.push(current.join("\n"));
            continue;
        }
        if in_mermaid {
            current.push(line.to_string());
            continue;
        }
        if let Some(heading) = line.strip_prefix('#') {
            let text = heading.trim_start_matches('#').trim();
            if !text.is_empty() {
                headings.push(text.to_string());
            }
        }
    }
    let tags = tag_re
        .captures_iter(content)
        .map(|c| c[1].to_string())
        .collect();
    let title = headings
        .first()
        .cloned()
        .unwrap_or_else(|| "Untitled".into());
    ParsedMarkdown {
        title,
        headings,
        tags,
        mermaid_blocks,
    }
}

fn make_snippet(body: &str, query: &str) -> String {
    let lower = body.to_lowercase();
    let needle = query
        .split_whitespace()
        .next()
        .unwrap_or(query)
        .to_lowercase();
    let start = lower.find(&needle).unwrap_or(0).saturating_sub(80);
    body.chars()
        .skip(start)
        .take(220)
        .collect::<String>()
        .replace('\n', " ")
}

fn index_path(workspace_root: &Path, index_dir: Option<&Path>) -> PathBuf {
    index_dir
        .map(Path::to_path_buf)
        .unwrap_or_else(|| workspace_root.join(".markdown-search"))
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}
impl<T> Pipe for T {}

pub fn global_service() -> &'static Mutex<SearchService> {
    SEARCH_SERVICE.get_or_init(|| Mutex::new(SearchService::default()))
}

#[cfg(feature = "tauri-commands")]
#[tauri::command]
pub async fn build_search_index(request: BuildSearchIndexRequest) -> Result<usize, String> {
    global_service()
        .lock()
        .map_err(|e| e.to_string())?
        .build_index(&request.workspace_root, request.index_dir.as_deref())
        .map_err(|e| e.to_string())
}

#[cfg(feature = "tauri-commands")]
#[tauri::command]
pub async fn update_search_index(request: UpdateSearchIndexRequest) -> Result<usize, String> {
    global_service()
        .lock()
        .map_err(|e| e.to_string())?
        .update_index(
            &request.workspace_root,
            &request.changed_paths,
            request.index_dir.as_deref(),
        )
        .map_err(|e| e.to_string())
}

#[cfg(feature = "tauri-commands")]
#[tauri::command]
pub async fn query_search_index(
    request: QuerySearchIndexRequest,
) -> Result<Vec<SearchHit>, String> {
    global_service()
        .lock()
        .map_err(|e| e.to_string())?
        .query(
            &request.workspace_root,
            &request.query,
            request.limit.unwrap_or(20),
            request.index_dir.as_deref(),
        )
        .map_err(|e| e.to_string())
}
