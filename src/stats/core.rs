use num_traits::{Num, NumCast, Float};

// 偏差値
pub fn standard_score<N: Float>(x:N, avg:N, standard_deviation:N) -> N {
    (x - avg) / standard_deviation * N::from(10.0).unwrap() + N::from(50.).unwrap()
}

// 標準偏差
pub fn standard_deviation<N: Num + Copy + NumCast>(xs: &[N]) -> f64 {
    variance(xs).sqrt()
}

// 分散
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

// 平均
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

    #[test]
    fn test_standard_score() {
        // 平成30年大学入試センター数学1A
        let avg = 61.91;
        let sd = 18.69;
        assert_eq!(format!("{:.2}",59.68), format!("{:.2}", standard_score(80., avg, sd)));
        assert_eq!(format!("{:.2}",70.38), format!("{:.2}", standard_score(100., avg, sd)));
    }
}
