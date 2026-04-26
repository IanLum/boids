use macroquad::prelude::*;
mod boid;
mod constants;

use boid::Boid;
use constants::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad 3D Starter".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

fn init_boids() -> Vec<Boid> {
    (0..NUM_BOIDS)
        .map(|_| {
            Boid::new(
                vec3(
                    rand::gen_range(-BOUNDS_X * 0.5, BOUNDS_X * 0.5),
                    rand::gen_range(-BOUNDS_Y * 0.5, BOUNDS_Y * 0.5),
                    rand::gen_range(-BOUNDS_Z * 0.5, BOUNDS_Z * 0.5),
                ),
                vec3(
                    rand::gen_range(-1.0, 1.0),
                    rand::gen_range(-1.0, 1.0),
                    rand::gen_range(-1.0, 1.0),
                )
                .normalize()
                    * INIT_SPEED,
                BOID_SIZE,
                SKYBLUE,
            )
        })
        .collect()
}

fn draw_world() {
    clear_background(Color::from_rgba(16, 24, 32, 255));
    draw_grid_ex(
        40,
        1.0,
        GRAY,
        DARKGRAY,
        vec3(0.0, -BOUNDS_Y * 0.5, 0.0),
        Quat::IDENTITY,
    );
    // Visual reference center.
    draw_cube_wires(
        vec3(0.0, 0.0, 0.0),
        vec3(BOUNDS_X, BOUNDS_Y, BOUNDS_Z),
        LIGHTGRAY,
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    let camera = Camera3D {
        position: vec3(24.0, 10.0, 24.0),
        target: vec3(0.0, -3.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let mut boids = init_boids();
    let mut boids_prior: Vec<Boid> = vec![];

    let mut sim_time = 0.0;

    loop {
        let dt = get_frame_time();
        sim_time += dt;

        set_camera(&camera);

        draw_world();

        boids_prior.clone_from(&boids);
        for boid in &mut boids {
            boid.update(&boids_prior, dt);
            boid.draw();
        }

        set_default_camera();

        next_frame().await;
    }
}
