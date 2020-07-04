use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, LineStyle};
use std::f64;
use rand::Rng;

fn main() {
    let data: Vec<(f64, f64)> = (0..1000).map(|i| {
        let mut y = 50.0 + 20.0 * ((f64::from(i) / 100.0).sin());
        let mut d = rand::thread_rng().sample::<f64,_>(rand::distributions::OpenClosed01);
        d = (d*2.-1.) * 20.;
        y += d;
        (i as f64, y)
    }).collect();

    let p = Plot::new(data).line_style(
        LineStyle::new().linejoin(plotlib::style::LineJoin::Round).colour("#DDDDDD")
    );

    let v = ContinuousView::new().add(p);

    Page::single(&v).save("sin.svg").unwrap();
}
