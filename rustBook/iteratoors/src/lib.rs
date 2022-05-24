use std::{array::IntoIter, collections::btree_map::Iter};

pub struct Flatten<O>
    where 
    O:Iterator, 
    O::Item: IntoIterator
{
    outer: O,
     inner: Option<<O::Item as IntoIterator>::IntoIter>
    // Here Into Iter is the associated type that represents the iterator type returned.
    // E.g. some_vec.into_iter() returns a value of type IntoIter
}

impl<O> Flatten<O> 
    where 
    O::Item: IntoIterator,
    O: Iterator,
{
    pub fn new(outer: O) -> Flatten<O> 
    {
        Flatten {
            outer: outer,
            inner: None
        }
    }
}

impl<O> Iterator for Flatten<O> 
    where 
    O: Iterator,
    O::Item: IntoIterator
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
                if let Some(ref mut inner_iter) = self.inner {
                    if let Some(item) = inner_iter.next() {
                        return Some(item)
                    }
                } 
                    self.inner = Some(self.outer.next()?.into_iter());
        }
    }
    // Check if stored outer is not the same as next, then store that value
}

 
pub fn flatten<O>(outer: O) -> Flatten<O> 
where 
O: Iterator,
O::Item: IntoIterator
{
    Flatten::new(outer)
}

// The Type the Outer item yields needs to be turned into an iterator,
// So should implement into_Iterator

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*; 

    #[test]
    fn work() {
        let vec_of_vecs = vec![vec![1,2,3,4,5], vec![1,2,3,4,5], vec![1,2,3,4,5]].into_iter();
        // assert_eq!(flatten(vec_of_vecs), 15);
        let p = flatten(vec_of_vecs).count();
        assert_eq!(p, 15);
        let mut iter = flatten((0..).map(|i| {
            0..i
        }));
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
    }
}