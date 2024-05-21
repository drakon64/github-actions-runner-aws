use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Webhook {
    pub(crate) action: Action,
    pub(crate) installation: Installation,
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
pub(crate) struct Installation {
    pub(crate) id: u32,
}

#[derive(Deserialize)]
pub(crate) struct WorkflowJob {
    pub(crate) labels: Vec<String>,
    pub(crate) runner_name: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct Repository {
    pub(crate) full_name: String,
    pub(crate) owner: Owner,
}

#[derive(Deserialize)]
pub(crate) struct Owner {
    pub(crate) login: String,
}
