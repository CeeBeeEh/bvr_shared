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
    fn is_running(&self) -> impl std::future::Future<Output = anyhow<bool>> + Send;
    fn init_detector(&mut self, model_details: ModelConfig) -> impl std::future::Future<Output = anyhow<()>> + Send;
    fn detect(&self, bvr_image: BvrImage) -> impl std::future::Future<Output = anyhow<Vec<BvrDetection>>> + Send;
    fn get_library_inference_devices(&self) -> Vec<&'static str>;
    fn is_valid_inference_device(&self, inference_device: String) -> bool;
    fn get_library_inference_processors(&self) -> Vec<&'static str>;
    fn is_valid_inference_processor(&self, inference_processor: String) -> bool;
    fn get_library_model_versions(&self) -> Vec<&'static str>;
    fn is_valid_model_version(&self, model_version: String) -> bool;
    fn get_interface_version(&self) -> String;
}