use crate::conversion::engine::ConversionEngine;
use crate::conversion::types::{
    ConversionError, ConversionJob, ConversionRequest, ConversionState,
};
use tauri::State;

/// Enqueue a conversion job and return the queued job record immediately.
///
/// A worker can consume queued jobs from `ConversionEngine`, update progress with
/// `set_running`, and finish with `complete` or `fail`. Imports should always
/// populate `canonical_markdown_path` so the editor opens Markdown, not the
/// original binary document.
#[tauri::command]
pub fn enqueue_conversion_job(
    engine: State<'_, ConversionEngine>,
    request: ConversionRequest,
) -> Result<ConversionJob, ConversionError> {
    engine.enqueue(request)
}

#[tauri::command]
pub fn conversion_job_status(
    engine: State<'_, ConversionEngine>,
    id: String,
) -> Result<ConversionJob, ConversionError> {
    engine.status(&id)
}

#[tauri::command]
pub fn list_conversion_jobs(
    engine: State<'_, ConversionEngine>,
) -> Result<ConversionState, ConversionError> {
    Ok(ConversionState {
        jobs: engine.list()?,
    })
}

#[tauri::command]
pub fn cancel_conversion_job(
    engine: State<'_, ConversionEngine>,
    id: String,
) -> Result<ConversionJob, ConversionError> {
    engine.cancel(&id)
}
