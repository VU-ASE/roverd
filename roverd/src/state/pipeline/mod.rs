use openapi::models::{PipelineGet200Response, PipelineStatus};

mod process;
use process::ProcessManager;


#[derive(Debug, Clone)]
pub struct Pipeline {
    response: PipelineGet200Response,
    process_manager: ProcessManager,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            process_manager: ProcessManager {

            },
            response: PipelineGet200Response {
                status: PipelineStatus::Startable,
                last_start: None,
                last_stop: None,
                last_restart: None,
                enabled: vec![],
            },
        }
    }
}
