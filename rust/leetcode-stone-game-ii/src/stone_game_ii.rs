pub struct Solution{} 

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let len = piles.len();
        let psum: Vec<Vec<i32>> = pile_sum(piles);
        let mut vals: Vec<Vec<i32>> = vec![vec![0; len]; 100];

        return 0
    }

    fn pile_sum(piles: Vec<i32>) -> Vec<Vec<i32>> {
        let len = piles.len();
        let mut psum = vec![vec![0; len]; len];
        for i in 0..len {
            for j in i..len {
                psum[i][j] = if i == j { piles[i] } else { psum[i][j-1] + piles[j] };
            }
        }
        return psum;
    }

    fn calc(m: i32, idx: i32, vals: Vec<Vec<i32>>) -> i32 {
        for x in 1..(m+1) {
            
        }
    }
}