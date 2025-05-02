use hecs::{ComponentError, Entity, World};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

// Position component
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// Team component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Red,
    Blue,
}

// Player component
#[derive(Debug)]
pub struct Player {
    pub name: String,
}

// Flag component
#[derive(Debug)]
pub struct Flag {
    pub held_by: Option<Entity>,
}

// Game struct that uses hecs ECS
pub struct Game {
    pub world: World,
    pub red_flag: Entity,
    pub blue_flag: Entity,
    pub player_map: HashMap<String, Entity>,
}

const GRID_X: f32 = 200.0;
const GRID_Y: f32 = 100.0;

pub struct TeamConfig {
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
            Flag { held_by: None },
            Position {
                x: RED_TEAM.flag_position.x,
                y: RED_TEAM.flag_position.y,
            },
            Team::Red,
        ));

        let blue_flag = world.spawn((
            Flag { held_by: None },
            Position {
                x: BLUE_TEAM.flag_position.x,
                y: BLUE_TEAM.flag_position.y,
            },
            Team::Blue,
        ));

        Self {
            world,
            red_flag,
            blue_flag,
            player_map: HashMap::new(),
        }
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
            Player { name: name.clone() },
            Position {
                x: start_x,
                y: start_y,
            },
            Velocity { dx: 0.0, dy: 0.0 },
            team,
        ));

        self.player_map.insert(name, player);
        player
    }

    // Set player's movement intent
    pub fn set_player_velocity(
        &mut self,
        player_id: Entity,
        dx: f32,
        dy: f32,
    ) -> Result<(), ComponentError> {
        let mut velocity = self.world.get::<&mut Velocity>(player_id)?;
        velocity.dx = dx;
        velocity.dy = dy;
        Ok(())
    }

    // Update the game state based on the delta time (frame-independently)
    // Delta time is the time elapsed between the current frame and the previous frame in a game loop
    pub fn update(&mut self, dt: f32) {
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
