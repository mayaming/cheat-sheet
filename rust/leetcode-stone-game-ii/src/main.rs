mod stone_game_ii;

fn main() {
    println!("{}", stone_game_ii::Solution::stone_game_ii(vec![2, 7, 9, 4, 4]));
    println!("{}", stone_game_ii::Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]));
    println!("{}", stone_game_ii::Solution::stone_game_ii(vec![5]));
    println!("{}", stone_game_ii::Solution::stone_game_ii(vec![1, 1, 1]));
}
