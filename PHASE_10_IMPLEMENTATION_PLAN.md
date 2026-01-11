# Phase 10: Local Stable Diffusion Integration

## Goal
Integrate a local ComfyUI Stable Diffusion server to dynamically generate high-quality visuals for Kaiju based on their genetic traits. This allows for essentially infinite visual variation without storing thousands of pre-rendered sprites.

## Reference
- `image_gen.ps1`: Provided PowerShell script demonstrating ComfyUI API interaction (Queue Prompt, Wait, Download).

## Architecture

We will implement a `ImageGenerationService` in `kaiju_server` that communicates with a local ComfyUI instance (default `http://127.0.0.1:8188`).

### Workflow
1.  **Trigger**: When a Kaiju is born (breeding/hatching), a request is queued if no image exists.
2.  **Prompt Construction**: Convert Kaiju genetics (body type, element, visuals) into a text prompt.
    *   *Positive*: "anime style kaiju, {body_type}, {element} element, {visual_traits}, full body, white background"
    *   *Negative*: "bad hands, blurry, low quality, cropped"
3.  **ComfyUI Client**:
    *   Connect to WebSocket (optional) or poll `/history` to track progress.
    *   Send generation request via `/prompt`.
    *   Download resulting image via `/view`.
4.  **Storage**: Save generated images to `assets/sprites/kaiju/{kaiju_id}.png`.
5.  **Fallback**: Use default sprites if generation fails or server is offline.

## Proposed Changes

### 1. New Module: `kaiju_server/src/image_gen/`

#### `comfy_client.rs`
- Struct `ComfyClient`
- Method `queue_prompt(prompt: String, negative_prompt: String, seed: u64) -> Result<String>`
- Method `check_status(prompt_id: &str) -> Result<Option<ImageLocation>>`
- Method `download_image(filename: &str) -> Result<Vec<u8>>`
- Replicates logic from `image_gen.ps1`.

#### `prompt_builder.rs`
- Function `build_kaiju_prompt(kaiju: &Kaiju) -> (String, String)`
- Maps `BodyType` -> "bipedal monster", "serpentine dragon", etc.
- Maps `Element` -> "glowing fire aura", "ice crystals", etc.

#### `service.rs`
- Struct `ImageGenerationService`
- Manages the queue of kaiju waiting for images.
- Runs a background task to process the queue one by one.

### 2. Integration
- Update `kaiju_server/src/lib.rs` to export the new module.
- Update `kaiju_server` startup to initialize the service (optional, feature flagged).

## Asset Requirements
- Local ComfyUI instance running.
- Checkpoint model (e.g., `juggernautXL` or anime model) available in ComfyUI.

## Steps
1.  Create `kaiju_server/src/image_gen` structure.
2.  Implement `ComfyClient` (HTTP interaction).
3.  Implement `PromptBuilder` (Genetics to Text).
4.  Implement `ImageGenerationService` (Queue management).
5.  Verify with a test generation.
