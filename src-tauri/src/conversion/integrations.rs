use crate::conversion::types::ConversionJobKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntegrationApproach {
    pub name: &'static str,
    pub best_for: Vec<ConversionJobKind>,
    pub strengths: &'static str,
    pub tradeoffs: &'static str,
    pub recommendation: IntegrationRecommendation,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationRecommendation {
    Primary,
    OptionalFallback,
    ResearchTrack,
    FormatSpecific,
}

pub fn evaluated_integration_approaches() -> Vec<IntegrationApproach> {
    vec![
        IntegrationApproach {
            name: "Pandoc",
            best_for: vec![
                ConversionJobKind::DocxToMarkdown,
                ConversionJobKind::HtmlToMarkdown,
                ConversionJobKind::MarkdownToPdf,
                ConversionJobKind::MarkdownToDocx,
                ConversionJobKind::MarkdownToHtml,
            ],
            strengths: "Mature document conversion CLI with strong Markdown, DOCX, HTML, and PDF workflows.",
            tradeoffs: "Requires a bundled or discovered binary; high-quality PDF export may also require a TeX or browser engine.",
            recommendation: IntegrationRecommendation::Primary,
        },
        IntegrationApproach {
            name: "MarkItDown",
            best_for: vec![
                ConversionJobKind::PdfToMarkdown,
                ConversionJobKind::DocxToMarkdown,
                ConversionJobKind::XlsxToMarkdownTables,
                ConversionJobKind::HtmlToMarkdown,
            ],
            strengths: "Optimized import pipeline for turning heterogeneous documents into Markdown.",
            tradeoffs: "Python dependency increases packaging complexity for a desktop application.",
            recommendation: IntegrationRecommendation::OptionalFallback,
        },
        IntegrationApproach {
            name: "Docling",
            best_for: vec![
                ConversionJobKind::PdfToMarkdown,
                ConversionJobKind::DocxToMarkdown,
                ConversionJobKind::XlsxToMarkdownTables,
            ],
            strengths: "Document AI pipeline that can preserve richer layout semantics for complex PDFs and office files.",
            tradeoffs: "Heavier runtime and model dependencies; should be gated behind capability detection.",
            recommendation: IntegrationRecommendation::ResearchTrack,
        },
        IntegrationApproach {
            name: "Rust-native libraries",
            best_for: vec![
                ConversionJobKind::TxtToMarkdown,
                ConversionJobKind::MarkdownToHtml,
                ConversionJobKind::MermaidToSvg,
                ConversionJobKind::MermaidToPng,
                ConversionJobKind::MermaidToPdf,
            ],
            strengths: "Best packaging story, predictable errors, and direct progress reporting inside Tauri.",
            tradeoffs: "Coverage varies by file type; complex PDF and DOCX fidelity may lag specialized tools.",
            recommendation: IntegrationRecommendation::FormatSpecific,
        },
    ]
}
