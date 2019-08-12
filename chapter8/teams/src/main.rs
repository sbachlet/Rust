use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let map: HashMap<_, _> = vec![("Blue", 10), ("Yellow", 50)]
        .iter()
        .cloned()
        .collect();
    let score: Option<&i32> = map.get("Blue");

}
