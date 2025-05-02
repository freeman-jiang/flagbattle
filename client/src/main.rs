//! Macroquad expose all the logging macros.
//! They will use browser console, android console or just stdout depending on the platform.
//! Those macros are the recommended way to output debug traces and logs.

use macroquad::prelude::*;

// Game constants
const PLAYER_SIZE: f32 = 20.0;
const PLAYER_SPEED: f32 = 200.0;
const WALL_COLOR: Color = DARKGRAY;
const FLAG_SIZE: f32 = 15.0;
const RED_BASE_COLOR: Color = Color::new(0.9, 0.2, 0.2, 0.5);
const BLUE_BASE_COLOR: Color = Color::new(0.2, 0.2, 0.9, 0.5);

// Game entities
struct Player {
    pos: Vec2,
    team: Team,
    has_flag: bool,
}

struct Flag {
    pos: Vec2,
    team: Team,
    is_captured: bool,
    original_pos: Vec2,
}

struct Wall {
    rect: Rect,
}

#[derive(Clone, Copy, PartialEq)]
enum Team {
    Red,
    Blue,
}

struct Game {
    player: Player,
    walls: Vec<Wall>,
    red_flag: Flag,
    blue_flag: Flag,
    red_score: i32,
    blue_score: i32,
}

impl Player {
    fn new(x: f32, y: f32, team: Team) -> Self {
        Self {
            pos: Vec2::new(x, y),
            team,
            has_flag: false,
        }
    }

    fn draw(&self) {
        let color = match self.team {
            Team::Red => RED,
            Team::Blue => BLUE,
        };
        draw_circle(self.pos.x, self.pos.y, PLAYER_SIZE, color);

        // Show if player has flag
        if self.has_flag {
            let flag_color = match self.team {
                Team::Red => BLUE, // Carrying enemy flag
                Team::Blue => RED, // Carrying enemy flag
            };
            draw_circle(self.pos.x, self.pos.y, PLAYER_SIZE / 2.0, flag_color);
        }
    }

    fn update(&mut self, dt: f32, walls: &[Wall]) {
        let mut movement = Vec2::new(0.0, 0.0);

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            movement.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            movement.y += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            movement.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            movement.x += 1.0;
        }

        // Normalize diagonal movement
        if movement.length() > 0.0 {
            movement = movement.normalize();
        }

        // Move player
        let new_pos = self.pos + movement * PLAYER_SPEED * dt;

        // Check wall collisions before applying movement
        let player_rect = Rect::new(
            new_pos.x - PLAYER_SIZE,
            new_pos.y - PLAYER_SIZE,
            PLAYER_SIZE * 2.0,
            PLAYER_SIZE * 2.0,
        );

        let mut collision = false;
        for wall in walls {
            if player_rect.intersect(wall.rect).is_some() {
                collision = true;
                break;
            }
        }

        // Check screen boundaries
        let out_of_bounds = new_pos.x < PLAYER_SIZE
            || new_pos.x > screen_width() - PLAYER_SIZE
            || new_pos.y < PLAYER_SIZE
            || new_pos.y > screen_height() - PLAYER_SIZE;

        if !collision && !out_of_bounds {
            self.pos = new_pos;
        }
    }
}

impl Flag {
    fn new(x: f32, y: f32, team: Team) -> Self {
        let pos = Vec2::new(x, y);
        Self {
            pos,
            team,
            is_captured: false,
            original_pos: pos,
        }
    }

    fn draw(&self) {
        if !self.is_captured {
            let color = match self.team {
                Team::Red => RED,
                Team::Blue => BLUE,
            };

            // Draw flag as a triangle
            let x = self.pos.x;
            let y = self.pos.y;
            draw_triangle(
                Vec2::new(x, y - FLAG_SIZE),
                Vec2::new(x, y + FLAG_SIZE),
                Vec2::new(x + FLAG_SIZE * 1.5, y),
                color,
            );
            draw_line(x, y - FLAG_SIZE, x, y + FLAG_SIZE, 3.0, DARKGRAY);
        }
    }

    fn reset(&mut self) {
        self.pos = self.original_pos;
        self.is_captured = false;
    }
}

impl Game {
    fn new() -> Self {
        // Create walls for the arena
        let mut walls = Vec::new();

        // Central barriers
        walls.push(Wall {
            rect: Rect::new(screen_width() / 2.0 - 10.0, 100.0, 20.0, 200.0),
        });
        walls.push(Wall {
            rect: Rect::new(screen_width() / 2.0 - 10.0, 400.0, 20.0, 200.0),
        });

        // Some additional walls
        walls.push(Wall {
            rect: Rect::new(200.0, 200.0, 150.0, 20.0),
        });
        walls.push(Wall {
            rect: Rect::new(450.0, 500.0, 150.0, 20.0),
        });
        walls.push(Wall {
            rect: Rect::new(200.0, 500.0, 20.0, 100.0),
        });
        walls.push(Wall {
            rect: Rect::new(600.0, 200.0, 20.0, 100.0),
        });

        // Create bases and flags
        let red_flag = Flag::new(100.0, screen_height() / 2.0, Team::Red);
        let blue_flag = Flag::new(screen_width() - 100.0, screen_height() / 2.0, Team::Blue);

        Self {
            player: Player::new(screen_width() / 4.0, screen_height() / 2.0, Team::Red),
            walls,
            red_flag,
            blue_flag,
            red_score: 0,
            blue_score: 0,
        }
    }

