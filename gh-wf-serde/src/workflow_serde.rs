use super::*; 
use std::{collections::HashMap, error::Error, fs, path::Path}; 

impl WorkflowConfig {
    pub fn build(
        name:String, 
        trigger: TriggerEvents,
        env:Option<HashMap<String, String>> , 
        jobs: HashMap<String, Job>
    ) -> Self {
        Self { name,  trigger,  env,  jobs,  }
    }

    pub fn serializer(wf: &WorkflowConfig) -> Result<(), Box<dyn Error>>{
        let yaml = serde_yaml::to_string(wf)?;
        let path = Path::new("./github/workflow"); 
        if !path.exists() {
            fs::create_dir_all(&path)?; 
        }
        let file_path = path.join("ci-cd.yaml"); 
        fs::write(file_path, yaml)?; 
        Ok(())
    }

    pub fn deserializer(wf_yaml: &String)-> Result<(), Box<dyn Error>>{
        let wf: Self = serde_yaml::from_str(wf_yaml)?;
        eprintln!("{:#?}", wf); 
        Ok(())
    }
    
    pub fn deserialize_with_path(path: Option<&String>)-> Result<(), Box<dyn Error>>{
        let  wf_path = match path {
            Some(val) => val,
            None => &String::from("./github/workflow/ci-cd.yaml")
        }; 

        let contents = fs::read(wf_path)?; 
        let wf: WorkflowConfig = serde_yaml::from_slice(&contents)?; 
        eprintln!("{:#?}", wf); 
        Ok(())
    }
    

    pub fn dummy_workflow() -> WorkflowConfig {
        let mut jobs = HashMap::new();

        jobs.insert(
            "build".to_string(),
            Job {
                runner: RunnerImage::Ubuntu2204,
                needs: None,
                env: None,
                steps: vec![
                    Step {
                        name: "Checkout code".to_string(),
                        env: None,
                        execution: StepExecutionMode::Plugin {
                            uses: "actions/checkout@v4".to_string(),
                            with: None,
                        },
                    },
                    Step {
                        name: "Run tests".to_string(),
                        env: None,
                        execution: StepExecutionMode::Script {
                            run: "cargo test".to_string(),
                            shell: ShellEnvironment::Bash,
                        },
                    },
                ],
            },
        );

        WorkflowConfig {
            name: "CI Pipeline".to_string(),
            trigger: TriggerEvents::List(vec![PipelineEvent::Push, PipelineEvent::PullRequest]),
            env: Some(HashMap::from([("RUST_LOG".to_string(), "info".to_string())])),
            jobs,
        }
    }
}