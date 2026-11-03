//! Persistent display preferences and the fullscreen confirmation trial.

use macroquad::prelude::set_fullscreen;
use macroquad_toolkit::settings::{DisplayPreview, GameSettings};

const GAME_NAME: &str = "kaiju_sim";

struct ActivePreview {
    candidate: GameSettings,
    preview: DisplayPreview,
}

pub struct DisplaySettings {
    committed: GameSettings,
    active_preview: Option<ActivePreview>,
}

impl DisplaySettings {
    pub fn load(capture_mode: bool) -> Self {
        let mut committed = GameSettings::load(GAME_NAME);
        if capture_mode {
            committed.fullscreen = false;
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Browsers require a user gesture before enabling fullscreen.
            committed.fullscreen = false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if committed.fullscreen && !capture_mode {
            set_fullscreen(true);
        }

        Self {
            committed,
            active_preview: None,
        }
    }

    pub fn fullscreen(&self) -> bool {
        self.active_preview
            .as_ref()
            .map(|preview| preview.candidate.fullscreen)
            .unwrap_or(self.committed.fullscreen)
    }

    pub fn begin_fullscreen_preview(&mut self) {
        if self.active_preview.is_some() {
            return;
        }
        let mut candidate = self.committed.clone();
        candidate.fullscreen = !candidate.fullscreen;
        let preview = DisplayPreview::begin(&self.committed, &candidate);
        self.active_preview = Some(ActivePreview { candidate, preview });
    }

    pub fn is_previewing(&self) -> bool {
        self.active_preview.is_some()
    }

    pub fn preview_remaining(&self) -> Option<f32> {
        self.active_preview
            .as_ref()
            .map(|active| active.preview.remaining())
    }

    /// Returns true after the toolkit restores the previous display on timeout.
    pub fn update(&mut self, real_dt: f32) -> bool {
        let expired = self
            .active_preview
            .as_mut()
            .map(|active| active.preview.update(real_dt))
            .unwrap_or(false);
        if expired {
            self.active_preview = None;
        }
        expired
    }

    pub fn keep_preview(&mut self) -> Result<(), String> {
        let Some(mut active) = self.active_preview.take() else {
            return Ok(());
        };
        if let Err(error) = active.candidate.save(GAME_NAME) {
            active.preview.revert();
            return Err(error);
        }
        active.preview.confirm();
        self.committed = active.candidate;
        Ok(())
    }

    pub fn revert_preview(&mut self) {
        if let Some(mut active) = self.active_preview.take() {
            active.preview.revert();
        }
    }
}
