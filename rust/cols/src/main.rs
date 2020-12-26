use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();
    names.insert("James".to_string());
    names.insert("Ted".to_string());
    names.insert("James".to_string());    
    println!("{:?} contains {}? {}, contains {}? {}", names, "James", names.contains("James"), "james", names.contains("james"));
}
