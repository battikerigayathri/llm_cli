use anyhow::Result;
use std::collections::HashMap;

pub struct TemplateEngine {
    templates: HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }
    
    pub fn add_template(&mut self, name: String, content: String) {
        self.templates.insert(name, content);
    }
    
    pub fn render(&self, name: &str, variables: &HashMap<String, String>) -> Result<String> {
        let template = self.templates.get(name)
            .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", name))?;
        
        let mut result = template.clone();
        for (key, value) in variables {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        
        Ok(result)
    }
}