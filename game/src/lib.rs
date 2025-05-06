use hecs::{ComponentError, Entity, NoSuchEntity, World};
use std::collections::HashMap;

pub mod public;
pub use public::*;
// Game struct that uses hecs ECS
pub struct Game {
    pub world: World,
    pub red_flag: Entity,
    pub blue_flag: Entity,
    pub player_map: HashMap<String, Entity>,
    systems: GameSystems,
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

// System structs for different game subsystems
pub struct MeleeSystem;
pub struct MovementSystem;
pub struct CollisionSystem;
pub struct FlagSystem;

// Group all systems together
pub struct GameSystems {
    melee_system: MeleeSystem,
    movement_system: MovementSystem,
    collision_system: CollisionSystem,
    flag_system: FlagSystem,
}

impl GameSystems {
    fn new() -> Self {
        Self {
            melee_system: MeleeSystem,
            movement_system: MovementSystem,
            collision_system: CollisionSystem,
            flag_system: FlagSystem,
        }
    }
}

impl MeleeSystem {
    fn run(&self, world: &mut World, dt: f32) {
        let mut players_needing_velocity_reset = Vec::new();

        // Update Melee cooldowns
        for (entity, melee) in world.query_mut::<&mut Melee>() {
            if melee.cooldown > 0.0 {
                melee.cooldown -= dt;

                // When melee attack duration ends
                if melee.active && melee.cooldown < melee.max_cooldown - MELEE_DURATION {
                    melee.active = false;
                    // Mark this player for velocity reset
                    players_needing_velocity_reset.push(entity);
                }
            }
        }

        // Reset velocities of players who finished melee attack
        for entity in players_needing_velocity_reset {
            if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
                velocity.dx = 0.0;
                velocity.dy = 0.0;
            }
        }
    }
}

impl MovementSystem {
    fn run(&self, world: &mut World, dt: f32) {
        // Apply velocities to positions
        for (_entity, (position, velocity)) in world.query_mut::<(&mut Position, &Velocity)>() {
            position.x += velocity.dx * dt;
            position.y += velocity.dy * dt;

            // Boundary checking
            position.x = position.x.max(0.0).min(GRID_X);
            position.y = position.y.max(0.0).min(GRID_Y);
        }
    }
}

impl CollisionSystem {
    fn run(&self, world: &mut World, red_flag: Entity, blue_flag: Entity) -> Vec<Entity> {
        let melee_players: Vec<(Entity, Team)> = world
            .query::<(&Team, &Melee)>()
            .into_iter()
            .filter(|(_, (_, melee))| melee.active)
            .map(|(entity, (team, _))| (entity, team.clone()))
            .collect();

        let all_players: Vec<(Entity, Team)> = world
            .query::<&Team>()
            .into_iter()
            .map(|(entity, team)| (entity, team.clone()))
            .collect();

        // Check for melee collisions and collect players to respawn
        let mut players_to_respawn = Vec::new();

        for (attacker, attacker_team) in melee_players.iter() {
            for (victim, victim_team) in all_players.iter() {
                // Don't check collision with self or same team
                if *attacker == *victim || attacker_team == victim_team {
                    continue;
                }

                // Check collision
                if Self::entities_collide(world, *attacker, *victim) {
                    players_to_respawn.push(*victim);
                }
            }
        }

        // Process respawns directly
        for player_entity in &players_to_respawn {
            // Drop flag if held
            Self::drop_flag_if_held_by(world, *player_entity, red_flag, blue_flag);

            // Respawn player
            Self::respawn_player(world, *player_entity);
        }

        // Return empty vec since we handled respawns directly
        Vec::new()
    }

    fn entities_collide(world: &World, a: Entity, b: Entity) -> bool {
        let Ok(mut query_a) = world.query_one::<(&Position, &Radius)>(a) else {
            return false;
        };

        let Some((pos_a, rad_a)) = query_a.get() else {
            return false;
        };

        let Ok(mut query_b) = world.query_one::<(&Position, &Radius)>(b) else {
            return false;
        };

        let Some((pos_b, rad_b)) = query_b.get() else {
            return false;
        };

        let dx = pos_a.x - pos_b.x;
        let dy = pos_a.y - pos_b.y;

        let dist_sq = dx * dx + dy * dy;
        let min_dist = rad_a.value + rad_b.value;

        dist_sq < min_dist * min_dist
    }

    fn drop_flag_if_held_by(
        world: &mut World,
        player_entity: Entity,
        red_flag: Entity,
        blue_flag: Entity,
    ) {
        let player_id = match world.get::<&Metadata>(player_entity) {
            Ok(metadata) => metadata.id.clone(),
            Err(_) => return,
        };

        // Check and handle red flag
        if let Ok(item) = world.get::<&Item>(red_flag) {
            if let Some(holder) = &item.held_by {
                if holder == &player_id {
                    if let Ok(mut item) = world.get::<&mut Item>(red_flag) {
                        item.held_by = None;

                        // Get player position to drop the flag there
                        if let Ok(player_pos) = world.get::<&Position>(player_entity) {
                            if let Ok(mut flag_pos) = world.get::<&mut Position>(red_flag) {
                                flag_pos.x = player_pos.x;
                                flag_pos.y = player_pos.y;
                            }
                        }
                    }
                }
            }
        }

        // Check and handle blue flag
        if let Ok(item) = world.get::<&Item>(blue_flag) {
            if let Some(holder) = &item.held_by {
                if holder == &player_id {
                    if let Ok(mut item) = world.get::<&mut Item>(blue_flag) {
                        item.held_by = None;

                        // Get player position to drop the flag there
                        if let Ok(player_pos) = world.get::<&Position>(player_entity) {
                            if let Ok(mut flag_pos) = world.get::<&mut Position>(blue_flag) {
                                flag_pos.x = player_pos.x;
                                flag_pos.y = player_pos.y;
                            }
                        }
                    }
                }
            }
        }
    }

