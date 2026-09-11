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

    
    pub fn deserializer(path: &String)-> Result<(), Box<dyn Error>>{
        let folder_path = Path::new(path);  
        let folder = fs::read_dir(&folder_path)?;
        for file in folder.flatten() {
            let path = file.path(); 
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "yaml" || extension == "yml" {
                        let contents = fs::read(path)?; 
                        let wf: WorkflowConfig = serde_yaml::from_slice(&contents)?; 
                        eprintln!("{:#?}", wf); 
                    }
                }
            }
        }

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