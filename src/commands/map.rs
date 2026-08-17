/// Google Maps command handler
/// Supports: map [query] -> redirects to Google Maps, with query if provided
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct MapCommand;

impl BunnylolCommand for MapCommand {
    const BINDINGS: &'static [&'static str] = &["map", "maps"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.google.com/maps".to_string()
        } else {
            build_search_url("https://www.google.com/maps/search/", "api=1&query", query)
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Search Google Maps".to_string(),
            example: "map golden gate bridge".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_command_no_args() {
        assert_eq!(
            MapCommand::process_args("map"),
            "https://www.google.com/maps"
        );
    }

    #[test]
    fn test_map_command_with_query() {
        assert_eq!(
            MapCommand::process_args("map golden gate bridge"),
            "https://www.google.com/maps/search/?api=1&query=golden%20gate%20bridge"
        );
    }

    #[test]
    fn test_map_command_alias() {
        assert_eq!(
            MapCommand::process_args("maps golden gate bridge"),
            "https://www.google.com/maps/search/?api=1&query=golden%20gate%20bridge"
        );
    }
}
