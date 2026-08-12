use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub type ConversionJobId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ConversionJobKind {
    PdfToMarkdown,
    DocxToMarkdown,
    XlsxToMarkdownTables,
    HtmlToMarkdown,
    TxtToMarkdown,
    MarkdownToPdf,
    MarkdownToDocx,
    MarkdownToHtml,
    MermaidToSvg,
    MermaidToPng,
    MermaidToPdf,
}

impl ConversionJobKind {
    pub fn source_target(&self) -> (ConversionTarget, ConversionTarget) {
        match self {
            Self::PdfToMarkdown => (ConversionTarget::Pdf, ConversionTarget::Markdown),
            Self::DocxToMarkdown => (ConversionTarget::Docx, ConversionTarget::Markdown),
            Self::XlsxToMarkdownTables => (ConversionTarget::Xlsx, ConversionTarget::Markdown),
            Self::HtmlToMarkdown => (ConversionTarget::Html, ConversionTarget::Markdown),
            Self::TxtToMarkdown => (ConversionTarget::Txt, ConversionTarget::Markdown),
            Self::MarkdownToPdf => (ConversionTarget::Markdown, ConversionTarget::Pdf),
            Self::MarkdownToDocx => (ConversionTarget::Markdown, ConversionTarget::Docx),
            Self::MarkdownToHtml => (ConversionTarget::Markdown, ConversionTarget::Html),
            Self::MermaidToSvg => (ConversionTarget::Mermaid, ConversionTarget::Svg),
            Self::MermaidToPng => (ConversionTarget::Mermaid, ConversionTarget::Png),
            Self::MermaidToPdf => (ConversionTarget::Mermaid, ConversionTarget::Pdf),
        }
    }

    pub fn preserves_markdown_as_canonical(&self) -> bool {
        matches!(
            self,
            Self::PdfToMarkdown
                | Self::DocxToMarkdown
                | Self::XlsxToMarkdownTables
                | Self::HtmlToMarkdown
                | Self::TxtToMarkdown
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConversionTarget {
    Pdf,
    Docx,
    Xlsx,
    Html,
    Txt,
    Markdown,
    Mermaid,
    Svg,
    Png,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MermaidOutputFormat {
    Svg,
    Png,
    Pdf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ConversionOptions {
    pub metadata: BTreeMap<String, String>,
    pub output_path: Option<PathBuf>,
    pub preserve_source_assets: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversionRequest {
    pub kind: ConversionJobKind,
    pub input_path: PathBuf,
    pub options: ConversionOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversionJob {
    pub id: ConversionJobId,
    pub request: ConversionRequest,
    pub status: ConversionJobStatus,
    pub created_at_ms: u128,
    pub updated_at_ms: u128,
}

impl ConversionJob {
    pub fn new(id: ConversionJobId, request: ConversionRequest) -> Self {
        let now = now_ms();
        Self {
            id,
            request,
            status: ConversionJobStatus::Queued,
            created_at_ms: now,
            updated_at_ms: now,
        }
    }

    pub fn set_status(&mut self, status: ConversionJobStatus) {
        self.status = status;
        self.updated_at_ms = now_ms();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ConversionJobStatus {
    Queued,
    Running { progress: ConversionProgress },
    Completed { result: ConversionJobResult },
    Failed { error: ConversionError },
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversionProgress {
    pub percent: u8,
    pub message: String,
}

impl ConversionProgress {
    pub fn new(percent: u8, message: impl Into<String>) -> Self {
        Self {
            percent: percent.min(100),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversionJobResult {
    pub output_path: PathBuf,
    pub canonical_markdown_path: Option<PathBuf>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversionError {
    pub message: String,
    pub recoverable: bool,
}

impl ConversionError {
    pub fn recoverable(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            recoverable: true,
        }
    }

    pub fn fatal(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            recoverable: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversionState {
    pub jobs: Vec<ConversionJob>,
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}
