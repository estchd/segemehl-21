#![feature(async_closure)]

use std::collections::{HashMap};
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use plotters::chart::ChartBuilder;
use plotters::coord::ranged1d::IntoSegmentedCoord;
use plotters::coord::Shift;
use plotters::drawing::{DrawingArea};
use plotters::prelude::*;
use plotters::style::BLUE;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget};

use declarative_test::plot_config::{Axis, AxisConfig, DatasetDescription};
use plots::builders::PlotBuilder;
use util::set_panic_hook;

use crate::data_repository::adapter::AdapterRepository;
use crate::data_repository::DataRepository;
use crate::data_repository::hash_map::HashMapRepository;
use crate::plots::Plot;
use crate::file_list::FILE_LIST;
use crate::transformers::builders::TransformerBuilder;
use segemehl_21_core::{
    statistics::presentation::PresentationData,
    statistics::presentation::frequency_map::PresentationFrequencyMap
};

mod transformers;
mod data_repository;
mod plots;
#[macro_use]
mod util;
mod declarative_test;
mod accessories;
mod file_list;
mod chromosome_list;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CAPTION_TEXT_STYLE_INFO: (&'static str, u32) = ("sans-serif",20);
static DESCRIPTION_TEXT_STYLE_INFO: (&'static str, u32) = ("sans-serif",15);
static LABEL_TEXT_STYLE_INFO: (&'static str, u32) = ("sans-serif",10);

lazy_static! {
    static ref STATISTICS: Mutex<Option<PresentationData>> = {
        Mutex::new(None)
    };
    static ref DATA_REPOSITORY: Arc<Mutex<Option<HashMapRepository>>> = {
        Arc::new(Mutex::new(None))
    };
    static ref PLOTS: Mutex<HashMap<String, Arc<dyn Plot + Send + Sync>>> = {
        Mutex::new(HashMap::new())
    };
    static ref PLOT_CLASSES: Mutex<HashMap<String, Vec<(String, Arc<dyn Plot + Send + Sync>)>>> = {
        Mutex::new(HashMap::new())
    };
}

#[wasm_bindgen]
pub fn main() {
    set_panic_hook();
    setup_plots();
}

fn setup_plots() {
    register_plot_class_update("chromosome-select".to_string(), "single_chromosome".to_string()).unwrap();

    setup_test_plot().expect("Error Setting up Test Plot");
    setup_length_of_chromosomes_plot().expect("Error Setting up Length of Chromosomes Plot");
    setup_covered_length_of_chromosomes_plot().expect("Error Setting up Covered Length of Chromosomes Plot");
    setup_coverage_per_chromosome_plot().expect("Error Setting up Coverage per Chromosome Plot");
    setup_coverage_per_bin_on_selected_chromosome_plot().expect("Error Setting up Coverage per Bin on selected Chromosome Plot");
    setup_file_quality_frequency_map_plot().expect("Error Setting up File Quality Frequency Map Plot");
    setup_selected_chromosome_quality_frequency_map_plot().expect("Error Setting up Selected Chromosome Quality Frequency Map Plot");
    setup_reads_per_chromosome_plot().expect("Error Setting up Reads per Chromosome Plot");
    setup_read_length_per_chromosome_plot().expect("Error Setting Read Length per Chromosome Plot");
}

fn setup_test_plot() -> Result<(),()> {
    let x_axis = AxisConfig {
        description: "Test X Axis".to_string(),
        margin: 0
    };
    let y_axis = AxisConfig {
        description: "Test Y Axis".to_string(),
        margin: 0
    };
    let test_logarithmic_transformer = TransformerBuilder::new()
        .per_file_logarithmic_transformer()
        .with_base("test_logarithmic_base".to_string())
        .with_toggle("test_logarithmic_toggle".to_string())
        .build("test_dataset".to_string())
        .map_err(|_| ())?;

    let test_plot = PlotBuilder::new()
        .set_x_axis(Some(x_axis))
        .set_y_axis(Some(y_axis))
        .with_title("Test Plot".to_string())
        .with_transformer(Arc::from(test_logarithmic_transformer))
        .stacked_bar_plot(get_data_repository())
        .with_colors(vec![BLUE,GREEN,YELLOW,RED,CYAN,MAGENTA,BLACK])
        .build("test_canvas".to_string(), "test_dataset".to_string())
        .map_err(|_| ())?;

    let test_plot = Arc::from(test_plot);

    add_plot("test_plot".to_string(),test_plot.clone())
        .map_err(|_| ())?;
    add_plot_to_class("all".to_string(),
                      "test_plot".to_string(),
                      test_plot.clone())
        .map_err(|_| ())?;

    register_plot_update("change".to_string(),
                         "test_logarithmic_base".to_string(),
                         "test_plot".to_string())
        .map_err(|_| ())?;
    register_plot_update("change".to_string(),
                         "test_logarithmic_toggle".to_string(),
                         "test_plot".to_string())
        .map_err(|_| ())?;

    register_plot_update("mousemove".to_string(),
                         "test_canvas".to_string(),
                         "test_plot".to_string())
        .map_err(|_| ())?;

    Ok(())
}

fn setup_length_of_chromosomes_plot() -> Result<(), ()> {
    let x_axis = AxisConfig {
        description: "Chromosome".to_string(),
        margin: 0
    };
    let y_axis = AxisConfig {
        description: "Length".to_string(),
        margin: 0
    };
    let logarithmic_transformer = TransformerBuilder::new()
        .per_file_logarithmic_transformer()
        .with_base("length_of_chromosomes_logarithmic_base".to_string())
        .with_toggle("length_of_chromosomes_logarithmic_toggle".to_string())
        .build("length_of_chromosomes".to_string())
        .map_err(|_| ())?;

    let plot = PlotBuilder::new()
        .set_x_axis(Some(x_axis))
        .set_y_axis(Some(y_axis))
        .with_title("Length of Chromosomes".to_string())
        .with_transformer(Arc::from(logarithmic_transformer))
        .stacked_bar_plot(get_data_repository())
        .with_colors(vec![BLUE,GREEN,YELLOW,RED,CYAN,MAGENTA,BLACK])
        .build("length_of_chromosomes_canvas".to_string(), "length_of_chromosomes".to_string())
        .map_err(|_| ())?;

    let plot = Arc::from(plot);

    add_plot("length_of_chromosomes_plot".to_string(), plot.clone())
        .map_err(|_| ())?;
    add_plot_to_class("all".to_string(),
                      "length_of_chromosomes_plot".to_string(),
                      plot.clone())
        .map_err(|_| ())?;

    register_plot_update("change".to_string(),
                         "length_of_chromosomes_logarithmic_base".to_string(),
                         "length_of_chromosomes_plot".to_string())
        .map_err(|_| ())?;
    register_plot_update("change".to_string(),
                         "length_of_chromosomes_logarithmic_toggle".to_string(),
                         "length_of_chromosomes_plot".to_string())
        .map_err(|_| ())?;

    Ok(())
}

fn setup_covered_length_of_chromosomes_plot() -> Result<(), ()> {
    let x_axis = AxisConfig {
        description: "Chromosome".to_string(),
        margin: 0
    };
    let y_axis = AxisConfig {
        description: "Covered Length".to_string(),
        margin: 0
    };
    let logarithmic_transformer = TransformerBuilder::new()
        .per_file_logarithmic_transformer()
        .with_base("covered_length_of_chromosomes_logarithmic_base".to_string())
        .with_toggle("covered_length_of_chromosomes_logarithmic_toggle".to_string())
        .build("covered_length_of_chromosomes".to_string())
        .map_err(|_| ())?;

    let plot = PlotBuilder::new()
        .set_x_axis(Some(x_axis))
        .set_y_axis(Some(y_axis))
        .with_title("Covered Length of Chromosomes".to_string())
        .with_transformer(Arc::from(logarithmic_transformer))
        .stacked_bar_plot(get_data_repository())
        .with_colors(vec![BLUE,GREEN,YELLOW,RED,CYAN,MAGENTA,BLACK])
        .build("covered_length_of_chromosomes_canvas".to_string(), "covered_length_of_chromosomes".to_string())
        .map_err(|_| ())?;

    let plot = Arc::from(plot);

    add_plot("covered_length_of_chromosomes_plot".to_string(), plot.clone())
        .map_err(|_| ())?;
    add_plot_to_class("all".to_string(),
                      "covered_length_of_chromosomes_plot".to_string(),
                      plot.clone())
        .map_err(|_| ())?;

    register_plot_update("change".to_string(),
                         "covered_length_of_chromosomes_logarithmic_base".to_string(),
                         "covered_length_of_chromosomes_plot".to_string())
        .map_err(|_| ())?;
    register_plot_update("change".to_string(),
                         "covered_length_of_chromosomes_logarithmic_toggle".to_string(),
                         "covered_length_of_chromosomes_plot".to_string())
        .map_err(|_| ())?;

    Ok(())
}

fn setup_coverage_per_chromosome_plot() -> Result<(), ()> {
    let x_axis = AxisConfig {
        description: "Chromosome".to_string(),
        margin: 0
    };
    let y_axis = AxisConfig {
        description: "Coverage".to_string(),
        margin: 0
    };

    let percentage_transform = TransformerBuilder::new()
        .per_file_percentage_transformer()
        .build("coverage_per_chromosome".to_string());

    let plot = PlotBuilder::new()
        .set_x_axis(Some(x_axis))
        .set_y_axis(Some(y_axis))
        .with_title("Coverage per Chromosome".to_string())
        .with_transformer(Arc::from(percentage_transform))
        .stacked_bar_plot(get_data_repository())
        .with_colors(vec![BLUE,GREEN,YELLOW,RED,CYAN,MAGENTA,BLACK])
        .build("coverage_per_chromosome_canvas".to_string(), "coverage_per_chromosome".to_string())
        .map_err(|_| ())?;

    let plot = Arc::from(plot);

    add_plot("coverage_per_chromosome_plot".to_string(), plot.clone())
        .map_err(|_| ())?;
    add_plot_to_class("all".to_string(),
                      "coverage_per_chromosome_plot".to_string(),
                      plot.clone())
        .map_err(|_| ())?;

    Ok(())
}

fn setup_coverage_per_bin_on_selected_chromosome_plot() -> Result<(), String> {
    let x_axis = AxisConfig {
        description: "Number of Reads covering the bin".to_string(),
        margin: 0
    };
    let y_axis = AxisConfig {
        description: "Bin".to_string(),
        margin: 0
    };

    let select_transformer = TransformerBuilder::new()
        .per_file_multiple_select_transformer()
        .add_select("coverage_per_bin_on_selected_chromosome_stat".to_string(), true)
        .add_select("chromosome-select".to_string(), false)
        .build("coverage_per_bin".to_string());

    let plot = PlotBuilder::new()
        .set_x_axis(Some(x_axis))
        .set_y_axis(Some(y_axis))
        .with_title("Coverage per Bin".to_string())
        .with_transformer(Arc::from(select_transformer))
        .stacked_bar_plot(get_data_repository())
        .with_colors(vec![BLUE,GREEN,YELLOW,RED,CYAN,MAGENTA,BLACK])
        .build("coverage_per_bin_on_selected_chromosome_canvas".to_string(), "coverage_per_bin".to_string())
        .map_err(|_| "Error Building Plot")?;

    let plot = Arc::from(plot);

    add_plot("coverage_per_bin_on_selected_chromosome_plot".to_string(), plot.clone())
        .map_err(|_| "Error Adding Plot")?;
    add_plot_to_class("all".to_string(),
                      "coverage_per_bin_on_selected_chromosome_plot".to_string(),
                      plot.clone())
        .map_err(|_| "Error Adding Plot to Class all")?;
    add_plot_to_class("single_chromosome".to_string(),
                      "coverage_per_bin_on_selected_chromosome_plot".to_string(),
                      plot.clone())
        .map_err(|_| "Error Adding Plot to Class single_chromosome")?;

    register_plot_update("change".to_string(),
                         "coverage_per_bin_on_selected_chromosome_stat".to_string(),
                         "coverage_per_bin_on_selected_chromosome_plot".to_string())
        .map_err(|err| {
            let err: String = js_sys::JSON::stringify(&err).unwrap().into();
            format!("Error Registering Plot Update: {}", err)
        })?;

    Ok(())
}

fn setup_file_quality_frequency_map_plot() -> Result<(), ()> {
    let x_axis = AxisConfig {
        description: "Quality".to_string(),
        margin: 0
    };
    let y_axis = AxisConfig {
        description: "Number of Reads".to_string(),
        margin: 0
    };
    let logarithmic_transformer = TransformerBuilder::new()
        .per_file_logarithmic_transformer()
        .with_base("file_quality_frequency_map_logarithmic_base".to_string())
        .with_toggle("file_quality_frequency_map_logarithmic_toggle".to_string())
        .build("file_quality_frequency_map".to_string())
        .map_err(|_| ())?;

    let plot = PlotBuilder::new()
        .set_x_axis(Some(x_axis))
        .set_y_axis(Some(y_axis))
        .with_title("Quality Frequency Map for Complete File".to_string())
        .with_transformer(Arc::from(logarithmic_transformer))
        .stacked_bar_plot(get_data_repository())
        .with_colors(vec![BLUE,GREEN,YELLOW,RED,CYAN,MAGENTA,BLACK])
        .build("file_quality_frequency_map_canvas".to_string(), "file_quality_frequency_map".to_string())
        .map_err(|_| ())?;

    let plot = Arc::from(plot);

    add_plot("file_quality_frequency_map_plot".to_string(), plot.clone())
        .map_err(|_| ())?;
    add_plot_to_class("all".to_string(),
                      "file_quality_frequency_map_plot".to_string(),
                      plot.clone())
        .map_err(|_| ())?;

    register_plot_update("change".to_string(),
                         "file_quality_frequency_map_logarithmic_base".to_string(),
                         "file_quality_frequency_map_plot".to_string())
        .map_err(|_| ())?;
    register_plot_update("change".to_string(),
                         "file_quality_frequency_map_logarithmic_toggle".to_string(),
                         "file_quality_frequency_map_plot".to_string())
        .map_err(|_| ())?;

    Ok(())
}

fn setup_selected_chromosome_quality_frequency_map_plot() -> Result<(), ()> {
    let x_axis = AxisConfig {
        description: "Number of Reads".to_string(),
        margin: 0
    };
    let y_axis = AxisConfig {
        description: "Quality".to_string(),
        margin: 0
    };

    let select_transformer = TransformerBuilder::new()
        .per_file_select_transformer()
        .build("chromosome-select".to_string(), "quality_frequency_map".to_string());

    let logarithmic_transformer = TransformerBuilder::new()
        .per_file_logarithmic_transformer()
        .with_base("selected_chromosome_quality_frequency_map_logarithmic_base".to_string())
        .with_toggle("selected_chromosome_quality_frequency_map_logarithmic_toggle".to_string())
        .build("quality_frequency_map".to_string())
        .map_err(|_| ())?;

    let plot = PlotBuilder::new()
        .set_x_axis(Some(x_axis))
        .set_y_axis(Some(y_axis))
        .with_title("Quality Frequency Map for Selected Chromosome".to_string())
        .with_transformer(Arc::from(select_transformer))
        .with_transformer(Arc::from(logarithmic_transformer))
        .stacked_bar_plot(get_data_repository())
        .with_colors(vec![BLUE,GREEN,YELLOW,RED,CYAN,MAGENTA,BLACK])
        .build("selected_chromosome_quality_frequency_map_canvas".to_string(), "quality_frequency_map".to_string())
        .map_err(|_| ())?;

    let plot = Arc::from(plot);

    add_plot("selected_chromosome_quality_frequency_map_plot".to_string(), plot.clone())
        .map_err(|_| ())?;
    add_plot_to_class("all".to_string(),
                      "selected_chromosome_quality_frequency_map_plot".to_string(),
                      plot.clone())
        .map_err(|_| ())?;
    add_plot_to_class("single_chromosome".to_string(),
                      "selected_chromosome_quality_frequency_map_plot".to_string(),
                      plot.clone())
        .map_err(|_| ())?;

    register_plot_update("change".to_string(),
                         "selected_chromosome_quality_frequency_map_logarithmic_base".to_string(),
                         "selected_chromosome_quality_frequency_map_plot".to_string())
        .map_err(|_| ())?;
    register_plot_update("change".to_string(),
                         "selected_chromosome_quality_frequency_map_logarithmic_toggle".to_string(),
                         "selected_chromosome_quality_frequency_map_plot".to_string())
        .map_err(|_| ())?;

    Ok(())
}

fn setup_reads_per_chromosome_plot() -> Result<(), ()> {
    let x_axis = AxisConfig {
        description: "Quality".to_string(),
        margin: 0
    };
    let y_axis = AxisConfig {
        description: "Number of Reads".to_string(),
        margin: 0
    };
    let logarithmic_transformer = TransformerBuilder::new()
        .per_file_logarithmic_transformer()
        .with_base("reads_per_chromosome_logarithmic_base".to_string())
        .with_toggle("reads_per_chromosome_logarithmic_toggle".to_string())
        .build("reads_per_chromosome".to_string())
        .map_err(|_| ())?;

    let plot = PlotBuilder::new()
        .set_x_axis(Some(x_axis))
        .set_y_axis(Some(y_axis))
        .with_title("Number of Reads per Chromosome".to_string())
        .with_transformer(Arc::from(logarithmic_transformer))
        .stacked_bar_plot(get_data_repository())
        .with_colors(vec![BLUE,GREEN,YELLOW,RED,CYAN,MAGENTA,BLACK])
        .build("reads_per_chromosome_canvas".to_string(), "reads_per_chromosome".to_string())
        .map_err(|_| ())?;

    let plot = Arc::from(plot);

    add_plot("reads_per_chromosome_plot".to_string(), plot.clone())
        .map_err(|_| ())?;
    add_plot_to_class("all".to_string(),
                      "reads_per_chromosome_plot".to_string(),
                      plot.clone())
        .map_err(|_| ())?;

    register_plot_update("change".to_string(),
                         "reads_per_chromosome_logarithmic_base".to_string(),
                         "reads_per_chromosome_plot".to_string())
        .map_err(|_| ())?;
    register_plot_update("change".to_string(),
                         "reads_per_chromosome_logarithmic_toggle".to_string(),
                         "reads_per_chromosome_plot".to_string())
        .map_err(|_| ())?;

    Ok(())
}

fn setup_read_length_per_chromosome_plot() -> Result<(), ()> {
    let x_axis = AxisConfig {
        description: "Chromosome".to_string(),
        margin: 0
    };
    let y_axis = AxisConfig {
        description: "Length of a Read".to_string(),
        margin: 0
    };

    let select_transformer = TransformerBuilder::new()
        .per_file_select_transformer()
        .build("length_of_read_per_chromosome_stat".to_string(), "length_of_read_per_chromosome".to_string());

    let logarithmic_transformer = TransformerBuilder::new()
        .per_file_logarithmic_transformer()
        .with_base("length_of_read_per_chromosome_logarithmic_base".to_string())
        .with_toggle("length_of_read_per_chromosome_logarithmic_toggle".to_string())
        .build("length_of_read_per_chromosome".to_string())
        .map_err(|_| ())?;

    let plot = PlotBuilder::new()
        .set_x_axis(Some(x_axis))
        .set_y_axis(Some(y_axis))
        .with_title("Number of Reads per Chromosome".to_string())
        .with_transformer(Arc::from(select_transformer))
        .with_transformer(Arc::from(logarithmic_transformer))
        .stacked_bar_plot(get_data_repository())
        .with_colors(vec![BLUE,GREEN,YELLOW,RED,CYAN,MAGENTA,BLACK])
        .build("length_of_read_per_chromosome_canvas".to_string(), "length_of_read_per_chromosome".to_string())
        .map_err(|_| ())?;

    let plot = Arc::from(plot);

    add_plot("length_of_read_per_chromosome_plot".to_string(), plot.clone())
        .map_err(|_| ())?;
    add_plot_to_class("all".to_string(),
                      "length_of_read_per_chromosome_plot".to_string(),
                      plot.clone())
        .map_err(|_| ())?;

    register_plot_update("change".to_string(),
                         "length_of_read_per_chromosome_logarithmic_base".to_string(),
                         "length_of_read_per_chromosome_plot".to_string())
        .map_err(|_| ())?;
    register_plot_update("change".to_string(),
                         "length_of_read_per_chromosome_logarithmic_toggle".to_string(),
                         "length_of_read_per_chromosome_plot".to_string())
        .map_err(|_| ())?;
    register_plot_update("change".to_string(),
                         "length_of_read_per_chromosome_stat".to_string(),
                         "length_of_read_per_chromosome_plot".to_string())
        .map_err(|_| ())?;

    Ok(())
}

pub fn register_plot_update(event_name: String, element_id: String, plot_name: String) -> Result<(), JsValue> {
    let element: Option<EventTarget> = util::get_cast_element(element_id.clone());

    let element: EventTarget = element.ok_or(format!("Element not found: {}", element_id.clone()))?;

    let closure = Closure::wrap(Box::new(
        move || {
            match update_plot(plot_name.clone()) {
                Ok(_) => {}
                Err(err) => {
                    let err: String = js_sys::JSON::stringify(&err).unwrap().into();
                    panic!("Error Updating Plot: {}, Err: {}", plot_name.clone(), err)
                }
            }
        }
    ) as Box<dyn FnMut()>);

    element.add_event_listener_with_callback(event_name.as_str(), closure.as_ref().unchecked_ref())?;

    closure.forget();
    Ok(())
}

pub fn register_plot_class_update(element_id: String, plot_class_name: String) -> Result<(), JsValue> {
    let element: Option<EventTarget> = util::get_cast_element(element_id.clone());

    let element: EventTarget = element.ok_or(format!("Element not found: {}", element_id.clone()))?;

    let closure = Closure::wrap(Box::new(
        move || {
            match update_plot_class(plot_class_name.clone()) {
                Ok(_) => {}
                Err(err) => {
                    let err: String = js_sys::JSON::stringify(&err).unwrap().into();
                    panic!("Error Updating Plot Class: {}, err: {}", plot_class_name.clone(), err)
                }
            }
        }
    ) as Box<dyn FnMut()>);

    element.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;

    closure.forget();
    Ok(())
}

fn get_data_repository() -> Arc<dyn DataRepository + Send + Sync> {
    Arc::from(AdapterRepository::from(DATA_REPOSITORY.clone()))
}

#[wasm_bindgen]
pub fn update_plot(plot_name: String) -> Result<(), JsValue> {
    let plots = PLOTS.lock()
        .map_err(|_| "Could not Lock Plots")
        .map_err(|err| format!("Error Drawing Plot: {}, Error: {}", &plot_name, err))?;

    let plot = plots.get(&plot_name)
        .ok_or("Could not get Plot")
        .map_err(|err| format!("Error Drawing Plot: {}, Error: {}", &plot_name, err))?;

    plot.draw()
        .map_err(|err| format!("Error Drawing Plot: {}", err))
        .map_err(|err| format!("Error Drawing Plot: {}, Error: {}", &plot_name, err))?;

    Ok(())
}

#[wasm_bindgen]
pub fn update_plot_class(class_name: String) -> Result<(), JsValue> {
    let plot_classes = PLOT_CLASSES.lock()
        .map_err(|_| "Could not Lock Plot Classes")
        .map_err(|err| format!("Error updating Plot Class: {}, Error: {}", &class_name, err))?;

    let plots = plot_classes.get(&class_name)
        .ok_or("Could not get Plots")
        .map_err(|err| format!("Error updating Plot Class: {}, Error: {}", &class_name, err))?;

    for (plot_name, plot) in plots {
        plot.draw()
            .map_err(|err| format!("Error updating Plot Class: {}, Error Updating Plot: {}, Error: {}", &class_name, plot_name, err))?;
    }
    Ok(())
}

fn add_plot(name: String, plot: Arc<dyn Plot + Send + Sync>) -> Result<(),()> {
    let mut plots = PLOTS.lock().map_err(|_| ())?;
    if plots.contains_key(&name) {
        return Err(());
    }
    plots.insert(name, plot);
    Ok(())
}

fn add_plot_to_class(class_name: String, plot_name: String, plot: Arc<dyn Plot + Send + Sync>) -> Result<(),()> {
    let mut plot_classes = PLOT_CLASSES.lock().map_err(|_| ())?;
    if plot_classes.contains_key(&class_name) {
        let class = plot_classes.get_mut(&class_name).unwrap();
        class.push((plot_name, plot));
    }
    else {
        plot_classes.insert(class_name, vec![(plot_name, plot)]);
    }

    Ok(())
}

#[wasm_bindgen]
pub struct PerFileStatistics {
    pub number_reads_in_file: u64,
    pub total_length_of_reads_in_file: u64,

    pub length_of_smallest_read: u32,
    pub length_of_longest_read: u32,
    pub smallest_number_of_reads_for_single_chromosome: u64,
    pub biggest_number_of_reads_for_single_chromosome: u64,

    pub median_length_of_read_in_file: f64,
    pub mode_length_of_read_in_file: u32,
    pub mean_length_of_read_in_file: f64,

    pub median_number_of_reads_per_chromosome: f64,
    pub mode_number_of_reads_per_chromosome: u64,
    pub mean_number_of_reads_per_chromosome: f64,

    pub total_chromosome_length: u64,
    pub median_length_of_chromosomes: f64,
    pub mode_length_of_chromosomes: u32,
    pub mean_length_of_chromosomes: f64,
    pub shortest_chromosome_length: u32,
    pub longest_chromosome_length: u32,

    pub median_chromosome_coverage: f64,
    pub mean_chromosome_coverage: f64,
    pub least_chromosome_coverage: f64,
    pub most_chromosome_coverage: f64,
}

pub struct StatisticsDisplayData {
    pub per_chromosome_data: Vec<StatisticsPerChromosomeDisplayData>
}

pub struct StatisticsPerChromosomeDisplayData {
    pub name: String,
    pub num_reads: usize,

    pub median_length_of_reads: f64,
    pub mode_length_of_reads: f64,
    pub mean_length_of_reads: f64
}

fn regenerate_data_repository() {
    let file_list = FILE_LIST.lock().unwrap();

    let mut repository = HashMapRepository::new();

    for (key, value) in file_list.iter() {
        match value {
            None => continue,
            Some(statistics) => {
                let length_of_chromosomes_data: Vec<f64> = statistics.get_per_reference_data()
                                                                     .map(|item| item.get_reference_length())
                                                                     .map(|item| item as f64).collect();

                repository.add_one_dimensional_data(format!("{}_test_dataset", key), length_of_chromosomes_data.clone());
                repository.add_one_dimensional_data(format!("{}_length_of_chromosomes",key), length_of_chromosomes_data.clone());

                let covered_length_of_chromosomes_data: Vec<f64> = statistics.get_per_reference_data()
                                                                             .map(|item| item.get_covered_length())
                                                                             .map(|item| item as f64).collect();

                repository.add_one_dimensional_data(format!("{}_covered_length_of_chromosomes",key), covered_length_of_chromosomes_data.clone());

                let coverage_per_chromosome_data: Vec<f64> = length_of_chromosomes_data.iter().zip(covered_length_of_chromosomes_data.iter())
                                                                                       .map(|(length, covered_length)| *covered_length / *length).collect();

                repository.add_one_dimensional_data(format!("{}_coverage_per_chromosome",key), coverage_per_chromosome_data.clone());

                let quality_frequency_map: Vec<f64> = statistics.get_complete_quality_frequency_map()
                                                                .into_iter()
                                                                .map(|(_,b)| b)
                                                                .map(|item| item as f64).collect();

                repository.add_one_dimensional_data(format!("{}_file_quality_frequency_map",key), quality_frequency_map.clone());

                let empty_data: Vec<f64> = vec![];

                repository.add_one_dimensional_data(format!("{}__Total_coverage_per_bin",key), empty_data.clone());
                repository.add_one_dimensional_data(format!("{}__Average_coverage_per_bin",key), empty_data.clone());
                repository.add_one_dimensional_data(format!("{}__quality_frequency_map",key), empty_data.clone());

                for (i, per_chromosome_data) in statistics.get_per_reference_data().enumerate() {
                    let total_coverage_name = format!("{}_{}_Total_coverage_per_bin", key, per_chromosome_data.get_reference_name().clone());
                    let average_coverage_name = format!("{}_{}_Average_coverage_per_bin", key, per_chromosome_data.get_reference_name().clone());
                    let quality_name = format!("{}_{}_quality_frequency_map", key, per_chromosome_data.get_reference_name().clone());

                    let quality_data: Vec<f64> = statistics.get_per_reference_by_index(i).unwrap()
                                                           .get_single_read_data()
                                                           .get_quality_frequency_map()
                                                           .iter()
                                                           .map(|(_, item)| *item as f64).collect();
                    let total_coverage_data: Vec<f64> = statistics.get_per_reference_by_index(i).unwrap()
                                                                  .get_single_read_data()
                                                                  .get_binned_statistics()
                                                                  .get_bins()
                                                                  .map(|item| item.get_coverage())
                                                                  .map(|item| item as f64)
                                                                  .collect();
                    let average_coverage_data: Vec<f64> = statistics.get_per_reference_by_index(i).unwrap()
                                                                    .get_single_read_data()
                                                                    .get_binned_statistics()
                                                                    .get_bins()
                                                                    .map(|item| item.get_average_coverage())
                                                                    .collect();

                    repository.add_one_dimensional_data(quality_name, quality_data);
                    repository.add_one_dimensional_data(total_coverage_name, total_coverage_data);
                    repository.add_one_dimensional_data(average_coverage_name, average_coverage_data);
                }

                let reads_per_chromosome_data: Vec<f64> = statistics.get_per_reference_data()
                                                                    .map(|item| item.get_single_read_data())
                                                                    .map(|item| item.get_read_length_map())
                                                                    .map(|item| item.get_frequency_sum())
                                                                    .map(|item| item as f64).collect();

                repository.add_one_dimensional_data(format!("{}_reads_per_chromosome",key), reads_per_chromosome_data.clone());

                let shortest_length_of_read_per_chromosome_data: Vec<f64> = statistics.get_per_reference_data()
                                                                                      .map(|item| item.get_single_read_data())
                                                                                      .map(|item| item.get_read_length_map())
                                                                                      .map(|item| item.get_min_entry())
                                                                                      .map(|item| item.unwrap_or((0,0)))
                                                                                      .map(|(item,_)| item as f64).collect();
                let longest_length_of_read_per_chromosome_data: Vec<f64> = statistics.get_per_reference_data()
                                                                                     .map(|item| item.get_single_read_data())
                                                                                     .map(|item| item.get_read_length_map())
                                                                                     .map(|item| item.get_max_entry())
                                                                                     .map(|item| item.unwrap_or((0,0)))
                                                                                     .map(|(item,_)| item as f64).collect();
                let mean_length_of_read_per_chromosome_data: Vec<f64> = statistics.get_per_reference_data()
                                                                                  .map(|item| item.get_single_read_data())
                                                                                  .map(|item| item.get_read_length_map())
                                                                                  .map(|item| item.get_mean_entry()).collect();
                let median_length_of_read_per_chromosome_data: Vec<f64> = statistics.get_per_reference_data()
                                                                                    .map(|item| item.get_single_read_data())
                                                                                    .map(|item| item.get_read_length_map())
                                                                                    .map(|item| item.get_median_entry())
                                                                                    .map(|item| item.unwrap_or(0.0)).collect();
                let mode_length_of_read_per_chromosome_data: Vec<f64> = statistics.get_per_reference_data()
                                                                                  .map(|item| item.get_single_read_data())
                                                                                  .map(|item| item.get_read_length_map())
                                                                                  .map(|item| item.get_max_frequency())
                                                                                  .map(|item| item.unwrap_or((0,0)))
                                                                                  .map(|(item,_)| item)
                                                                                  .map(|item| item as f64).collect();

                repository.add_one_dimensional_data(format!("{}_Shortest_length_of_read_per_chromosome", key), shortest_length_of_read_per_chromosome_data.clone());
                repository.add_one_dimensional_data(format!("{}_Longest_length_of_read_per_chromosome", key), longest_length_of_read_per_chromosome_data.clone());
                repository.add_one_dimensional_data(format!("{}_Mean_length_of_read_per_chromosome", key), mean_length_of_read_per_chromosome_data.clone());
                repository.add_one_dimensional_data(format!("{}_Median_length_of_read_per_chromosome", key), median_length_of_read_per_chromosome_data.clone());
                repository.add_one_dimensional_data(format!("{}_Mode_length_of_read_per_chromosome", key), mode_length_of_read_per_chromosome_data.clone());
            }
        }
    }

    let repository_lock = DATA_REPOSITORY.lock();
    let mut repository_lock = match repository_lock {
        Ok(lock) => lock,
        Err(_) => return,
    };
    *repository_lock = Some(repository);
}

#[wasm_bindgen]
pub fn generate_per_file_stats(file_name: String) -> Option<PerFileStatistics> {
    let file_list = FILE_LIST.lock().unwrap();

    let statistics = file_list.get(&file_name)?;

    if statistics.is_none() {return None;}

    let statistics = statistics.as_ref().unwrap();

    let total_chromosome_length: u64 = statistics.get_per_reference_data()
        .map(|item| item.get_reference_length() as u64)
        .sum();

    let chromosome_count = statistics.get_per_reference_data().count();

    let shortest_chromosome_length = statistics.get_per_reference_data()
        .map(|item| item.get_reference_length())
        .min()
        .unwrap_or(0);

    let longest_chromosome_length = statistics.get_per_reference_data()
        .map(|item| item.get_reference_length())
        .max()
        .unwrap_or(0);

    let median_length_of_chromosomes = (longest_chromosome_length as usize + shortest_chromosome_length as usize) as f64 / 2.0;

    let mean_length_of_chromosomes = total_chromosome_length as f64 / chromosome_count as f64;

    let mut chromosome_length_map = PresentationFrequencyMap::<u32>::new();

    for chromosome in statistics.get_per_reference_data() {
        chromosome_length_map.add_entry(chromosome.get_reference_length())
    }

    let coverages: Vec<f64> = statistics.get_per_reference_data()
        .map(|item| item.get_covered_percentage())
        .collect();

    let least_chromosome_coverage = *coverages.iter().reduce(|a,b| {
        if a <= b {
            a
        }
        else {
            b
        }
    }).unwrap_or(&0.0);
    let most_chromosome_coverage = *coverages.iter().reduce(|a,b| {
        if a >= b {
            a
        }
        else {
            b
        }
    }).unwrap_or(&0.0);

    let coverage_sum: f64 = coverages.iter().sum();

    let mean_chromosome_coverage: f64 = coverage_sum / coverages.len() as f64;

    let median_chromosome_coverage = least_chromosome_coverage + most_chromosome_coverage / 2.0;

    let (mode_length_of_chromosomes, _) = chromosome_length_map.get_max_frequency().unwrap_or((0,0));

    let complete_read_length_map = statistics.get_complete_read_length_map();

    let statistics = PerFileStatistics {
        number_reads_in_file: complete_read_length_map.get_frequency_sum(),
        total_length_of_reads_in_file: complete_read_length_map.get_weighted_frequency_sum(),
        length_of_smallest_read: complete_read_length_map.get_min_entry().unwrap_or((0,0)).0,
        length_of_longest_read: complete_read_length_map.get_max_entry().unwrap_or((0,0)).0,
        smallest_number_of_reads_for_single_chromosome: statistics.get_least_read_count(),
        biggest_number_of_reads_for_single_chromosome: statistics.get_most_read_count(),
        median_length_of_read_in_file:complete_read_length_map.get_median_entry().unwrap_or(0.0),
        mode_length_of_read_in_file: complete_read_length_map.get_max_frequency().unwrap_or((0,0)).0,
        mean_length_of_read_in_file: complete_read_length_map.get_mean_entry(),
        median_number_of_reads_per_chromosome: statistics.get_median_read_count(),
        mode_number_of_reads_per_chromosome: statistics.get_mode_read_count(),
        mean_number_of_reads_per_chromosome: statistics.get_mean_read_count(),
        total_chromosome_length,
        median_length_of_chromosomes,
        mode_length_of_chromosomes,
        mean_length_of_chromosomes,
        shortest_chromosome_length,
        longest_chromosome_length,
        median_chromosome_coverage,
        mean_chromosome_coverage,
        least_chromosome_coverage,
        most_chromosome_coverage
    };

    return Some(statistics);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
