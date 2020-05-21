mod core;
mod error;
mod population;
mod scatter_plot;

type Result<T> = std::result::Result<T, self::error::StatsError>;

pub use self::core::{average, standard_deviation, standard_score, variance};
pub use self::error::StatsError;
pub use self::population::Population;
pub use self::scatter_plot::ScatterPlot;
