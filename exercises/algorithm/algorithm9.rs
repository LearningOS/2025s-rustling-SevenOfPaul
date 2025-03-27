/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::{Ord, Ordering};
use std::default::Default;
trait Swap<T>
where
    T: Default + Clone,
{
    fn swap(&mut self, idx1: usize, idx2: usize);
}

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}
impl<T> Swap<T> for Heap<T>
where
    T: Default + Clone,
{
    fn swap(&mut self, idx1: usize, idx2: usize) {
        let tmp = self.items[idx1].clone();
        self.items[idx1] = self.items[idx2].clone();
        self.items[idx2] = tmp;
    }
}
impl<T> Heap<T>
where
    T: Default+Clone+Ord,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
          self.count+=1;
          self.items.push(value);
          let mut count = self.count;
          while count > 1 && (self.comparator)(&self.items[count], &self.items[self.parent_idx(count)]) {
             self.swap(count,self.parent_idx(count));
             count = self.parent_idx(count);
          }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }   

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
         if self.right_child_idx(idx) > self.count {
             self.left_child_idx(idx)
         }else{
           if (self.comparator)(&self.items[self.left_child_idx(idx)],&self.items[self.right_child_idx(idx)]){
               self.left_child_idx(idx)
           }else{
               self.right_child_idx(idx)
           }
         }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord+Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count==0 {
            return None;
        }else{
            let res=self.items[1].clone();
            self.items[1]=self.items[self.count].clone();
            self.items.pop();
            self.count-=1;
           //准备下沉
           let mut idx:usize=1;
           while (idx*2)<=self.count{
               if idx*2+1>self.count{
                 if (self.comparator)(&self.items[idx],&self.items[idx*2]){
                    break;
                }
                   self.swap(idx,idx*2);
                   idx=idx*2;
               }else{
                if (self.comparator)(&self.items[idx*2],&self.items[idx*2+1]){
                if (self.comparator)(&self.items[idx],&self.items[idx*2]){
                    break;
                }
                   self.swap(idx,idx*2);
                   idx=idx*2;
            }else{
                if (self.comparator)(&self.items[idx],&self.items[idx*2+1]){
                    break;
                }
                   self.swap(idx,idx*2+1);
                   idx=idx*2+1;
            }
               }    
           }
           Some(res)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}