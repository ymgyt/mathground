use itertools::Itertools;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    let p: f64 = 0.5;
    let q = 1. - p;
    let n = 20;
    let data: Vec<(f64, f64)> = (0..=n)
        .map(|k| {
            let c = (1..=n).combinations(k).collect::<Vec<_>>().len() as f64;
            let v = c * p.powf(k as f64) * q.powf((n - k) as f64);
            (k as f64, v)
        })
        .collect();

    println!("{:?}", data);

    let sum = data.iter().fold(0., |acc, x| acc + x.1);
    println!("sum {}", sum);

    let plot = Plot::new(data)
        .line_style(LineStyle::new())
        .point_style(PointStyle::new().marker(PointMarker::Circle));
    let view = ContinuousView::new().add(plot).y_range(0.0, 1.0);

    Page::single(&view).save("bin.svg").unwrap();
}
