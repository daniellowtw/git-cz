use git_commitizen::{
    add_all_tracked_files, build_commit_message, build_commit_types, format_commit_types, 
    has_unstaged_tracked_files, perform_commit,
};
use promkit::preset::query_selector::QuerySelector;
use promkit::{preset::readline::Readline, suggest::Suggest};
use std::env;
use std::path::Path;
use std::process::Command;
use tempfile;

fn optional_confirm(message: &str, default: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let default_text = if default { "Y" } else { "N" };
    let prompt_text = format!("{} (y/n, default {})", message, default_text);
    
    let mut input = Readline::default()
        .title(&prompt_text)
        .prompt()?;
    
    let response = input.run()?.trim().to_lowercase();
    
    match response.as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        "" => Ok(default), // Empty input uses default
        _ => Ok(default), // Invalid input uses default
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if there are unstaged tracked files and prompt to add them
    if has_unstaged_tracked_files(Path::new("."))? {
        if optional_confirm("There are unstaged changes for tracked files. Do you want to add them?", true)? {
            add_all_tracked_files(Path::new("."))?;
            println!("Staged all tracked files with changes.");
        }
    }
    
    let commit_types = build_commit_types();
    let commit_types_display = format_commit_types(commit_types);

    let mut p = QuerySelector::new(commit_types_display.clone(), |text, items| -> Vec<String> {
        items
            .iter()
            .filter(|item| item.contains(text))
            .cloned()
            .collect()
    })
    .title("Select the type of change that you're committing:")
    .listbox_lines(10)
    .prompt()?;

    let mut scope_input = Readline::default()
        .title("Denote the scope of this change (optional):")
        .enable_suggest(Suggest::from_iter([
            "app", "core", "ui", "db", "api", "frontend", "backend", "config", "build", "sec",
            "infra", "deps",
        ]))
        .prompt()?;

    let mut description_input = Readline::default()
        .title("Write a short, imperative tense description of the change:")
        .prompt()?;
    let mut body_input = Readline::default()
        .title("Provide a longer description of the change(press 'e' to open editor):")
        .prompt()?;

    let selection = p.run()?;
    let selected_type = selection.split_whitespace().next();

    if let Some(commit_type) = selected_type {
        let scope = scope_input.run()?;
        let description = description_input.run()?;
        let body = body_input.run()?;

        let body = if body.trim().to_lowercase() == "e" {
            // Create a temporary file
            let temp_file = tempfile::NamedTempFile::new()?;
            let temp_path = temp_file
                .path()
                .to_str()
                .expect("Failed to get temp file path");

            // Determine the editor command
            let editor_command = if cfg!(target_os = "windows") {
                env::var("EDITOR").unwrap_or_else(|_| "notepad".to_string())
            } else {
                env::var("EDITOR").unwrap_or_else(|_| "vim".to_string())
            };

            // Open the editor
            let status = Command::new(&editor_command).arg(temp_path).status()?;

            if !status.success() {
                eprintln!("Editor exited with non-zero status");
            }

            // Read the contents of the temp file
            std::fs::read_to_string(temp_path)?
        } else {
            body
        };

        // New footer confirmation prompt
        let footer = if optional_confirm("Do you want to add a footer?", false)? {
            let mut footer_type_input = QuerySelector::new(
                vec!["fix".to_string(), "close".to_string()],
                |text, items| -> Vec<String> {
                    items
                        .iter()
                        .filter(|item| item.contains(text))
                        .cloned()
                        .collect()
                },
            )
            .title("Select the footer type:")
            .listbox_lines(2)
            .prompt()?;

            let mut issue_number_input = Readline::default()
                .title("Enter the issue number:")
                .validator(
                    |text| text.trim().parse::<i32>().is_ok(),
                    |text| format!("'{}' is not a valid integer", text),
                )
                .prompt()?;

            let footer_type = footer_type_input.run()?;
            let issue_number = issue_number_input.run()?;
            format!("{}: #{}", footer_type, issue_number)
        } else {
            String::new()
        };

        let full_commit_message =
            build_commit_message(&commit_type, &scope, &description, &body, &footer);

        perform_commit(Path::new("."), &full_commit_message)?;
        println!("Commit successful!");
    }

    Ok(())
}
