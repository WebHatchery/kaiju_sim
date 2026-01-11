# Phase 7 Implementation Plan: Visual & Audio Polish

Based on my exploration of the Kaiju Breeding Simulator codebase, I've created a comprehensive implementation plan for Phase 7: Visual & Audio Polish. This phase focuses on bringing the game to life with procedural visual generation, battle animations, particle effects, and audio system preparation.

## Overview

Phase 7 transforms the functional game (from Phases 1-6) into a polished, visually engaging experience. The core design principle is **deterministic visual generation from genetic data** - every kaiju's appearance is reproducible from its genome and visual seed, ensuring consistency across platforms and sessions.

## 1. Seed-Based Kaiju Appearance Generation

### 1.1 Visual Seed System

From GENOME_ENCODING_SPEC.md, the visual seed is a 64-bit value constructed from:
- **Bits 224-239 of genome** (16-bit derivative stored on-chain)
- **Parent A hash** (16 bits)
- **Parent B hash** (16 bits)
- **Breeding seed hash** (16 bits)

**Implementation Requirements:**

**File: `src/engine/visual_gen.rs`**
- Construct full 64-bit visual seed from genome + parent data
- Deterministic RNG seeded with ChaCha8Rng for reproducibility
- Visual seed inheritance during breeding (combine parent seeds)

```rust
pub struct VisualSeed {
    genome_derivative: u16,  // From genome bits 224-239
    parent_a_hash: u16,
    parent_b_hash: u16,
    breeding_seed_hash: u16,
}

pub fn construct_full_visual_seed(
    genome: &Genome,
    parents: &ParentData,
    breeding_seed: u64
) -> u64;
```

### 1.2 Procedural Appearance Generation

**File: `src/engine/appearance.rs`**

Generate deterministic kaiju appearance from:
- Visual seed (RNG source)
- Decoded traits (mandatory visual features)
- Generation number (complexity scaling)
- Stat distribution (body proportions hint)

**Appearance Components:**
```rust
pub struct Appearance {
    pub body_type: BodyType,           // Quadruped, bipedal, serpentine, etc.
    pub size_category: SizeCategory,   // Based on HP stat
    pub features: Vec<VisualFeature>,  // Wings, horns, spikes, etc.
    pub color_scheme: ColorScheme,     // Primary, secondary, accent colors
    pub texture_pattern: Pattern,      // Scales, fur, armor, smooth
    pub scale_factor: f32,             // 0.9-1.1 variance
}
```

**Body Type Selection:**
- Based on stat distribution (high speed = leaner, high defense = bulky)
- Generation influences complexity (Gen 0-2: simple, Gen 3-5: moderate, Gen 6+: complex)
- RNG determines from weighted list

**Feature Mapping (Trait → Visual):**
From TRAIT_SYSTEM_DESIGN.md, traits with mandatory visual requirements:
- "Wings" trait → wing feature MUST appear
- "Electric Breath" → electricity visual effects around mouth/body
- "Armored Scales" → plated/armored texture
- "Aqua Hide" → aquatic shimmer, water droplets
- Fire traits → flame/heat aura
- Ice traits → frost/crystal formations

### 1.3 Color Scheme Generation

**File: `src/engine/color_gen.rs`**

Deterministic color generation from visual seed + traits:

```rust
pub struct ColorScheme {
    pub primary: Color,      // Base body color
    pub secondary: Color,    // Accent/belly color
    pub tertiary: Color,     // Detail color (eyes, claws)
    pub glow: Option<Color>, // Element-based glow
}
```

**Color Rules:**
- Element traits influence color palette (Electric = blue/yellow, Fire = red/orange, Ice = cyan/white)
- Multiple elements blend colors
- Generation affects saturation (higher gen = more vibrant)
- Visual seed determines hue variation within element palette

## 2. Sprite/Portrait Rendering System

### 2.1 Placeholder Sprite Generation

**Phase 7 Focus:** Geometric/abstract representation (AI generation is Phase 10)

**File: `src/engine/sprite_gen.rs`**

