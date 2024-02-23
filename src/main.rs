use macroquad::prelude::*;

struct MainState {
    ball: Rect,
    ball_vel: Vec2,
}

#[macroquad::main("InputKeys")]
async fn main() {
    let mut state = MainState {
        ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
        ball_vel: Vec2::new(1.0, 2.0),
    };

    loop {
        clear_background(BLACK);

        state.ball.move_to(Vec2::new(
                state.ball.x + state.ball_vel.x,
                state.ball.y + state.ball_vel.y,
        ));

        draw_rectangle(state.ball.x, state.ball.y, state.ball.w, state.ball.h, WHITE);
        next_frame().await
    }
}
            
