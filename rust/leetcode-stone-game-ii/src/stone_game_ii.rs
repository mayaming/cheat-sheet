use std::cmp::min;

pub struct Solution{} 

// first one score, second one score
#[derive(Clone)]
struct MemoItem(i32, i32);

impl Solution {
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

    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let len = piles.len();
        let psum: Vec<Vec<i32>> = Solution::pile_sum(piles);
        let mut memo: Vec<Vec<MemoItem>> = vec![vec![MemoItem(-1, -1); len]; len];
        Solution::calc(0, 1, len, &psum, &mut memo);
        return memo[0][1].0;
    }

    // memo的2个值分别是idx和m
    fn calc(idx: usize, m: usize, len: usize, psum: &Vec<Vec<i32>>, memo: &mut Vec<Vec<MemoItem>>) -> () {
        for x in 1..(m+1) {
            if idx + x > len {
                break;
            }

            let take_start = idx;
            let take_end = idx+x-1;
            let take_sum = psum[take_start][take_end];
            let new_m = min(len, x*2);
            if take_end < len-1 && memo[take_end+1][new_m].0 < 0 {
                Solution::calc(take_end+1, new_m, len, psum, memo);
            }
            let mut this_sum = take_sum;
            if take_end < len-1 {
                // println!("{} {} {}", take_end+1, new_m, memo[take_end+1][new_m].1);
                this_sum += memo[take_end+1][new_m].1;
            }
            // println!("{} {} {}", this_sum, take_sum, memo[take_end+1][new_m].1);
            if this_sum > memo[idx][m].0 {
                memo[idx][m] = MemoItem(this_sum, psum[idx][len-1]-this_sum);
                // println!("{} {} {} {}", idx, m, memo[idx][m].0, memo[idx][m].1);
            }
        }
    }
}