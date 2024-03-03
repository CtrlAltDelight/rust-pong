use macroquad::prelude::*;

use std::io::ErrorKind;
use std::net::UdpSocket;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::str;

struct MainState {
    ball: Rect,
    ball_vel: Vec2,

    top_paddle: Rect,
    bottom_paddle: Rect,

    top_player_score: i32,
    bottom_player_score: i32,

    socket: UdpSocket,
    peer_addr: String,
}


const PADDLE_SPEED: f32 = 5.0;
impl MainState {
    fn reset_ball(&mut self) {
        self.ball = Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0);
        self.ball_vel = Vec2::new(1.0, 2.0);
    }

    fn send_movement_packet(&mut self, movement: &str) {
        let message = movement.as_bytes();
        let _ = self.socket.send_to(message, &self.peer_addr);
    }

    pub fn update(&mut self) {
        // Ball movement
        self.ball.move_to(Vec2::new(
                self.ball.x + self.ball_vel.x,
                self.ball.y + self.ball_vel.y,
        ));

        // Controlling paddles
        if is_key_down(KeyCode::Left) && self.bottom_paddle.left() > 0.0 {
            self.bottom_paddle.x -= PADDLE_SPEED;
            self.send_movement_packet("LEFT");
        }
        else if is_key_down(KeyCode::Right) && self.bottom_paddle.right() < screen_width() {
            self.bottom_paddle.x += PADDLE_SPEED;
            self.send_movement_packet("RIGHT");
        }
        else {
            self.send_movement_packet("STOP");
        }

        if is_key_down(KeyCode::A) && self.top_paddle.left() > 0.0 {
            self.top_paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::D) && self.top_paddle.right() < screen_width() {
            self.top_paddle.x += PADDLE_SPEED;
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

enum PaddleCommand {
    Left,
    Right,
    Stop,
}

fn start_listening_thread(socket: UdpSocket, tx: Sender<PaddleCommand>) {
    thread::spawn(move || loop {
        let mut buf = [0; 10];
        match socket.recv_from(&mut buf) {
            Ok((amt, _src)) => {
                let command_str = match str::from_utf8(&buf[..amt]) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Invalid UTF-8 sequence: {}", e);
                        continue;
                    },
                };
                // Interpret the command and send it to the game loop
                match command_str {
                    "LEFT" => tx.send(PaddleCommand::Left).unwrap(),
                    "RIGHT" => tx.send(PaddleCommand::Right).unwrap(),
                    "STOP" => tx.send(PaddleCommand::Stop).unwrap(),
                    _ => eprintln!("Unknown command: {}", command_str),
                }
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // No data available yet
                continue;
            },
            Err(e) => eprintln!("Error receiving: {}", e),
        }
    });
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

async fn start_screen(ip_addr: &mut String) {

    loop {
        clear_background(BLACK);

        let printable_chars = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.', ':'];
        if let Some(key) = get_char_pressed() {
            println!("Key pressed: {}", key);
            if is_key_pressed(KeyCode::Enter) {
                return;
            }
            else if is_key_pressed(KeyCode::Backspace) {
                ip_addr.pop();
            }
            else if printable_chars.contains(&key) {
                ip_addr.push(key);
            }
        }

        draw_text("Enter the IP address of the opponent:", 20.0, 20.0, 40.0, WHITE);
        draw_text(&ip_addr, 20.0, 60.0, 40.0, WHITE);
        next_frame().await;
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    /*
    let listener = TcpListener::bind("192.168.1.100:7878").expect("Couldn't bind tcp listener.");
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // Handle scoring and critical events here
            }
            Err(_e) => { /* Handle errors */ }
        }
    }
    */

    let own_ip = "192.168.1.100:7878";
    //let peer_ip = "192.168.1.16:7878";

    // Get the IP address of the opponent
    let mut state = MainState {
        ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
        ball_vel: Vec2::new(1.0, 2.0),
        top_paddle: Rect::new(screen_width() / 2.0, 20.0, 100.0, 5.0),
        bottom_paddle: Rect::new(screen_width() / 2.0, screen_height() - 20.0, 100.0, 5.0),
        top_player_score: 0,
        bottom_player_score: 0,
        socket: UdpSocket::bind(own_ip).expect("Couldn't bind udp socket."),
        peer_addr: "".to_string(),
    };
    start_screen(&mut state.peer_addr).await;
    state.socket.set_nonblocking(true).expect("Couldn't set non-blocking mode.");
    let socket_clone = state.socket.try_clone().expect("Couldn't clone socket.");


    // Handle incoming data from opponent
    let (tx, rx): (Sender<PaddleCommand>, Receiver<PaddleCommand>) = mpsc::channel();
    start_listening_thread(socket_clone, tx);


    const TARGET_FRAME_TIME: f64 = 1.0 / 60.0;
    let mut last_frame_time = get_time();
    loop {
        // Set frame rate
        let now = get_time();
        let elapsed = now - last_frame_time;
        if elapsed < TARGET_FRAME_TIME {
            let delay = TARGET_FRAME_TIME - elapsed;
            thread::sleep(std::time::Duration::from_secs_f64(delay));
        }
        last_frame_time = get_time();

        // Render screen and update game state
        clear_background(BLACK);

        state.update();
        state.draw();

        while let Ok(command) = rx.try_recv() {
            match command {
                PaddleCommand::Left  => state.top_paddle.x -= PADDLE_SPEED,
                PaddleCommand::Right => state.top_paddle.x += PADDLE_SPEED,
                PaddleCommand::Stop  => (),
            }
        }

        next_frame().await
    }
}
            
