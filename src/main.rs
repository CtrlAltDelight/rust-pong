use macroquad::prelude::*;


struct MainState {
    ball: Rect,
    ball_vel: Vec2,

    top_paddle: Rect,
    bottom_paddle: Rect,

    top_player_score: i32,
    bottom_player_score: i32,
}

impl MainState {
    fn reset_ball(&mut self) {
        self.ball = Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0);
        self.ball_vel = Vec2::new(1.0, 2.0);
    }

    pub fn update(&mut self) {
        // Ball movement
        self.ball.move_to(Vec2::new(
                self.ball.x + self.ball_vel.x,
                self.ball.y + self.ball_vel.y,
        ));

        // Controlling paddles
        const PADDLE_SPEED: f32 = 5.0;
        if is_key_down(KeyCode::Left) && self.top_paddle.left() > 0.0 {
            self.top_paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::Right) && self.top_paddle.right() < screen_width() {
            self.top_paddle.x += PADDLE_SPEED;
        }
        if is_key_down(KeyCode::A) && self.bottom_paddle.left() > 0.0 {
            self.bottom_paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::D) && self.bottom_paddle.right() < screen_width() {
            self.bottom_paddle.x += PADDLE_SPEED;
        }

        // Ball bounce off paddles
        if self.ball.overlaps(&self.top_paddle) && self.ball_vel.y < 0.0 ||
           self.ball.overlaps(&self.bottom_paddle) && self.ball_vel.y > 0.0 {

            self.ball_vel.y *= -1.0;
        }

        // Bounce ball off sides of screen
        if self.ball.right() >= screen_width() && self.ball_vel.x > 0.0 ||
           self.ball.left()  <= 0.0            && self.ball_vel.x < 0.0 {
               self.ball_vel.x *= -1.0;
        }

        // Scoring
        if self.ball.bottom() >= screen_height() && self.ball_vel.y > 0.0 {
            self.top_player_score += 1;
            self.reset_ball()
        }
        if self.ball.top() <= 0.0 && self.ball_vel.y < 0.0 {
            self.bottom_player_score += 1;
            self.reset_ball()
        }
    }

    fn draw_rect_object(&mut self, rect: Rect) {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, WHITE);
    }


    pub fn draw(&mut self) {
        self.draw_rect_object(self.ball);
        self.draw_rect_object(self.top_paddle);
        self.draw_rect_object(self.bottom_paddle);

        draw_text(&self.top_player_score.to_string(), 20.0, 30.0, 40.0, WHITE);
        draw_text(&self.bottom_player_score.to_string(), 20.0, screen_height() - 30.0, 40.0, WHITE);
    }
}

#[macroquad::main("InputKeys")]
async fn main() {
    let mut state = MainState {
        ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
        ball_vel: Vec2::new(1.0, 2.0),
        top_paddle: Rect::new(screen_width() / 2.0, 20.0, 100.0, 5.0),
        bottom_paddle: Rect::new(screen_width() / 2.0, screen_height() - 20.0, 100.0, 5.0),
        top_player_score: 0,
        bottom_player_score: 0,
    };

    loop {
        clear_background(BLACK);

        state.update();
        state.draw();

        next_frame().await
    }
}
            
