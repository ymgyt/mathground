use num_traits::{Num, NumCast};

pub fn variance<N: Num + Copy + NumCast>(xs: &[N]) -> f64 {
    let n = xs.len();
    if n == 0 {
        return 0.;
    }
    let pow_avg = xs
        .iter()
        .map(|&x| x.mul(x))
        .fold(N::zero(), |acc, x| acc.add(x))
        .to_f64()
        .unwrap()
        / n as f64;

    let avg_pow = average(xs).powi(2);

    pow_avg - avg_pow
}

pub fn average<N: Num + NumCast + Copy>(xs: &[N]) -> f64 {
    let n = xs.len();
    if n == 0 {
        return 0.;
    }
    let sum = xs
        .iter()
        .fold(N::zero(), |acc, x| acc.add(*x))
        .to_f64()
        .unwrap();
    sum / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_average() {
        assert_eq!(1., average(&[1, 1]));
        assert_eq!(0., average::<u64>(&[]));
    }

    #[test]
    fn test_variance() {
        assert_eq!(200., variance::<i64>(&[80, 90, 100, 110, 120]));
        assert_eq!(1800., variance::<i64>(&[40, 70, 100, 130, 160]));
    }
}
