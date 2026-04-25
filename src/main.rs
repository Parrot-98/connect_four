use raylib::prelude::*;

fn main() {
    let height = 480.0;
    let width = 640.0;
    let mut ball_pos = Vector2::new(300.0, 50.0); 
    let mut velocity_y = 0.0;
    let restitution = 0.7;

    const GRAVITY: f32 = 0.5;
    const RADIUS: f32 = 20.0;

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Checkers")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        //input
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            ball_pos.x = mouse_pos.x;
            ball_pos.y = mouse_pos.y;
            velocity_y = 0.0;
        }

        velocity_y += GRAVITY;
        ball_pos.y += velocity_y;

        //collision
        if ball_pos.y > height - RADIUS {
            ball_pos.y = height - RADIUS;
            velocity_y = -velocity_y * restitution;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        d.draw_text("Checkers", 12, 12, 20, Color::WHITE);
        d.draw_circle(ball_pos.x as i32, ball_pos.y as i32, RADIUS, Color::BLUE);
    }
}