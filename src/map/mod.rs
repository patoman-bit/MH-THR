use serde::Deserialize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const DEFAULT_MAP_PATH: &str = "data/california_map.json";

#[derive(Debug, Deserialize, Clone)]
struct City {
    name: String,
    connections: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct MapData {
    cities: HashMap<String, City>,
}

pub fn start_map_module() {
    println!("\nWelcome to Map Visualization.");
    println!("Available Commands: View, Route, Ping, Upload, Exit\n");

    let mut map_data = load_map_data(DEFAULT_MAP_PATH).unwrap_or_else(|err| {
        eprintln!("Failed to load default map ({}). Using fallback map.", err);
        fallback_map()
    });
    let mut pings: HashSet<String> = HashSet::new();

    loop {
        print!("MAP: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "view" => view_map(&map_data, &pings),
            "route" => run_route_prompt(&map_data),
            "ping" => add_ping(&map_data, &mut pings),
            "upload" => {
                if upload_map(&mut map_data) {
                    pings.clear();
                }
            }
            "exit" => {
                println!("Exiting Map Module.");
                break;
            }
            _ => println!("Command not recognized."),
        }
    }
}

fn view_map(map_data: &MapData, pings: &HashSet<String>) {
    println!("\nCities and connections:");
    for (city_name, city) in &map_data.cities {
        let connections = city
            .connections
            .iter()
            .map(|conn| format!("-> {}", conn))
            .collect::<Vec<_>>()
            .join(", ");
        let ping_marker = if pings.contains(city_name) {
            " [PINGED]"
        } else {
            ""
        };
        println!("- {}: {}{}", city.name, connections, ping_marker);
    }
    println!();
}

fn run_route_prompt(map_data: &MapData) {
    let start = prompt_for_city("Enter starting city:");
    let end = prompt_for_city("Enter destination city:");

    let Some(start_city) = resolve_city(map_data, &start) else {
        println!("City '{}' not found.", start);
        return;
    };
    let Some(end_city) = resolve_city(map_data, &end) else {
        println!("City '{}' not found.", end);
        return;
    };

    let route = find_route(map_data, start_city, end_city);
    if route.is_empty() {
        println!("No route found from '{}' to '{}'.", start_city, end_city);
    } else {
        println!("Route: {}", route.join(" -> "));
    }
}

fn add_ping(map_data: &MapData, pings: &mut HashSet<String>) {
    let city_input = prompt_for_city("Enter city to ping:");
    let Some(city_name) = resolve_city(map_data, &city_input) else {
        println!("City '{}' not found.", city_input);
        return;
    };

    if pings.insert(city_name.to_string()) {
        println!("Ping added to {}.", city_name);
    } else {
        println!("{} was already pinged.", city_name);
    }
}

fn upload_map(map_data: &mut MapData) -> bool {
    let path = prompt_for_city("Enter path to a map JSON file:");
    match load_map_data(&path) {
        Ok(new_map) => {
            *map_data = new_map;
            println!("Loaded map from {}.", path);
            true
        }
        Err(err) => {
            println!("Failed to load map '{}': {}", path, err);
            false
        }
    }
}

fn prompt_for_city(message: &str) -> String {
    print!("{} ", message);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    input.trim().to_string()
}

fn resolve_city<'a>(map_data: &'a MapData, input: &str) -> Option<&'a str> {
    map_data
        .cities
        .keys()
        .find(|name| name.eq_ignore_ascii_case(input))
        .map(|name| name.as_str())
}

fn load_map_data<P: AsRef<Path>>(path: P) -> Result<MapData, String> {
    let path_ref = path.as_ref();
    let file_content = fs::read_to_string(path_ref)
        .map_err(|err| format!("Could not read {}: {}", path_ref.display(), err))?;
    serde_json::from_str(&file_content)
        .map_err(|err| format!("Failed to parse JSON in {}: {}", path_ref.display(), err))
}

fn fallback_map() -> MapData {
    let cities = [
        (
            "San Francisco".to_string(),
            City {
                name: "San Francisco".to_string(),
                connections: vec!["Los Angeles".to_string(), "Sacramento".to_string()],
            },
        ),
        (
            "Los Angeles".to_string(),
            City {
                name: "Los Angeles".to_string(),
                connections: vec!["San Francisco".to_string(), "San Diego".to_string()],
            },
        ),
        (
            "San Diego".to_string(),
            City {
                name: "San Diego".to_string(),
                connections: vec!["Los Angeles".to_string(), "Sacramento".to_string()],
            },
        ),
        (
            "Sacramento".to_string(),
            City {
                name: "Sacramento".to_string(),
                connections: vec!["San Francisco".to_string(), "San Diego".to_string()],
            },
        ),
    ];

    MapData {
        cities: HashMap::from_iter(cities),
    }
}

fn find_route(map_data: &MapData, start: &str, end: &str) -> Vec<String> {
    if start == end {
        return vec![start.to_string()];
    }

    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<Vec<&str>> = VecDeque::new();

    queue.push_back(vec![start]);

    while let Some(path) = queue.pop_front() {
        if let Some(&current) = path.last() {
            if !visited.insert(current) {
                continue;
            }

            if let Some(city) = map_data.cities.get(current) {
                for neighbor in &city.connections {
                    let neighbor_ref = neighbor.as_str();
                    let mut next_path = path.clone();
                    next_path.push(neighbor_ref);

                    if neighbor_ref.eq(end) {
                        return next_path.into_iter().map(String::from).collect();
                    }

                    queue.push_back(next_path);
                }
            }
        }
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_map() -> MapData {
        fallback_map()
    }

    #[test]
    fn finds_route_between_connected_cities() {
        let map = sample_map();
        let route = find_route(&map, "San Francisco", "San Diego");
        assert_eq!(route.first().map(String::as_str), Some("San Francisco"));
        assert_eq!(route.last().map(String::as_str), Some("San Diego"));
        assert!(route.len() >= 2);
    }

    #[test]
    fn returns_empty_when_no_route_exists() {
        let mut map = sample_map();
        map.cities.insert(
            "Isolated".into(),
            City {
                name: "Isolated".into(),
                connections: vec![],
            },
        );
        let route = find_route(&map, "Isolated", "San Francisco");
        assert!(route.is_empty());
    }

    #[test]
    fn resolves_city_case_insensitively() {
        let map = sample_map();
        assert_eq!(resolve_city(&map, "san francisco"), Some("San Francisco"));
    }
}
