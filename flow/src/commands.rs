use crate::storage::{self, Prompt};
use dialoguer::{Editor, Input, theme::ColorfulTheme};
use uuid::Uuid;

pub fn add(alias_arg: Option<String>, tags_arg: Option<String>) {
    let theme = ColorfulTheme::default();

    let alias: String = match alias_arg {
        Some(a) => a,
        None => Input::with_theme(&theme)
            .with_prompt("Alias")
            .interact_text()
            .unwrap(),
    };

    let description: String = Input::with_theme(&theme)
        .with_prompt("Description")
        .interact_text()
        .unwrap();

    let tags_input: String = match tags_arg {
        Some(t) => t,
        None => Input::with_theme(&theme)
            .with_prompt("Tags (comma separated)")
            .interact_text()
            .unwrap_or_default(),
    };

    let tags: Vec<String> = tags_input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let template = Editor::new()
        .edit("# Enter your prompt template here. Lines starting with # are ignored.\n")
        .unwrap();

    let template = match template {
        Some(t) => t
            .lines()
            .filter(|l| !l.trim().starts_with('#'))
            .collect::<Vec<&str>>()
            .join("\n")
            .trim()
            .to_string(),
        None => {
            println!("Aborted: No template provided.");
            return;
        }
    };

    if template.is_empty() {
        println!("Aborted: Empty template.");
        return;
    }

    let prompt = Prompt {
        id: Uuid::new_v4().to_string(),
        alias: alias.clone(),
        description,
        tags,
        template,
    };

    match storage::save_prompt(prompt) {
        Ok(_) => println!("✔ Saved prompt '{}'", alias),
        Err(e) => println!("Error saving prompt: {}", e),
    }
}

pub fn list() {
    let prompts = storage::load_prompts();
    if prompts.is_empty() {
        println!("No prompts found. Run `flow add` to create one.");
        return;
    }

    println!("{:<15} {:<40} {:<20}", "ALIAS", "DESCRIPTION", "TAGS");
    println!("{}", "-".repeat(80));
    for prompt in prompts {
        let tags = prompt.tags.join(", ");
        let desc = if prompt.description.len() > 37 {
            format!("{}...", &prompt.description[..37])
        } else {
            prompt.description.clone()
        };
        println!("{:<15} {:<40} {:<20}", prompt.alias, desc, tags);
    }
}

use crate::engine;
use arboard::Clipboard;
use skim::prelude::*;
use std::io::Cursor;

pub fn use_prompt(alias: String, print_output: bool) {
    let prompts = storage::load_prompts();
    let prompt = prompts.into_iter().find(|p| p.alias == alias);

    let prompt = match prompt {
        Some(p) => p,
        None => {
            println!("Error: Prompt with alias '{}' not found.", alias);
            println!("Run `flow list` to see available prompts.");
            return;
        }
    };

    let result = engine::process(&prompt.template);

    if print_output {
        println!("{}", result);
    } else {
        match Clipboard::new() {
            Ok(mut clipboard) => match clipboard.set_text(result.clone()) {
                Ok(_) => eprintln!("✔ Copied '{}' prompt to clipboard", alias),
                Err(e) => eprintln!("Error copying to clipboard: {}", e),
            },
            Err(e) => eprintln!("Error initializing clipboard: {}", e), // Handle headless or failure
        }
    }
}

pub fn search() {
    let prompts = storage::load_prompts();
    if prompts.is_empty() {
        println!("No prompts found. Run `flow add` to create one.");
        return;
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    let input: String = prompts
        .iter()
        .map(|p| format!("{} | {}", p.alias, p.description))
        .collect::<Vec<String>>()
        .join("\n");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    if let Some(item) = selected_items.first() {
        let text = item.output().to_string();
        let alias = text.split(" | ").next().unwrap_or("").to_string();
        if !alias.is_empty() {
            use_prompt(alias, false);
        }
    }
}

use crate::tui;

pub fn ui() {
    match tui::run() {
        Ok(Some(alias)) => {
            // Selected an alias, run use_prompt logic
            // We default to clipboard (print_output = false) for TUI mode
            use_prompt(alias, false);
        }
        Ok(None) => {} // User quit
        Err(e) => eprintln!("Error running TUI: {}", e),
    }
}
