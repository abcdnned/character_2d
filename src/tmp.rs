#[derive(Component)]
struct Move {
    pub move_metadata: MoveMetadata,
    pub move_progress: Timer,
    current_phase: MovePhase,
    next_move: MoveMetadata,
}

pub enum MovePhase {
    Startup,
    Active, // accept next move during Active and Recorvery phase
    Recovery,
}

pub enum MoveType { // MoveType determine how weapon moves
    Swing,
    Stub,
    DashStub,
}

enum MoveInput {
    Attack,
}

pub struct MoveMetadata {
    start_pos: Vec2,
    start_rotation: f32,
    radius: f32, // parameter to move weapon
    pub startup_time: f32,
    pub active_time: f32,
    recovery_time: f32,
    pub move_type: MoveType,
    accept_input: MoveInput,
    next_move: MoveMetadata,
}