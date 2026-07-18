/// ChatGPT command handler
/// Supports: chatgpt -> redirects to chatgpt.com
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct ChatGPTCommand;

impl BunnylolCommand for ChatGPTCommand {
    const BINDINGS: &'static [&'static str] = &["chatgpt"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://chatgpt.com".to_string()
        } else {
            build_search_url("https://chatgpt.com", "q", query)
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to ChatGPT".to_string(),
            example: "chatgpt".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chatgpt_command() {
        assert_eq!(
            ChatGPTCommand::process_args("chatgpt"),
            "https://chatgpt.com"
        );
    }

    #[test]
    fn test_chatgpt_command_with_args() {
        assert_eq!(
            ChatGPTCommand::process_args("chatgpt some args"),
            "https://chatgpt.com?q=some%20args"
        );
    }
}
