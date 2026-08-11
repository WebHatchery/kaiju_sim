//! Asset manager for loading and caching textures.

use macroquad::prelude::*;
use macroquad_toolkit::assets::{AssetManager as ToolkitAssetManager, TextureConfig};

const TEXTURE_MANIFEST_JSON: &str =
    macroquad_toolkit::include_json_str!("../../assets/data/texture_manifest.json");
const ASSET_PACK_PATH: &str = "assets.zip";

pub struct AssetManager {
    inner: ToolkitAssetManager,
}

impl AssetManager {
    pub fn new() -> Self {
        let mut inner = ToolkitAssetManager::new();
        inner.set_default_filter(FilterMode::Linear);
        Self { inner }
    }

    /// Load a texture from file asynchronously
    pub async fn load_texture(&mut self, key: &str, path: &str) {
        match self
            .inner
            .load_texture_with_filter(key, path, FilterMode::Linear)
            .await
        {
            Ok(()) => println!("Loaded texture: {}", key),
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    /// Get a texture by key
    pub fn get_texture(&self, key: &str) -> Option<&Texture2D> {
        self.inner.get_texture(key)
    }

    /// Load all static textures from the toolkit texture manifest.
    pub async fn load_all_assets(&mut self) {
        let _ = self.inner.load_asset_pack(ASSET_PACK_PATH).await;
        let textures = match TextureConfig::from_json(TEXTURE_MANIFEST_JSON) {
            Ok(textures) => textures,
            Err(e) => {
                eprintln!("Failed to parse texture manifest: {}", e);
                return;
            }
        };

        let expected = textures.len();
        let loaded = self.inner.load_texture_configs(&textures).await;
        if loaded != expected {
            eprintln!(
                "Loaded {}/{} textures from texture manifest",
                loaded, expected
            );
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
