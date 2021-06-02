mod cat_and_mouse;

fn main() {
    println!("{}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![1,3], vec![0], vec![3], vec![0,2]]));
}
