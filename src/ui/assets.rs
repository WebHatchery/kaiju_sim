//! Asset manager for loading and caching textures.

use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::Path;

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
        // Hardcoded generic loading for now based on known files (or scan if possible)
        // Since we can't easily glob async in macroquad without other crates,
        // we'll rely on specific known paths or the filesystem crate if available.
        // We can use std::fs::read_dir since this is a desktop app (not web).

        let sprite_dir = "assets/sprites/kaiju";
        if let Ok(entries) = std::fs::read_dir(sprite_dir) {
            for entry in entries.flatten() {
                if let Ok(path) = entry.path().into_os_string().into_string() {
                    if path.ends_with(".png") {
                        // Use filename as key
                        let filename = entry.file_name().to_string_lossy().to_string();
                        let key = filename.clone();
                        self.load_texture(&key, &path).await;
                    }
                }
            }
        } else {
            eprintln!("Failed to read sprite directory: {}", sprite_dir);
        }
    }

    /// Check cache and download if missing (Sync - Blocking)
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

    pub fn get_filename_from_url(&self, url: &str) -> String {
        let name = url.split('/').last().unwrap_or("");
        if name.is_empty() {
            "unknown.png".to_string()
        } else {
            name.to_string()
        }
    }
}
