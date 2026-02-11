use colored::*;

pub struct OutputFormatter {
    syntax_highlighting: bool,
    markdown_rendering: bool,
}

impl OutputFormatter {
    pub fn new(syntax_highlighting: bool, markdown_rendering: bool) -> Self {
        Self {
            syntax_highlighting,
            markdown_rendering,
        }
    }
    
    pub fn print_response(&self, text: &str) {
        if self.markdown_rendering {
            // TODO: Implement markdown rendering
            println!("{}", text);
        } else {
            println!("{}", text);
        }
    }
    
    pub fn print_error(&self, error: &str) {
        eprintln!("{} {}", "Error:".red().bold(), error);
    }
    
    pub fn print_success(&self, message: &str) {
        println!("{} {}", "✓".green(), message);
    }
    
    pub fn print_info(&self, message: &str) {
        println!("{} {}", "→".blue(), message);
    }
}