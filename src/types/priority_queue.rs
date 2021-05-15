use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct MinPriorityQueue<T> {
    size: usize,
    min: Option<T>,
    elements: BinaryHeap<T>,
}

impl<T> MinPriorityQueue<T>
where
    T: Clone + PartialOrd + Ord,
{
    pub fn new(size: usize) -> Self {
        MinPriorityQueue {
            size,
            min: None,
            elements: BinaryHeap::new(),
        }
    }

    pub fn append(&mut self, element: T) {
        if self.min.is_none() || self.min.as_ref().unwrap() > &element {
            self.min = Some(element.clone());
        }

        if self.elements.len() < self.size {
            self.elements.push(element);
        } else {
            if self.elements.peek().unwrap() > &element {
                let _ = self.elements.pop();
                self.elements.push(element);
            }
        }
    }

    pub fn get_min_priority(&self) -> Option<&T> {
        if self.min.is_none() {
            None
        } else {
            self.min.as_ref()
        }
    }

    pub fn get_max_priority(&self) -> Option<&T> {
        self.elements.peek()
    }

    pub fn min_sorted(self) -> Vec<T> {
        self.elements.into_sorted_vec()
    }
}