Generate simple but distinctive kaiju sprites:
- **256x256px** base resolution (from UI_UX_SPECIFICATION.md)
- Layered composition:
  1. Body silhouette (determined by body type)
  2. Feature overlays (wings, horns, etc.)
  3. Color fill
  4. Texture pattern
  5. Glow/aura effects

**Rendering Approach:**
```rust
pub fn render_kaiju_sprite(
    appearance: &Appearance,
    size: (f32, f32)
) -> Image {
    let mut canvas = Image::gen_image_color(256, 256, TRANSPARENT);

    // Layer 1: Body silhouette
    draw_body_shape(&mut canvas, appearance.body_type, appearance.color_scheme.primary);

    // Layer 2: Features
    for feature in &appearance.features {
        draw_feature(&mut canvas, feature, appearance.color_scheme.secondary);
    }

    // Layer 3: Texture pattern
    apply_texture_pattern(&mut canvas, appearance.texture_pattern);

    // Layer 4: Glow effects
    if let Some(glow_color) = appearance.color_scheme.glow {
        apply_glow(&mut canvas, glow_color);
    }

    canvas
}
```

### 2.2 Texture Caching

**File: `src/ui/texture_cache.rs`**

Cache generated sprites to avoid regeneration:
```rust
pub struct TextureCache {
    cache: HashMap<KaijuId, Texture2D>,
    max_size: usize,
}

impl TextureCache {
    pub fn get_or_generate(&mut self, kaiju: &Kaiju) -> &Texture2D;
    pub fn invalidate(&mut self, id: KaijuId);
    pub fn clear(&mut self);
}
```

## 3. Battle Animations System

### 3.1 Turn Animation Framework

**File: `src/engine/battle_animations.rs`**

Animate turn-by-turn combat from battle logs:

```rust
pub struct BattleAnimator {
    current_turn: usize,
    animation_state: AnimationState,
    turn_duration: f32,  // Seconds per turn
}

pub enum AnimationState {
    Idle,
    Attacking { progress: f32 },
    TakingDamage { progress: f32 },
    Victory,
    Defeat,
}
```

**Turn Animation Sequence:**
1. **Idle** (0.5s) - Both kaiju in neutral stance
2. **Wind-up** (0.3s) - Attacker moves forward/charges
3. **Strike** (0.2s) - Attack animation plays
4. **Impact** (0.1s) - Screen shake, damage number spawn
5. **Recoil** (0.3s) - Defender takes damage, HP bar updates
6. **Return** (0.4s) - Attacker returns to position
7. **Next Turn** - Repeat

### 3.2 Attack Animations

**Simple Attack Types:**
- **Melee**: Lunge forward, slash/bite
- **Ranged Element**: Projectile from mouth/body
- **AOE Effect**: Area pulse/wave
- **Status**: Buff/debuff visual indicator

**Trait-Specific Animations:**
- Electric traits: Lightning bolt projectile
- Fire traits: Flame breath cone
- Ice traits: Frost beam
- Water traits: Water jet
- Physical traits: Claw swipe/tail whip

### 3.3 Damage Numbers

**File: `src/ui/damage_numbers.rs`**

Floating damage numbers with easing:

```rust
pub struct DamageNumber {
    value: i32,
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
    max_lifetime: f32,
    color: Color,
}

impl DamageNumber {
    pub fn update(&mut self, dt: f32);
    pub fn draw(&self);
}
```

**Visual Properties:**
- Font size scales with damage (bigger = more damage)
- Color coding: Normal (white), Critical (yellow), Healing (green)
- Easing: Float upward with decay
- Lifetime: 1.5 seconds

## 4. Trait Activation Effects

### 4.1 Visual Effect System

**File: `src/engine/visual_effects.rs`**

Trait-specific visual feedback:

```rust
pub struct VisualEffect {
    pub effect_type: EffectType,
    pub position: Vec2,
    pub duration: f32,
    pub color: Color,
}

pub enum EffectType {
    ElementalBurst { element: Element },
    StatBuff { stat: StatType },
    EnvironmentReaction,
    SynergyActivation,
}
```