    fn respawn_player(world: &mut World, player_entity: Entity) {
        // Get player team
        let team = match world.get::<&Team>(player_entity) {
            Ok(team) => team.clone(),
            Err(_) => return, // Can't respawn if no team
        };

        // Get spawn position
        let spawn_pos = match *team {
            Team::Red => RED_TEAM.spawn_position,
            Team::Blue => BLUE_TEAM.spawn_position,
        };

        // Update position
        if let Ok(mut pos) = world.get::<&mut Position>(player_entity) {
            pos.x = spawn_pos.x;
            pos.y = spawn_pos.y;
        }

        // Reset velocity
        if let Ok(mut vel) = world.get::<&mut Velocity>(player_entity) {
            vel.dx = 0.0;
            vel.dy = 0.0;
        }

        // Reset melee
        if let Ok(mut melee) = world.get::<&mut Melee>(player_entity) {
            melee.active = false;
            melee.cooldown = 0.0;
        }
    }
}

impl FlagSystem {
    fn run(&self, world: &mut World, red_flag: Entity, blue_flag: Entity) {
        // Check for flag pickups
        // This would be implemented here

        // Check for flag captures
        // This would be implemented here

        // Check for flag returns
        // This would be implemented here
    }
}

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
            Radius { value: 5.0 },
        ));

        let blue_flag = world.spawn((
            Item { held_by: None },
            Position {
                x: BLUE_TEAM.flag_position.x,
                y: BLUE_TEAM.flag_position.y,
            },
            Team::Blue,
            Radius { value: 5.0 },
        ));

        Self {
            world,
            red_flag,
            blue_flag,
            player_map: HashMap::new(),
            systems: GameSystems::new(),
        }
    }

    fn collides(&self, a: Entity, b: Entity) -> bool {
        CollisionSystem::entities_collide(&self.world, a, b)
    }

    pub fn make_snapshot(&self) -> Snapshot {
        let players = self
            .world
            .query::<(&Metadata, &Position, &Team, &Velocity, &Melee)>()
            .into_iter()
            .map(|(_, (metadata, position, team, velocity, melee))| Player {
                metadata: metadata.clone(),
                position: position.clone(),
                velocity: velocity.clone(),
                team: team.clone(),
                melee_active: melee.active,
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

    pub fn add_player(&mut self, id: String, team: Team) -> Entity {
        self.remove_player(&id); // Deduplicate players

        let start_x = match team {
            Team::Red => RED_TEAM.spawn_position.x,
            Team::Blue => BLUE_TEAM.spawn_position.x,
        };

        let start_y = match team {
            Team::Red => RED_TEAM.spawn_position.y,
            Team::Blue => BLUE_TEAM.spawn_position.y,
        };

        let player_entity = self.world.spawn((
            Metadata { id: id.clone() },
            Position {
                x: start_x,
                y: start_y,
            },
            Radius { value: 5.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            team,
            Melee {
                active: false,
                cooldown: 0.0,
                max_cooldown: MELEE_COOLDOWN,
            },
        ));

        self.player_map.insert(id, player_entity);
        player_entity
    }

    pub fn remove_player(&mut self, id: &str) {
        if let Some(entity) = self.player_map.remove(id) {
            let _ = self.world.despawn(entity);
        }
    }

    pub fn get_player(&self, id: String) -> &Entity {
        self.player_map.get(&id).unwrap()
    }

    // Set player's movement intent
    pub fn apply_input(&mut self, input: Input) -> Result<(), ComponentError> {
        match input {
            Input::CreatePlayer { team, id } => {
                self.add_player(id, team);
            }
            Input::RemovePlayer { id } => self.remove_player(&id),
            Input::PlayerMove {
                velocity,
                player_id,
            } => {
                let entity = self.get_player(player_id);
                let mut player_velocity = self.world.get::<&mut Velocity>(*entity)?;
                player_velocity.dx = velocity.dx;
                player_velocity.dy = velocity.dy;
            }
            Input::PlayerMelee { player_id } => {
                let player = self.get_player(player_id);

                // Get the velocity values first without keeping the borrow
                let (dx, dy) = {
                    let velocity = self.world.get::<&Velocity>(*player)?;
                    (velocity.dx, velocity.dy)
                }; // Borrow is dropped here

                // Check if the player is moving
                if dx != 0.0 || dy != 0.0 {
                    let mut melee = self.world.get::<&mut Melee>(*player)?;

                    if melee.cooldown <= 0.0 && !melee.active {
                        melee.active = true;
                        melee.cooldown = melee.max_cooldown;

                        // Now we can borrow velocity mutably since the immutable borrow is gone
                        let mut player_velocity = self.world.get::<&mut Velocity>(*player)?;

                        let length = (dx * dx + dy * dy).sqrt();
                        if length > 0.0 {
                            player_velocity.dx = (dx / length) * MELEE_SPEED_MULTIPLIER;
                            player_velocity.dy = (dy / length) * MELEE_SPEED_MULTIPLIER;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // Update the game state based on the delta time (frame-independently)
    pub fn step(&mut self, dt: f32) {
        // Run each system in order
        self.systems.melee_system.run(&mut self.world, dt);
        self.systems.movement_system.run(&mut self.world, dt);

        // Run collision system which now handles respawns internally
        self.systems
            .collision_system
            .run(&mut self.world, self.red_flag, self.blue_flag);

        // Handle flag logic
        self.systems
            .flag_system
            .run(&mut self.world, self.red_flag, self.blue_flag);
    }
}
