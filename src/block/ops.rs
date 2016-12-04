use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem};

use super::Block;

macro_rules! define_numeric_op {
    ($t:ident, $m:ident) => {

        // Broadcast
        impl<'i, 'c, V, I, C, O> $t<V> for Block<'i, 'c, V, I, C>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: Copy {

            type Output = Block<'i, 'c, O, I, C>;
            fn $m(self, _rhs: V) -> Self::Output {

                let mut new_values: Vec<Vec<O>> = Vec::with_capacity(self.values.len());
                for value in self.values.into_iter() {
                    let new_value = value.into_iter()
                                         .map(|x| x.$m(_rhs.clone()))
                                         .collect();
                    new_values.push(new_value);
                }
                Block::from_cow(new_values,
                                self.index,
                                self.columns)
            }
        }

        impl<'i, 'c, 'r, V, I, C, O> $t<&'r V> for Block<'i, 'c, V, I, C>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: Copy {

            type Output = Block<'i, 'c, O, I, C>;
            fn $m(self, _rhs: &V) -> Self::Output {
                let mut new_values: Vec<Vec<O>> = Vec::with_capacity(self.values.len());
                for value in self.values.into_iter() {
                    let new_value = value.into_iter()
                                         .map(|x| x.$m((*_rhs).clone()))
                                         .collect();
                    new_values.push(new_value);
                }
                Block::from_cow(new_values,
                                self.index,
                                self.columns)
            }
        }

        impl<'i, 'c, 'l, V, I, C, O> $t<V> for &'l Block<'i, 'c, V, I, C>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: Copy {

            type Output = Block<'l, 'l, O, I, C>;
            fn $m(self, _rhs: V) -> Self::Output {
                let mut new_values: Vec<Vec<O>> = Vec::with_capacity(self.values.len());
                for value in self.values.iter() {
                    let new_value = value.iter()
                                         .map(|&x| x.clone().$m(_rhs.clone()))
                                         .collect();
                    new_values.push(new_value);
                }
                Block::from_cow(new_values,
                                Cow::Borrowed(self.index.borrow()),
                                Cow::Borrowed(self.columns.borrow()))
            }
        }

        impl<'i, 'c, 'l, 'r, V, I, C, O> $t<&'r V> for &'l Block<'i, 'c, V, I, C>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: Copy {

            type Output = Block<'l, 'l, O, I, C>;
            fn $m(self, _rhs: &V) -> Self::Output {
                let mut new_values: Vec<Vec<O>> = Vec::with_capacity(self.values.len());
                for value in self.values.iter() {
                    let new_value = value.iter()
                                         .map(|&x| x.clone().$m(_rhs.clone()))
                                         .collect();
                    new_values.push(new_value);
                }
                Block::from_cow(new_values,
                                Cow::Borrowed(self.index.borrow()),
                                Cow::Borrowed(self.columns.borrow()))
            }
        }

        // Element-wise
        impl<'li, 'lc, 'ri, 'rc, V, I, C, O> $t<Block<'ri, 'rc, V, I, C>>
            for Block<'li, 'lc, V, I, C>

            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: Copy {

            type Output = Block<'li, 'lc, O, I, C>;
            fn $m(self, _rhs: Block<V, I, C>) -> Self::Output {
                self.assert_binop(&_rhs);
                let mut new_values: Vec<Vec<O>> = Vec::with_capacity(self.values.len());
                for (value, rvalue) in self.values.into_iter()
                                           .zip(_rhs.values.into_iter()) {
                    let new_value = value.into_iter()
                                         .zip(rvalue.into_iter())
                                         .map(|(x, y)| x.$m(y))
                                         .collect();
                    new_values.push(new_value);
                }
                Block::from_cow(new_values,
                                self.index,
                                self.columns)
            }
        }

        impl<'li, 'lc, 'ri, 'rc, 'r, V, I, C, O> $t<&'r Block<'ri, 'rc, V, I, C>>
            for Block<'li, 'lc, V, I, C>

            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: Copy {

            type Output = Block<'li, 'lc, O, I, C>;
            fn $m(self, _rhs: &'r Block<V, I, C>) -> Self::Output {
                self.assert_binop(&_rhs);
                let mut new_values: Vec<Vec<O>> = Vec::with_capacity(self.values.len());
                for (value, rvalue) in self.values.into_iter()
                                           .zip(_rhs.values.iter()) {
                    let new_value = value.into_iter()
                                         .zip(rvalue.iter())
                                         .map(|(x, &y)| x.$m(y.clone()))
                                         .collect();
                    new_values.push(new_value);
                }
                Block::from_cow(new_values,
                                self.index,
                                self.columns)
            }
        }

        impl<'li, 'lc, 'ri, 'rc, 'l, V, I, C, O> $t<Block<'ri, 'rc, V, I, C>>
            for &'l Block<'li, 'lc, V, I, C>

            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: Copy {

            type Output = Block<'l, 'l, O, I, C>;
            fn $m(self, _rhs: Block<V, I, C>) -> Self::Output {
                self.assert_binop(&_rhs);
                let mut new_values: Vec<Vec<O>> = Vec::with_capacity(self.values.len());
                for (value, rvalue) in self.values.iter()
                                           .zip(_rhs.values.into_iter()) {
                    let new_value = value.iter()
                                         .zip(rvalue.into_iter())
                                         .map(|(&x, y)| x.clone().$m(y))
                                         .collect();
                    new_values.push(new_value);
                }
                Block::from_cow(new_values,
                                Cow::Borrowed(self.index.borrow()),
                                Cow::Borrowed(self.columns.borrow()))
            }
        }

        impl<'li, 'lc, 'ri , 'rc, 'l, 'r, V, I, C, O> $t<&'r Block<'ri, 'rc, V, I, C>>
            for &'l Block<'li, 'lc, V, I, C>

            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: Copy {

            type Output = Block<'l, 'l, O, I, C>;
            fn $m(self, _rhs: &'r Block<V, I, C>) -> Self::Output {
                self.assert_binop(&_rhs);
                let mut new_values: Vec<Vec<O>> = Vec::with_capacity(self.values.len());
                for (value, rvalue) in self.values.iter()
                                           .zip(_rhs.values.iter()) {
                    let new_value = value.iter()
                                         .zip(rvalue.iter())
                                         .map(|(&x, &y)| x.clone().$m(y.clone()))
                                         .collect();
                    new_values.push(new_value);
                }
                Block::from_cow(new_values,
                                Cow::Borrowed(self.index.borrow()),
                                Cow::Borrowed(self.columns.borrow()))
            }
        }
    }
}

