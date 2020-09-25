use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
struct Solution;

impl Solution {
     pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut g: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();
        for ticket in tickets {
            g.entry(ticket[0].clone()).or_default().push(Reverse(ticket[1].clone()));
        }
        let mut result = Vec::new();
        Solution::visit(&mut g, &"JFK".to_string(), &mut result);

        result
     }

     fn visit(g: &mut HashMap<String, BinaryHeap<Reverse<String>>>, airport: &String, result: &mut Vec<String>){
        loop {
            match g.entry(airport.clone()).or_default().pop() {
                Some(Reverse(ref airport_to)) => {
                    Solution::visit(g, airport_to, result);
                },
                _ => {break;}
            }
        }

        result.insert(0, airport.clone());
     }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct Test(pub i32);
    #[test]
    fn test_string_comparison() {
        assert!("B" > "A");
        assert!("JBA" < "JBB");
    }

    #[test]
    fn test_example_1() {
        assert_eq!(vec!["JFK", "MUC", "LHR", "SFO", "SJC"], Solution::find_itinerary(vec![
            vec!["MUC".to_string(), "LHR".to_string()], 
            vec!["JFK".to_string(), "MUC".to_string()], 
            vec!["SFO".to_string(), "SJC".to_string()], 
            vec!["LHR".to_string(), "SFO".to_string()]
        ]))
    }

    #[test]
    fn test_single_cycle_single_out() {
        assert_eq!(vec!["JFK","NRT","JFK","KUL"], Solution::find_itinerary(vec![
            vec!["JFK".to_string(), "KUL".to_string()],
            vec!["JFK".to_string(), "NRT".to_string()],
            vec!["NRT".to_string(), "JFK".to_string()]
        ]))
    }
}