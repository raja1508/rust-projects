use serde::{Deserialize, Serialize};
use std::collections::HashMap; 

#[derive(Debug, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct WorkflowConfig {
    pub name: String,
    #[serde(rename = "on")]
    pub trigger: TriggerEvents,
    #[serde(skip_serializing_if = "Option::is_none" )]
    pub env: Option<HashMap<String, String>>,
    pub jobs: HashMap<String, Job>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerEvents {
    Single(PipelineEvent),
    List(Vec<PipelineEvent>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PipelineEvent {
    Push,
    PullRequest,
    Release,
    ManualTrigger,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    #[serde(rename = "runs-on")]
    pub runner: RunnerImage,
    #[serde(skip_serializing_if = "Option::is_none" )]
    pub needs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none" )]
    pub env: Option<HashMap<String, String>>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RunnerImage {
    UbuntuLatest,
    Ubuntu2204,
    Ubuntu2404,
    MacosLatest,
    Macos14,
    WindowsLatest,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShellEnvironment {
    Bash,
    Sh,
    Powershell,
    Cmd,
}

/// Wrapper representing an individual execution item inside a job.
#[derive(Debug, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct Step {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none" )]
    pub env: Option<HashMap<String, String>>,
   
    // Flattened so that the keys inside `StepExecutionMode` sit directly within `Step`
    #[serde(flatten)]
    pub execution: StepExecutionMode,
}

/// An enum structure forcing mutual exclusivity.
/// A step can natively execute a script OR execute an Action Plugin, but never both.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum StepExecutionMode {
    /// A custom command execution configuration.
    Script {
        run: String,
        // Enforces a structured enum default value if a shell isn't explicitly defined
        #[serde(default = "default_shell")]
        shell: ShellEnvironment,
    },
    /// A pre-built modular action compilation invocation.
    Plugin {
        uses: String,
        // Leverages standard generic values instead of raw text maps for modular actions
        #[serde(skip_serializing_if = "Option::is_none" )]
        with: Option<HashMap<String, serde_yaml::Value>>,
    },
}

// Helper to safely fall back to a safe default shell environment
fn default_shell() -> ShellEnvironment {
    ShellEnvironment::Bash
}

