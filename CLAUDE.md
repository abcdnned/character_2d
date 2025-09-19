# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a 2D character action game built with Bevy Engine 0.16.0 and Rust. The game features a top-down perspective with player-controlled characters, AI enemies, physics-based combat, and real-time combat mechanics.

## Development Commands

### Build and Run
- `cargo build --release` - Build optimized release version
- `cargo check` - Quick syntax and type checking without building

### Testing
- `cargo test` - Run all unit tests
- `cargo test <test_name>` - Run specific test

### Code Quality
- `cargo clippy` - Run linter (note: some clippy warnings are allowed in Cargo.toml)
- `cargo fmt` - Format code

## Architecture Overview

### Core Systems
The game uses Bevy's Entity Component System (ECS) architecture with a modular plugin system:

**Main Components:**
- `Player` - Player character marker
- `Unit` - Core entity with HP, speed, unit type (Hero, SwordMan, Dummy)
- `AI` - AI behavior system with move queues and target detection
- `Force` - Faction system (Player force vs Enemy force)
- `TargetDetector` - Range-based target acquisition and engagement

**Key Plugins:**
- `InputPlugin` - Handles WASD movement input
- `MovePlugin` - Custom movement system with animations
- `AIPlugin` - Enemy AI behaviors and targeting
- `UnitPlugin` - Core unit mechanics and HP events
- `BerserkerPlugin` - Special character class with rage mechanics
- `WeaponPlugin` - Sword combat and weapon management
- `ParticlePlugin` - Visual effects system

### Physics Integration
- Uses `bevy_rapier2d` for 2D physics simulation
- Physics entities have velocity, collision detection, and knockback mechanics
- All moving entities use dynamic physics bodies with damping

### Movement and Combat
- Custom move system supports combo attacks and special abilities
- Move database stores move definitions with timing, damage, and effects
- Combat features knockback levels (LITE, HEAVY, SUPER) and critical hits
- Sprint system with cooldown mechanics

### AI System
The AI uses a queue-based approach:
- Each unit type has configurable AI options with activation ranges
- Target detection with alert/disengage ranges
- Lock types: Lock (aggressive) vs Free (defensive)
- Move selection based on distance to target

### Animation and Visuals
- `AnimationDatabasePlugin` manages character animations
- Particle effects using `bevy_hanabi`
- Transform interpolation for smooth movement
- Sword trail effects during attacks
- Floating damage text system

### Constants and Configuration
All game constants are centralized in `src/constants.rs`:
- Movement speeds, attack timings, damage values
- Color definitions and visual constants
- Physics parameters (damping, knockback forces)
- AI behavior ranges and cooldowns

## File Organization

**Core Systems:**
- `main.rs` - App setup, scene initialization, camera system
- `unit.rs` - Core unit component and HP event system
- `ai.rs` - AI behavior and target detection
- `movement.rs` - Player movement and sprint mechanics

**Combat:**
- `custom_move.rs` - Move execution and animation system
- `weapon.rs` - Weapon equipment and sword mechanics
- `damage.rs` - Damage calculation and application
- `collisions.rs` - Combat collision detection

**Character Classes:**
- `berserker.rs` - Berserker character class with rage mechanics

**Visual/Effects:**
- `particle.rs` - Particle effect system
- `float_text.rs` - Floating damage numbers
- `sword_trail.rs` - Visual sword trail effects
- `health_bar.rs` - UI health bars

**Utilities:**
- `constants.rs` - Game configuration constants
- `global_entity_map.rs` - Entity relationship management
- `input.rs` - Input handling system

## Development Notes
No need to run cargo run after eatch time code finish.

### Performance Optimizations
- Dynamic linking enabled for faster iteration during development

### Physics Setup
- Rapier physics uses 100 pixels per meter scaling
- Debug rendering available via `RapierDebugRenderPlugin`
- Linear damping: 2.0, Angular damping: 5.0


### Code Conventions
- Bevy ECS patterns: Components, Resources, Systems, Events
- Builder pattern used for complex entity creation (see Unit::builder())
- Plugin-based architecture for feature organization
- Constants centralized and prefixed by category