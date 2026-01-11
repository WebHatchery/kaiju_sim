# PHASE 6 IMPLEMENTATION PLAN: User Interface

**Version**: 1.0
**Phase**: 6 of 10
**Dependencies**: Phase 1 (Data Models), Phase 5 (State Management)
**Parallel Work**: Can be developed alongside Phase 2-4 (core systems)
**Target**: Complete UI implementation using macroquad-toolkit
**Estimated Time**: 3-4 weeks

---

## 1. Overview

### 1.1 Scope

This phase implements the complete user interface for Kaiju Breeding Simulator using macroquad-toolkit. The UI consists of 10 screens, 7 core components, a comprehensive navigation system, and full accessibility features.

### 1.2 Design Philosophy

- **Information-Dense but Scannable**: Complex data presented clearly
- **Progressive Disclosure**: Hide complexity until relevant
- **Consequence Clarity**: Make risk/reward obvious
- **Legacy Persistence**: Death and history are first-class visual elements
- **Immediate Mode UI**: No retained state, pure functions that read game state and return actions

### 1.3 Key Deliverables

1. **Component Library**: 7 reusable UI components
2. **Screen Implementations**: 10 complete game screens
3. **Navigation System**: State-based navigation with history
4. **UiAction System**: Type-safe action dispatch
5. **Color & Typography**: Consistent theming
6. **Keyboard Shortcuts**: Full keyboard navigation
7. **Accessibility**: High contrast, screen reader support
8. **Performance**: 60 FPS target with optimization

---

## 2. Implementation Strategy

### 2.1 Development Order

**Phase 6A: Foundation (Week 1)**
1. UI Core: Colors, typography, spacing constants
2. UiAction enum and dispatch system
3. Base component implementations (button wrappers, panels)

**Phase 6B: Core Components (Week 1-2)**
4. KaijuCard component
5. StatBar component
6. TraitBadge component
7. BattleLog component

**Phase 6C: Essential Screens (Week 2)**
8. MainMenu screen
9. Laboratory hub screen
10. RosterView screen
11. KaijuDetailView modal

**Phase 6D: Gameplay Screens (Week 3)**
12. BreedingScreen
13. TournamentLobby
14. BattleView
15. ResultsScreen

**Phase 6E: Meta Screens (Week 3-4)**
16. LeaderboardScreen
17. LineageViewer

**Phase 6F: Polish & Testing (Week 4)**
18. Animations and transitions
19. Keyboard navigation
20. Performance optimization
21. Integration testing

### 2.2 Parallel Development Approach

UI development can proceed alongside core systems (Phase 2-4) using:
- **Mock data generators** for testing components
- **Stub implementations** for game engine calls
- **Placeholder assets** for images and sprites

---

## 3. File Structure

### 3.1 Complete Directory Layout

```
kaiju_sim/
├── src/
│   ├── main.rs                      # Entry point, game loop
│   ├── ui/
│   │   ├── mod.rs                   # Re-exports all UI modules
│   │   ├── actions.rs               # UiAction enum (250 lines)
│   │   ├── colors.rs                # Extended color palette (100 lines)
│   │   ├── typography.rs            # Font size constants (50 lines)
│   │   ├── spacing.rs               # Layout spacing system (50 lines)
│   │   ├── components/
│   │   │   ├── mod.rs               # Component re-exports
│   │   │   ├── kaiju_card.rs        # KaijuCard widget (200 lines)
│   │   │   ├── stat_bar.rs          # StatBar widget (150 lines)
│   │   │   ├── trait_badge.rs       # TraitBadge widget (180 lines)
│   │   │   ├── battle_log.rs        # BattleLog scroller (200 lines)
│   │   │   ├── lineage_tree.rs      # LineageTree renderer (300 lines)
│   │   │   ├── tournament_bracket.rs # Bracket visualization (250 lines)
│   │   │   └── progress_bar.rs      # Progress indicators (100 lines)
│   │   └── utils/
│   │       ├── mod.rs               # Utility re-exports
│   │       ├── layout.rs            # Layout helpers (150 lines)
│   │       ├── animation.rs         # Animation interpolation (100 lines)
│   │       └── modal.rs             # Modal dialog wrapper (150 lines)
│   ├── screens/
│   │   ├── mod.rs                   # Screen re-exports
│   │   ├── main_menu.rs             # MainMenu screen (200 lines)
│   │   ├── laboratory.rs            # Laboratory hub (300 lines)
│   │   ├── roster_view.rs           # RosterView screen (350 lines)
│   │   ├── kaiju_detail.rs          # KaijuDetailView modal (400 lines)
│   │   ├── breeding.rs              # BreedingScreen (450 lines)
│   │   ├── tournament_lobby.rs      # TournamentLobby (400 lines)
│   │   ├── battle_view.rs           # BattleView (500 lines)
│   │   ├── results.rs               # ResultsScreen (350 lines)
│   │   ├── leaderboard.rs           # LeaderboardScreen (300 lines)
│   │   └── lineage_viewer.rs        # LineageViewer (400 lines)
│   └── state/
│       └── navigation.rs            # Navigation state (150 lines)
```

**Total Estimated Lines**: ~5,500 lines of UI code

---

## 4. Detailed Implementation Plans

### 4.1 UI Core System (`src/ui/`)

#### 4.1.1 Actions Module (`src/ui/actions.rs`)

**Purpose**: Define all user intents as type-safe enum

**Implementation**:
```rust
/// Master UI action enum returned by all UI components
#[derive(Debug, Clone)]
pub enum UiAction {
    // Navigation
    GoToMenu,
    GoToLaboratory,
    GoToRoster,
    GoToBreeding,
    GoToResearch,
    GoToLineage,
    GoToTournament,
    GoToLeaderboard,
    GoToSettings,
    Back,

    // Kaiju Management
    SelectKaiju(KaijuId),
    DeselectKaiju(KaijuId),
    ViewKaijuDetails(KaijuId),
    CompareKaiju(Vec<KaijuId>),
    ExportKaiju(KaijuId),

    // Breeding
    SelectParentA(KaijuId),
    SelectParentB(KaijuId),
    PreviewOffspring,
    ConfirmBreeding,
    CancelBreeding,

    // Tournament
    EnterTournament(TournamentId, KaijuId),
    ViewTournamentOdds(TournamentId),
    WatchTournament(TournamentId),
    ViewTournamentBracket(TournamentId),

    // Battle
    SkipBattle,
    PauseBattle,
    ResumeBattle,
    ViewBattleReplay(BattleId),

    // Research
    ResearchTrait(KaijuId, TraitId),
    UpgradeFacility(FacilityType),
    AnalyzeGenome(KaijuId),

    // Lineage
    ExpandLineageNode(KaijuId),
    CollapseLineageNode(KaijuId),
    ExportLineage(KaijuId),
    ViewFullLineageTree(KaijuId),

    // Leaderboard
    FilterLeaderboard(LeaderboardFilter),
    ViewHallOfFame,
    ViewMyKaiju,
    RefreshLeaderboard,

    // System
    SaveGame,
    LoadGame,
    ToggleFullscreen,
    OpenHelp,
    ExitGame,
}

#[derive(Debug, Clone)]
pub enum LeaderboardFilter {
    AllTime,
    ThisSeason,
    ThisMonth,
    ByGeneration(u32),
    LethalOnly,
    RankedOnly,
    MyBloodline,
}

#[derive(Debug, Clone, Copy)]
pub enum FacilityType {
    ResearchLab,
    BreedingChamber,
    TrainingArena,
}
```

