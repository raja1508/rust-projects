use std::collections::HashMap;

use dialoguer::{Input, Select, theme};
use crate::cli_steps::prompt_step;

use super::workflow::*; 

pub fn deserialize_yaml_file_cli() {
    let theme = theme::ColorfulTheme::default(); 
    let path = Input::with_theme(&theme)
    .with_prompt("Enter the folder where .yaml file lives")
    .default("./github/workflow".to_string())
    .interact_text()
    .unwrap()
    .trim()
    .to_string(); 

    WorkflowConfig::deserializer(&path).unwrap(); 
}

pub fn create_yaml_file_cli() {
    let theme = theme::ColorfulTheme::default(); 
    let name = Input::with_theme(&theme)
    .with_prompt("Enter the name of this workflow")
    .default("CI/CD Workflow".to_string())
    .interact_text()
    .unwrap()
    .trim()
    .to_string(); 

    let mut triggers:Vec<PipelineEvent> = Vec::new(); 
    let trigger_options = ["push", "pull request", "release", "manual_trigger"];

    let trigger_quantity = Input::with_theme(&theme)
    .with_prompt("Enter the number of triggers you want")
    .default("0".to_string())
    .interact_text().unwrap(); 

    for _i in 0..trigger_quantity.parse().unwrap() {
        let trigger_idx = Select::with_theme(&theme)
        .with_prompt("Select what triggers this workflow")
        .items(&trigger_options)
        .default(0)
        .interact().unwrap();
        let trigger = match trigger_idx {
            0 => PipelineEvent::Push,
            1 => PipelineEvent::PullRequest,
            2 => PipelineEvent::Release,
            3 => PipelineEvent::ManualTrigger,
            _ => PipelineEvent::Push,
        }; 
        triggers.push(trigger);
    }

    
    let trigger =  TriggerEvents::List(triggers);
    let env: Option<HashMap<String, String>> = None;
    
    let runner_options = [
        "Ubuntu2204", "Ubuntu2404", "UbuntuLatest", "Macos14", 
        "MacosLatest", "WindowsLatest"];
        
    let runner_idx = Select::with_theme(&theme)
    .with_prompt("Select what runner image you want ")
    .items(&runner_options)
    .default(0)
    .interact().unwrap(); 

    let runner_image = match runner_idx {
        0 => RunnerImage::Ubuntu2204,
        1 => RunnerImage::Ubuntu2404,
        2 => RunnerImage::UbuntuLatest,
        3 => RunnerImage::Macos14, 
        4 => RunnerImage::MacosLatest,
        5 => RunnerImage::WindowsLatest, 
        _ => RunnerImage::Ubuntu2404
    }; 
        
        let mut steps: Vec<Step> = Vec::new(); 

        
        let step_quantity = Input::with_theme(&theme)
        .with_prompt("Enter the number of steps you want")
        .default("0".to_string())
        .interact_text().unwrap(); 
    
    for _i in 0..step_quantity.parse().unwrap() {
            let step = prompt_step();
            steps.push(step);
        }
        let mut jobs = HashMap::new();

        jobs.insert(
            "build".to_string(),
            Job {
                runner: runner_image,
                needs: None,
                env: None,
                steps: steps
                    // Step {
                    //     name: "Checkout code".to_string(),
                    //     env: None,
                    //     execution: StepExecutionMode::Plugin {
                    //         uses: "actions/checkout@v4".to_string(),
                    //         with: None,
                    //     },
                    // },
                    // Step {
                    //     name: "Run tests".to_string(),
                    //     env: None,
                    //     execution: StepExecutionMode::Script {
                    //         run: "cargo test".to_string(),
                    //         shell: ShellEnvironment::Bash,
                    //     },
                    // },
                ,
            },
        );

    let workflow = WorkflowConfig::build(name, trigger, env, jobs);
    WorkflowConfig::serializer(&workflow).unwrap(); 
}