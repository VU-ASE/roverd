use openapi::models::{PipelineGet200Response, PipelineStatus};

#[derive(Debug, Clone)]
pub struct Pipeline {
    response: PipelineGet200Response,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
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
