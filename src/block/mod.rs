extern crate num;

use num::{Num, Zero, ToPrimitive};
use std::hash::Hash;

use super::blockgroupby::BlockGroupBy;
use super::computations;
use super::indexer::Indexer;
use super::series::Series;

mod formatting;
mod ops;

#[derive(Clone)]
pub struct Block<T, U: Hash, V: Hash> {
    /// 2-dimentional block contains a single type.
    /// T: type of values
    /// U: type of indexer
    /// V: type of columns

    // ToDo: may be simpler to use 1-d Vec?
    pub values: Vec<Vec<T>>,
    pub index: Indexer<U>,
    pub columns: Indexer<V>,
}

// Indexing

impl<T, U, V> Block<T, U, V>
    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    /// Instanciate from column-wise Vec
    pub fn from_col_vec(values: Vec<T>, index: Vec<U>, columns: Vec<V>) -> Block<T, U, V> {
        let len: usize = index.len();
        let cols: usize = columns.len();

        if values.len() != len * cols {
            panic!("Length mismatch!");
        }

        let mut new_values: Vec<Vec<T>> = vec![];
        for value in values.chunks(len) {
            let v: Vec<T> = value.iter().cloned().collect();
            new_values.push(v);
        }
        Block {
            values: new_values,
            index: Indexer::new(index),
            columns: Indexer::new(columns),
        }
    }

    /// Instanciate from column-wise Vec
    pub fn from_row_vec(values: Vec<T>, index: Vec<U>, columns: Vec<V>) -> Block<T, U, V> {
        let len: usize = index.len();
        let cols: usize = columns.len();

        if values.len() != len * cols {
            panic!("Length mismatch!");
        }

        let mut new_values: Vec<Vec<T>> = vec![];
        for i in 0..cols {
            let mut new_value: Vec<T> = vec![];
            for j in 0..len {
                new_value.push(values[j * cols + i]);
            }
            new_values.push(new_value);
        }
        Block {
            values: new_values,
            index: Indexer::new(index),
            columns: Indexer::new(columns),
        }
    }

    /// Instanciate from nested Vec
    pub fn from_nested_vec(values: Vec<Vec<T>>, index: Vec<U>, columns: Vec<V>) -> Block<T, U, V> {
        if values.len() != columns.len() {
            panic!("Length mismatch!");
        }
        let len = index.len();
        for value in values.iter() {
            if value.len() != len {
                panic!("Length mismatch!");
            }
        }
        Block {
            values: values,
            index: Indexer::new(index),
            columns: Indexer::new(columns),
        }
    }

    /// Instanciate from Series
    pub fn from_series(series: Series<T, U>, name: V) -> Block<T, U, V> {
        let mut values: Vec<Vec<T>> = vec![];
        values.push(series.values);

        // mapper is not updated properly by vec![name]
        let mut columns = Indexer::new(vec![]);
        columns.push(name);

        Block {
            values: values,
            index: series.index,
            columns: columns,
        }
    }

    /// Instanciate from instanciated MultiMap and Indexer. Used internally
    fn from_internal(values: Vec<Vec<T>>, index: Indexer<U>,
                     columns: Indexer<V>) -> Block<T, U, V> {
        Block {
            values: values,
            index: index,
            columns: columns,
        }
    }

    fn assert_binop(&self, other: &Block<T, U, V>) {
        if self.index != other.index {
            panic!("index must be the same!");
        }
        if self.columns != other.columns {
            panic!("columns must be the same!");
        }
    }

    pub fn add_columns(&mut self, values: Vec<T>, name: V) {
        if self.len() != values.len() {
            panic!("Length mismatch!");
        }
        self.values.push(values);
        self.columns.push(name);
    }

    pub fn len(&self) -> usize {
        return self.index.len();
    }

    pub fn get_column_by_label(&mut self, label: &V) -> Series<T, U> {
        let loc = self.columns.get_label_loc(label);
        let new_values = self.values[loc].clone();
        return Series::new(new_values, self.index.copy_values());
    }

    pub fn slice_by_label(&mut self, labels: &Vec<U>) -> Block<T, U, V> {
        let locs = self.index.slice_label_loc(labels);
        return self.slice_by_index(&locs);
    }

    pub fn slice_by_index(&self, locations: &Vec<usize>) -> Block<T, U, V> {
        let new_index: Vec<U> = locations.iter()
                                         .map(|loc| self.index.values[*loc])
                                         .collect();

        let mut new_values: Vec<Vec<T>> = vec![];
        for current in self.values.iter() {
            let new_value = locations.iter().map(|x| current[*x]).collect();
            new_values.push(new_value);
        }
        return Block::<T, U, V>::from_internal(new_values,
                                               Indexer::new(new_index),
                                               self.columns.clone());
    }

    pub fn append(&self, other: &Block<T, U, V>) -> Block<T, U, V> {
        if self.columns != other.columns {
            panic!("columns must be the same!")
        }

        let mut new_index: Vec<U> = self.index.values.clone();
        new_index.append(&mut other.index.values.clone());

        let mut new_values: Vec<Vec<T>> = vec![];
        for (svalues, ovalues) in self.values.iter().zip(&other.values) {
            let mut new_value = svalues.clone();
            new_value.append(&mut ovalues.clone());
            new_values.push(new_value);
        }

        return Block::<T, U, V>::from_nested_vec(new_values, new_index,
                                                 self.columns.copy_values());
    }

    pub fn groupby<G>(&self, other: Vec<G>) -> BlockGroupBy<T, U, V, G>
        where G: Copy + Eq + Hash + Ord {
        return BlockGroupBy::new(&self, other);
    }

    /// Apply passed function to each columns.
    pub fn apply<W: Copy>(&self, func: &Fn(&Vec<T>) -> W) -> Series<W, V> {
        let mut new_values = vec![];
        for current in self.values.iter() {
            new_values.push(func(&current));
        }
        return Series::new(new_values, self.columns.copy_values());
    }

    pub fn transpose(&self) -> Block<T, V, U> {

        let mut new_values: Vec<Vec<T>> = vec![];
        for i in 0..self.index.len() {
            let mut new_value: Vec<T> = vec![];
            for value in self.values.iter() {
                new_value.push(value[i]);
            }
            new_values.push(new_value);
        }
        return Block::from_internal(new_values,
                                    self.columns.clone(),
                                    self.index.clone());
    }
}

