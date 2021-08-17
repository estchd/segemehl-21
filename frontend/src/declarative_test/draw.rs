use plotters::drawing::DrawingArea;
use plotters::coord::Shift;
use crate::data_repository::DataRepository;
use std::sync::Arc;
use plotters_canvas::CanvasBackend;

pub trait Draw {
	fn draw(&self, area: &DrawingArea<CanvasBackend, Shift>, data: Arc<dyn DataRepository>) -> Result<(),()>;
}