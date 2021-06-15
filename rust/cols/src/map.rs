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

fn default_rk() -> i32 {
    -1
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
    println!("rank for {} is {}", &city_name, city_2_rank.get(&city_name).map(|r| *r).unwrap_or_else(|| default_rk()));

    for (key, value) in &city_2_rank {
        println!("{}: {}", key, value);
    }

    let mut pt_2_str = HashMap::new();
    pt_2_str.insert(Point{x:0, y:0}, "original");
    pt_2_str.insert(Point{x:0, y:1}, "y1");
    pt_2_str.insert(Point{x:1, y:0}, "x1");
    let pt = Point{x:0, y:1};
    get_and_print(&pt_2_str, &pt);

    let pt = Point{x:0, y:1};
    pt_2_str.insert(pt, "y2");
    get_and_print(&pt_2_str, &Point{x:0, y:1});
    
    println!("get or else: {}", pt_2_str.get(&Point{x:9, y:9}).or_else(||Some(&"N/A")).unwrap());
    println!("**********end of map baisc demo*********");
}

fn get_and_print(m: &HashMap<Point, &str>, p: &Point) {
    println!("value for {} is {}", p, m.get(p).unwrap_or(&"n/a"));
}