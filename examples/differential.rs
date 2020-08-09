fn main() {
    let f1 = |x: f64| (1./3.)*x.powf(3.) + 2.*x.powf(2.)+ 5.;
    let f1 = plotlib::repr::Plot::from_function(f1, -5., 5.)
        .line_style(plotlib::style::LineStyle::new().colour("#aa0000"));

    let mut v = plotlib::view::ContinuousView::new().add(f1);
    v = v.y_range(-1.0, 15. );


    plotlib::page::Page::single(&v).save("differential.svg").expect("saving svg");
}
