use std::collections::{HashMap, HashSet};
pub use super::macros;
struct Solution;

impl Solution {
    pub fn calc_equation(equation:Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let g = Solution::build_graph(equation, values);

        queries.iter().map(|query| match Solution::path_finding(&g, query, &mut HashSet::new()) {
            Some(res) => res,
            None => -1.0,
        }).collect()
    }

    fn build_graph(nodes: Vec<Vec<String>>, edges: Vec<f64>) -> HashMap<String, Vec<(f64, String)>> {
        let mut g: HashMap<String, Vec<(f64, String)>> = HashMap::new();
        for node in nodes.iter().enumerate() {
            g.entry(node.1[0].to_owned()).or_default().push((edges[node.0], node.1[1].to_owned()));
            g.entry(node.1[1].to_owned()).or_default().push((1.0 / edges[node.0], node.1[0].to_owned()));
        }

        g
    }

    fn path_finding(g: &HashMap<String, Vec<(f64, String)>>, nodes: &Vec<String>, set: &mut HashSet<String>) -> Option<f64> {
        if set.contains(&nodes[0]) {
            return None;
        }
        if g.contains_key(&nodes[0]) && nodes[0].eq(&nodes[1]) {
            return Some(1.0)
        }
        set.insert(nodes[0].to_owned());
        g.get(&nodes[0]).and_then(|v| {
            for edge in v.iter() {
                match Solution::path_finding(g, &vec![edge.1.clone(), nodes[1].clone()], set) {
                    Some(n) => {
                        return Some(edge.0 * n);
                    },
                    None => {},
                }
            }

            None
        })
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[macro_use]
    use crate::string_data;
    #[test]
    fn test_example_1() {
        assert_eq!(vec![6.0, 0.5, -1.0, 1.0, -1.0], 
            Solution::calc_equation(
                vec![string_data!["a", "b"], string_data!["b", "c"]],
                vec![2.0, 3.0],
                vec![ 
                    string_data!["a", "c"], 
                    string_data!["b", "a"], 
                    string_data!["a", "e"], 
                    string_data!["a", "a"], 
                    string_data!["x", "x"] ]
                )
        )
    }
}