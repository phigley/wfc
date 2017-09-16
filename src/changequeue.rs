use std::fmt::Debug;

pub struct ChangeQueue<T: PartialEq + Debug> {
    pending: Vec<T>,
}

impl<T: PartialEq + Debug> ChangeQueue<T> {
    pub fn new() -> ChangeQueue<T> {
        ChangeQueue { pending: Vec::new() }
    }

    pub fn add(&mut self, value: T) {

        for p in self.pending.iter() {
            if *p == value {
                return;
            }
        }

        self.pending.push(value);
    }

    pub fn next(&mut self) -> Option<T> {
        self.pending.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple_change_queue() {

        let mut q = ChangeQueue::new();

        assert!(q.is_empty());

        q.add(1);

        assert!(!q.is_empty());

        let n = q.next();

        assert_eq!(n, Some(1));

        assert!(q.is_empty());

        q.add(1);
        q.add(2);
        q.add(2);
        q.add(3);
        q.add(2);

        assert!(!q.is_empty());
        q.next();
        q.next();
        q.next();
        assert!(q.is_empty());
    }
}