**Testing Strategy**:
- Unit tests for enum serialization (if needed for save/load)
- Integration tests for action dispatch in main loop

---

#### 4.1.2 Colors Module (`src/ui/colors.rs`)

**Purpose**: Extended color palette beyond macroquad-toolkit defaults

**Implementation**:
```rust
use macroquad::prelude::Color;

/// Extended dark theme colors for Kaiju Sim
pub mod dark {
    use super::*;

    // Re-export toolkit colors
    pub use macroquad_toolkit::colors::dark::*;

    // Extended semantic colors
    pub const ALIVE: Color = POSITIVE;
    pub const DEAD: Color = Color::from_rgba(100, 100, 110, 255);
    pub const IN_BATTLE: Color = WARNING;

    // Trait category colors
    pub const ELEMENT: Color = Color::from_rgba(66, 133, 244, 255);   // Blue
    pub const MODIFIER: Color = Color::from_rgba(156, 39, 176, 255);  // Purple
    pub const MUTATION: Color = Color::from_rgba(244, 67, 54, 255);   // Red
    pub const SYNERGY: Color = Color::from_rgba(76, 175, 80, 255);    // Green
    pub const HIDDEN: Color = Color::from_rgba(80, 80, 90, 255);      // Dark grey

    // Tournament types
    pub const RANKED: Color = Color::from_rgba(255, 193, 7, 255);     // Gold
    pub const LETHAL: Color = Color::from_rgba(211, 47, 47, 255);     // Dark Red
    pub const SPECIAL: Color = Color::from_rgba(123, 31, 162, 255);   // Deep Purple

    // Stat colors
    pub const HP_COLOR: Color = Color::from_rgba(76, 175, 80, 255);   // Green
    pub const ATK_COLOR: Color = Color::from_rgba(244, 67, 54, 255);  // Red
    pub const DEF_COLOR: Color = Color::from_rgba(33, 150, 243, 255); // Blue
    pub const SPD_COLOR: Color = Color::from_rgba(255, 193, 7, 255);  // Yellow
}

/// Get color for trait category
pub fn trait_category_color(category: TraitCategory) -> Color {
    match category {
        TraitCategory::Element => dark::ELEMENT,
        TraitCategory::Modifier => dark::MODIFIER,
        TraitCategory::Mutation => dark::MUTATION,
        TraitCategory::Synergy => dark::SYNERGY,
    }
}

/// Get color for tournament type
pub fn tournament_type_color(tourney_type: TournamentType) -> Color {
    match tourney_type {
        TournamentType::Ranked => dark::RANKED,
        TournamentType::Lethal => dark::LETHAL,
        TournamentType::Special => dark::SPECIAL,
    }
}

/// Get color for stat type
pub fn stat_color(stat_type: StatType) -> Color {
    match stat_type {
        StatType::HP => dark::HP_COLOR,
        StatType::Attack => dark::ATK_COLOR,
        StatType::Defense => dark::DEF_COLOR,
        StatType::Speed => dark::SPD_COLOR,
    }
}
```

---

#### 4.1.3 Typography Module (`src/ui/typography.rs`)

**Purpose**: Font size and text style constants

**Implementation**:
```rust
/// Font size constants
pub const FONT_SIZE_TINY: f32 = 12.0;       // Metadata, footnotes
pub const FONT_SIZE_SMALL: f32 = 14.0;      // Body text, descriptions
pub const FONT_SIZE_NORMAL: f32 = 16.0;     // Default UI text
pub const FONT_SIZE_MEDIUM: f32 = 20.0;     // Subheadings, buttons
pub const FONT_SIZE_LARGE: f32 = 28.0;      // Section headers
pub const FONT_SIZE_TITLE: f32 = 40.0;      // Screen titles
pub const FONT_SIZE_HERO: f32 = 60.0;       // Main menu title

/// Draw text with shadow for emphasis
pub fn draw_text_with_shadow(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
) {
    // Shadow
    draw_text(text, x + 2.0, y + 2.0, font_size, Color::from_rgba(0, 0, 0, 150));
    // Text
    draw_text(text, x, y, font_size, color);
}

/// Simulate bold by drawing text twice (offset by 1px)
pub fn draw_text_bold(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
) {
    draw_text(text, x, y, font_size, color);
    draw_text(text, x + 1.0, y, font_size, color);
}

/// Measure text dimensions (wrapper around macroquad)
pub fn measure_text_dims(text: &str, font_size: f32) -> (f32, f32) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    (dims.width, dims.height)
}
```

---

#### 4.1.4 Spacing Module (`src/ui/spacing.rs`)

**Purpose**: Consistent layout spacing system

**Implementation**:
```rust
/// Spacing constants (8px grid system)
pub const SPACING_TINY: f32 = 4.0;
pub const SPACING_SMALL: f32 = 8.0;
pub const SPACING_NORMAL: f32 = 16.0;
pub const SPACING_MEDIUM: f32 = 24.0;
pub const SPACING_LARGE: f32 = 32.0;
pub const SPACING_HUGE: f32 = 48.0;

/// Border radius constants
pub const RADIUS_SMALL: f32 = 4.0;   // Buttons
pub const RADIUS_MEDIUM: f32 = 8.0;  // Cards
pub const RADIUS_LARGE: f32 = 12.0;  // Panels

/// Standard component sizes
pub const BUTTON_HEIGHT: f32 = 40.0;
pub const CARD_WIDTH: f32 = 180.0;
pub const CARD_HEIGHT: f32 = 240.0;
pub const PANEL_HEADER_HEIGHT: f32 = 50.0;
```

---

### 4.2 Component Library (`src/ui/components/`)

#### 4.2.1 KaijuCard Component (`src/ui/components/kaiju_card.rs`)

**Purpose**: Compact kaiju display card (180x240px)

**Features**:
- Portrait image (180x100px)
- Name and generation
- Trait icons (visible only)
- 4 stat bars (HP, ATK, DEF, SPD)
- Rank and level footer
- Select/View buttons
- State highlighting (normal/hovered/selected/dead/in_battle)

**API**:
```rust
pub enum CardState {
    Normal,
    Hovered,
    Selected,
    Dead,
    InBattle,
}

pub enum CardAction {
    Select,
    View,
}

/// Draw a kaiju card at (x, y)
/// Returns Some(action) if user interacted
pub fn draw_kaiju_card(
    x: f32,
    y: f32,
    kaiju: &Kaiju,
    state: CardState,
) -> Option<CardAction> {
    // Implementation details:
    // 1. Draw background rectangle with state-based border
    // 2. Draw portrait (or placeholder)
    // 3. Draw name + generation
    // 4. Draw trait icon badges (max 3 visible)
    // 5. Draw 4 stat bars
    // 6. Draw footer (rank #, level)
    // 7. Draw buttons: [Select] [View]
    // 8. Handle hover/click detection
    // 9. Return action if clicked
}
```

