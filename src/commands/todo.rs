/// todo command handler
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct TodoCommand;

const BASE_URL: &str = "https://todo.lxwhomelab.cc";

impl BunnylolCommand for TodoCommand {
    const BINDINGS: &'static [&'static str] = &["todo"];
    fn process_args(args: &str) -> String {
        let base_url = BASE_URL;
        let query = Self::get_command_args(args);
        if query.is_empty() {
            base_url.to_string()
        } else {
            //format!("{}/?add={}", base_url, query)
            build_search_url(base_url, "add", query)
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to todo".to_string(),
            example: "todo".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_command() {
        assert_eq!(TodoCommand::process_args("todo"), BASE_URL);
    }

    #[test]
    fn test_todo_command_with_args() {
        assert_eq!(
            TodoCommand::process_args("todo some args"),
            format!("{}?add=some%20args", BASE_URL)
        );
    }
}
