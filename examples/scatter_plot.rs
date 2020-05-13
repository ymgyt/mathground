use mathground::stats;
fn main() -> anyhow::Result<()> {
    let data = vec![
        (0., 12.),
        (4., 9.),
        (9., 8.),
        (14., 7.),
        (18., 7.),
        (26., 7.),
        (30., 8.),
        (36., 6.),
        (42., 7.),
        (50., 5.),
    ];

    let scatter = stats::ScatterPlot::with(data);
    scatter
        .render_svg("scatter.svg")
        .map_err(anyhow::Error::from)
}
