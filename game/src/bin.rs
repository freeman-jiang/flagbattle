use game::{ArchivedSnapshot, Game, Input, Snapshot, Team, Velocity};
use rkyv::rancor::Error;
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

        // Serialize snapshot using rkyv
        let bytes = rkyv::to_bytes::<Error>(&snapshot).unwrap();
        println!("Serialized snapshot size: {} bytes", bytes.len());

        let archived = rkyv::access::<ArchivedSnapshot, Error>(&bytes).unwrap();
        println!("Archived snapshot: {:?}", &archived.players.len());
        let owned = rkyv::deserialize::<Snapshot, Error>(archived).unwrap();
        dbg!(&owned.players[0]);

        frame_count += 1;

        // Slow down simulation
        thread::sleep(Duration::from_millis(100));

        // Exit after 100 frames
        if frame_count >= 100 {
            break;
        }
    }
}

// fn print_game_state(game: &Game, player1: hecs::Entity, player2: hecs::Entity) {
//     // Print player positions
//     if let Ok(pos) = game.world.get::<&Position>(player1) {
//         println!("Player1: Position({:.1}, {:.1})", pos.x, pos.y);
//     }

//     if let Ok(pos) = game.world.get::<&Position>(player2) {
//         println!("Player2: Position({:.1}, {:.1})", pos.x, pos.y);
//     }

//     // Print flag positions and state
//     if let Ok(pos) = game.world.get::<&Position>(game.red_flag) {
//         if let Ok(flag) = game.world.get::<&game::Flag>(game.red_flag) {
//             let status = if flag.held_by.is_some() {
//                 "CAPTURED"
//             } else {
//                 "HOME"
//             };
//             println!(
//                 "Red Flag: Position({:.1}, {:.1}) - {}",
//                 pos.x, pos.y, status
//             );
//         }
//     }

//     if let Ok(pos) = game.world.get::<&Position>(game.blue_flag) {
//         if let Ok(flag) = game.world.get::<&game::Flag>(game.blue_flag) {
//             let status = if flag.held_by.is_some() {
//                 "CAPTURED"
//             } else {
//                 "HOME"
//             };
//             println!(
//                 "Blue Flag: Position({:.1}, {:.1}) - {}",
//                 pos.x, pos.y, status
//             );
//         }
//     }

//     println!("-------------------------------------");
// }
