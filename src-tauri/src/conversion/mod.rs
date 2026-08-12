//! Backend conversion layer for import and export workflows.
//!
//! Markdown is the canonical editable representation for imported documents:
//! every import job writes Markdown, and every export job reads Markdown.

pub mod api;
pub mod engine;
pub mod integrations;
pub mod registry;
pub mod types;

pub use api::{
    cancel_conversion_job, conversion_job_status, enqueue_conversion_job, list_conversion_jobs,
};
pub use engine::ConversionEngine;
pub use registry::ConversionRegistry;
pub use types::{
    ConversionError, ConversionJob, ConversionJobId, ConversionJobKind, ConversionJobResult,
    ConversionJobStatus, ConversionOptions, ConversionProgress, ConversionRequest, ConversionState,
    ConversionTarget, MermaidOutputFormat,
};
