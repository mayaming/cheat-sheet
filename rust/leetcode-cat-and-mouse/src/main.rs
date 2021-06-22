mod cat_and_mouse;

fn main() {
    println!("1 -> {}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![1,3], vec![0], vec![3], vec![0,2]]));
    println!("0 -> {}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![2,5], vec![3], vec![0,4,5], vec![1,4,5], vec![2,3], vec![0,2,3]]));
    // [[2,3],[3,4],[0,4],[0,1],[1,2]]
    println!("1 -> {}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![2,3], vec![3,4], vec![0,4], vec![0,1], vec![1,2]]));
    // [[2,3],[2],[0,1],[0,4],[3]]
    println!("2 -> {}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![2,3], vec![2], vec![0,1], vec![0,4], vec![3]]));
    // [[6],[4,11],[9,12],[5],[1,5,11],[3,4,6],[0,5,10],[8,9,10],[7],[2,7,12],[6,7],[1,4],[2,9]]
    println!("1 -> {}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![6], vec![4,11], vec![9,12], vec![5], vec![1,5,11], vec![3,4,6], vec![0,5,10], vec![8,9,10], vec![7], vec![2,7,12], vec![6,7], vec![1,4], vec![2,9]]));
    // [[2,3,4],[2,4],[0,1,4],[0,4],[0,1,2,3]]
    println!("2 -> {}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![2,3,4], vec![2,4], vec![0,1,4], vec![0,4], vec![0,1,2,3]]));
    println!("0 -> {}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![2,9,14], vec![2,5,7], vec![0,1,3,8,9,11,14], vec![2,12], vec![5,11,18], vec![1,4,18], vec![9,17,19], vec![1,11,12,13,14,17,19], vec![2,16,17], vec![0,2,6,12,14,18], vec![14], vec![2,4,7], vec![3,7,9,13], vec![7,12,14], vec![0,2,7,9,10,13,17], vec![18], vec![8,19], vec![6,7,8,14,19], vec![4,5,9,15], vec![6,7,16,17]]));
}
