use std::process;

use gh_wf_serde::*; 
fn main() {
    eprintln!("-------------- GITHUB WORKFLOW SERIALIZER DESERIALIZER --------------");
    eprintln!("-------------- 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 --------------"); 

    let dummy_data = WorkflowConfig::dummy_workflow();
    let yaml = WorkflowConfig::serializer(&dummy_data); 
    
    if let Err(err) = yaml {
        eprintln!("Error: {}", err); 
        process::exit(1); 

    } 

    let wf = WorkflowConfig::deserialize_with_path(None); 
    if let Err(err) = wf {
        eprintln!("Error: {}", err); 
        process::exit(1); 

    } 


 

}
