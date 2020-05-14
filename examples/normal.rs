use statrs::distribution::Normal;
use statrs::distribution::{Continuous, Univariate};

fn main() -> anyhow::Result<()> {
   let n =  Normal::new(0.0, 1.0)?;
   // 標準正規分布の確率密度関数
   println!("{:?}", n.pdf(0.0));

   // 確率密度関数の積分
   println!("{:?}", n.cdf(1.5) - n.cdf(0.0));

   Ok(())
}
