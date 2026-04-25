use raylib::prelude::*;

#[derive(Clone)]
struct Ball {
    position: Vector2,
    velocity: Vector2,
    mass: f32,
}

fn main() {
    const WIDTH: i32 = 1240;
    const HEIGHT: i32 = 980;
    const RADIUS: f32 = 50.0;
    const G: f32 = 1000.0;
    const SOFTENING: f32 = 25.0;
    const MASS: f32 = 300.0;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("gravity")
        .build();

    rl.set_target_fps(120);

    let mut balls: Vec<Ball> = Vec::new();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time().min(0.05);

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if balls.len() < 10 {
                let mouse = rl.get_mouse_position();
                balls.push(Ball {
                    position: mouse,
                    velocity: Vector2::zero(),
                    mass: MASS,
                });
            }
        }

        let mut accelerations = vec![Vector2::zero(); balls.len()];

        for i in 0..balls.len() {
            for j in 0..balls.len() {
                if i == j {
                    continue;
                }

                let dx = balls[j].position.x - balls[i].position.x;
                let dy = balls[j].position.y - balls[i].position.y;

                let dist_sq = dx * dx + dy * dy + SOFTENING * SOFTENING;
                let inv_dist = dist_sq.sqrt().recip(); 

                let accel_mag = G * balls[j].mass / dist_sq;

                accelerations[i].x += accel_mag * dx * inv_dist;
                accelerations[i].y += accel_mag * dy * inv_dist;
            }
        }

        for i in 0..balls.len() {
            balls[i].velocity.x += accelerations[i].x * dt;
            balls[i].velocity.y += accelerations[i].y * dt;

            balls[i].position.x += balls[i].velocity.x * dt;
            balls[i].position.y += balls[i].velocity.y * dt;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for ball in &balls {
            d.draw_circle(
                ball.position.x as i32,
                ball.position.y as i32,
                RADIUS,
                Color::RED,
            );
        }

        if balls.len() >= 10 {
            d.draw_text(
                "Ball limit reached",
                20,
                20,
                20,
                Color::YELLOW,
            );
        }
    }
}