impl<T: PartialEq, U: Hash + Eq, V: Hash + Eq> PartialEq for Block<T, U, V> {
    fn eq(&self, other: &Block<T, U, V>) -> bool {
        (self.index == other.index) &&
        (self.columns == other.columns) &&
        (self.values == other.values)
    }
}

// Aggregation

impl<T, U, V> Block<T, U, V>
    where T: Copy + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    // ToDo: Merge definition to Series
    pub fn sum(&self) -> Series<T, V> {
        return self.apply(&computations::vec_sum);
    }

    pub fn count(&self) -> Series<usize, V> {
        return self.apply(&computations::vec_count);
    }

    pub fn mean(&self) -> Series<f64, V> {
        return self.apply(&computations::vec_mean);
    }

    pub fn var(&self) -> Series<f64, V> {
        return self.apply(&computations::vec_var);
    }

    pub fn unbiased_var(&self) -> Series<f64, V> {
        return self.apply(&computations::vec_unbiased_var);
    }

    pub fn std(&self) -> Series<f64, V> {
        return self.apply(&computations::vec_std);
    }

    pub fn unbiased_std(&self) -> Series<f64, V> {
        return self.apply(&computations::vec_unbiased_std);
    }
}

impl<T, U, V> Block<T, U, V>
    where T: Copy + Num + Zero + computations::NanMinMax<T>,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    pub fn min(&self) -> Series<T, V> {
        return self.apply(&computations::vec_min);
    }

    pub fn max(&self) -> Series<T, V> {
        return self.apply(&computations::vec_max);
    }
}

#[cfg(test)]
mod tests {

    use super::Block;
    use super::super::series::Series;

    #[test]
    fn test_block_creation_from_col_vec() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let mut b = Block::from_col_vec(values,
                                        vec!["A", "BB", "CC", "D", "EEE"],
                                        vec!["X", "YYY", "ZZ"]);
        assert_eq!(&b.len(), &5);

