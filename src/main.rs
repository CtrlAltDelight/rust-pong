use macroquad::prelude::*;


struct MainState {
    ball: Rect,
    ball_vel: Vec2,

    top_paddle: Rect,
    bottom_paddle: Rect,
}

impl MainState {
    fn update(&mut self) {
        // Ball movement
        self.ball.move_to(Vec2::new(
                self.ball.x + self.ball_vel.x,
                self.ball.y + self.ball_vel.y,
        ));

        // Controlling paddles
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

        // Ball bounce off paddles
        if self.ball.overlaps(&self.top_paddle) && self.ball_vel.y < 0.0 ||
           self.ball.overlaps(&self.bottom_paddle) && self.ball_vel.y > 0.0 {

            self.ball_vel.y *= -1.0;
        }
    }
}

fn _draw_rect_object(rect: Rect) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, WHITE);
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

        _draw_rect_object(state.ball);
        _draw_rect_object(state.top_paddle);
        _draw_rect_object(state.bottom_paddle);
        next_frame().await
    }
}
            
