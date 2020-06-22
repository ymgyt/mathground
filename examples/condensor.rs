use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, LineStyle};

fn main() {
    let v = 9.0;
    let data1: Vec<(f64, f64)> = (1..6).map(|i| (i as f64, condensor(i, v))).collect();
    let data2: Vec<(f64, f64)> = (1..6).map(|i| (i as f64, condensor_2(i, v))).collect();

    let s1: Plot = Plot::new(data1).line_style(
        LineStyle::new()
            .linejoin(plotlib::style::LineJoin::Round)
            .colour("#DD3355"),
    ); // and a custom colour

    let s2: Plot = Plot::new(data2).line_style(
        LineStyle::new()
            .linejoin(plotlib::style::LineJoin::Round)
            .colour("#003355"),
    ); // and a custom colour


    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .add(s2);

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("scatter.svg").unwrap();
}

fn condensor(tc: u8, v: f64) -> f64 {
    if tc == 1 {
        return v * 0.63;
    }
    let curr = condensor(tc - 1, v);
    curr + (v - curr) * 0.63
}

fn condensor_2(tc: u8, v: f64) -> f64 {
    return v * (1.0 - (1.0 / std::f64::consts::E).powi(tc as i32));
}
