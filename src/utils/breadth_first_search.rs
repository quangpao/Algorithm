use std::collections::VecDeque;

    /// Return `Vec<Vec<i32>>` after change all nearby element to `color`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use utils::breadth_first_search;
    /// 
    /// let mut image = vec![vec![1,1,1], vec![1,1,0], vec![1,0,1]];
    /// let sr = 1;
    /// let sc = 1;
    /// let new_color = 2;
    /// let result = breadth_first_search::flood_fill(image, sr, sc, new_color);
    /// assert_eq!(result, vec![vec![2,2,2], vec![2,2,0], vec![2,0,1]]);
    /// ```
    #[allow(dead_code)]
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited = vec![vec![false; image[0].len()]; image.len()];
        let current_color = image[sr as usize][sc as usize];
        queue.push_back((sr as usize, sc as usize));
        visited[sr as usize][sc as usize] = true;
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            image[x][y] = color;
            if x != 0 {
                if visited[x - 1][y] == false {
                    visited[x - 1][y] = true;
                    if image[x - 1][y] == current_color {
                        queue.push_back((x - 1, y));
                    }
                }
            }
            if x + 1 < image.len() {
                if visited[x + 1][y] == false {
                    visited[x + 1][y] = true;
                    if image[x + 1][y] == current_color {
                        queue.push_back((x + 1, y));
                    }
                }
            }
    
            if y != 0 {
                if visited[x][y - 1] == false {
                    visited[x][y - 1] = true;
                    if image[x][y - 1] == current_color {
                        queue.push_back((x, y - 1));
                    }
                }
            }
            if y + 1 < image[0].len() {
                if visited[x][y + 1] == false {
                    visited[x][y + 1] = true;
                    if image[x][y + 1] == current_color {
                        queue.push_back((x, y + 1));
                    }
                }
            }
        }
    
        return image;
    }

    /// Return `i32` after the largest island area.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use utils::breadth_first_search;
    /// 
    /// let grid = vec![vec![1,1,0], vec![1,0,0], vec![0,0,0]];
    /// let result = breadth_first_search::largest_island(grid);
    /// assert_eq!(result, 3);
    /// ```
    #[allow(dead_code)]
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        // let mut grid = grid;
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {

                if !visited[i][j] {
                    if grid[i][j] == 1 {
                        queue.clear();
                        queue.push_back((i, j));
                        visited[i][j] = true;
                        let mut current = 0;
                        while !queue.is_empty() {
                            current += 1;
                            let (x, y) = queue.pop_front().unwrap();
                            if x != 0 {
                                if !visited[x-1][y] && grid[x-1][y] == 1 {
                                    queue.push_back((x-1,y));
                                    visited[x-1][y] = true;
                                }
                            }
                            if x + 1 < grid.len() {
                                if !visited[x+1][y] && grid[x+1][y] == 1 {
                                    queue.push_back((x+1, y));
                                    visited[x+1][y] = true;
                                }
                            }
                            if y != 0 {
                                if !visited[x][y-1] && grid[x][y-1] == 1 {
                                    queue.push_back((x,y-1));
                                    visited[x][y-1] = true;
                                }
                            }
                            if y + 1 < grid[0].len() {
                                if !visited[x][y+1] && grid[x][y+1] == 1 {
                                    queue.push_back((x, y+1));
                                    visited[x][y+1] = true;
                                }
                            }
                        }
                        if current > max {max = current}
                    }
                }
            }
        }
        max
    }