**Visual States**:
- Normal: White 2px border
- Hovered: Accent color 3px border, 2px shadow
- Selected: Thick accent border (4px)
- Dead: Greyscale filter, "DECEASED" ribbon
- InBattle: Yellow border, "IN BATTLE" badge

**Testing**:
- Visual regression tests with screenshots
- Interaction tests for hover/click zones
- Dead kaiju rendering test

---

#### 4.2.2 StatBar Component (`src/ui/components/stat_bar.rs`)

**Purpose**: Horizontal progress bar for stats

**Features**:
- Label (HP/ATK/DEF/SPD)
- Filled progress bar
- Current/max value display
- Color-coded by stat type
- Animated transitions (0.3s lerp)

**API**:
```rust
pub enum StatType {
    HP,
    Attack,
    Defense,
    Speed,
}

pub struct StatBarState {
    current: f32,
    previous: f32,
    animation_progress: f32,
}

impl StatBarState {
    pub fn new(value: i32) -> Self { /* ... */ }
    pub fn update(&mut self, new_value: i32, delta_time: f32) { /* animate */ }
    pub fn current_display_value(&self) -> f32 { /* lerp */ }
}

/// Draw a stat bar with label and value
pub fn draw_stat_bar(
    x: f32,
    y: f32,
    width: f32,
    stat_type: StatType,
    current: i32,
    max: i32,
    state: &mut StatBarState,
) {
    // Implementation:
    // 1. Update animation state
    // 2. Draw label
    // 3. Draw background bar
    // 4. Draw filled portion (with lerp)
    // 5. Draw value text
}
```

**Animation Logic**:
- When value changes, animate over 0.3s
- Use `lerp(previous, current, progress)`
- Damage flashes red overlay
- Healing flashes green overlay

---

#### 4.2.3 TraitBadge Component (`src/ui/components/trait_badge.rs`)

**Purpose**: Pill-shaped trait indicator with tooltip

**Features**:
- Icon + text label
- Color-coded by category
- Hover tooltip with full description
- Hidden trait rendering ("??")
- Inheritance indicator (optional)

**API**:
```rust
pub enum TraitDisplayMode {
    Visible { name: String, icon: String, category: TraitCategory },
    PartiallyRevealed { category: TraitCategory, hint: String },
    CompletelyHidden { count: u32 },
}

pub struct TooltipData {
    pub trait_name: String,
    pub category: String,
    pub power: i32,
    pub description: String,
    pub inherited_from: Option<String>,
}

/// Draw trait badge with optional tooltip
/// Returns true if clicked (for detail view)
pub fn draw_trait_badge(
    x: f32,
    y: f32,
    mode: &TraitDisplayMode,
    tooltip: Option<&TooltipData>,
    hover_timer: &mut f32,
) -> bool {
    // Implementation:
    // 1. Calculate badge dimensions based on text
    // 2. Draw rounded rectangle background (category color)
    // 3. Draw icon (if visible)
    // 4. Draw text label
    // 5. Check hover state
    // 6. If hovered for 0.5s, draw tooltip
    // 7. Return true if clicked
}

/// Draw tooltip near cursor
fn draw_tooltip(x: f32, y: f32, data: &TooltipData) {
    // Draw panel with trait details
}
```

**Tooltip Positioning**:
- Default: 20px below and right of cursor
- If near screen edge, flip to opposite side
- Max width: 300px
- Auto-height based on content

---

#### 4.2.4 BattleLog Component (`src/ui/components/battle_log.rs`)

**Purpose**: Scrollable turn-by-turn combat log

**Features**:
- Auto-scroll to latest entry
- Color-coded event types
- Icons for special events
- Copy log button
- Scroll bar when content exceeds height

**API**:
```rust
pub enum BattleLogEntry {
    Damage { attacker: String, defender: String, damage: i32 },
    Healing { target: String, amount: i32 },
    TraitActivation { kaiju: String, trait_name: String },
    Critical { attacker: String, damage: i32 },
    Environmental { effect: String },
    Victory { winner: String, hp_remaining: i32 },
}

pub struct BattleLogState {
    pub entries: Vec<BattleLogEntry>,
    pub scroll_offset: f32,
    pub auto_scroll: bool,
}

impl BattleLogState {
    pub fn add_entry(&mut self, entry: BattleLogEntry) {
        self.entries.push(entry);
        if self.auto_scroll {
            self.scroll_to_bottom();
        }
    }

    pub fn scroll_to_bottom(&mut self) { /* ... */ }
}

/// Draw scrollable battle log
pub fn draw_battle_log(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    state: &mut BattleLogState,
) -> Option<BattleLogAction> {
    // Implementation:
    // 1. Draw background panel
    // 2. Calculate visible entries based on scroll
    // 3. Draw each entry with color coding
    // 4. Draw scroll bar if needed
    // 5. Handle scroll input (mouse wheel)
    // 6. Return copy action if copy button clicked
}
```

**Color Coding**:
- Damage: Red text
- Healing: Green text
- Trait activation: Blue text with icon
- Critical: Yellow text, larger font
- Environmental: Cyan text
- Victory: Bold white text

---

#### 4.2.5 LineageTree Component (`src/ui/components/lineage_tree.rs`)

**Purpose**: Recursive tree visualization for ancestry

**Features**:
- Horizontal layout (left to right)
- Collapsible nodes
- Lines connecting parents to children
- Node highlighting (current/champion/dead)
- Click to expand/collapse
- Zoom controls

**API**:
```rust
pub struct LineageNode {
    pub kaiju_id: KaijuId,
    pub name: String,
    pub generation: u32,
    pub is_alive: bool,
    pub is_champion: bool,
    pub is_current: bool,
    pub children: Vec<LineageNode>,
    pub expanded: bool,
}

pub struct LineageTreeState {
    pub root: LineageNode,
    pub zoom: f32,
    pub pan_offset: (f32, f32),
}

pub enum LineageTreeAction {
    ExpandNode(KaijuId),
    CollapseNode(KaijuId),
    SelectNode(KaijuId),
    ZoomIn,
    ZoomOut,
    ResetView,
}

/// Draw lineage tree with pan/zoom support
pub fn draw_lineage_tree(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    state: &mut LineageTreeState,
) -> Option<LineageTreeAction> {
    // Implementation:
    // 1. Apply zoom and pan transforms
    // 2. Recursively draw nodes and connections
    // 3. Draw node boxes with kaiju info
    // 4. Handle click detection for expand/collapse
    // 5. Handle mouse wheel for zoom
    // 6. Handle drag for pan
    // 7. Return action if interaction occurred
}

fn draw_node(
    x: f32,
    y: f32,
    node: &LineageNode,
    state: &LineageTreeState,
) -> (f32, f32) {
    // Draw single node box
    // Returns (width, height) for layout calculations
}
```

**Visual Style**:
- Node box: 120x60px
- Connection lines: 2px solid
- Current kaiju: Accent border (4px)
- Champions: Gold highlight
- Dead kaiju: Grey with tombstone icon
- Collapsed indicator: "+" icon

---

#### 4.2.6 TournamentBracket Component (`src/ui/components/tournament_bracket.rs`)

**Purpose**: Single-elimination bracket visualization

**Features**:
- Automatic layout calculation
- Match state indicators (pending/in_progress/completed)
- Death markers for lethal tournaments
- Clickable matches for details

