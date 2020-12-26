use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();
    names.insert("James".to_string());
    names.insert("Ted".to_string());
    names.insert("James".to_string());    
    println!("{:?} contains {}? {}, contains {}? {}", names, "James", names.contains("James"), "james", names.contains("james"));

    let mut pairs = HashSet::new();
    pairs.insert((1, 2));
    pairs.insert((2, 1));
    pairs.insert((1, 2));
    pairs.insert((2, 3));
    println!("{:?} contains {:?}? {}, contains {:?}? {}", pairs, (1, 2), pairs.contains(&(1, 2)), (1, 3), pairs.contains(&(1, 3)));
}
