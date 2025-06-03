use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

fn group_tuples_into_map<K, V>(data: Vec<(K, V)>) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash,
{
    let mut result_map:HashMap<K, Vec<V>> = HashMap::new();
    for (key, value) in data {
        result_map.entry(key).or_default().push(value);
    }
    result_map
}

fn main() {
    let example_data_1 = vec![
        ("alpha", 1),
        ("beta", 2),
        ("alpha", 3),
        ("gamma", 4),
        ("beta", 5),
        ("alpha", 6),
    ];

    let grouped_1 = group_tuples_into_map(example_data_1);
    println!("{:?}", grouped_1);
    // Example output (order of keys might vary):
    // {"alpha": [1, 3, 6], "beta": [2, 5], "gamma": [4]}

    let example_data_2 = vec![
        (100, "apple".to_string()),
        (200, "banana".to_string()),
        (100, "avocado".to_string()),
        (300, "cherry".to_string()),
    ];
    let grouped_2 = group_tuples_into_map(example_data_2);
    println!("{:?}", grouped_2);
    // Example output (order of keys might vary):
    // {100: ["apple", "avocado"], 200: ["banana"], 300: ["cherry"]}
}