**API**:
```rust
pub struct BracketMatch {
    pub match_id: MatchId,
    pub round: u32,
    pub kaiju_a: Option<(KaijuId, String)>,
    pub kaiju_b: Option<(KaijuId, String)>,
    pub winner: Option<KaijuId>,
    pub status: MatchStatus,
    pub death_occurred: bool,
}

pub enum MatchStatus {
    Pending,
    InProgress,
    Completed,
}

pub struct BracketState {
    pub matches: Vec<BracketMatch>,
    pub num_rounds: u32,
}

/// Draw tournament bracket
pub fn draw_tournament_bracket(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    state: &BracketState,
) -> Option<MatchId> {
    // Implementation:
    // 1. Calculate round positions
    // 2. Calculate match positions within rounds
    // 3. Draw bracket lines connecting matches
    // 4. Draw match boxes with kaiju names
    // 5. Highlight winner, strike-through loser
    // 6. Draw death markers if lethal
    // 7. Return match_id if clicked
}
```

**Layout Algorithm**:
- Evenly space rounds horizontally
- Stack matches vertically within each round
- Draw lines from previous round winners to next match

---

#### 4.2.7 ProgressBar Component (`src/ui/components/progress_bar.rs`)

**Purpose**: Generic progress indicator (XP, breeding, research)

**API**:
```rust
pub struct ProgressBarStyle {
    pub bg_color: Color,
    pub fill_color: Color,
    pub border_color: Color,
    pub show_percentage: bool,
    pub show_values: bool,
}

/// Draw progress bar with optional label
pub fn draw_progress_bar(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    current: f32,
    max: f32,
    style: &ProgressBarStyle,
    label: Option<&str>,
) {
    // Implementation:
    // 1. Draw background
    // 2. Calculate fill width
    // 3. Draw filled portion
    // 4. Draw border
    // 5. Draw label (if provided)
    // 6. Draw percentage or values (if enabled)
}
```

---

### 4.3 Utility Modules (`src/ui/utils/`)

#### 4.3.1 Layout Helpers (`src/ui/utils/layout.rs`)

**Purpose**: Common layout calculations

**API**:
```rust
/// Center element horizontally
pub fn center_x(element_width: f32) -> f32 {
    (screen_width() - element_width) / 2.0
}

/// Center element vertically
pub fn center_y(element_height: f32) -> f32 {
    (screen_height() - element_height) / 2.0
}

/// Calculate grid layout positions
pub struct GridLayout {
    pub x: f32,
    pub y: f32,
    pub cols: usize,
    pub cell_width: f32,
    pub cell_height: f32,
    pub spacing: f32,
}

impl GridLayout {
    pub fn cell_position(&self, index: usize) -> (f32, f32) {
        let col = index % self.cols;
        let row = index / self.cols;
        let x = self.x + col as f32 * (self.cell_width + self.spacing);
        let y = self.y + row as f32 * (self.cell_height + self.spacing);
        (x, y)
    }
}

/// Vertical stack layout helper
pub struct VStack {
    pub x: f32,
    pub current_y: f32,
    pub spacing: f32,
}

impl VStack {
    pub fn new(x: f32, start_y: f32, spacing: f32) -> Self { /* ... */ }

    pub fn next(&mut self, height: f32) -> f32 {
        let y = self.current_y;
        self.current_y += height + self.spacing;
        y
    }
}
```

---

#### 4.3.2 Animation Helpers (`src/ui/utils/animation.rs`)

**Purpose**: Interpolation and animation utilities

**API**:
```rust
/// Linear interpolation
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t.clamp(0.0, 1.0)
}

/// Ease-in-out interpolation
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

/// Animated value that lerps to target
pub struct AnimatedValue {
    pub current: f32,
    pub target: f32,
    pub speed: f32,  // units per second
}

impl AnimatedValue {
    pub fn new(initial: f32) -> Self { /* ... */ }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn update(&mut self, delta_time: f32) {
        let diff = self.target - self.current;
        let step = self.speed * delta_time;
        if diff.abs() < step {
            self.current = self.target;
        } else {
            self.current += step * diff.signum();
        }
    }

    pub fn is_complete(&self) -> bool {
        (self.current - self.target).abs() < 0.01
    }
}
```

---

#### 4.3.3 Modal Dialog Wrapper (`src/ui/utils/modal.rs`)

**Purpose**: Standardized modal rendering

**API**:
```rust
pub struct ModalStyle {
    pub width: f32,
    pub height: f32,
    pub title: String,
    pub closeable: bool,
}

/// Draw modal background overlay
pub fn draw_modal_overlay() {
    draw_rectangle(
        0.0, 0.0,
        screen_width(), screen_height(),
        Color::from_rgba(0, 0, 0, 180),
    );
}

/// Calculate centered modal position
pub fn modal_position(width: f32, height: f32) -> (f32, f32) {
    (center_x(width), center_y(height))
}

/// Draw modal frame with title bar
pub fn draw_modal_frame(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    title: &str,
) -> bool {
    // Returns true if close button clicked
}
```

---

### 4.4 Screen Implementations (`src/screens/`)

#### 4.4.1 MainMenu Screen (`src/screens/main_menu.rs`)

**Purpose**: Entry point and game mode selection

**Layout**:
- Large title (centered)
- 5 buttons (200x50px, vertically stacked)
- Version info (bottom-left)
- Connection status (bottom-right)

**API**:
```rust
pub struct MainMenuScreen;

impl MainMenuScreen {
    pub fn draw(game_state: &GameState) -> Option<UiAction> {
        // Implementation:
        // 1. Draw background
        // 2. Draw title with hero font
        // 3. Draw buttons using VStack
        // 4. Draw version info
        // 5. Draw connection status
        // 6. Return action if button clicked
    }
}
```

**Buttons**:
- New Laboratory → `UiAction::GoToLaboratory` (new game)
- Continue Game → `UiAction::LoadGame`
- Settings → `UiAction::GoToSettings`
- Leaderboards → `UiAction::GoToLeaderboard`
- Credits → (show credits modal)

---

#### 4.4.2 Laboratory Screen (`src/screens/laboratory.rs`)

**Purpose**: Central hub for all activities

**Layout**:
- Tab navigation (top)
- 3-4 kaiju cards in grid
- Activity feed (scrollable)
- Resource display (top-right)
- Back button

**API**:
```rust
pub struct LaboratoryScreen {
    activity_scroll: f32,
}

impl LaboratoryScreen {
    pub fn new() -> Self { /* ... */ }

    pub fn draw(
        &mut self,
        game_state: &GameState,
    ) -> Option<UiAction> {
        // Implementation:
        // 1. Draw tab bar
        // 2. Draw featured kaiju cards (3-4)
        // 3. Draw activity feed
        // 4. Draw resource panel
        // 5. Return action based on interaction
    }
}
```

**Tabs**:
- Roster
- Breed
- Research
- Lineage
- Tournament

---

#### 4.4.3 RosterView Screen (`src/screens/roster_view.rs`)

**Purpose**: Browse and manage kaiju collection

**Layout**:
- Search box + filter dropdown (top)
- Kaiju card grid (3-4 per row, scrollable)
- Selection checkboxes
- Action buttons (bottom)

