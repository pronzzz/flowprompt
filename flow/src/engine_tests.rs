#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_detection() {
        let template = "Hello {{name}}, welcome to {{place}}!";
        let re = Regex::new(r"\{\{\s*([a-zA-Z0-9_]+)\s*\}\}").unwrap();
        let mut variables = Vec::new();
        for cap in re.captures_iter(template) {
            variables.push(cap[1].to_string());
        }
        assert!(variables.contains(&"name".to_string()));
        assert!(variables.contains(&"place".to_string()));
    }
}
