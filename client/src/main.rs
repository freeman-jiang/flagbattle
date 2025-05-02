use client::console_error_panic_hook;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("WebSocket Game Client - Native Testing Environment");
    println!("==================================================");
    println!("This binary provides a testing environment for the game logic.");
    println!("The actual WebAssembly client can be built with 'wasm-pack build --target web'.");
    println!();

    // Demo of a simple square that can be moved with WASD in the console
    let mut x = 10;
    let mut y = 10;
    let width = 40;
    let height = 20;

    // Track key states (W, A, S, D)
    let mut keys = [false, false, false, false];

    println!("Controls: W/A/S/D to move, Q to quit");
    println!("Press Enter after each key");

    let mut running = true;
    while running {
        // Clear screen (in a primitive way)
        print!("\x1B[2J\x1B[1;1H");

        // Draw the board
        let mut board = vec![vec![' '; width]; height];

        // Draw the player square
        for i in 0..3 {
            for j in 0..3 {
                if y + i < height && x + j < width {
                    board[y + i][x + j] = '■';
                }
            }
        }

        // Draw the board
        for row in &board {
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }

        println!("\nPosition: x={}, y={}", x, y);
        println!("Enter move (W/A/S/D) or Q to quit:");

        // Get input
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        // Process input
        match input.trim().to_lowercase().as_str() {
            "w" => {
                if y > 0 {
                    y -= 1;
                }
                keys = [true, false, false, false];
            }
            "a" => {
                if x > 0 {
                    x -= 1;
                }
                keys = [false, true, false, false];
            }
            "s" => {
                if y < height - 3 {
                    y += 1;
                }
                keys = [false, false, true, false];
            }
            "d" => {
                if x < width - 3 {
                    x += 1;
                }
                keys = [false, false, false, true];
            }
            "q" => running = false,
            _ => {}
        }

        // Simulate what would happen in WebSocket
        let message = format!("{{\"x\":{},\"y\":{}}}", x, y);
        println!("Would send to server: {}", message);

        // Small delay
        thread::sleep(Duration::from_millis(50));
    }

    println!("Native test client closed.");
}

