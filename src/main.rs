use macroquad::prelude::*;


struct MainState {
    ball: Rect,
    ball_vel: Vec2,

    top_paddle: Rect,
    bottom_paddle: Rect,
}

impl MainState {
    fn update(&mut self) {
        self.ball.move_to(Vec2::new(
                self.ball.x + self.ball_vel.x,
                self.ball.y + self.ball_vel.y,
        ));

        const PADDLE_SPEED: f32 = 5.0;
        if is_key_down(KeyCode::Left) {
            self.top_paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::Right) {
            self.top_paddle.x += PADDLE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            self.bottom_paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            self.bottom_paddle.x += PADDLE_SPEED;
        }
    }
}

#[macroquad::main("InputKeys")]
async fn main() {
    let mut state = MainState {
        ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
        ball_vel: Vec2::new(1.0, 2.0),
        top_paddle: Rect::new(screen_width() / 2.0, 20.0, 100.0, 5.0),
        bottom_paddle: Rect::new(screen_width() / 2.0, screen_height() - 20.0, 100.0, 5.0),
    };

    loop {
        clear_background(BLACK);

        state.update();

        draw_rectangle(state.ball.x, state.ball.y, state.ball.w, state.ball.h, WHITE);
        draw_rectangle(state.top_paddle.x, state.top_paddle.y, state.top_paddle.w, state.top_paddle.h, WHITE);
        draw_rectangle(state.bottom_paddle.x, state.bottom_paddle.y, state.bottom_paddle.w, state.bottom_paddle.h, WHITE);
        next_frame().await
    }
}
            
