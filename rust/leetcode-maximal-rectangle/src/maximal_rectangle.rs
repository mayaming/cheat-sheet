/*
 * @lc app=leetcode id=85 lang=rust
 *
 * [85] Maximal Rectangle
 */

// @lc code=start

/*
    ^
    |
    y
    轴

    8    0 0 0 0 0 0 0 1 1
    7    0 1 0 0 0 0 0 1 1 
    6    0 0 1 0 0 1 1 1 1
    5    0 0 1 0 0 1 1 1 1
    4    0 0 0 1 0 1 1 1 1
    3    0 0 0 0 1 1 1 1 1
    2    0 1 1 1 1 1 1 1 1
    1    0 1 1 1 1 1 1 1 1
    0    0 0 0 0 1 1 1 1 1

    原点 0 1 2 3 4 5 6 7 8  x轴->

    Point(7, 1)的对角点有：[(7, 8), (5, 6), (4, 3), (1, 2)]
*/

use std::cell::{RefCell};

pub struct Solution{}

struct Point {
    ch: char,
    pos: (usize, usize),
    diagonal_pts: RefCell<Vec<(usize, usize)>>,
    x_reach: i16,  // 从该point一直沿着x轴往左，一直到x_reach，都是'1'
    y_reach: i16   // 从该point一直沿着y轴往上，一直到y_reach，都是'1'
}

impl Point {
    fn x(&self) -> usize { self.pos.0 }
    fn y(&self) -> usize { self.pos.1 }
}

fn get_point_at(pts_matrix: Vec<Vec<Point>>, x: usize, y: usize) -> Point {
    pts_matrix[pts_matrix.len()-1-y][x]
}

fn left_point(point: &Point, pts_matrix: &Vec<Vec<Point>>) -> Option<Point> {
    if point.x() == 0 {
        None
    }
    else {
        Some(get_point_at(pts_matrix, point.x()-1, point.y()))
    }
}

fn up_point(point: &Point, pts_matrix: &Vec<Vec<Point>>) -> Option<Point> {
    if point.y() == pts_matrix.len() {
        None
    }
    else {
        Some(get_point_at(pts_matrix, point.x(), point.y()-1))
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.x(), self.y())
    }
}

impl Solution {
    // 从point一路向左，连续的1一直到哪个x坐标为止（包含）
    fn determine_x_reach(point: &Point, pts_matrix: &Vec<Vec<Point>>) -> i16 {
        if point.ch == '0' { -1 }
        else {
            let lp = left_point(point, pts_matrix);
            match lp {
                Some(p) => if p.x_reach >= 0 { p.x_reach as i16 } else { point.x() as i16 }
                None => point.x() as i16
            }
        }
    }

    // 从point一路向上，连续的1一直到哪个y坐标为止（包含）
    fn determine_y_reach(point: &Point, pts_matrix: &Vec<Vec<Point>>) -> i16 {
        if point.ch == '0' { -1 }
        else {
            let up = up_point(point, pts_matrix);
            match up {
                Some(p) => if p.y_reach >= 0 { p.y_reach as i16 } else { point.y() as i16 }
                None => point.y() as i16
            }
        }
    }

    // 确定某个节点的对角点
    fn determine_diagonal_pts(point: &Point, pts_matrix: Vec<Vec<Point>>) -> Vec<(usize, usize)> {
        let mut diagonal_pts: Vec<(usize, usize)> = vec![];
        let y_max = pts_matrix.len()-1;
        if point.x() == 0 && point.y() == y_max {
            if point.ch == '1' {
                diagonal_pts.push(point.pos)
            }

        }
        else if point.x() == 0 {
            diagonal_pts.push(get_point_at(pts_matrix, 0, point.y_reach as usize).pos)
        }
        else if point.y() == y_max {
            diagonal_pts.push(get_point_at(pts_matrix, point.x_reach as usize, y_max).pos)
        }
        else {

        }
        diagonal_pts
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let pts_matrix: RefCell<Vec<Vec<Point>>> = RefCell::new(Vec::new());

        for (i, row) in matrix.iter().enumerate() {
            pts_matrix.borrow_mut().push(vec![]);
            for (j, col) in row.iter().enumerate() {
                let mut p = Point{ch: *col, pos: (i, j), diagonal_pts: vec![], x_reach: -1, y_reach: -1};
                p.x_reach = Solution::determine_x_reach(&p, pts_matrix.borrow());
                p.y_reach = Solution::determine_y_reach(&p, pts_matrix.borrow());
                p.diagonal_pts = Solution::determine_diagonal_pts(&p, pts_matrix.borrow());
                pts_matrix[i].push(p);
            }
        }

        for (_, row) in pts_matrix.borrow().iter().enumerate() {
            for (_, col) in row.iter().enumerate() {
                println!("{}", col);
            }
        }
        0
    }
}

// @lc code=end

