pub mod logarithmic;
pub mod multiple_select;
pub mod percentage;
pub mod select;
pub mod builders;

use std::sync::Arc;
use crate::declarative_test::plot_config::PlotAxesConfig;
use crate::data_repository::DataRepository;

pub trait Transformer {
	fn transform_data(&self, data_repository: Arc<dyn DataRepository + Send + Sync>) -> Result<Arc<dyn DataRepository + Send + Sync>, String>;
	fn transform_axes_config(&self, axes_config: PlotAxesConfig) -> Result<PlotAxesConfig, ()>;
}
