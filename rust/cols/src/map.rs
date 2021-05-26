use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

pub fn map_basic() {
    println!("**********start of map baisc demo*********");
    let mut city_2_rank = HashMap::new();
    city_2_rank.insert(String::from("beijing"), 1);
    city_2_rank.insert(String::from("shanghai"), 2);

    let city_name = String::from("shanghai");
    println!("rank for {} is {}", &city_name, city_2_rank.get(&city_name).unwrap_or(&0));
    let city_name = String::from("sanghai");
    println!("rank for {} is {}", &city_name, city_2_rank.get(&city_name).unwrap_or(&0));

    for (key, value) in &city_2_rank {
        println!("{}: {}", key, value);
    }

    let mut pt_2_str = HashMap::new();
    pt_2_str.insert(Point{x:0, y:0}, "original");
    pt_2_str.insert(Point{x:0, y:1}, "y1");
    pt_2_str.insert(Point{x:1, y:0}, "x1");
    let pt = Point{x:0, y:1};
    get_and_print(&pt_2_str, &pt);

    println!("**********end of map baisc demo*********");
}

fn get_and_print(m: &HashMap<Point, &str>, p: &Point) {
    println!("value for {} is {}", p, m.get(p).unwrap_or(&"n/a"));
}