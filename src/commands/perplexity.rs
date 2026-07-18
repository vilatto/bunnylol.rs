/// Perplexity command handler
/// Supports: perplexity [query] -> redirects to perplexity.ai, with query if provided
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct PerplexityCommand;

impl BunnylolCommand for PerplexityCommand {
    const BINDINGS: &'static [&'static str] = &["perplexity", "pplx", "p"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.perplexity.ai".to_string()
        } else {
            build_search_url("https://www.perplexity.ai/search", "q", query)
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Search Perplexity AI".to_string(),
            example: "perplexity rust vs go performance".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perplexity_command_no_args() {
        assert_eq!(
            PerplexityCommand::process_args("perplexity"),
            "https://www.perplexity.ai"
        );
    }

    #[test]
    fn test_perplexity_command_with_query() {
        assert_eq!(
            PerplexityCommand::process_args("perplexity what is glm 5.2"),
            "https://www.perplexity.ai/search?q=what%20is%20glm%205.2"
        );
    }

    #[test]
    fn test_perplexity_command_short_binding() {
        assert_eq!(
            PerplexityCommand::process_args("pplx rust vs go"),
            "https://www.perplexity.ai/search?q=rust%20vs%20go"
        );
    }

    #[test]
    fn test_perplexity_command_single_char_binding() {
        assert_eq!(
            PerplexityCommand::process_args("p rust vs go"),
            "https://www.perplexity.ai/search?q=rust%20vs%20go"
        );
    }
}