**API**:
```rust
pub struct RosterViewScreen {
    search_query: String,
    filter: RosterFilter,
    scroll_offset: f32,
    selected_kaiju: Vec<KaijuId>,
}

pub enum RosterFilter {
    All,
    Alive,
    Dead,
    Generation(u32),
    Element(ElementType),
}

impl RosterViewScreen {
    pub fn new() -> Self { /* ... */ }

    pub fn draw(
        &mut self,
        game_state: &GameState,
    ) -> Option<UiAction> {
        // Implementation:
        // 1. Draw search box
        // 2. Draw filter dropdown
        // 3. Calculate visible kaiju based on scroll
        // 4. Draw kaiju cards in grid
        // 5. Draw selection indicators
        // 6. Draw action buttons
        // 7. Handle scroll input
        // 8. Return action
    }
}
```

**Actions**:
- Card click → ViewKaijuDetails
- Select → Toggle selection
- Compare Selected → CompareKaiju
- Export → ExportRoster

---

#### 4.4.4 KaijuDetailView Modal (`src/screens/kaiju_detail.rs`)

**Purpose**: Full kaiju stats, history, and management

**Layout**:
- Portrait (left, 180x180px)
- Stats panel (center-left)
- Traits panel (center-right)
- Experience bar
- Battle history (scrollable)
- Action buttons (bottom)

**API**:
```rust
pub struct KaijuDetailModal {
    kaiju_id: KaijuId,
    history_scroll: f32,
    selected_trait: Option<TraitId>,
}

impl KaijuDetailModal {
    pub fn new(kaiju_id: KaijuId) -> Self { /* ... */ }

    pub fn draw(
        &mut self,
        game_state: &GameState,
    ) -> Option<UiAction> {
        // Implementation:
        // 1. Draw modal overlay
        // 2. Draw modal frame
        // 3. Draw kaiju portrait
        // 4. Draw stats with stat bars
        // 5. Draw trait badges
        // 6. Draw experience progress bar
        // 7. Draw battle history list
        // 8. Draw action buttons
        // 9. Handle trait tooltip
        // 10. Return action or close
    }
}
```

**Sections**:
- Header: Name, generation, token ID
- Portrait: AI-generated or placeholder
- Stats: 4 stat bars with values
- Traits: Visible badges + hidden count
- Experience: Level + progress bar
- History: Last 5 battles
- Actions: [Breed] [Tournament] [Lineage]

---

#### 4.4.5 BreedingScreen (`src/screens/breeding.rs`)

**Purpose**: Select parents and breed offspring

**Layout**:
- Parent A card (left)
- Offspring preview (center)
- Parent B card (right)
- Compatibility meter
- Inheritance rates
- Cost display
- Validation messages
- Confirm/Cancel buttons

**API**:
```rust
pub struct BreedingScreen {
    parent_a: Option<KaijuId>,
    parent_b: Option<KaijuId>,
    preview_valid: bool,
    offspring_preview: Option<OffspringPreview>,
}

pub struct OffspringPreview {
    pub generation: u32,
    pub stat_ranges: StatRanges,
    pub trait_probabilities: Vec<(TraitId, f32)>,
    pub mutation_chance: f32,
}

impl BreedingScreen {
    pub fn new() -> Self { /* ... */ }

    pub fn update_preview(&mut self, game_state: &GameState) {
        // Calculate offspring preview based on selected parents
    }

    pub fn draw(
        &mut self,
        game_state: &GameState,
    ) -> Option<UiAction> {
        // Implementation:
        // 1. Draw parent A card (clickable to change)
        // 2. Draw parent B card (clickable to change)
        // 3. Draw offspring preview panel
        // 4. Draw compatibility meter
        // 5. Draw trait inheritance probabilities
        // 6. Draw breeding cost
        // 7. Draw validation warnings
        // 8. Draw confirm/cancel buttons
        // 9. Return action
    }
}
```

**Validation Rules** (displayed):
- Both parents selected
- Both alive
- No parent-child breeding
- Player owns breeding rights
- Sufficient resources

---

#### 4.4.6 TournamentLobby Screen (`src/screens/tournament_lobby.rs`)

**Purpose**: Browse and enter tournaments

**Layout**:
- Filter dropdown (top-right)
- Refresh button
- Tournament card list (scrollable)
- "My Entries" button

**API**:
```rust
pub struct TournamentLobbyScreen {
    filter: TournamentFilter,
    scroll_offset: f32,
    selected_tournament: Option<TournamentId>,
}

pub enum TournamentFilter {
    All,
    Ranked,
    Lethal,
    Special,
    MyGeneration,
}

impl TournamentLobbyScreen {
    pub fn new() -> Self { /* ... */ }

    pub fn draw(
        &mut self,
        game_state: &GameState,
    ) -> Option<UiAction> {
        // Implementation:
        // 1. Draw filter dropdown
        // 2. Draw refresh button
        // 3. Draw tournament cards
        // 4. Draw entry buttons
        // 5. Draw death warnings (lethal)
        // 6. Draw countdown timers
        // 7. Return action
    }
}
```

**Tournament Card Info**:
- Type badge (Ranked/Lethal/Special)
- Entry count / max
- Prize pool
- Restrictions
- Status + countdown
- Death warning (if lethal)
- [Enter] [Odds] buttons

**Entry Confirmation Modal**:
- Kaiju selection dropdown
- Tournament details
- Death warning (if lethal)
- Win chance estimate
- "Type CONFIRM" text box
- [Submit] [Cancel]

---

#### 4.4.7 BattleView Screen (`src/screens/battle_view.rs`)

**Purpose**: Display auto-battle simulation

**Layout**:
- Kaiju sprites (left and right)
- HP bars (below sprites)
- Damage numbers (floating)
- Trait activation notifications
- Battle log (bottom panel)
- Environment display (background)
- Skip/Pause buttons

**API**:
```rust
pub struct BattleViewScreen {
    battle_id: BattleId,
    current_turn: usize,
    animation_state: BattleAnimationState,
    log_state: BattleLogState,
    paused: bool,
}

pub struct BattleAnimationState {
    turn_progress: f32,  // 0.0 to 1.0
    damage_numbers: Vec<FloatingDamageNumber>,
    trait_flashes: Vec<TraitFlash>,
}

pub struct FloatingDamageNumber {
    value: i32,
    x: f32,
    y: f32,
    lifetime: f32,
    alpha: f32,
}

impl BattleViewScreen {
    pub fn new(battle_id: BattleId) -> Self { /* ... */ }

    pub fn update(&mut self, delta_time: f32, battle_log: &[BattleLogEntry]) {
        // Advance turn animation
        // Update floating damage numbers
        // Update trait activation flashes
    }

    pub fn draw(
        &mut self,
        game_state: &GameState,
    ) -> Option<UiAction> {
        // Implementation:
        // 1. Draw environment background
        // 2. Draw kaiju sprites
        // 3. Draw HP bars
        // 4. Draw floating damage numbers
        // 5. Draw trait activation effects
        // 6. Draw battle log panel
        // 7. Draw skip/pause buttons
        // 8. Return action
    }
}
```

**Animation Timing**:
- Turn length: 1.5s
- Damage number display: 0.5s fade
- HP bar transition: 0.3s lerp
- Trait flash: 0.8s

---

#### 4.4.8 ResultsScreen (`src/screens/results.rs`)

**Purpose**: Show battle outcome and rewards

