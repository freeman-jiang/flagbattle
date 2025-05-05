use game::{Game, Input, Snapshot, Team, Velocity};

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    // Create game
    let mut game = Game::new();

    // Add two players
    let player1 = game.add_player("Player1".to_string(), Team::Red);
    let player2 = game.add_player("Player2".to_string(), Team::Blue);

    // Set initial velocity for player1 (moving toward blue flag)
    game.apply_input(Input::PlayerMove {
        velocity: Velocity { dx: 1.0, dy: 1.0 },
        player_id: player1.to_bits(),
    })
    .unwrap();

    // Simple game loop
    let mut last_time = Instant::now();
    let mut frame_count = 0;

    loop {
        // Calculate delta time
        let current_time = Instant::now();
        let dt = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // Update game state
        game.step(dt);

        // Create a snapshot of the current game state
        let snapshot = game.make_snapshot();

        // Serialize snapshot using messagepack
        let buf = rmp_serde::to_vec(&snapshot).unwrap();
        println!("Serialized snapshot size: {} bytes", buf.len());

        let owned = rmp_serde::from_slice::<Snapshot>(&buf).unwrap();
        dbg!(&owned.players[0].position);

        frame_count += 1;

        // Slow down simulation
        thread::sleep(Duration::from_millis(100));

        // Exit after 100 frames
        if frame_count >= 100 {
            break;
        }
    }
}
