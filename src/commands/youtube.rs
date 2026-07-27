/// YouTube command handler
/// Supports: youtube [query] -> redirects to youtube.com, searching if a query is provided
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct YouTubeCommand;

impl BunnylolCommand for YouTubeCommand {
    const BINDINGS: &'static [&'static str] = &["youtube", "yt"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.youtube.com".to_string()
        } else {
            build_search_url("https://www.youtube.com/results", "search_query", query)
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Search YouTube videos".to_string(),
            example: "youtube rust tutorial".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_youtube_command_no_args() {
        assert_eq!(
            YouTubeCommand::process_args("youtube"),
            "https://www.youtube.com"
        );
    }

    #[test]
    fn test_youtube_command_with_query() {
        assert_eq!(
            YouTubeCommand::process_args("youtube rust tutorial"),
            "https://www.youtube.com/results?search_query=rust%20tutorial"
        );
    }

    #[test]
    fn test_youtube_command_short_binding() {
        assert_eq!(
            YouTubeCommand::process_args("yt rust tutorial"),
            "https://www.youtube.com/results?search_query=rust%20tutorial"
        );
    }
}
