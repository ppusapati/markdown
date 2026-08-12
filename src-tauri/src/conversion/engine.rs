use crate::conversion::registry::ConversionRegistry;
use crate::conversion::types::{
    ConversionError, ConversionJob, ConversionJobId, ConversionJobResult, ConversionJobStatus,
    ConversionProgress, ConversionRequest,
};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Default)]
pub struct ConversionEngine {
    registry: ConversionRegistry,
    jobs: Arc<Mutex<BTreeMap<ConversionJobId, ConversionJob>>>,
}

impl ConversionEngine {
    pub fn enqueue(&self, request: ConversionRequest) -> Result<ConversionJob, ConversionError> {
        let matching = self.registry.approaches_for(&request.kind);
        if matching.is_empty() {
            return Err(ConversionError::fatal(
                "No conversion backend is registered for this job type",
            ));
        }

        let id = format!("conversion-{}", now_ms());
        let job = ConversionJob::new(id.clone(), request);
        self.jobs
            .lock()
            .map_err(|_| ConversionError::fatal("Conversion job store is unavailable"))?
            .insert(id, job.clone());
        Ok(job)
    }

    pub fn list(&self) -> Result<Vec<ConversionJob>, ConversionError> {
        Ok(self
            .jobs
            .lock()
            .map_err(|_| ConversionError::fatal("Conversion job store is unavailable"))?
            .values()
            .cloned()
            .collect())
    }

    pub fn status(&self, id: &str) -> Result<ConversionJob, ConversionError> {
        self.jobs
            .lock()
            .map_err(|_| ConversionError::fatal("Conversion job store is unavailable"))?
            .get(id)
            .cloned()
            .ok_or_else(|| ConversionError::recoverable(format!("Unknown conversion job: {id}")))
    }

    pub fn set_running(
        &self,
        id: &str,
        percent: u8,
        message: impl Into<String>,
    ) -> Result<(), ConversionError> {
        self.update_status(
            id,
            ConversionJobStatus::Running {
                progress: ConversionProgress::new(percent, message),
            },
        )
    }

    pub fn complete(
        &self,
        id: &str,
        output_path: PathBuf,
        canonical_markdown_path: Option<PathBuf>,
        warnings: Vec<String>,
    ) -> Result<(), ConversionError> {
        self.update_status(
            id,
            ConversionJobStatus::Completed {
                result: ConversionJobResult {
                    output_path,
                    canonical_markdown_path,
                    warnings,
                },
            },
        )
    }

    pub fn fail(&self, id: &str, error: ConversionError) -> Result<(), ConversionError> {
        self.update_status(id, ConversionJobStatus::Failed { error })
    }

    pub fn cancel(&self, id: &str) -> Result<ConversionJob, ConversionError> {
        self.update_status(id, ConversionJobStatus::Cancelled)?;
        self.status(id)
    }

    fn update_status(&self, id: &str, status: ConversionJobStatus) -> Result<(), ConversionError> {
        let mut jobs = self
            .jobs
            .lock()
            .map_err(|_| ConversionError::fatal("Conversion job store is unavailable"))?;
        let job = jobs
            .get_mut(id)
            .ok_or_else(|| ConversionError::recoverable(format!("Unknown conversion job: {id}")))?;
        job.set_status(status);
        Ok(())
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}
