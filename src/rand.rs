use rand::Rng;

#[derive(Debug)]
struct RandomArray<'a, T: 'a> {
    data: &'a [T],
    rng: rand::rngs::ThreadRng,
}

impl<'a, T: Clone> RandomArray<'a, T> {
    fn new(data: &'a [T]) -> Self {
        Self {
            data,
            rng: rand::thread_rng(),
        }
    }
}

impl<'a, T: Clone> Iterator for RandomArray<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            None
        } else {
            let index = self.rng.gen_range(0..self.data.len());
            let result = &self.data[index];
            Some(result)
        }
    }
}

macro_rules! str {
    ($e:expr) => {
        format!("{:?}", $e)
    };
}

#[test]
fn _test() {
    let data = [1, 2, 3, 4, 5];
    let b = str!(data);
}
