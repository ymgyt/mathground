fn main() {
    let f1 = |x: f64| x.sin();
    let f2 = |x: f64| (2. * x).sin();

    let pi = std::f64::consts::PI;
    let (min, max) = (0., pi * 2.);

    let f1 = plotlib::repr::Plot::from_function(f1, min, max)
        .line_style(plotlib::style::LineStyle::new().colour("#aa0000"));
    let f2 = plotlib::repr::Plot::from_function(f2, min, max)
        .line_style(plotlib::style::LineStyle::new().colour("#00aa00"));

    let mut v = plotlib::view::ContinuousView::new().add(f1).add(f2);
    let v = v.x_range(min, max);

    plotlib::page::Page::single(&v)
        .save("differential.svg")
        .expect("saving svg");
}
