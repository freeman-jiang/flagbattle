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
    pub score: HashMap<Team, u32>,
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
            score: HashMap::from([(Team::Red, 0), (Team::Blue, 0)]),
        }
    }

    fn step_melee_system(&mut self, dt: f32) {
        let mut players_needing_velocity_reset = Vec::new();

        // Update Melee cooldowns
        for (entity, melee) in self.world.query_mut::<&mut Melee>() {
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
            if let Ok(mut velocity) = self.world.get::<&mut Velocity>(entity) {
                velocity.dx = 0.0;
                velocity.dy = 0.0;
            }
        }
    }

    fn step_movement_system(&mut self, dt: f32) {
        // Apply velocities to positions
        for (_entity, (position, velocity)) in self.world.query_mut::<(&mut Position, &Velocity)>()
        {
            position.x += velocity.dx * dt;
            position.y += velocity.dy * dt;

            // Boundary checking
            position.x = position.x.max(0.0).min(GRID_X);
            position.y = position.y.max(0.0).min(GRID_Y);
        }
    }

    fn step_collision_system(&mut self) -> Vec<Entity> {
        let melee_players: Vec<(Entity, Team)> = self
            .world
            .query::<(&Team, &Melee)>()
            .into_iter()
            .filter(|(_, (_, melee))| melee.active)
            .map(|(entity, (team, _))| (entity, team.clone()))
            .collect();

        // Get all entities with a team and a metadata because only players have metadata.
        let all_players: Vec<(Entity, Team)> = self
            .world
            .query::<(&Team, &Metadata)>()
            .into_iter()
            .map(|(entity, (team, _))| (entity, team.clone()))
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
                if self.entities_collide(*attacker, *victim) {
                    players_to_respawn.push(*victim);
                }
            }
        }

        // Process respawns directly
        for player_entity in &players_to_respawn {
            // Drop flag if held
            self.drop_flag_if_held_by(*player_entity);

            // Respawn player
            self.respawn_player(*player_entity);
        }

        // Return empty vec since we handled respawns directly
        Vec::new()
    }

    fn entities_collide(&self, a: Entity, b: Entity) -> bool {
        let Ok(mut query_a) = self.world.query_one::<(&Position, &Radius)>(a) else {
            return false;
        };

        let Some((pos_a, rad_a)) = query_a.get() else {
            return false;
        };

        let Ok(mut query_b) = self.world.query_one::<(&Position, &Radius)>(b) else {
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

    fn drop_flag_if_held_by(&mut self, player_entity: Entity) {
        let player_id = match self.world.get::<&Metadata>(player_entity) {
            Ok(metadata) => metadata.id.clone(),
            Err(_) => return,
        };

        // Check and handle red flag
        if let Ok(item) = self.world.get::<&Item>(self.red_flag) {
            if let Some(holder) = &item.held_by {
                if holder == &player_id {
                    if let Ok(mut item) = self.world.get::<&mut Item>(self.red_flag) {
                        item.held_by = None;

                        // Get player position to drop the flag there
                        if let Ok(player_pos) = self.world.get::<&Position>(player_entity) {
                            if let Ok(mut flag_pos) = self.world.get::<&mut Position>(self.red_flag)
                            {
                                flag_pos.x = player_pos.x;
                                flag_pos.y = player_pos.y;
                            }
                        }
                    }
                }
            }
        }

        // Check and handle blue flag
        if let Ok(item) = self.world.get::<&Item>(self.blue_flag) {
            if let Some(holder) = &item.held_by {
                if holder == &player_id {
                    if let Ok(mut item) = self.world.get::<&mut Item>(self.blue_flag) {
                        item.held_by = None;

                        // Get player position to drop the flag there
                        if let Ok(player_pos) = self.world.get::<&Position>(player_entity) {
                            if let Ok(mut flag_pos) =
                                self.world.get::<&mut Position>(self.blue_flag)
                            {
                                flag_pos.x = player_pos.x;
                                flag_pos.y = player_pos.y;
                            }
                        }
                    }
                }
            }
        }
    }

    fn respawn_player(&mut self, player_entity: Entity) {
        // Get player team
        let team = match self.world.get::<&Team>(player_entity) {
            Ok(team) => team.clone(),
            Err(_) => return, // Can't respawn if no team
        };

        // Get spawn position
        let spawn_pos = match *team {
            Team::Red => RED_TEAM.spawn_position,
            Team::Blue => BLUE_TEAM.spawn_position,
        };

        // Update position
        if let Ok(mut pos) = self.world.get::<&mut Position>(player_entity) {
            pos.x = spawn_pos.x;
            pos.y = spawn_pos.y;
        }

        // Reset velocity
        if let Ok(mut vel) = self.world.get::<&mut Velocity>(player_entity) {
            vel.dx = 0.0;
            vel.dy = 0.0;
        }

        // Reset melee
        if let Ok(mut melee) = self.world.get::<&mut Melee>(player_entity) {
            melee.active = false;
            melee.cooldown = 0.0;
        }
    }

    fn step_flag_system(&mut self) {
        // Check for flag pickups
        // This would be implemented here

        let player_entities = self.player_map.values().collect::<Vec<_>>();

        for player_entity in player_entities {
            // Get player team
            let player_team = match self.world.get::<&Team>(*player_entity) {
                Ok(team) => team,
                Err(_) => continue, // Skip if no team
            };

            // Check for red flag capture (only by blue team)
            if *player_team == Team::Blue && self.entities_collide(*player_entity, self.red_flag) {
                // Update flag position to follow the player
                if let (Ok(player_pos), Ok(mut flag_pos)) = (
                    self.world.get::<&Position>(*player_entity),
                    self.world.get::<&mut Position>(self.red_flag),
                ) {
                    flag_pos.x = player_pos.x;
                    flag_pos.y = player_pos.y;
                }

                // Did blue team bring the flag to the base?
                // TODO: Make it blue flag spawn instead of blue flag position
                if self.entities_collide(*player_entity, self.blue_flag) {
                    self.score.entry(Team::Blue).and_modify(|count| *count += 1);

                    // Reset red flag position
                    let mut red_flag = self.world.get::<&mut Position>(self.red_flag).unwrap();
                    red_flag.x = RED_TEAM.flag_position.x;
                    red_flag.y = RED_TEAM.flag_position.y;
                }
            }

            // Check for blue flag capture (only by red team)
            if *player_team == Team::Red && self.entities_collide(*player_entity, self.blue_flag) {
                // Update flag position to follow the player
                if let (Ok(player_pos), Ok(mut flag_pos)) = (
                    self.world.get::<&Position>(*player_entity),
                    self.world.get::<&mut Position>(self.blue_flag),
                ) {
                    flag_pos.x = player_pos.x;
                    flag_pos.y = player_pos.y;
                }

                // Did blue team bring the flag to the base?
                // TODO: Make it blue flag spawn instead of blue flag position
                if self.entities_collide(*player_entity, self.red_flag) {
                    self.score.entry(Team::Red).and_modify(|count| *count += 1);

                    // Reset blue flag position
                    let mut blue_flag = self.world.get::<&mut Position>(self.blue_flag).unwrap();
                    blue_flag.x = BLUE_TEAM.flag_position.x;
                    blue_flag.y = BLUE_TEAM.flag_position.y;
                }
            }
        }

        // Check for flag captures
        // This would be implemented here

        // Check for flag returns
        // This would be implemented here
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

        Snapshot {
            players,
            flags,
            score: self.score.clone(),
        }
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
                let melee = self.world.get::<&Melee>(*entity)?;

                let mut player_velocity = self.world.get::<&mut Velocity>(*entity)?;
                if melee.active {
                } else if melee.cooldown > 0. {
                    player_velocity.dx = velocity.dx * MELEE_COOLDOWN_SPEED_MULTIPLIER;
                    player_velocity.dy = velocity.dy * MELEE_COOLDOWN_SPEED_MULTIPLIER;
                } else {
                    player_velocity.dx = velocity.dx;
                    player_velocity.dy = velocity.dy;
                }
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
        self.step_melee_system(dt);
        self.step_movement_system(dt);

        // Run collision system which now handles respawns internally
        self.step_collision_system();

        // Handle flag logic
        self.step_flag_system();
    }
}
