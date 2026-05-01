// ==== INITIALIZATION PARAMS ====
pub const NUM_BOIDS: usize = 300;
pub const INIT_SPEED: f32 = 5.0;

//
pub const MIN_SPEED: f32 = 8.0;
pub const MAX_SPEED: f32 = 16.0;
pub const ATTRACTION_RANGE: f32 = 6.0;
pub const SEPERATION_RANGE: f32 = ATTRACTION_RANGE / 4.0;

// ==== FORCE PARAMS ====
pub const BORDER_FORCE: f32 = 20.0;
pub const COHESION_FORCE: f32 = 0.8;
pub const SEPERATION_FORCE: f32 = 0.7;
pub const ALIGNMENT_FORCE: f32 = 0.4;

// ==== SIMULATION BOUNDS ====
// Defines an X by Y by Z box centered at the world origin
pub const BOUNDS_X: f32 = 36.0;
pub const BOUNDS_Y: f32 = 20.0;
pub const BOUNDS_Z: f32 = BOUNDS_X;

// ==== BOID MESH SIZE ====
pub const BOID_SIZE: f32 = 0.5;
pub const BOID_LENGTH: f32 = 1.0;
pub const BOID_WIDTH: f32 = 0.8;
pub const BOID_HEIGHT: f32 = 0.3;