**Layout**:
- Victory/Defeat banner
- Battle summary panel
- Rewards panel
- Battle analysis panel
- Action buttons

**API**:
```rust
pub struct ResultsScreen {
    battle_result: BattleResult,
    death_occurred: bool,
}

pub struct BattleResult {
    pub winner: KaijuId,
    pub loser: KaijuId,
    pub winner_hp_remaining: i32,
    pub turns: u32,
    pub rewards: Rewards,
    pub analysis: Vec<String>,
}

pub struct Rewards {
    pub xp: u32,
    pub gold: u32,
    pub rank_change: i32,
    pub level_up: bool,
}

impl ResultsScreen {
    pub fn new(battle_result: BattleResult) -> Self { /* ... */ }

    pub fn draw(
        &mut self,
        game_state: &GameState,
    ) -> Option<UiAction> {
        // Implementation:
        // 1. Draw victory/defeat banner
        // 2. Draw battle summary
        // 3. Draw rewards with animations
        // 4. Draw battle analysis hints
        // 5. Draw action buttons
        // 6. Handle death display (if lethal)
        // 7. Return action
    }
}
```

**Death Result** (Lethal Tournament):
- "IN MEMORIAM" banner
- Kaiju name, generation, token ID
- Born/Died dates
- Legacy stats (wins, offspring, peak rank)
- Breeding rights refund
- [View Hall of Fame] button

---

#### 4.4.9 LeaderboardScreen (`src/screens/leaderboard.rs`)

**Purpose**: Global and category rankings

**Layout**:
- Filter dropdowns (top)
- Leaderboard table (scrollable)
- Player highlights
- Death markers
- Action buttons (bottom)

**API**:
```rust
pub struct LeaderboardScreen {
    filter_time: TimeFilter,
    filter_generation: GenerationFilter,
    scroll_offset: f32,
}

pub enum TimeFilter {
    AllTime,
    ThisSeason,
    ThisMonth,
}

pub enum GenerationFilter {
    All,
    Gen1to3,
    Gen4to6,
    Gen7Plus,
}

impl LeaderboardScreen {
    pub fn new() -> Self { /* ... */ }

    pub fn draw(
        &mut self,
        game_state: &GameState,
    ) -> Option<UiAction> {
        // Implementation:
        // 1. Draw filter dropdowns
        // 2. Draw table header (Rank, Name, Gen, Owner, W/L)
        // 3. Draw leaderboard rows
        // 4. Highlight player's kaiju
        // 5. Draw death markers
        // 6. Draw pagination
        // 7. Draw action buttons
        // 8. Return action
    }
}
```

**Table Columns**:
- Rank (1-1000)
- Kaiju name
- Generation
- Owner wallet (truncated)
- Win/Loss record
- Status icon (alive/dead)

**Special Indicators**:
- Top 3: Medal icons (gold/silver/bronze)
- Player's kaiju: Highlighted row
- Dead kaiju: Grey with tombstone

---

#### 4.4.10 LineageViewer Screen (`src/screens/lineage_viewer.rs`)

**Purpose**: Visualize kaiju ancestry

**Layout**:
- Lineage tree (center, scrollable)
- Notable ancestors panel (right)
- Trait inheritance path panel (bottom)
- Zoom controls (top-right)
- Action buttons (bottom)

**API**:
```rust
pub struct LineageViewerScreen {
    root_kaiju: KaijuId,
    tree_state: LineageTreeState,
    selected_ancestor: Option<KaijuId>,
}

impl LineageViewerScreen {
    pub fn new(root_kaiju: KaijuId) -> Self { /* ... */ }

    pub fn draw(
        &mut self,
        game_state: &GameState,
    ) -> Option<UiAction> {
        // Implementation:
        // 1. Draw lineage tree component
        // 2. Draw notable ancestors list
        // 3. Draw trait inheritance paths
        // 4. Draw zoom controls
        // 5. Draw action buttons
        // 6. Handle tree interaction
        // 7. Return action
    }
}
```

**Notable Ancestors**:
- Champions (tournament winners)
- Died in tournament (with context)
- Trait originators

**Trait Inheritance Path**:
- Show path from root to ancestors
- Highlight where each visible trait came from

---

### 4.5 Navigation System (`src/state/navigation.rs`)

**Purpose**: Manage screen transitions and history

**API**:
```rust
#[derive(Debug, Clone)]
pub enum Screen {
    MainMenu,
    Laboratory,
    Roster,
    Breeding,
    TournamentLobby,
    Battle { battle_id: BattleId },
    Results { battle_id: BattleId },
    Leaderboard,
    LineageViewer { kaiju_id: KaijuId },

    // Modals (overlay on current screen)
    KaijuDetail { kaiju_id: KaijuId },
    ConfirmBreeding,
    ConfirmTournamentEntry { tournament_id: TournamentId },
}

pub struct NavigationState {
    current_screen: Screen,
    modal_stack: Vec<Screen>,  // For overlays
    history: Vec<Screen>,
}

impl NavigationState {
    pub fn new() -> Self {
        Self {
            current_screen: Screen::MainMenu,
            modal_stack: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn push_screen(&mut self, screen: Screen) {
        self.history.push(self.current_screen.clone());
        self.current_screen = screen;
    }

    pub fn push_modal(&mut self, modal: Screen) {
        self.modal_stack.push(modal);
    }

    pub fn pop_modal(&mut self) -> Option<Screen> {
        self.modal_stack.pop()
    }

    pub fn back(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            self.current_screen = prev;
            true
        } else {
            false
        }
    }

    pub fn handle_action(&mut self, action: UiAction) {
        match action {
            UiAction::GoToMenu => self.push_screen(Screen::MainMenu),
            UiAction::GoToLaboratory => self.push_screen(Screen::Laboratory),
            UiAction::GoToRoster => self.push_screen(Screen::Roster),
            UiAction::ViewKaijuDetails(id) => self.push_modal(Screen::KaijuDetail { kaiju_id: id }),
            UiAction::Back => { self.back(); },
            // ... handle all actions
            _ => {}
        }
    }
}
```

**History Management**:
- Max history depth: 10 screens
- Modal stack doesn't affect history
- ESC key pops modal or goes back

---

### 4.6 Keyboard Shortcuts

**Implementation** (`src/ui/keyboard.rs`):
```rust
pub fn handle_keyboard_shortcuts(
    nav_state: &mut NavigationState,
) -> Option<UiAction> {
    if is_key_pressed(KeyCode::Escape) {
        return Some(UiAction::Back);
    }

    if is_key_pressed(KeyCode::F5) {
        return Some(UiAction::RefreshLeaderboard);
    }

    if is_key_pressed(KeyCode::F11) {
        return Some(UiAction::ToggleFullscreen);
    }

    // Screen-specific shortcuts
    match nav_state.current_screen {
        Screen::Laboratory => {
            if is_key_pressed(KeyCode::B) {
                return Some(UiAction::GoToBreeding);
            }
            if is_key_pressed(KeyCode::T) {
                return Some(UiAction::GoToTournament);
            }
            if is_key_pressed(KeyCode::R) {
                return Some(UiAction::GoToRoster);
            }
        }
        Screen::Battle { .. } => {
            if is_key_pressed(KeyCode::Space) {
                return Some(UiAction::SkipBattle);
            }
        }
        _ => {}
    }

    None
}
```

