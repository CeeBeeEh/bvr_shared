mod bvr_box;
mod bvr_detection;
mod bvr_image;

pub const INTERFACE_VERSION: &str = "0.2.1";

use std::error::Error;
use crate::bvr_detection::BvrDetection;
use crate::bvr_image::BvrImage;

pub type Result<T, E = dyn Error> = std::result::Result<T, E>;
pub trait BvrDetector {
    fn is_running(&self) -> impl std::future::Future<Output = Result<bool>> + Send;
    fn init_detector(&mut self, model_details: String) -> impl std::future::Future<Output = Result<()>> + Send;
    fn detect(&self, bvr_image: BvrImage) -> impl std::future::Future<Output = Result<Vec<BvrDetection>>> + Send;
    fn get_library_device_types(&self) -> Vec<&'static str>;
    fn is_valid_device_type(&self, device_type: String) -> bool;
    fn get_interface_version(&self) -> String;
}