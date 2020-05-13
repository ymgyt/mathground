use mathground::stats;
use num_traits::{Float, FromPrimitive};
use std::collections::BTreeMap;
fn main() -> anyhow::Result<()> {
    let data = gen_data(1.0, 101.0, 100);
    let p = stats::Population::with(data);
    let samples = vec![5, 10, 50, 100];
    let mut v = plotlib::view::ContinuousView::new()
        .x_range(0.0, 100.0);

    for sample_size in samples {
        let mut dist = BTreeMap::new();

        for _ in 0..1000 {
            let sample: Vec<f64> = p.sampling().take(sample_size).collect();
            let m = sample.into_iter().sum::<f64>() as usize / sample_size;
            let counter = dist.entry(m).or_insert(0);
            *counter += 1;
        }

        let data = dist
            .into_iter()
            .map(|(m, freq)| (m as f64, freq as f64))
            .collect::<Vec<(f64, f64)>>();

        let line = plotlib::repr::Plot::new(data)
            .legend(sample_size.to_string())
            .line_style(
                plotlib::style::LineStyle::new()
                    .colour(match sample_size {
                        0..=9 => "#ff0000",
                        10..=49 => "#00ff00",
                        50..=99 => "#0000ff",
                        _ => "#000000",
                    })
                    .linejoin(plotlib::style::LineJoin::Round),
            );
        v = v.add(line);
    }

    plotlib::page::Page::single(&v)
        .save("examples/population.svg")
        .map_err(|e| anyhow::anyhow!(e))?;

    println!("population sd: {}", p.sd());
    Ok(())
}

fn gen_data<T>(start: T, stop: T, nstep: u32) -> Vec<T>
where
    T: Float + FromPrimitive,
{
    let delta: T = (stop - start) / T::from_u32(nstep).unwrap();
    return (0..(nstep))
        .map(|i| start + T::from_u32(i).unwrap() * delta)
        .collect();
}