        let exp_index: Vec<&str> = vec!["A", "BB", "CC", "D", "EEE"];
        let exp_columns: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        let c = b.get_column_by_label(&"X");
        let exp_values: Vec<i64> = vec![1, 2, 3, 4, 5];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&"YYY");
        let exp_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&"ZZ");
        let exp_values: Vec<i64> = vec![11, 12, 13, 14, 15];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);
    }

    #[test]
    fn test_block_creation_from_row_vec() {
        let values = vec![1, 6, 11,
                          2, 7, 12,
                          3, 8, 13,
                          4, 9, 14,
                          5, 10, 15];
        let mut b = Block::from_row_vec(values,
                                        vec!["A", "BB", "CC", "D", "EEE"],
                                        vec!["X", "YYY", "ZZ"]);
        assert_eq!(&b.len(), &5);

        let exp_index: Vec<&str> = vec!["A", "BB", "CC", "D", "EEE"];
        let exp_columns: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        let c = b.get_column_by_label(&"X");
        let exp_values: Vec<i64> = vec![1, 2, 3, 4, 5];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&"YYY");
        let exp_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&"ZZ");
        let exp_values: Vec<i64> = vec![11, 12, 13, 14, 15];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);
    }

    #[test]
    fn test_block_creation_from_nested_vec() {
        let values = vec![vec![1, 2, 3, 4, 5],
                          vec![6, 7, 8, 9, 10],
                          vec![11, 12, 13, 14, 15]];
        let mut b = Block::from_nested_vec(values,
                                           vec!["A", "BB", "CC", "D", "EEE"],
                                           vec!["X", "YYY", "ZZ"]);
        assert_eq!(&b.len(), &5);

        let exp_index: Vec<&str> = vec!["A", "BB", "CC", "D", "EEE"];
        let exp_columns: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        let c = b.get_column_by_label(&"X");
        let exp_values: Vec<i64> = vec![1, 2, 3, 4, 5];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&"YYY");
        let exp_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&"ZZ");
        let exp_values: Vec<i64> = vec![11, 12, 13, 14, 15];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);
    }

    #[test]
    fn test_block_creation_from_series() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);

        let mut b = Block::<f64, &str, i64>::from_series(s, 1);
        assert_eq!(&b.len(), &3);

        let exp_index: Vec<&str> = vec!["A", "B", "C"];
        let exp_columns: Vec<i64> = vec![1];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        let c = b.get_column_by_label(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);
    }

    #[test]
    fn test_add_columns() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);

        let mut b = Block::<f64, &str, i64>::from_series(s, 1);

        assert_eq!(&b.len(), &3);
        let exp_index: Vec<&str> = vec!["A", "B", "C"];
        let exp_columns: Vec<i64> = vec![1];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        // add columns
        let values2: Vec<f64> = vec![4., 5., 6.];
        b.add_columns(values2, 3);
        assert_eq!(&b.len(), &3);
        let exp_columns: Vec<i64> = vec![1, 3];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        assert_eq!(&b.columns.get_label_loc(&1), &0);
        assert_eq!(&b.columns.get_label_loc(&3), &1);
        let c = b.get_column_by_label(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&3);
        let exp_values: Vec<f64> = vec![4., 5., 6.];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);
    }

    #[test]
    fn test_slice_by_index() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);
        let mut b = Block::<f64, &str, i64>::from_series(s, 1);
        // add columns
        let values2: Vec<f64> = vec![4., 5., 6.];
        b.add_columns(values2, 3);
        assert_eq!(&b.len(), &3);

        // slice
        let mut sliced = b.slice_by_index(&vec![0, 2]);
        let exp_index: Vec<&str> = vec!["A", "C"];
        let exp_columns: Vec<i64> = vec![1, 3];
        assert_eq!(&sliced.index.values, &exp_index);
        assert_eq!(&sliced.columns.values, &exp_columns);

        // compare columns
        let c = sliced.get_column_by_label(&1);
        let exp_values: Vec<f64> = vec![1., 3.];
        assert_eq!(&c.values, &exp_values);
        let c = sliced.get_column_by_label(&3);
        let exp_values: Vec<f64> = vec![4., 6.];
        assert_eq!(&c.values, &exp_values);
    }

    #[test]
    fn test_slice_by_label() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);
        let mut b = Block::<f64, &str, i64>::from_series(s, 1);
        // add columns
        let values2: Vec<f64> = vec![4., 5., 6.];
        b.add_columns(values2, 3);
        assert_eq!(&b.len(), &3);

        // slice
        let mut sliced = b.slice_by_label(&vec!["B", "C"]);
        let exp_index: Vec<&str> = vec!["B", "C"];
        let exp_columns: Vec<i64> = vec![1, 3];
        assert_eq!(&sliced.index.values, &exp_index);
        assert_eq!(&sliced.columns.values, &exp_columns);

        // compare columns
        let c = sliced.get_column_by_label(&1);
        let exp_values: Vec<f64> = vec![2., 3.];
        assert_eq!(&c.values, &exp_values);
        let c = sliced.get_column_by_label(&3);
        let exp_values: Vec<f64> = vec![5., 6.];
        assert_eq!(&c.values, &exp_values);
    }

    #[test]
    fn test_block_append() {
        let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                     vec!["A", "B", "C"],
                                     vec!["X", "Y"]);
        let b2 = Block::from_col_vec(vec![7., 8., 9., 10., 11., 12.],
                                     vec!["D", "E", "F"],
                                     vec!["X", "Y"]);

        let mut res = b1.append(&b2);

        let exp_index: Vec<&str> = vec!["A", "B", "C", "D", "E", "F"];
        let exp_columns: Vec<&str> = vec!["X", "Y"];
        assert_eq!(&res.index.values, &exp_index);
        assert_eq!(&res.columns.values, &exp_columns);

        let c = res.get_column_by_label(&"X");
        assert_eq!(&c.values, &vec![1., 2., 3., 7., 8., 9.]);
        let c = res.get_column_by_label(&"Y");
        assert_eq!(&c.values, &vec![4., 5., 6., 10., 11., 12.]);
    }

    #[test]
    fn test_block_transpose() {
        let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                     vec!["A", "B", "C"],
                                     vec!["X", "Y"]);
        let mut res = b1.transpose();

        let exp_index: Vec<&str> = vec!["X", "Y"];
        let exp_columns: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(&res.index.values, &exp_index);
        assert_eq!(&res.columns.values, &exp_columns);

        let c = res.get_column_by_label(&"A");
        assert_eq!(&c.values, &vec![1., 4.]);
        let c = res.get_column_by_label(&"B");
        assert_eq!(&c.values, &vec![2., 5.]);
        let c = res.get_column_by_label(&"C");
        assert_eq!(&c.values, &vec![3., 6.]);
    }

    #[test]
    fn test_block_sum() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);
        let mut b = Block::from_series(s, "X");

        let new_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        b.add_columns(new_values, "Y");

        let sum = b.sum();

        let exp_values: Vec<i64> = vec![15, 40];
        let exp_index: Vec<&str> = vec!["X", "Y"];
        assert_eq!(&sum.values, &exp_values);
        assert_eq!(&sum.index.values, &exp_index);
    }

    #[test]
    fn test_block_mean() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);
        let mut b = Block::from_series(s, "X");

        let new_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        b.add_columns(new_values, "Y");

        let mean = b.mean();

        let exp_values: Vec<f64> = vec![3., 8.];
        let exp_index: Vec<&str> = vec!["X", "Y"];
        assert_eq!(&mean.values, &exp_values);
        assert_eq!(&mean.index.values, &exp_index);
    }

    #[test]
    fn test_minmax_int() {
        let values = vec![3, 2, 1, 4, 5,
                          7, 6, 8, 10, 10,
                          12, 14, 11, 14, 15];
        let b = Block::from_col_vec(values,
                                    vec!["A", "BB", "CC", "D", "EEE"],
                                    vec!["X", "YYY", "ZZ"]);
        assert_eq!(&b.len(), &5);

        let min = b.min();
        let exp_values: Vec<i64> = vec![1, 6, 11];
        let exp_index: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&min.values, &exp_values);
        assert_eq!(&min.index.values, &exp_index);

        let min = b.max();
        let exp_values: Vec<i64> = vec![5, 10, 15];
        let exp_index: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&min.values, &exp_values);
        assert_eq!(&min.index.values, &exp_index);
    }

    #[test]
    fn test_minmax_float() {
        let values = vec![3., 2., 1., 4., 5.,
                          7., 6., 8., 10., 10.,
                          12., 14., 11., 14., 15.];
        let b = Block::from_col_vec(values,
                                    vec!["A", "BB", "CC", "D", "EEE"],
                                    vec!["X", "YYY", "ZZ"]);
        assert_eq!(&b.len(), &5);

        let min = b.min();
        let exp_values: Vec<f64> = vec![1., 6., 11.];
        let exp_index: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&min.values, &exp_values);
        assert_eq!(&min.index.values, &exp_index);

        let min = b.max();
        let exp_values: Vec<f64> = vec![5., 10., 15.];
        let exp_index: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&min.values, &exp_values);
        assert_eq!(&min.index.values, &exp_index);
    }
}