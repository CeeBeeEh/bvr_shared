pub mod bvr_box;
pub mod bvr_detection;
pub mod bvr_image;
pub mod model_config;

pub const INTERFACE_VERSION: &str = "0.2.1";

use std::error::Error;
use anyhow;
use crate::bvr_detection::BvrDetection;
use crate::bvr_image::BvrImage;
use crate::model_config::ModelConfig;

pub trait BvrDetector {
    fn is_running() -> impl std::future::Future<Output = anyhow<bool>> + Send;
    fn init_detector(model_details: ModelConfig) -> impl std::future::Future<Output = anyhow<()>> + Send;
    fn detect(bvr_image: BvrImage) -> impl std::future::Future<Output = anyhow<Vec<BvrDetection>>> + Send;
    fn get_library_inference_devices() -> Vec<&'static str>;
    fn is_valid_inference_device(inference_device: String) -> bool;
    fn get_library_inference_processors() -> Vec<&'static str>;
    fn is_valid_inference_processor(inference_processor: String) -> bool;
    fn get_library_model_versions() -> Vec<&'static str>;
    fn is_valid_model_version(model_version: String) -> bool;
    fn get_interface_version() -> String;
}