//! Build a list of ordered data from the `track.yaml` file

use crate::utils::print;
use crate::yaml::parse::Entry;

use std::collections::{HashMap, HashSet};

pub fn to_ordered_vec(entries: Vec<Entry>) -> Vec<Entry> {
    if let Some(conflicts) = find_circular_dependencies(&entries) {
        print::error(format!(
            "circular dependency detected in the provided entries: {:?}",
            conflicts
        ));
        std::process::exit(1);
    }

    let mut adj_list: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut entry_map: HashMap<&str, &Entry> = HashMap::new();

    // Initialize adjacency list and in-degree map
    for entry in &entries {
        let name = entry.name.as_str();
        adj_list.insert(name, Vec::new());
        in_degree.insert(name, 0);
        entry_map.insert(name, entry);
    }

    // Populate adjacency list and in-degree map
    for entry in &entries {
        let name = entry.name.as_str();
        if let Some(depends_on) = &entry.depends_on {
            if let Some(neighbors) = adj_list.get_mut(depends_on.as_str()) {
                neighbors.push(name);
            }
            if let Some(degree) = in_degree.get_mut(name) {
                *degree += 1;
            }
        }
    }

    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&name, _)| name)
        .collect();

    let mut ordered_entries: Vec<Entry> = Vec::new();

    while let Some(node) = queue.pop() {
        if let Some(entry) = entry_map.get(node) {
            ordered_entries.push((*entry).clone());

            if let Some(neighbors) = adj_list.get(node) {
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(neighbor);
                        }
                    }
                }
            }
        }
    }

    ordered_entries
}

/// Determines if there is a circular dependency in a vector of `Entry` structs
fn find_circular_dependencies(entries: &Vec<Entry>) -> Option<Vec<String>> {
    if entries.is_empty() {
        return None;
    }

    let all_entry_names: HashSet<&str> = entries.iter().map(|e| e.name.as_str()).collect();

    let mut adj_map: HashMap<&str, &str> = HashMap::new();
    for entry in entries {
        if let Some(dep_name_str) = &entry.depends_on {
            if all_entry_names.contains(dep_name_str.as_str()) {
                adj_map.insert(entry.name.as_str(), dep_name_str.as_str());
            }
        }
    }

    let mut visiting: HashSet<&str> = HashSet::new();
    let mut resolved: HashSet<&str> = HashSet::new();

    for entry in entries {
        let node_name = entry.name.as_str();
        if !resolved.contains(node_name)
            && detect_cycle_recursive(node_name, &adj_map, &mut visiting, &mut resolved).is_some()
        {
            return Some(visiting.iter().map(|s| s.to_string()).collect()); // Cycle detected
        }
    }

    None // No cycles found after checking all nodes
}

// Helper recursive function to detect cycles using DFS
fn detect_cycle_recursive<'a>(
    current_node_name: &'a str,          // The node currently being visited
    adj_map: &HashMap<&'a str, &'a str>, // The dependency graph
    visiting: &mut HashSet<&'a str>,     // Nodes in the current recursion path
    resolved: &mut HashSet<&'a str>,     // Nodes already processed and confirmed
) -> Option<Vec<String>> {
    visiting.insert(current_node_name);

    // Check if the current node has a dependency in our filtered adjacency map
    if let Some(dependency_target_ref) = adj_map.get(current_node_name) {
        let dependency_target_name: &'a str = dependency_target_ref;

        // If the dependency target is already in the 'visiting' set, we've found a cycle
        if visiting.contains(dependency_target_name) {
            return Some(visiting.iter().map(|s| s.to_string()).collect());
        }

        // If the dependency target hasn't been 'resolved' yet, recurse to explore further
        // Nodes in 'resolved' are known to be safe (or part of an already reported cycle path)
        if !resolved.contains(dependency_target_name)
            && detect_cycle_recursive(dependency_target_name, adj_map, visiting, resolved).is_some()
        {
            return Some(visiting.iter().map(|s| s.to_string()).collect()); // Cycle found in a deeper part of the dependency chain
        }
    }

    // Backtrack: Remove the current node from 'visiting' as we are done exploring from it in THIS path
    visiting.remove(current_node_name);
    // Mark the current node as 'resolved': all its outgoing paths have been explored from this point,
    // and no cycle was found involving it in this specific exploration path
    resolved.insert(current_node_name);

    None // No cycle detected starting from this node in this path
}
