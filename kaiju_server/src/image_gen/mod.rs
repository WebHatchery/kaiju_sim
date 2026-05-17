//! Image generation module using local ComfyUI.

pub mod comfy_client;
pub mod generation_service;
pub mod prompt_builder;

pub use comfy_client::ComfyClient;
pub use generation_service::{GenerationStatus, ImageGenerationService};
pub use prompt_builder::{build_kaiju_prompt, generate_negative_prompt};
