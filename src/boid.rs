use crate::constants::*;
use macroquad::prelude::*;
#[derive(Clone)]
pub struct Boid {
    position: Vec3,
    velocity: Vec3,
    size: f32,
    color: Color,
}

impl Boid {
    pub fn new(position: Vec3, velocity: Vec3, size: f32, color: Color) -> Self {
        Self {
            position,
            velocity,
            size,
            color,
        }
    }

    pub fn update(&mut self, boids: &[Boid], dt: f32) {
        let cohesion = self.cohesion(&boids) * dt;
        let seperation = self.seperation(&boids) * dt;
        let alignment = self.alignment(&boids) * dt;

        self.velocity += cohesion;
        self.velocity += seperation;
        self.velocity += alignment;
        self.velocity += self.avoid_borders() * dt;
        self.cap_speed();

        println!(
            "cohesion: {:<10.2} seperation: {:<10.2} alignment: {:<10.2} velocity: {:<10.2}",
            cohesion.length(),
            seperation.length(),
            alignment.length(),
            self.velocity.length()
        );

        self.position += self.velocity * dt;
    }

    pub fn draw(&self) {
        let (local_forward, local_right, local_up) = self.local_coordinates();

        let tip = self.position + local_forward * BOID_LENGTH * 0.5 * self.size;
        let left = self.position
            + (-local_forward * BOID_LENGTH * 0.5 - local_right * BOID_WIDTH * 0.5) * self.size;
        let right: Vec3 = self.position
            + (-local_forward * BOID_LENGTH * 0.5 + local_right * BOID_WIDTH * 0.5) * self.size;
        let up: Vec3 = self.position
            + (-local_forward * BOID_LENGTH * 0.5 + local_up * BOID_HEIGHT * 0.5) * self.size;
        let down: Vec3 = self.position
            + (-local_forward * BOID_LENGTH * 0.5 - local_up * BOID_HEIGHT * 0.5) * self.size;

        let boid_mesh = Mesh {
            vertices: vec![
                Vertex::new2(tip, vec2(0.0, 0.0), self.color),
                Vertex::new2(up, vec2(0.5, 1.0), self.color),
                Vertex::new2(left, vec2(1.0, 0.0), self.color),
                Vertex::new2(down, vec2(0.5, 1.0), self.color),
                Vertex::new2(right, vec2(0.5, 1.0), self.color),
            ],
            indices: vec![
                0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 1, // sides
                1, 3, 2, 1, 4, 3, // base
            ],
            texture: None,
        };

        draw_mesh(&boid_mesh);
    }

    fn cap_speed(&mut self) {
        if self.velocity.length_squared() > MAX_SPEED.powi(2) {
            self.velocity = self.velocity.normalize() * MAX_SPEED;
        }
    }

    fn is_self(&self, other: &Boid) -> bool {
        self.position.distance_squared(other.position) < 1e-6
    }

    fn cohesion(&self, boids: &[Boid]) -> Vec3 {
        let mut center = Vec3::ZERO;
        let mut num_neighbors = 0;

        for other in boids {
            if self.is_self(other) {
                continue;
            }
            if (other.position - self.position).length_squared() < ATTRACTION_RANGE.powi(2) {
                center += other.position;
                num_neighbors += 1;
            }
        }

        if num_neighbors > 0 {
            center /= num_neighbors as f32;
            return (center - self.position) * COHESION_FORCE;
        }

        return Vec3::ZERO;
    }

    fn seperation(&self, boids: &[Boid]) -> Vec3 {
        let mut force = Vec3::ZERO;

        for other in boids {
            if self.is_self(other) {
                continue;
            }

            if (other.position - self.position).length_squared() < SEPERATION_RANGE.powi(2) {
                force += (self.position - other.position) * SEPERATION_FORCE;
            }
        }
        return force;
    }

    fn alignment(&self, boids: &[Boid]) -> Vec3 {
        let mut avg_velocity = Vec3::ZERO;
        let mut num_neighbors = 0;

        for other in boids {
            if self.is_self(other) {
                continue;
            }
            if (self.position - other.position).length_squared() < ATTRACTION_RANGE.powi(2) {
                avg_velocity += other.velocity;
                num_neighbors += 1;
            }
        }
        if num_neighbors > 0 {
            avg_velocity /= num_neighbors as f32;
            return (avg_velocity - self.velocity) * ALIGNMENT_FORCE;
        }

        return Vec3::ZERO;
    }

    fn avoid_borders(&self) -> Vec3 {
        let mut force = Vec3::ZERO;
        if self.position.x < -BOUNDS_X * 0.5 {
            force.x += BORDER_FORCE;
        } else if self.position.x > BOUNDS_X * 0.5 {
            force.x -= BORDER_FORCE;
        }
        if self.position.y < -BOUNDS_Y * 0.5 {
            force.y += BORDER_FORCE;
        } else if self.position.y > BOUNDS_Y * 0.5 {
            force.y -= BORDER_FORCE;
        }
        if self.position.z < -BOUNDS_Z * 0.5 {
            force.z += BORDER_FORCE;
        } else if self.position.z > BOUNDS_Z * 0.5 {
            force.z -= BORDER_FORCE;
        }
        return force;
    }

    fn local_coordinates(&self) -> (Vec3, Vec3, Vec3) {
        let local_forward = if self.velocity.length_squared() < 0.0001 {
            vec3(0.0, 0.0, 1.0)
        } else {
            self.velocity.normalize()
        };
        let world_up = vec3(0.0, 1.0, 0.0);
        let mut local_right = local_forward.cross(world_up);

        if local_right.length_squared() < 0.0001 {
            local_right = vec3(1.0, 0.0, 0.0);
        } else {
            local_right = local_right.normalize();
        }

        let mut local_up = local_right.cross(local_forward);
        if local_up.length_squared() < 0.0001 {
            local_up = world_up;
        } else {
            local_up = local_up.normalize();
        }

        (local_forward, local_right, local_up)
    }
}
