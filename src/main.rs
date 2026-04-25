use macroquad::prelude::*;
mod boid;

use boid::Boid;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad 3D Starter".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

fn draw_world() {
    clear_background(Color::from_rgba(16, 24, 32, 255));
    draw_grid(40, 1.0, GRAY, DARKGRAY);
    // Visual reference center.
    draw_cube_wires(vec3(0.0, 6.0, 0.0), vec3(22.0, 12.0, 22.0), LIGHTGRAY);
}

#[macroquad::main(window_conf)]
async fn main() {
    let camera = Camera3D {
        position: vec3(24.0, 18.0, 24.0),
        target: vec3(0.0, 6.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let mut boid = Boid::new(3.0, SKYBLUE);
    let mut sim_time = 0.0;

    loop {
        let dt = get_frame_time();
        sim_time += dt;

        set_camera(&camera);

        draw_world();

        boid.update(dt, sim_time);
        boid.draw();

        set_default_camera();

        next_frame().await;
    }
}
