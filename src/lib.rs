use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct ShuffleVector<T> {
    vector: Vec<T>,
}

impl<T> ShuffleVector<T> {
    pub fn new(v: Vec<T>) -> ShuffleVector<T> {
        ShuffleVector {
            vector: v,
        }
    }

    pub fn push(&mut self, item: T) {
        // push the item to the end of the vector
        self.vector.push(item);

        // swap last item with random item (which could be itself)
        let highest_index =  self.vector.len() - 1;
        if highest_index != 0 {
            self.vector.swap(thread_rng().gen_range(0, highest_index), highest_index);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vector.pop()
    }
}

#[cfg(test)]
mod tests {
    use crate::ShuffleVector;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn create_empty() {
        let v = ShuffleVector::<u32> { vector: vec!()};
        println!("{:?}", v);
    }

    #[test]
    fn create_one_item() {
        let mut v = ShuffleVector::<u32> { vector: vec!()};
        v.push(1);
        println!("{:?}", v);
    }

    #[test]
    fn create_1_to_5() {
        let mut v = ShuffleVector::<u32> { vector: vec!()};
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);
        println!("{:?}", v);
    }
}
