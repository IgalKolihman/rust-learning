/// FizzBuzz game iterator
struct FizzBuzzIterator {
    /// The starting point of the iterator.
    start: u32,
    /// Vector with the fizzbuzz dividers.
    dividers: Vec<u32>,
}

impl Iterator for FizzBuzzIterator {
    type Item = u32;

    /// Get the next number that divides with one of the dividers
    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.start + 1;

        for elem in &self.dividers {
            if self.start % elem == 0 {
                return Some(self.start);
            }
        }

        return self.next();
    }
}

fn main() {
    let fizzbuzz_iter = FizzBuzzIterator {
        start: 0,
        dividers: vec![3, 5],
    };

    for elem in fizzbuzz_iter {
        println!("{}", elem)
    }
}