**Effect Examples:**
- **Electric Breath activates**: Blue lightning bolts around kaiju
- **Armored Scales**: Metallic shield pulse
- **Regeneration**: Green healing aura
- **Berserker (low HP)**: Red rage effect
- **Environment synergy**: Matching color flash (storm + electric = bright blue pulse)

### 4.2 Effect Rendering

Use simple geometric shapes + alpha blending:
- Circle pulses for AOE
- Line segments for lightning
- Particle sprays for elemental bursts
- Color overlays for stat buffs

## 5. Environment Backdrops

### 5.1 Environment Assets

**File: `assets/environments.json`**

Define visual properties for each environment (from COMBAT_SYSTEM_SPEC.md):

```json
{
  "environments": [
    {
      "id": "neutral",
      "name": "Neutral Arena",
      "background_color": "#2a2a3a",
      "floor_color": "#3a3a4a",
      "ambient_particles": null
    },
    {
      "id": "storm",
      "name": "Storm Arena",
      "background_color": "#1a1a2a",
      "floor_color": "#2a2a3a",
      "ambient_particles": "rain",
      "effect_color": "#4488ff"
    }
  ]
}
```

### 5.2 Background Rendering

**File: `src/screens/battle_background.rs`**

Simple but effective background rendering:

```rust
pub fn draw_environment_background(env: &Environment, screen_bounds: Rect) {
    // Gradient background
    draw_gradient(screen_bounds, env.background_color, darker_variant);

    // Floor
    draw_rectangle(floor_rect, env.floor_color);

    // Ambient effects
    if let Some(particles) = env.ambient_particles {
        draw_ambient_particles(particles);
    }

    // Environment-specific overlays
    match env.id {
        "storm" => draw_lightning_flashes(),
        "volcanic" => draw_lava_glow(),
        "aquatic" => draw_water_surface(),
        // ...
    }
}
```

## 6. Particle Effects System

### 6.1 Particle Engine

**File: `src/engine/particles.rs`**

Lightweight particle system for effects:

```rust
pub struct ParticleSystem {
    particles: Vec<Particle>,
    emitters: Vec<ParticleEmitter>,
}

pub struct Particle {
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
    max_lifetime: f32,
    size: f32,
    color: Color,
}

pub struct ParticleEmitter {
    position: Vec2,
    emit_rate: f32,
    particle_lifetime: f32,
    velocity_range: (Vec2, Vec2),
    color: Color,
    active: bool,
}
```

### 6.2 Particle Effect Types

**Combat Effects:**
- Hit sparks (impact particles)
- Elemental bursts (fire, ice, electric)
- Blood/damage spray (optional, can be abstract)
- Dust clouds (movement)

**UI Effects:**
- Breeding success sparkles
- Level up glow
- Selection highlights
- Transition particles

**Environment Effects:**
- Rain (storm)
- Embers (volcanic)
- Snow (tundra)
- Leaves (forest)

### 6.3 Performance Optimization

**Particle Pooling:**
```rust
pub struct ParticlePool {
    pool: Vec<Particle>,
    active_count: usize,
    max_particles: usize,
}

impl ParticlePool {
    pub fn spawn(&mut self, config: ParticleConfig) -> Option<&mut Particle>;
    pub fn recycle(&mut self, index: usize);
    pub fn update_all(&mut self, dt: f32);
}
```

**Performance Targets:**
- Max 500 particles simultaneously
- Cull particles off-screen
- Use simple shapes (circles, squares) instead of sprites
- Alpha blending for smooth effects

## 7. Audio System Preparation (Future)

### 7.1 Audio Architecture (Stub Implementation)

**File: `src/engine/audio.rs`**

Prepare structure for Phase 7+ audio integration:

