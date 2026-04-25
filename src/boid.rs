use macroquad::prelude::*;

const LENGTH: f32 = 1.0;
const WIDTH: f32 = 0.8;
const HEIGHT: f32 = 0.2;

pub struct Boid {
    position: Vec3,
    velocity: Vec3,
    size: f32,
    color: Color,
}

impl Boid {
    pub fn new(size: f32, color: Color) -> Self {
        let boid = Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            size,
            color,
        };
        boid
    }

    pub fn update(&mut self, dt: f32, sim_time: f32) {
        self.recompute_path_state(sim_time);
    }

    pub fn local_coordinates(&self) -> (Vec3, Vec3, Vec3) {
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

    pub fn draw(&self) {
        let (local_forward, local_right, local_up) = self.local_coordinates();

        let tip = self.position + local_forward * LENGTH * 0.5 * self.size;
        let left =
            self.position + (-local_forward * LENGTH * 0.5 + local_right * WIDTH * 0.5) * self.size;
        let right: Vec3 =
            self.position + (-local_forward * LENGTH * 0.5 - local_right * WIDTH * 0.5) * self.size;
        let up: Vec3 =
            self.position + (-local_forward * LENGTH * 0.5 + local_up * HEIGHT * 0.5) * self.size;
        let down: Vec3 =
            self.position + (-local_forward * LENGTH * 0.5 - local_up * HEIGHT * 0.5) * self.size;

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
        draw_line_3d(tip, left, BLACK);
        draw_line_3d(left, right, BLACK);
        draw_line_3d(right, tip, BLACK);
    }

    fn recompute_path_state(&mut self, sim_time: f32) {
        // A looping 3D path for a single boid prototype.
        self.position = vec3(
            10.0 * (sim_time * 0.9).cos(),
            6.0 + 3.5 * (sim_time * 1.3).sin(),
            10.0 * (sim_time * 0.9).sin() + 4.0 * (sim_time * 0.5).sin(),
        );

        // Analytic derivative of the path gives stable movement direction.
        self.velocity = vec3(
            -9.0 * (sim_time * 0.9).sin(),
            4.55 * (sim_time * 1.3).cos(),
            9.0 * (sim_time * 0.9).cos() + 2.0 * (sim_time * 0.5).cos(),
        );
    }
}
