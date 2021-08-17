#![allow(dead_code)]

use crate::declarative_test::plot_config::PlotConfig;
use crate::declarative_test::plot::Plot;
use std::collections::HashMap;
use std::sync::Arc;
use crate::declarative_test::transformer_constructor::TransformerConstructor;
use crate::declarative_test::draw_constructor::DrawConstructor;

pub struct PlotConstructor {
	transformer_constructors: HashMap<String, Vec<Arc<dyn TransformerConstructor>>>,
	draw_constructors: HashMap<String, Vec<Arc<Arc<dyn DrawConstructor>>>>
}

impl PlotConstructor {
	pub fn construct_plot(_config: PlotConfig) -> Result<Plot, ()> {
		unimplemented!()
	}
}