```rust
pub struct AudioManager {
    enabled: bool,
    sfx_volume: f32,
    music_volume: f32,
}

pub enum SoundEffect {
    BattleHit,
    BattleKO,
    UIClick,
    BreedingSuccess,
    TraitActivation,
    EnvironmentAmbient,
}

impl AudioManager {
    pub fn play_sfx(&self, effect: SoundEffect) {
        // Stub for now - implement in future phase
    }

    pub fn play_music(&self, track: &str) {
        // Stub for now
    }
}
```

### 7.2 Sound Effect Mapping

**File: `assets/audio_config.json`**

Define sound mappings (even if not implemented yet):

```json
{
  "sfx": {
    "battle_hit": "assets/sfx/hit.ogg",
    "battle_electric": "assets/sfx/electric_zap.ogg",
    "battle_fire": "assets/sfx/flame_burst.ogg",
    "ui_click": "assets/sfx/click.ogg",
    "breeding_success": "assets/sfx/success.ogg"
  },
  "music": {
    "menu": "assets/music/menu_theme.ogg",
    "laboratory": "assets/music/lab_theme.ogg",
    "battle": "assets/music/battle_theme.ogg"
  }
}
```

## 8. Performance Optimization

### 8.1 Rendering Optimization

**Batching:**
- Group sprite draws by texture
- Minimize state changes
- Use instancing for particles

**Culling:**
- Off-screen particle culling
- Frustum culling for large battles
- LOD system for distant/background elements

**Frame Budget:**
- Target: 60 FPS (16.67ms per frame)
- Budget allocation:
  - Game logic: 4ms
  - Rendering: 10ms
  - UI: 2ms
  - Buffer: 0.67ms

### 8.2 Memory Management

**Texture Management:**
```rust
pub struct TextureManager {
    loaded_textures: HashMap<String, Texture2D>,
    lru_cache: VecDeque<String>,
    max_cache_size: usize,
}

impl TextureManager {
    pub fn load_or_get(&mut self, path: &str) -> &Texture2D;
    pub fn evict_lru(&mut self);
}
```

**Memory Targets:**
- WebGL: <100MB total
- Native: <500MB total
- Sprite cache: Max 50 kaiju sprites (256x256 = ~256KB each)

### 8.3 WebGL Optimization

**WASM Bundle Size:**
- Optimize build with `--release`
- Use `wasm-opt` for additional compression
- Lazy-load textures
- Compress JSON data

**Canvas Rendering:**
- Use `pixelated` image rendering for crisp sprites
- Disable antialiasing where appropriate
- Use `requestAnimationFrame` timing

## 9. Testing & Quality Assurance

### 9.1 Visual Consistency Tests

**Determinism Verification:**
```rust
#[test]
fn test_visual_seed_determinism() {
    let genome = create_test_genome();
    let seed1 = construct_full_visual_seed(&genome, &parents, 12345);
    let seed2 = construct_full_visual_seed(&genome, &parents, 12345);
    assert_eq!(seed1, seed2);
}

#[test]
fn test_appearance_reproducibility() {
    let appearance1 = generate_appearance(&genome, visual_seed, &traits);
    let appearance2 = generate_appearance(&genome, visual_seed, &traits);
    assert_eq!(appearance1, appearance2);
}
```

### 9.2 Animation Testing

**Manual Test Cases:**
- Battle with 10+ turns (animation stability)
- Rapid battles (memory leaks?)
- All environment types (visual correctness)
- All trait activation types (effect triggers)

### 9.3 Performance Profiling

**Benchmark Targets:**
- Sprite generation: <5ms per kaiju
- Particle update: <2ms for 500 particles
- Battle animation frame: <16ms (60 FPS)
- Texture loading: <50ms per texture

## 10. Integration with Existing Phases

### 10.1 Phase 2 Integration (Genetics)

From breeding system, extract visual seed:
```rust
fn generate_offspring_genome(...) -> Genome {
    // ... existing breeding logic ...

    let visual_derivative = derive_visual_seed(
        parent_a.visual_seed_derivative,
        parent_b.visual_seed_derivative,
        breeding_seed
    );

    genome.with_visual_seed(visual_derivative)
}
```

### 10.2 Phase 3 Integration (Combat)

