pub mod bvr_box;
pub mod bvr_detection;
pub mod bvr_image;

pub const INTERFACE_VERSION: &str = "0.2.1";

use std::error::Error;
use anyhow;
use crate::bvr_detection::BvrDetection;
use crate::bvr_image::BvrImage;

pub trait BvrDetector {
    fn is_running(&self) -> impl std::future::Future<Output = anyhow<bool>> + Send;
    fn init_detector(&mut self, model_details: String) -> impl std::future::Future<Output = anyhow<()>> + Send;
    fn detect(&self, bvr_image: BvrImage) -> impl std::future::Future<Output = anyhow<Vec<BvrDetection>>> + Send;
    fn get_library_device_types(&self) -> Vec<&'static str>;
    fn is_valid_device_type(&self, device_type: String) -> bool;
    fn get_library_processing_types(&self) -> Vec<&'static str>;
    fn is_valid_processing_type(&self, processing_type: String) -> bool;
    fn get_interface_version(&self) -> String;
}