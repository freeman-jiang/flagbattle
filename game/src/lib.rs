use hecs::{ComponentError, Entity, World};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, num::NonZeroU64};

pub mod public;
pub use public::*;
// Game struct that uses hecs ECS
pub struct Game {
    pub world: World,
    pub red_flag: Entity,
    pub blue_flag: Entity,
    pub player_map: HashMap<String, Entity>,
}

const GRID_X: f32 = 200.0;
const GRID_Y: f32 = 100.0;

struct TeamConfig {
    pub flag_position: Position,
    pub spawn_position: Position,
}

const RED_TEAM: TeamConfig = TeamConfig {
    flag_position: Position { x: 10.0, y: 50.0 },
    spawn_position: Position { x: 5.0, y: 5.0 },
};

const BLUE_TEAM: TeamConfig = TeamConfig {
    flag_position: Position { x: 190.0, y: 50.0 },
    spawn_position: Position { x: 195.0, y: 95.0 },
};

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();

        // Create flags
        let red_flag = world.spawn((
            Item { held_by: None },
            Position {
                x: RED_TEAM.flag_position.x,
                y: RED_TEAM.flag_position.y,
            },
            Team::Red,
            Radius { values: 5.0 },
        ));

        let blue_flag = world.spawn((
            Item { held_by: None },
            Position {
                x: BLUE_TEAM.flag_position.x,
                y: BLUE_TEAM.flag_position.y,
            },
            Team::Blue,
            Radius { values: 5.0 },
        ));

        Self {
            world,
            red_flag,
            blue_flag,
            player_map: HashMap::new(),
        }
    }

    fn collides(&self, a: Entity, b: Entity) -> bool {
        let Ok((pos_a, rad_a)) = self.world.query_one::<(&Position, &Radius)>(a) else {
            return false;
        };
        let Ok((pos_b, rad_b)) = self.world.query_one::<(&Postion, &Radius)>(b) else {
            return false;
        };

        let dx = pos_a.x - pos_b.x;
        let dy = pos_a.y - pos_b.y;

        let dist_sq = dx * dx + dy * dy;
        let min_dist = rad_a.value + rad_b.value;
        return dist_sq < min_dist * min_dist;
    }

    pub fn make_snapshot(&self) -> Snapshot {
        let players = self
            .world
            .query::<(&Metadata, &Position, &Team, &Velocity)>()
            .into_iter()
            .map(|(_, (metadata, position, team, velocity))| Player {
                metadata: metadata.clone(),
                position: position.clone(),
                velocity: velocity.clone(),
                team: team.clone(),
            })
            .collect();

        let flags = self
            .world
            .query::<(&Item, &Position, &Team)>()
            .into_iter()
            .map(|(_, (item, position, team))| Flag {
                position: position.clone(),
                team: team.clone(),
                item: item.clone(),
            })
            .collect();

        Snapshot { players, flags }
    }

    pub fn add_player(&mut self, name: String, team: Team) -> Entity {
        let start_x = match team {
            Team::Red => RED_TEAM.spawn_position.x,
            Team::Blue => BLUE_TEAM.spawn_position.x,
        };

        let start_y = match team {
            Team::Red => RED_TEAM.spawn_position.y,
            Team::Blue => BLUE_TEAM.spawn_position.y,
        };

        let player = self.world.spawn((
            Metadata { name: name.clone() },
            Position {
                x: start_x,
                y: start_y,
            },
            Radius { values: 5.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            team,
        ));

        self.player_map.insert(name, player);
        player
    }

    // Set player's movement intent
    pub fn apply_input(&mut self, input: Input) -> Result<(), ComponentError> {
        match input {
            Input::PlayerMove {
                velocity,
                player_id,
            } => {
                let entity = Entity::from_bits(player_id.get()).unwrap();
                let mut player_velocity = self.world.get::<&mut Velocity>(entity)?;
                player_velocity.dx = velocity.dx;
                player_velocity.dy = velocity.dy;
            }
        }

        Ok(())
    }

    // Update the game state based on the delta time (frame-independently)
    // Delta time is the time elapsed between the current frame and the previous frame in a game loop
    pub fn step(&mut self, dt: f32) {
        // Apply velocities to positions
        for (_entity, (position, velocity)) in self.world.query_mut::<(&mut Position, &Velocity)>()
        {
            position.x += velocity.dx * dt;
            position.y += velocity.dy * dt;

            // Optional: add simple boundary checking
            position.x = position.x.max(0.0).min(GRID_X);
            position.y = position.y.max(0.0).min(GRID_Y);
        }

        // Check for flag captures
        // This would iterate through all players and check if they can capture flags

        // Check for flag returns
        // This would check if players with flags have reached their base
    }
}