Battle simulator triggers visual effects:
```rust
fn execute_turn(attacker: &Kaiju, defender: &Kaiju, env: &Environment) -> TurnResult {
    let damage = calculate_damage(attacker, defender, env);

    // Trigger trait activation effects
    for trait in &attacker.traits {
        if trait.category == TraitCategory::Element {
            spawn_element_effect(trait.name, attacker.position);
        }
    }

    // Spawn damage number
    spawn_damage_number(damage, defender.position);

    TurnResult { damage, effects: collected_effects }
}
```

### 10.3 Phase 6 Integration (UI)

UI components use appearance system:
```rust
pub fn draw_kaiju_card(kaiju: &Kaiju, position: Vec2, texture_cache: &mut TextureCache) {
    let texture = texture_cache.get_or_generate(kaiju);
    draw_texture(texture, position.x, position.y, WHITE);

    // Draw stat bars, trait badges, etc.
}
```

## 11. Asset Pipeline

### 11.1 Asset Structure

```
assets/
├── sprites/
│   ├── placeholder/          # Geometric placeholder sprites
│   │   ├── body_types/       # Body silhouettes
│   │   └── features/         # Wing, horn, spike overlays
│   └── particles/            # Particle textures (16x16)
├── environments/
│   ├── neutral.png
│   ├── storm.png
│   └── ... (other environments)
├── ui/
│   ├── panels/
│   └── icons/
├── audio/                    # Future
│   ├── sfx/
│   └── music/
└── config/
    ├── environments.json
    ├── visual_features.json
    └── particle_configs.json
```

### 11.2 Data Files

**`assets/config/visual_features.json`:**
```json
{
  "body_types": [
    {
      "id": "quadruped",
      "name": "Quadruped",
      "weight": 30,
      "stat_affinity": {"speed": 1.2, "defense": 0.9}
    },
    {
      "id": "bipedal",
      "name": "Bipedal",
      "weight": 25,
      "stat_affinity": {"attack": 1.1, "hp": 1.0}
    },
    {
      "id": "serpentine",
      "name": "Serpentine",
      "weight": 20,
      "stat_affinity": {"speed": 1.3, "defense": 0.8}
    }
  ],
  "features": [
    {
      "id": "wings",
      "required_by_trait": "Wings",
      "size_scale": 1.5,
      "attachment_point": "back"
    },
    {
      "id": "horns",
      "required_by_trait": null,
      "size_scale": 0.8,
      "attachment_point": "head"
    }
  ]
}
```

**`assets/config/particle_configs.json`:**
```json
{
  "hit_spark": {
    "count": 20,
    "lifetime": 0.3,
    "velocity_min": [-50, -50],
    "velocity_max": [50, 50],
    "size": 4.0,
    "color": "#ffffff"
  },
  "electric_burst": {
    "count": 30,
    "lifetime": 0.5,
    "velocity_min": [-80, -80],
    "velocity_max": [80, 80],
    "size": 6.0,
    "color": "#4488ff"
  }
}
```

---

## Critical Files for Implementation

Based on this implementation plan, here are the 5 most critical files to create:

- **H:\RustGames\kaiju_sim\src\engine\visual_gen.rs** - Core visual seed construction and appearance generation from genome data. This is the foundation that connects genetics to visuals.

- **H:\RustGames\kaiju_sim\src\engine\sprite_gen.rs** - Procedural sprite/portrait rendering system using layered composition (body + features + colors + effects). Handles geometric placeholder generation.

- **H:\RustGames\kaiju_sim\src\engine\battle_animations.rs** - Turn-by-turn animation system that brings combat to life with attack sequences, damage numbers, and trait activation effects.

- **H:\RustGames\kaiju_sim\src\engine\particles.rs** - Lightweight particle system for combat effects, environmental ambience, and UI feedback. Includes pooling for performance.

- **H:\RustGames\kaiju_sim\assets\config\visual_features.json** - Data-driven configuration for body types, visual features, color schemes, and particle effects. Allows tweaking without recompilation per CODE_STANDARDS.md requirements.
