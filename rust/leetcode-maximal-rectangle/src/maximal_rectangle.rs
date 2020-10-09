/*
 * @lc app=leetcode id=85 lang=rust
 *
 * [85] Maximal Rectangle
 */

// @lc code=start

/*
    0 1 2 3 4 5 6 7 8 9 y
    1 0 0 0 0 0 0 0 1 1
    2 0 1 0 0 0 0 0 1 1 
    3 0 0 1 0 0 1 1 1 1
    4 0 0 1 0 0 1 1 1 1
    5 0 0 0 1 0 1 1 1 1
    6 0 0 0 0 1 1 1 1 1
    7 0 1 1 1 1 1 1 1 1
    8 0 1 1 1 1 1 1 1 1
    9 0 0 0 0 1 1 1 1 1

    x
    Point(8, 8)的对角点有：[(1, 8), (3, 6), (6, 5), (7, 2)]
*/

pub struct Solution{}

struct Point<'a> {
    ch: char,
    x: usize,
    y: usize,
    diagonal_pts: Vec<&'a Point<'a>>,
    x_reach: i16,  // 从该point一直沿着x轴往左，一直到x_reach，都是'1'
    y_reach: i16   // 从该point一直沿着y轴往上，一直到y_reach，都是'1'
}

impl std::fmt::Display for Point<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl Solution {
    fn determine_x_reach(point: & Point, pts_matrix: & Vec<Vec<Point>>) -> i16 {
        if point.ch == '0' { -1 }
        else {
            if point.y > 0 && pts_matrix[point.x][point.y-1].x_reach >= 0 { pts_matrix[point.x][point.y-1].x_reach }
            else { point.y as i16 }
        }
    }

    fn determine_y_reach(point: & Point, pts_matrix: & Vec<Vec<Point>>) -> i16 {
        if point.ch == '0' { -1 }
        else {
            if point.x > 0 && pts_matrix[point.x-1][point.y].y_reach >= 0 { pts_matrix[point.x-1][point.y].y_reach }
            else { point.x as i16 }
        }
    }

    fn determine_diagonal_pts<'a>(point: &'a Point, pts_matrix: &'a Vec<Vec<Point<'a>>>) -> Vec<&'a Point<'a>> {
        let mut diagonal_pts: Vec<&'a Point> = vec![];
        if point.x == 0 && point.y == 0 {
            if point.ch == '1' {
                diagonal_pts.push(point)
            }

        }
        else if point.x == 0 {
            if point.y_reach >= 0 {
                diagonal_pts.push(& pts_matrix[0][point.y_reach as usize])
            }
        }
        else if point.y == 0 {
            if point.x_reach >= 0 {
                diagonal_pts.push(& pts_matrix[point.x_reach as usize][0])
            }
        }
        else {

        }
        diagonal_pts
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut pts_matrix: Vec<Vec<Point>> = Vec::new();

        for (i, row) in matrix.iter().enumerate() {
            pts_matrix.push(vec![]);
            for (j, col) in row.iter().enumerate() {
                let mut p = Point{ch: *col, x:i, y:j, diagonal_pts: vec![], x_reach: -1, y_reach: -1};
                p.x_reach = Solution::determine_x_reach(&p, &pts_matrix);
                p.y_reach = Solution::determine_y_reach(&p, &pts_matrix);
                p.diagonal_pts = Solution::determine_diagonal_pts(&p, &pts_matrix);
                pts_matrix[i].push(p);
            }
        }

        for (_, row) in pts_matrix.iter().enumerate() {
            for (_, col) in row.iter().enumerate() {
                println!("{}", col);
            }
        }
        0
    }
}

// @lc code=end