define_numeric_op!(Add, add);
define_numeric_op!(Mul, mul);
define_numeric_op!(Sub, sub);
define_numeric_op!(Div, div);
define_numeric_op!(Rem, rem);


#[cfg(test)]
mod tests {

    use super::super::Block;

    #[test]
    fn test_block_ops_i64_broadcast() {
        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let res = b + 3;
        let exp = Block::from_col_vec(vec![4, 5, 6, 7, 8, 9],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b * 2;
        let exp = Block::from_col_vec(vec![2, 4, 6, 8, 10, 12],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b - 3;
        let exp = Block::from_col_vec(vec![-2, -1, 0, 1, 2, 3],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b / 2;
        let exp = Block::from_col_vec(vec![0, 1, 1, 2, 2, 3],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b % 2;
        let exp = Block::from_col_vec(vec![1, 0, 1, 0, 1, 0],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_i64_broadcast_refs() {
        let exp = Block::from_col_vec(vec![4, 5, 6, 7, 8, 9],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + 3;
        assert_eq!(res, exp);

        let res = &b + &3;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_i64_broadcast_move() {
        let exp = Block::from_col_vec(vec![4, 5, 6, 7, 8, 9],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b + &3;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_broadcast() {
        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let res = b + 3.;
        let exp = Block::from_col_vec(vec![4., 5., 6., 7., 8., 9.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b * 2.;
        let exp = Block::from_col_vec(vec![2., 4., 6., 8., 10., 12.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b - 3.;
        let exp = Block::from_col_vec(vec![-2., -1., 0., 1., 2., 3.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b / 2.;
        let exp = Block::from_col_vec(vec![0.5, 1., 1.5, 2., 2.5, 3.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b % 2.;
        let exp = Block::from_col_vec(vec![1., 0., 1., 0., 1., 0.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_broadcast_refs() {
        let exp = Block::from_col_vec(vec![4., 5., 6., 7., 8., 9.],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + 3.;
        assert_eq!(res, exp);

        let res = &b + &3.;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_broadcast_move() {
        let exp = Block::from_col_vec(vec![4., 5., 6., 7., 8., 9.],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b + &3.;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_i64_elemwise() {
        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let res = b + r;
        let exp = Block::from_col_vec(vec![3, 5, 4, 6, 8, 7],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b * r;
        let exp = Block::from_col_vec(vec![2, 6, 3, 8, 15, 6],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b - r;
        let exp = Block::from_col_vec(vec![-1, -1, 2, 2, 2, 5],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b / r;
        let exp = Block::from_col_vec(vec![0, 0, 3, 2, 1, 6],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b % r;
        let exp = Block::from_col_vec(vec![1, 2, 0, 0, 2, 0],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_i64_elemwise_refs() {
        let exp = Block::from_col_vec(vec![3, 5, 4, 6, 8, 7],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + r;
        assert_eq!(res, exp);

        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + &r;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_i64_elemwise_move() {
        let exp = Block::from_col_vec(vec![3, 5, 4, 6, 8, 7],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b + &r;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_elemwise() {
        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let res = b + r;
        let exp = Block::from_col_vec(vec![3., 5., 4., 6., 8., 7.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b * r;
        let exp = Block::from_col_vec(vec![2., 6., 3., 8., 15., 6.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b - r;
        let exp = Block::from_col_vec(vec![-1., -1., 2., 2., 2., 5.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b / r;
        let exp = Block::from_col_vec(vec![0.5, 0.6666666666666666, 3.,
                                           2., 1.6666666666666667, 6.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b % r;
        let exp = Block::from_col_vec(vec![1., 2., 0., 0., 2., 0.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_elemwise_refs() {
        let exp = Block::from_col_vec(vec![3., 5., 4., 6., 8., 7.],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);

        let res = &b + r;
        assert_eq!(res, exp);

        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + &r;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_elemwise_move() {
        let exp = Block::from_col_vec(vec![3., 5., 4., 6., 8., 7.],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);

        let res = b + &r;
        assert_eq!(res, exp);
    }
}
