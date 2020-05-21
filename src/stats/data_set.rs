use crate::stats::{
    core::{average, standard_deviation},
    error::StatsError,
    Result,
};
use num_traits::{Num, NumCast};
use std::{convert::Into, ops::Deref};

pub struct DataSet<T> {
    data: Box<[T]>,
}

impl<T> DataSet<T>
where
    T: Num + NumCast + Copy,
{
    pub fn from<S>(s: S) -> Self
    where
        S: Into<Box<[T]>>,
    {
        Self { data: s.into() }
    }

    // Correlation coefficient
    // 相関係数
    pub fn correlation_coefficient(&self, other: &DataSet<T>) -> Result<f64> {
        Ok(self.covariance(other)?
            / (standard_deviation(self.as_ref()) * standard_deviation(other.as_ref())))
    }

    // 共分散
    pub fn covariance(&self, other: &DataSet<T>) -> Result<f64> {
        let n = self.len();
        if n != other.len() {
            return Err(StatsError::DifferentDataSize);
        } else if n == 0 {
            return Ok(0.);
        }

        let x_avg = average(self.as_ref());
        let y_avg = average(other.as_ref());
        let sum = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|xy| (xy.0.to_f64().unwrap(), xy.1.to_f64().unwrap()))
            .fold(T::zero().to_f64().unwrap(), |acc, xy| {
                acc + ((xy.0 - x_avg) * (xy.1 - y_avg))
            });
        Ok(sum / n as f64)
    }
}

impl<T> AsRef<[T]> for DataSet<T> {
    fn as_ref(&self) -> &[T] {
        self.data.as_ref()
    }
}

impl<T> Deref for DataSet<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.data.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn correlation_coefficient() {
        // 都心ターミナル駅からの所要時間(分)
        let x = DataSet::from(vec![0, 4, 9, 14, 18, 26, 30, 36, 42, 50]);
        // 家賃(万)
        let y = DataSet::from(vec![12, 9, 8, 7, 7, 7, 8, 6, 7, 5]);

        assert_approx_eq!(-0.7940763, x.correlation_coefficient(&y).unwrap());
    }
}
