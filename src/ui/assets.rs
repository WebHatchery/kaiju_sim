//! Asset manager for loading and caching textures.

use macroquad::prelude::*;
use std::collections::HashMap;

const STATIC_TEXTURES: &[(&str, &str)] = &[
    ("title_page", "assets/title/title_page.png"),
    (
        "kaiju_bipedal_neutral_1768091093175.png",
        "assets/sprites/kaiju/kaiju_bipedal_neutral_1768091093175.png",
    ),
    (
        "kaiju_electric_elemental_1768091175509.png",
        "assets/sprites/kaiju/kaiju_electric_elemental_1768091175509.png",
    ),
    (
        "kaiju_fire_elemental_1768091138860.png",
        "assets/sprites/kaiju/kaiju_fire_elemental_1768091138860.png",
    ),
    (
        "kaiju_ice_elemental_1768091156648.png",
        "assets/sprites/kaiju/kaiju_ice_elemental_1768091156648.png",
    ),
    (
        "kaiju_quadruped_neutral_1768091073894.png",
        "assets/sprites/kaiju/kaiju_quadruped_neutral_1768091073894.png",
    ),
    (
        "kaiju_serpentine_neutral_1768091108255.png",
        "assets/sprites/kaiju/kaiju_serpentine_neutral_1768091108255.png",
    ),
    (
        "1cc214d8-d89d-4771-b201-95e46e8be9f6.png",
        "assets/cache/1cc214d8-d89d-4771-b201-95e46e8be9f6.png",
    ),
    (
        "642f1b93-e520-4862-ad7e-d36437f10e81.png",
        "assets/cache/642f1b93-e520-4862-ad7e-d36437f10e81.png",
    ),
    (
        "e0bd86e8-6a58-48f6-8fe7-b9d9a039e236.png",
        "assets/cache/e0bd86e8-6a58-48f6-8fe7-b9d9a039e236.png",
    ),
    ("kaiju_bred_3.png", "assets/cache/kaiju_bred_3.png"),
];

pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
    placeholder: Option<Texture2D>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            placeholder: None,
        }
    }

    /// Load a texture from file asynchronously
    pub async fn load_texture(&mut self, key: &str, path: &str) {
        match load_texture(path).await {
            Ok(tex) => {
                tex.set_filter(FilterMode::Linear);
                self.textures.insert(key.to_string(), tex);
                println!("Loaded texture: {}", key);
            }
            Err(e) => {
                eprintln!("Failed to load texture {}: {}", path, e);
            }
        }
    }

    /// Get a texture by key
    pub fn get_texture(&self, key: &str) -> Option<&Texture2D> {
        self.textures.get(key).or(self.placeholder.as_ref())
    }

    /// Load all kaiju sprites from the assets directory
    pub async fn load_all_assets(&mut self) {
        for (key, path) in STATIC_TEXTURES {
            self.load_texture_if_missing(key, path).await;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.load_local_image_dir("assets/sprites/kaiju").await;
            self.load_local_image_dir("assets/cache").await;
        }
    }

    async fn load_texture_if_missing(&mut self, key: &str, path: &str) {
        if !self.textures.contains_key(key) {
            self.load_texture(key, path).await;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn load_local_image_dir(&mut self, image_dir: &str) {
        if let Ok(entries) = std::fs::read_dir(image_dir) {
            for entry in entries.flatten() {
                if let Ok(path) = entry.path().into_os_string().into_string() {
                    if path.ends_with(".png") {
                        let key = entry.file_name().to_string_lossy().to_string();
                        self.load_texture_if_missing(&key, &path).await;
                    }
                }
            }
        }
    }

    /// Check cache and download if missing (Sync - Blocking)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn download_if_missing(&self, url: &str) -> Option<String> {
        let filename = self.get_filename_from_url(url);
        if filename.is_empty() || filename == "unknown.png" {
            return None;
        }

        let cache_dir = "assets/cache";
        let path = format!("{}/{}", cache_dir, filename);
        let path_obj = std::path::Path::new(&path);

        if !path_obj.exists() {
            // Ensure cache directory exists
            if let Err(e) = std::fs::create_dir_all(cache_dir) {
                eprintln!("Failed to create cache dir: {}", e);
                return None;
            }

            // Handle relative URLs (assume server)
            let full_url = if url.starts_with("http") {
                url.to_string()
            } else {
                // Remove leading slash if present to avoid double slash
                let clean_url = url.trim_start_matches('/');
                format!("http://127.0.0.1:3000/{}", clean_url)
            };

            println!("Downloading asset: {} -> {}", full_url, path);
            match reqwest::blocking::get(&full_url) {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.bytes() {
                            Ok(bytes) => {
                                if let Err(e) = std::fs::write(&path, bytes) {
                                    eprintln!("Failed to write to cache: {}", e);
                                    return None;
                                }
                                println!("Asset cached successfully.");
                            }
                            Err(e) => {
                                eprintln!("Failed to get bytes: {}", e);
                                return None;
                            }
                        }
                    } else {
                        eprintln!("Failed to download asset: {}", response.status());
                        return None;
                    }
                }
                Err(e) => {
                    eprintln!("Network error downloading asset: {}", e);
                    return None;
                }
            }
        }

        Some(path)
    }

    /// Browser builds load remote assets directly instead of caching to disk.
    #[cfg(target_arch = "wasm32")]
    pub fn download_if_missing(&self, url: &str) -> Option<String> {
        if url.is_empty() {
            None
        } else {
            Some(url.to_string())
        }
    }

    pub fn get_filename_from_url(&self, url: &str) -> String {
        let clean = url
            .split('?')
            .next()
            .unwrap_or(url)
            .split('#')
            .next()
            .unwrap_or(url);
        let name = clean.rsplit(['/', '\\']).next().unwrap_or("");
        if name.is_empty() {
            "unknown.png".to_string()
        } else {
            name.to_string()
        }
    }
}