**Full Shortcut List**:
- `ESC` - Back/Close modal
- `F5` - Refresh data
- `F11` - Toggle fullscreen
- `F1` - Help overlay
- `B` - Breeding (from Lab)
- `T` - Tournament (from Lab)
- `R` - Roster (from Lab)
- `Space` - Skip/Pause battle
- `1-9` - Quick select (context-dependent)

---

### 4.7 Accessibility Features

#### 4.7.1 High Contrast Mode (`src/ui/accessibility.rs`)

**Implementation**:
```rust
pub struct AccessibilitySettings {
    pub high_contrast: bool,
    pub large_text: bool,
    pub reduce_motion: bool,
}

impl AccessibilitySettings {
    pub fn apply_high_contrast_colors(&self) -> ColorPalette {
        if self.high_contrast {
            ColorPalette {
                background: BLACK,
                text: WHITE,
                accent: Color::from_rgba(255, 255, 0, 255),  // Yellow
                positive: Color::from_rgba(0, 255, 0, 255),  // Bright green
                negative: Color::from_rgba(255, 0, 0, 255),  // Bright red
                // ... all other colors with max contrast
            }
        } else {
            ColorPalette::default_dark()
        }
    }

    pub fn font_scale(&self) -> f32 {
        if self.large_text {
            1.25  // 25% larger text
        } else {
            1.0
        }
    }
}
```

#### 4.7.2 Screen Reader Support (Future)

**Placeholder Implementation**:
```rust
pub struct ScreenReaderHint {
    pub element_type: String,
    pub text: String,
    pub state: String,
}

pub fn generate_screen_reader_hints(
    screen: &Screen,
    game_state: &GameState,
) -> Vec<ScreenReaderHint> {
    // Generate semantic labels for UI elements
    // Will be used when screen reader API is available
    vec![]
}
```

#### 4.7.3 Colorblind-Friendly Trait Icons

**Strategy**:
- All trait badges use icon + text (not color alone)
- Distinct shapes for different categories
- Patterns (stripes, dots) in addition to colors

---

### 4.8 Performance Optimization

#### 4.8.1 Render Culling

**Implementation**:
```rust
pub fn is_visible_in_viewport(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    scroll_offset: f32,
    viewport_height: f32,
) -> bool {
    let adjusted_y = y - scroll_offset;
    adjusted_y + height >= 0.0 && adjusted_y <= viewport_height
}

// Usage in roster view
for (index, kaiju) in roster.iter().enumerate() {
    let (x, y) = grid.cell_position(index);
    if is_visible_in_viewport(x, y, CARD_WIDTH, CARD_HEIGHT, scroll_offset, screen_height()) {
        draw_kaiju_card(x, y, kaiju, CardState::Normal);
    }
}
```

#### 4.8.2 Text Caching

**Implementation**:
```rust
use std::collections::HashMap;

pub struct TextCache {
    cache: HashMap<(String, u16), TextDimensions>,
}

impl TextCache {
    pub fn measure_cached(&mut self, text: &str, font_size: f32) -> TextDimensions {
        let key = (text.to_string(), font_size as u16);
        *self.cache.entry(key).or_insert_with(|| {
            measure_text(text, None, font_size as u16, 1.0)
        })
    }
}
```

#### 4.8.3 Animation Frame Budget

**Monitoring**:
```rust
pub struct PerformanceMonitor {
    frame_times: Vec<f32>,
    animation_count: usize,
    particle_count: usize,
}

impl PerformanceMonitor {
    pub fn update(&mut self, delta_time: f32) {
        self.frame_times.push(delta_time);
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }
    }

    pub fn average_fps(&self) -> f32 {
        let avg_frame_time = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
        1.0 / avg_frame_time
    }

    pub fn should_reduce_effects(&self) -> bool {
        self.average_fps() < 45.0 || self.animation_count > 10
    }
}
```

**Limits**:
- Max 100 active particles
- Max 10 simultaneous animations
- Reduce effects if FPS < 45

---

## 5. Testing Strategy

### 5.1 Unit Tests

**Component Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_bar_animation() {
        let mut state = StatBarState::new(100);
        state.set_target(80);
        state.update(0.1);  // 100ms

        assert!(state.current_display_value() < 100.0);
        assert!(state.current_display_value() > 80.0);
    }

    #[test]
    fn test_grid_layout_positions() {
        let grid = GridLayout {
            x: 10.0,
            y: 10.0,
            cols: 3,
            cell_width: 100.0,
            cell_height: 150.0,
            spacing: 10.0,
        };

        assert_eq!(grid.cell_position(0), (10.0, 10.0));
        assert_eq!(grid.cell_position(1), (120.0, 10.0));
        assert_eq!(grid.cell_position(3), (10.0, 170.0));
    }
}
```

### 5.2 Integration Tests

**Screen Navigation Tests**:
```rust
#[test]
fn test_navigation_flow() {
    let mut nav = NavigationState::new();

    nav.handle_action(UiAction::GoToLaboratory);
    assert_eq!(nav.current_screen, Screen::Laboratory);

    nav.handle_action(UiAction::GoToBreeding);
    assert_eq!(nav.current_screen, Screen::Breeding);

    assert!(nav.back());
    assert_eq!(nav.current_screen, Screen::Laboratory);
}
```

### 5.3 Visual Regression Tests

**Strategy**:
- Capture screenshots of each screen
- Compare against baseline images
- Detect unintended visual changes

**Tools** (future):
- `image` crate for screenshot comparison
- Manual review for new features

### 5.4 Accessibility Tests

**Checklist**:
- [ ] All text meets 4.5:1 contrast ratio
- [ ] All interactive elements have 44px+ hit targets
- [ ] Keyboard navigation reaches all functions
- [ ] No color-only information (icons + text)
- [ ] Focus indicators visible

---

## 6. Integration with Game Systems

### 6.1 Data Flow

**UI reads game state, returns actions**:
```rust
// In main.rs game loop
let ui_action = match nav_state.current_screen {
    Screen::MainMenu => main_menu::draw(&game_state),
    Screen::Laboratory => laboratory::draw(&game_state),
    Screen::Roster => roster_view.draw(&game_state),
    Screen::Breeding => breeding_screen.draw(&game_state),
    // ... other screens
};

if let Some(action) = ui_action {
    handle_ui_action(action, &mut game_state, &mut nav_state);
}
```

### 6.2 Action Handling

**Implementation** (`src/main.rs`):
```rust
fn handle_ui_action(
    action: UiAction,
    game_state: &mut GameState,
    nav_state: &mut NavigationState,
) {
    match action {
        // Navigation
        UiAction::GoToLaboratory => nav_state.push_screen(Screen::Laboratory),
        UiAction::Back => { nav_state.back(); },

        // Breeding
        UiAction::ConfirmBreeding => {
            if let Some(result) = breeding_engine::breed(
                game_state.breeding.parent_a,
                game_state.breeding.parent_b,
            ) {
                game_state.roster.push(result);
                nav_state.push_screen(Screen::Laboratory);
            }
        },

        // Tournament
        UiAction::EnterTournament(t_id, k_id) => {
            tournament_engine::enter(t_id, k_id, game_state);
            nav_state.push_screen(Screen::TournamentLobby);
        },

        // ... handle all actions
        _ => {}
    }
}
```

---

## 7. Mock Data for Development

### 7.1 Mock Kaiju Generator

**Implementation** (`src/dev/mock_data.rs`):
```rust
#[cfg(debug_assertions)]
pub mod mock {
    use super::*;

