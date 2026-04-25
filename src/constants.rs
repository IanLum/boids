// ==== INITIALIZATION PARAMS ====
pub const NUM_BOIDS: usize = 50;
pub const INIT_SPEED: f32 = 5.0;

//
pub const MAX_SPEED: f32 = 24.0;
pub const ATTRACTION_RANGE: f32 = 16.0;
pub const SEPERATION_RANGE: f32 = 2.0;

// ==== FORCE PARAMS ====
pub const BORDER_FORCE: f32 = 20.0;
pub const COHESION_FORCE: f32 = 5.0;
pub const SEPERATION_FORCE: f32 = 1.5;
pub const ALIGNMENT_FORCE: f32 = 1.5;
// pub const ALIGNMENT_FACTOR: f32 = 0.05;

// ==== SIMULATION BOUNDS ====
// Defines an X by Y by Z box centered at the world origin
pub const BOUNDS_X: f32 = 22.0;
pub const BOUNDS_Y: f32 = 16.0;
pub const BOUNDS_Z: f32 = 22.0;

// ==== BOID MESH SIZE ====
pub const BOID_LENGTH: f32 = 1.0;
pub const BOID_WIDTH: f32 = 0.8;
pub const BOID_HEIGHT: f32 = 0.2;
