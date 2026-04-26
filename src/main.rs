use raylib::prelude::*;

#[derive(Clone)]
struct Ball {
    position: Vector2,
    velocity: Vector2,
    mass: f32,
    radius: f32,
    color: Color,
}

fn main() {
    const WIDTH: i32 = 1240;
    const HEIGHT: i32 = 980;
    //const RADIUS: f32 = 50.0;
    const G: f32 = 1000.0;
    const SOFTENING: f32 = 150.0;
    //const MASS: f32 = 300.0;
    const LIMIT: usize = 4;
    const MULTIPLYER_SPEED: f32 = 10.0;


    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Gravity")
        .build();

    rl.set_target_fps(120);

    let mut balls: Vec<Ball> = Vec::new();

    while !rl.window_should_close() {
        let dt = 1.0 / 120.0;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if balls.len() < LIMIT {
                let mouse = rl.get_mouse_position();

                let (mass, radius, color) = match balls.len() {
                    0 => (1000.0, 50.0, Color::ORANGE), // sun 1
                    1 => (1000.0, 50.0, Color::LIGHTBLUE), // sun 2
                    2 => (1000.0, 50.0, Color::RED), // sun 3
                    _ => (50.0, 20.0, Color::GREEN), // YOU
                };

                balls.push(Ball {
                    position: mouse,
                    velocity: Vector2::zero(),
                    mass,
                    radius,
                    color,
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
            balls[i].velocity.x += accelerations[i].x * dt * MULTIPLYER_SPEED;
            balls[i].velocity.y += accelerations[i].y * dt * MULTIPLYER_SPEED;

            balls[i].position.x += balls[i].velocity.x * dt * MULTIPLYER_SPEED;
            balls[i].position.y += balls[i].velocity.y * dt * MULTIPLYER_SPEED;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("3 body simulation", 20, 20, 30, Color::AQUA);

        for ball in &balls {
            d.draw_circle(
                ball.position.x as i32,
                ball.position.y as i32,
                ball.radius,
                ball.color,
            );
        }

        if balls.len() >= LIMIT {
            d.draw_text(
                "Spwan limit reached",
                20,
                60,
                20,
                Color::RED,
            );
        }
    }
}