use raylib::prelude::*;

struct Ball {
    position: Vector2,
    velocity_y: f32,
}

fn main() {
    let height = 480.0;
    let width = 640.0;
    
    let mut balls: Vec<Ball> = Vec::new();

    const GRAVITY: f32 = 0.5;
    const RADIUS: f32 = 20.0;
    const RESTITUTION: f32 = 0.75;

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Gravity")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let m_pos = rl.get_mouse_position();
            balls.push(Ball {
                position: m_pos,
                velocity_y: 0.0,
            });
        }

        for ball in balls.iter_mut() {
            ball.velocity_y += GRAVITY;
            ball.position.y += ball.velocity_y;

            if ball.position.y > height - RADIUS {
                ball.position.y = height - RADIUS;
                ball.velocity_y = -ball.velocity_y * RESTITUTION;
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        for ball in &balls {
            d.draw_circle(ball.position.x as i32, ball.position.y as i32, RADIUS, Color::RED);
        }
        
    }
}