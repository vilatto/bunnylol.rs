/// Google Docs command handler
/// Supports: docs, gdoc -> redirects to Google Docs; with a title, creates a
/// new doc pre-titled with it (the same trick as Chrome's "doc.new <title>")
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct GoogleDocsCommand;

impl BunnylolCommand for GoogleDocsCommand {
    const BINDINGS: &'static [&'static str] = &["docs", "gdoc", "doc"];

    fn process_args(args: &str) -> String {
        let title = Self::get_command_args(args);
        if title.is_empty() {
            "https://docs.google.com/document/u/0/".to_string()
        } else {
            build_search_url("https://docs.google.com/document/create", "title", title)
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Google Docs, or create a new doc with a title".to_string(),
            example: "docs My Meeting Notes".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_docs_command() {
        assert_eq!(
            GoogleDocsCommand::process_args("docs"),
            "https://docs.google.com/document/u/0/"
        );
        assert_eq!(
            GoogleDocsCommand::process_args("gdoc"),
            "https://docs.google.com/document/u/0/"
        );
    }

    #[test]
    fn test_google_docs_command_with_title() {
        assert_eq!(
            GoogleDocsCommand::process_args("docs My Meeting Notes"),
            "https://docs.google.com/document/create?title=My%20Meeting%20Notes"
        );
        assert_eq!(
            GoogleDocsCommand::process_args("gdoc Q3 Planning"),
            "https://docs.google.com/document/create?title=Q3%20Planning"
        );
        assert_eq!(
            GoogleDocsCommand::process_args("doc Q3 Planning"),
            "https://docs.google.com/document/create?title=Q3%20Planning"
        );
    }
}