    fn update(&mut self, dt: f32) {
        // Update player
        self.player.update(dt, &self.walls);

        // Check flag capture
        let player_rect = Rect::new(
            self.player.pos.x - PLAYER_SIZE,
            self.player.pos.y - PLAYER_SIZE,
            PLAYER_SIZE * 2.0,
            PLAYER_SIZE * 2.0,
        );

        // Player can pick up enemy flag
        if !self.blue_flag.is_captured && self.player.team == Team::Red && !self.player.has_flag {
            let flag_rect = Rect::new(
                self.blue_flag.pos.x - FLAG_SIZE,
                self.blue_flag.pos.y - FLAG_SIZE,
                FLAG_SIZE * 2.0,
                FLAG_SIZE * 2.0,
            );

            if player_rect.intersect(flag_rect).is_some() {
                self.player.has_flag = true;
                self.blue_flag.is_captured = true;
            }
        }

        if !self.red_flag.is_captured && self.player.team == Team::Blue && !self.player.has_flag {
            let flag_rect = Rect::new(
                self.red_flag.pos.x - FLAG_SIZE,
                self.red_flag.pos.y - FLAG_SIZE,
                FLAG_SIZE * 2.0,
                FLAG_SIZE * 2.0,
            );

            if player_rect.intersect(flag_rect).is_some() {
                self.player.has_flag = true;
                self.red_flag.is_captured = true;
            }
        }

        // Move captured flag with player
        if self.blue_flag.is_captured && self.player.team == Team::Red && self.player.has_flag {
            self.blue_flag.pos = self.player.pos;
        }

        if self.red_flag.is_captured && self.player.team == Team::Blue && self.player.has_flag {
            self.red_flag.pos = self.player.pos;
        }

        // Check if player returned to their base with flag
        let red_base_rect = Rect::new(0.0, 0.0, screen_width() * 0.2, screen_height());
        let blue_base_rect = Rect::new(
            screen_width() * 0.8,
            0.0,
            screen_width() * 0.2,
            screen_height(),
        );

        if self.player.team == Team::Red
            && self.player.has_flag
            && red_base_rect.contains(self.player.pos)
        {
            // Red team scores
            self.red_score += 1;
            self.player.has_flag = false;
            self.blue_flag.reset();
        }

        if self.player.team == Team::Blue
            && self.player.has_flag
            && blue_base_rect.contains(self.player.pos)
        {
            // Blue team scores
            self.blue_score += 1;
            self.player.has_flag = false;
            self.red_flag.reset();
        }

        // Press space to switch teams for testing
        if is_key_pressed(KeyCode::Space) {
            self.player.team = match self.player.team {
                Team::Red => Team::Blue,
                Team::Blue => Team::Red,
            };
            self.player.has_flag = false;

            // Reset flag positions if they were captured
            if self.red_flag.is_captured {
                self.red_flag.reset();
            }
            if self.blue_flag.is_captured {
                self.blue_flag.reset();
            }

            // Move player to appropriate side
            match self.player.team {
                Team::Red => self.player.pos.x = screen_width() / 4.0,
                Team::Blue => self.player.pos.x = 3.0 * screen_width() / 4.0,
            }
        }
    }

    fn draw(&self) {
        // Draw bases
        draw_rectangle(
            0.0,
            0.0,
            screen_width() * 0.2,
            screen_height(),
            RED_BASE_COLOR,
        );

        draw_rectangle(
            screen_width() * 0.8,
            0.0,
            screen_width() * 0.2,
            screen_height(),
            BLUE_BASE_COLOR,
        );

        // Draw walls
        for wall in &self.walls {
            draw_rectangle(
                wall.rect.x,
                wall.rect.y,
                wall.rect.w,
                wall.rect.h,
                WALL_COLOR,
            );
        }

        // Draw flags
        self.red_flag.draw();
        self.blue_flag.draw();

        // Draw player
        self.player.draw();

        // Draw scores
        let score_text = format!("Red: {} - Blue: {}", self.red_score, self.blue_score);
        let text_size = 30.0;
        let text_width = measure_text(&score_text, None, text_size as u16, 1.0).width;

        draw_text(
            &score_text,
            screen_width() / 2.0 - text_width / 2.0,
            50.0,
            text_size,
            WHITE,
        );

        // Draw controls info
        draw_text(
            "Use arrow keys or WASD to move",
            20.0,
            screen_height() - 40.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Press SPACE to switch teams",
            20.0,
            screen_height() - 20.0,
            20.0,
            WHITE,
        );
    }
}

#[macroquad::main("Capture The Flag")]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        game.update(dt);
        game.draw();

        next_frame().await
    }
}