    pub fn generate_mock_kaiju(id: usize) -> Kaiju {
        Kaiju {
            id: KaijuId(id),
            name: format!("TestKaiju{}", id),
            generation: (id % 10) as u32,
            stats: KaijuStats {
                hp: 200 + (id * 10) as i32,
                attack: 50 + (id * 2) as i32,
                defense: 30 + (id * 2) as i32,
                speed: 20 + (id * 3) as i32,
            },
            traits: vec![
                Trait::mock_electric(),
                Trait::mock_aquatic(),
            ],
            hidden_traits: vec![],
            alive: id % 5 != 0,  // Every 5th is dead
            experience: (id * 100) as u32,
            // ... other fields with mock data
        }
    }

    pub fn generate_mock_roster(count: usize) -> Vec<Kaiju> {
        (0..count).map(generate_mock_kaiju).collect()
    }
}
```

### 7.2 Stub Engine Functions

**Implementation** (`src/dev/stubs.rs`):
```rust
#[cfg(debug_assertions)]
pub mod stubs {
    use super::*;

    pub fn stub_breeding_preview(
        _parent_a: KaijuId,
        _parent_b: KaijuId,
    ) -> OffspringPreview {
        OffspringPreview {
            generation: 5,
            stat_ranges: StatRanges::default(),
            trait_probabilities: vec![],
            mutation_chance: 0.1,
        }
    }

    pub fn stub_battle_simulation(
        _kaiju_a: KaijuId,
        _kaiju_b: KaijuId,
    ) -> BattleResult {
        BattleResult {
            winner: _kaiju_a,
            loser: _kaiju_b,
            winner_hp_remaining: 120,
            turns: 8,
            rewards: Rewards::default(),
            analysis: vec!["Electric traits were effective".to_string()],
        }
    }
}
```

---

## 8. Development Milestones

### Week 1: Foundation
- [ ] UI core modules (colors, typography, spacing)
- [ ] UiAction enum complete
- [ ] MainMenu screen
- [ ] Navigation system
- [ ] Mock data generators

### Week 2: Core Components & Screens
- [ ] KaijuCard component
- [ ] StatBar component
- [ ] TraitBadge component
- [ ] Laboratory screen
- [ ] RosterView screen
- [ ] KaijuDetailView modal

### Week 3: Gameplay UI
- [ ] BreedingScreen complete
- [ ] TournamentLobby screen
- [ ] BattleView screen
- [ ] ResultsScreen
- [ ] BattleLog component

### Week 4: Meta & Polish
- [ ] LeaderboardScreen
- [ ] LineageViewer + LineageTree component
- [ ] TournamentBracket component
- [ ] All animations implemented
- [ ] Keyboard shortcuts
- [ ] Performance optimization
- [ ] Integration testing

---

## 9. Risk Mitigation

### 9.1 Complexity Risks

**Risk**: UI code becomes too coupled to game logic
**Mitigation**:
- Strict UiAction pattern enforcement
- UI components are pure functions
- No game state mutation in UI code
- Regular refactoring reviews

### 9.2 Performance Risks

**Risk**: Too many components cause frame drops
**Mitigation**:
- Render culling for off-screen elements
- Animation limits (max 10 simultaneous)
- Performance monitoring in debug builds
- Reduce effects mode for low-end devices

### 9.3 Accessibility Risks

**Risk**: UI inaccessible to colorblind or keyboard users
**Mitigation**:
- Icons + text for all semantic info
- Full keyboard navigation
- High contrast mode
- Regular accessibility audits

---

## 10. Future Enhancements (Post-v1.0)

### 10.1 Advanced Features
- Touch/mobile support
- Controller input
- Custom UI themes
- Advanced animations (particles, transitions)
- 3D kaiju viewer
- Voice commands (experimental)

### 10.2 Performance Improvements
- GPU-accelerated rendering for large rosters
- Texture atlasing for trait icons
- Lazy loading for kaiju portraits
- Background thread for layout calculations

---

## 11. Documentation Requirements

### 11.1 Component Documentation

Each component must have:
- Purpose description
- API documentation with examples
- Visual state descriptions
- Interaction behavior
- Performance considerations

### 11.2 Screen Documentation

Each screen must document:
- Layout specifications
- User flows
- Action handling
- Keyboard shortcuts
- Accessibility features

---

## 12. Acceptance Criteria

### 12.1 Functional Requirements
- [ ] All 10 screens implemented and functional
- [ ] All 7 components work as specified
- [ ] Navigation system handles all transitions
- [ ] All UiActions dispatched correctly
- [ ] Keyboard shortcuts work on all screens
- [ ] Modals close properly (ESC, click outside, close button)

### 12.2 Visual Requirements
- [ ] Consistent color palette across all screens
- [ ] Typography hierarchy clear and readable
- [ ] 8px grid system followed everywhere
- [ ] All interactive elements have hover states
- [ ] Animations smooth (60 FPS target)

### 12.3 Accessibility Requirements
- [ ] 4.5:1 contrast ratio minimum
- [ ] All text 12px minimum
- [ ] 44px minimum click targets
- [ ] Keyboard navigation complete
- [ ] No color-only information

### 12.4 Performance Requirements
- [ ] 60 FPS on target hardware
- [ ] < 50ms frame time 99th percentile
- [ ] Roster with 100 kaiju scrolls smoothly
- [ ] Battle animations don't drop frames
- [ ] Screen transitions instant (<16ms)

---

### Critical Files for Implementation

Based on this implementation plan, here are the 5 most critical files to create first:

1. **H:\RustGames\kaiju_sim\src\ui\actions.rs** - Master UiAction enum that all screens return (250 lines)
   - Reason: Foundation for all UI interactions, must be defined before any screen work begins

2. **H:\RustGames\kaiju_sim\src\ui\colors.rs** - Extended color palette constants (100 lines)
   - Reason: Consistent theming across all components, prevents color duplication

3. **H:\RustGames\kaiju_sim\src\ui\components\kaiju_card.rs** - Core UI component used everywhere (200 lines)
   - Reason: Most frequently used component across 6+ screens, defines visual language

4. **H:\RustGames\kaiju_sim\src\screens\laboratory.rs** - Central hub screen tying everything together (300 lines)
   - Reason: Primary navigation hub, integrates multiple systems, validates overall architecture

5. **H:\RustGames\kaiju_sim\src\state\navigation.rs** - Navigation state management (150 lines)
   - Reason: Core infrastructure for screen transitions, history, and modal management

**Total Priority Lines**: ~1,000 lines for core UI infrastructure

**Next Priority** (after core 5):
- `src\ui\components\stat_bar.rs` - Reusable stat visualization
- `src\ui\components\trait_badge.rs` - Trait display system
- `src\screens\main_menu.rs` - Entry point screen
- `src\screens\roster_view.rs` - Kaiju browsing interface
- `src\screens\breeding.rs` - Complex breeding logic UI

---

**End of Phase 6 Implementation Plan v1.0**
