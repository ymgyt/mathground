use crate::stats::{Result, StatsError};
use plotlib::{
    page::Page,
    repr::Plot,
    style::{PointMarker, PointStyle},
    view::ContinuousView,
};
use std::path::Path;

pub struct ScatterPlot {
    data: Vec<(f64, f64)>,
}

impl ScatterPlot {
    pub fn with(data: Vec<(f64, f64)>) -> Self {
        Self { data }
    }

    pub fn render_svg<P: AsRef<Path>>(self, path: P) -> Result<()> {
        let _len = self.data.len();
        let (x_range, y_range) =
            self.data
                .iter()
                .fold(((0.0_f64, 0.0_f64), (0.0_f64, 0.0_f64)), |xy_range, p| {
                    (
                        ((xy_range.0).0.min(p.0), (xy_range.0).1.max(p.0)),
                        ((xy_range.1).0.min(p.1), (xy_range.1).1.max(p.1)),
                    )
                });
        let plot = Plot::new(self.data).point_style(
            PointStyle::new()
                .marker(PointMarker::Circle)
                .colour("#DD3355"),
        );

        let view = ContinuousView::new()
            .add(plot)
            .x_range(x_range.0, x_range.1)
            .y_range(y_range.0, y_range.1);

        let page = Page::single(&view);
        page.save(path)
            .map_err(|err| StatsError::Unknown(format!("{}", err)))
    }
}
