extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_block_sum() {
    let values: Vec<Vec<i64>> = vec![vec![1, 2, 3, 4, 5],
                                     vec![6, 7, 8, 9, 10]];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let columns: Vec<&str> = vec!["X", "Y"];
    let b = Block::from_vec(values, index, columns);

    let sum = b.sum();

    let exp: Series<i64, &str> = Series::new(vec![15, 40], vec!["X", "Y"]);
    assert_eq!(sum, exp);
}

#[test]
fn test_block_mean() {
    let values: Vec<Vec<i64>> = vec![vec![1, 2, 3, 4, 5],
                                     vec![6, 7, 8, 9, 10]];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let columns: Vec<&str> = vec!["X", "Y"];
    let b = Block::from_vec(values, index, columns);

    let mean = b.mean();

    let exp: Series<f64, &str> = Series::new(vec![3., 8.], vec!["X", "Y"]);
    assert_eq!(mean, exp);
}

#[test]
fn test_minmax_int() {
    let values = vec![3, 2, 1, 4, 5,
                      7, 6, 8, 10, 10,
                      12, 14, 11, 14, 15];
    let b = Block::from_col_vec(values,
                                vec!["A", "BB", "CC", "D", "EEE"],
                                vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.len(), 5);

    let min = b.min();
    let exp: Series<i64, &str> = Series::new(vec![1, 6, 11], vec!["X", "YYY", "ZZ"]);
    assert_eq!(min, exp);

    let min = b.max();
    let exp: Series<i64, &str> = Series::new(vec![5, 10, 15], vec!["X", "YYY", "ZZ"]);
    assert_eq!(min, exp);
}

#[test]
fn test_minmax_float() {
    let values = vec![3., 2., 1., 4., 5.,
                      7., 6., 8., 10., 10.,
                      12., 14., 11., 14., 15.];
    let b = Block::from_col_vec(values,
                                vec!["A", "BB", "CC", "D", "EEE"],
                                vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.len(), 5);

    let min = b.min();
    let exp: Series<f64, &str> = Series::new(vec![1., 6., 11.], vec!["X", "YYY", "ZZ"]);
    assert_eq!(min, exp);

    let min = b.max();
    let exp: Series<f64, &str> = Series::new(vec![5., 10., 15.], vec!["X", "YYY", "ZZ"]);
    assert_eq!(min, exp);
}