use dialoguer::{Confirm, Input, Select, theme};
use std::collections::HashMap;
use crate::{ShellEnvironment, Step, StepExecutionMode};

pub fn prompt_step() -> Step {
    let theme = theme::ColorfulTheme::default();
    let name: String = Input::with_theme(&theme)
        .with_prompt("Step name")
        .interact_text()
        .unwrap();

    let env = None; 

    let mode_options = &["Script (run a shell command)", "Plugin (use an action)"];
    let mode_choice = Select::with_theme(&theme)
        .with_prompt("Execution mode")
        .items(mode_options)
        .default(0)
        .interact()
        .unwrap();

    let execution = if mode_choice == 0 {
        let run: String = Input::new()
            .with_prompt("Command to run")
            .interact_text()
            .unwrap();

        let shell = prompt_shell();

        StepExecutionMode::Script { run, shell }
    } else {
        let uses: String = Input::new()
            .with_prompt("Plugin/action to use (e.g. actions/checkout@v4)")
            .interact_text()
            .unwrap();

        // let with = prompt_with_args();

        StepExecutionMode::Plugin { uses, with:None}
        // StepExecutionMode::Plugin None
    };

    Step { name, env, execution }
}

pub fn prompt_shell() -> ShellEnvironment {
    let theme = theme::ColorfulTheme::default(); 
    let shells = &["Bash", "Zsh", "Powershell"]; // adjust to match your actual enum variants
    let choice = Select::with_theme(&theme)
        .with_prompt("Shell")
        .items(shells)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => ShellEnvironment::Bash,
        1 => ShellEnvironment::Zsh,
        2 => ShellEnvironment::Powershell,
        _ => unreachable!(),
    }
}

pub fn prompt_with_args() -> Option<HashMap<String, String>> {
    let has_args = Confirm::new()
        .with_prompt("Add 'with' arguments for this plugin?")
        .default(false)
        .interact()
        .unwrap();

    if !has_args {
        return None;
    }

    let mut with = HashMap::new();
    loop {
        let key: String = Input::new()
            .with_prompt("Arg name (leave empty to finish)")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if key.trim().is_empty() {
            break;
        }

        let value: String = Input::new()
            .with_prompt(format!("Value for {key}"))
            .interact_text()
            .unwrap();

        with.insert(key, value);
    }

    if with.is_empty() { None } else { Some(with) }
}