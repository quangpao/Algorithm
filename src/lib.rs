pub(crate) mod utils;
#[cfg(test)]
mod bfs_test {
    
    use std::vec;

    use super::utils::breadth_first_search;

    #[test]
    fn normal_case() {

        let image = vec![
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1]
        ];
        let result = vec![
            vec![2, 2, 2],
            vec![2, 2, 0],
            vec![2, 0, 1]
        ];
        assert_eq!(result, breadth_first_search::flood_fill(image, 1 , 1, 2));
    }

    #[test]
    fn max_area_of_island_test() {

        let grid = vec![
            vec![1,1,0,0,0],
            vec![1,1,0,0,0],
            vec![0,0,0,1,1],
            vec![0,0,0,1,1]
        ];

        assert_eq!(4, breadth_first_search::max_area_of_island(grid));

    }
}