use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Webhook {
    pub(crate) action: Action,
    pub(crate) workflow_job: WorkflowJob,
    pub(crate) repository: Repository,
}

#[derive(Deserialize)]
pub(crate) enum Action {
    #[serde(rename = "waiting")]
    Waiting,

    #[serde(rename = "queued")]
    Queued,

    #[serde(rename = "in_progress")]
    InProgress,

    #[serde(rename = "completed")]
    Completed,
}

#[derive(Deserialize)]
pub(crate) struct WorkflowJob {
    pub(crate) id: u64,
    pub(crate) labels: Vec<String>,
    pub(crate) run_id: u64,
    pub(crate) runner_name: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct Repository {
    pub(crate) full_name: String,
}
