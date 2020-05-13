use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Population {
    data: Vec<f64>,
}

impl Population {
    pub fn with(data: Vec<f64>) -> Self {
        Population { data }
    }

    pub fn sd(&self) -> f64 {
        let m = self.data.iter().sum::<f64>() / self.data.len() as f64;
        let v = self.data.iter().map(|n| (n - m).powi(2)).sum::<f64>() / self.data.len() as f64;
        v.sqrt()
    }

    pub fn sampling(&self) -> Sampling<'_> {
        Sampling {
            rng: thread_rng(),
            data: &self.data,
        }
    }
}

pub struct Sampling<'data> {
    rng: ThreadRng,
    data: &'data Vec<f64>,
}

impl Iterator for Sampling<'_> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.choose(&mut self.rng).cloned()
    }
}
