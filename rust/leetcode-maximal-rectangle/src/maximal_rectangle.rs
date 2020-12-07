/*
 * @lc app=leetcode id=85 lang=rust
 *
 * [85] Maximal Rectangle
 */

// @lc code=start

/*

        0 0 0 0 0 0 0 1 1
        0 1 0 0 0 0 0 1 1 
        0 0 1 0 0 1 1 1 1
        0 0 1 0 0 1 1 1 1
        0 0 0 1 0 1 1 1 1
        0 0 0 0 1 1 1 1 1
        0 1 1 1 1 1 1 1 1
        0 1 1 1 1 1 1 1 1
        0 0 0 0 1 1 1 1 1

    Point[7, 7]的对角点有：[(0, 7), (2, 5), (5, 4), (6, 1)]
*/

use std::cmp::max;

pub struct Solution{}

struct Point {
    ch: char,
    pos: (usize, usize),
    diagonal_pts: Vec<(usize, usize)>,
    left_reach: i16,  // 从该point一直沿着x轴往左，一直到left_reach，都是'1'
    up_reach: i16   // 从该point一直沿着y轴往上，一直到up_reach，都是'1'
}

impl Point {
    fn r(&self) -> usize { self.pos.0 }
    fn c(&self) -> usize { self.pos.1 }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.pos.0, self.pos.1)
    }
}

fn left_point<'a>(point: &'a Point, pts_matrix: &'a Vec<Vec<Point>>) -> Option<&'a Point> {
    if point.c() == 0 {
        None
    }
    else {
        Some(&pts_matrix[point.r()][point.c()-1])
    }
}

fn up_point<'a>(point: &'a Point, pts_matrix: &'a Vec<Vec<Point>>) -> Option<&'a Point> {
    if point.r() == 0 {
        None
    }
    else {
        Some(&pts_matrix[point.r()-1][point.c()])
    }
}

impl Solution {
    // 从point一路向左，连续的1一直到哪个col为止（包含）
    fn determine_left_reach(point: &Point, pts_matrix: &Vec<Vec<Point>>) -> i16 {
        if point.ch == '0' { -1 }
        else {
            let lp = left_point(point, pts_matrix);
            match lp {
                Some(p) => if p.left_reach >= 0 { p.left_reach as i16 } else { point.c() as i16 }
                None => point.c() as i16
            }
        }
    }

    // 从point一路向上，连续的1一直到哪个row为止（包含）
    fn determine_up_reach(point: &Point, pts_matrix: &Vec<Vec<Point>>) -> i16 {
        if point.ch == '0' { -1 }
        else {
            let up = up_point(point, pts_matrix);
            match up {
                Some(p) => if p.up_reach >= 0 { p.up_reach as i16 } else { point.r() as i16 }
                None => point.r() as i16
            }
        }
    }

    // 确定某个节点的对角点
    fn determine_diagonal_pts(point: &Point, pts_matrix: &Vec<Vec<Point>>) -> Vec<(usize, usize)> {
        let mut diagonal_pts: Vec<(usize, usize)> = vec![];
        let up_reach = point.up_reach;
        let left_reach = point.left_reach;
        if point.ch == '1' {
            diagonal_pts.push(point.pos)
        }
        if point.r() == 0 && point.c() == 0 {
            if point.ch == '1' {
                diagonal_pts.push(point.pos);
            }
        }
        else if point.c() == 0 {
            if up_reach == point.r() as i16 {
                diagonal_pts.push(point.pos)
            }
            else if up_reach >= 0 {
                diagonal_pts.push(pts_matrix[up_reach as usize][0].pos);
            }
        }
        else if point.r() == 0 {
            if left_reach == point.c() as i16 {
                diagonal_pts.push(point.pos)
            }
            else if left_reach >= 0 {
                diagonal_pts.push(pts_matrix[0][left_reach as usize].pos);
            }
        }
        else {
            let buddy_pt = &pts_matrix[point.r()-1][point.c()];
            let buddy_diag_pts = &buddy_pt.diagonal_pts;
            for pt in buddy_diag_pts {
                if (up_reach >= 0 && pt.0 as i16 >= up_reach) && (left_reach >= 0 && pt.1 as i16 >= left_reach) {
                    diagonal_pts.push(*pt);
                }
            }
            let buddy_pt = &pts_matrix[point.r()][point.c()-1];
            let buddy_diag_pts = &buddy_pt.diagonal_pts;
            for pt in buddy_diag_pts {
                if (up_reach >= 0 && pt.0 as i16 >= up_reach) && (left_reach >= 0 && pt.1 as i16 >= left_reach) {
                    diagonal_pts.push(*pt);
                }
            }
        }
        diagonal_pts
    }

    fn maximal_rectangle_for_pt(point: &Point) -> i32 {
        let mut max_size = 0;
        for diag_pt_pos in point.diagonal_pts.iter() {
            max_size = max(max_size, (i32::abs(point.r() as i32 - diag_pt_pos.0 as i32) + 1) * (i32::abs(point.c() as i32 - diag_pt_pos.1 as i32) + 1));
        }
        max_size
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut pts_matrix: Vec<Vec<Point>> = Vec::new();
        let mut max_size = 0;

        for (i, row) in matrix.iter().enumerate() {
            pts_matrix.push(vec![]);
            for (j, col) in row.iter().enumerate() {
                let mut p = Point{ch: *col, pos: (i, j), diagonal_pts: vec![], left_reach: -1, up_reach: -1};
                p.left_reach = Solution::determine_left_reach(&p, &pts_matrix);
                p.up_reach = Solution::determine_up_reach(&p, &pts_matrix);
                p.diagonal_pts = Solution::determine_diagonal_pts(&p, &pts_matrix);
                max_size = max(max_size, Solution::maximal_rectangle_for_pt(&p));
                // println!("point={:?}, diag={:?}, left_reach={}, up_reach={}", p.pos, p.diagonal_pts, p.left_reach, p.up_reach);
                pts_matrix[i].push(p);
            }
        }

        /*
        for (_, row) in pts_matrix.iter().enumerate() {
            for (_, col) in row.iter().enumerate() {
                println!("{}", col);
            }
        }
        */
        max_size
    }
}

// @lc code=end

