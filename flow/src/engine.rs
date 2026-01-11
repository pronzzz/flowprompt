use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use is_terminal::IsTerminal;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::Read;

pub fn process(template: &str) -> String {
    // 1. Detect variables
    let re = Regex::new(r"\{\{\s*([a-zA-Z0-9_]+)\s*\}\}").unwrap();
    let mut variables: HashSet<String> = HashSet::new();
    for cap in re.captures_iter(template) {
        variables.insert(cap[1].to_string());
    }

    let mut values: HashMap<String, String> = HashMap::new();

    // 2. Handle stdin for {{input}}
    if variables.contains("input") {
        if !std::io::stdin().is_terminal() {
            let mut buffer = String::new();
            // We ignore errors here for now, or log them
            if let Ok(_) = std::io::stdin().read_to_string(&mut buffer) {
                // Only use if not empty to avoid empty string overriding user input if they want to type it?
                // Requirement: "If nothing is piped, prompt user".
                // If pipe is open but empty, it might be an empty pipe.
                // We trim trailing newline from pipe usually.
                if !buffer.is_empty() {
                    values.insert("input".to_string(), buffer.trim_end().to_string());
                }
            }
        }
    }

    // 3. Ask for missing variables
    let theme = ColorfulTheme::default();
    // Sort variables to ensure consistent order (HashSet is unordered)
    let mut sorted_vars: Vec<String> = variables.into_iter().collect();
    sorted_vars.sort();

    for var in sorted_vars {
        if values.contains_key(&var) {
            continue;
        }

        let value: String = Input::with_theme(&theme)
            .with_prompt(&format!("Enter value for '{}'", var))
            .interact_text()
            .unwrap();
        values.insert(var, value);
    }

    // 4. Replace
    let mut result = template.to_string();
    for (key, value) in values {
        // We use the same regex to replace to handle spacing e.g. {{ input }}
        // format!(...) requires {{ to output {
        let pattern = format!(r"\{{\{{\s*{}\s*\}}\}}", key);
        let re_replace = Regex::new(&pattern).unwrap();
        result = re_replace.replace_all(&result, &value).to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_replacement_logic() {
        // We can't easily test the interactive part, but we can test the regex
        let template = "Hello {{name}}";
        let re = Regex::new(r"\{\{\s*([a-zA-Z0-9_]+)\s*\}\}").unwrap();
        assert!(re.is_match(template));
        let cap = re.captures(template).unwrap();
        assert_eq!(&cap[1], "name");
    }
}
