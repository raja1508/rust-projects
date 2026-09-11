use std::process; 

use dialoguer::{Select, theme::ColorfulTheme}; 

use gh_wf_serde::{workflow_cli::{create_yaml_file_cli, deserialize_yaml_file_cli}, *}; 
fn main() {
    eprintln!("-------------- GITHUB WORKFLOW SERIALIZER DESERIALIZER --------------");
    eprintln!("-------------- 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 --------------"); 

    let dummy_data = WorkflowConfig::dummy_workflow();
    let yaml = WorkflowConfig::serializer(&dummy_data); 
    
    if let Err(err) = yaml {
        eprintln!("Error: {}", err); 
        process::exit(1); 

    } 

    let wf = WorkflowConfig::deserializer(&"./github/workflow".to_string()); 
    if let Err(err) = wf {
        eprintln!("Error: {}", err); 
        process::exit(1); 

    } 

    let theme = ColorfulTheme::default(); 

    let actions = ["Create a yaml file", "Deserialize a yaml file"]; 

    let action = Select::with_theme(&theme)
    .with_prompt("Select what actions  you want to perform ")
    .items(&actions)
    .default(0)
    .interact().unwrap(); 

    match action {
        0 => create_yaml_file_cli(),
        1 => deserialize_yaml_file_cli(),
        _ => create_yaml_file_cli()
    }

 
}
