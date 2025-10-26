use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use pathfinding::prelude::*;
use rand;

use crate::{
    characters::thunwa::{PlayerHealth, Thunwa},
    terrains::{DynamicsZOrder, GRID_SIZE, MAP_SIZE},
};

// Collision groups for physics
const ZOMBIE_COLLISION_GROUP: u32 = 0b0001;
const PLAYER_COLLISION_GROUP: u32 = 0b0010;
const WALL_COLLISION_GROUP: u32 = 0b0100;

#[derive(Component)]
pub struct Zombie {
    pub speed: f32,
    pub health: f32,
    pub damage: f32,
    pub attack_cooldown: Timer,
    pub path_update_timer: Timer,
    pub current_path: Option<Vec<(i32, i32)>>,
    pub current_target_index: usize,
    pub stuck_timer: Timer,
    pub last_position: Vec2,
    pub avoidance_direction: Vec2,
    pub personal_space_timer: Timer,
    pub target_offset: Vec2,
}

#[derive(Component)]
pub struct ZombieCollider;

#[derive(Component)]
pub struct ZombieSensor;

#[derive(Component)]
pub struct ZombieTarget;

#[derive(Resource)]
pub struct ZombieConfig {
    pub spawn_positions: Vec<Vec2>,
    pub max_zombies: usize,
}

#[derive(Resource)]
pub struct PlayerHealthDisplay {
    pub last_health: f32,
}

impl Default for PlayerHealthDisplay {
    fn default() -> Self {
        Self { last_health: 100.0 }
    }
}

impl Default for ZombieConfig {
    fn default() -> Self {
        Self {
            spawn_positions: vec![
                Vec2::new(-400.0, 200.0),
                Vec2::new(400.0, -200.0),
                Vec2::new(-200.0, -300.0),
                Vec2::new(300.0, 300.0),
            ],
            max_zombies: 2,
        }
    }
}

pub fn setup_zombies(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Insert zombie configuration resource
    commands.insert_resource(ZombieConfig::default());
    commands.insert_resource(PlayerHealthDisplay::default());

    // Load zombie sprite
    let zombie_sprite = asset_server.load("characters/zombie/zombie_sprite.aseprite");

    // Spawn initial zombies
    spawn_zombie(
        &mut commands,
        zombie_sprite.clone(),
        Vec2::new(-400.0, 200.0),
    );
    spawn_zombie(&mut commands, zombie_sprite, Vec2::new(400.0, -200.0));
}

fn spawn_zombie(commands: &mut Commands, sprite: Handle<Aseprite>, position: Vec2) {
    // Use the first available animation from the sprite
    let animation = Animation::default().with_speed(1.0);

    commands
        .spawn((
            Zombie {
                speed: 80.0,
                health: 100.0,
                damage: 25.0,
                attack_cooldown: Timer::from_seconds(1.5, TimerMode::Repeating),
                path_update_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                current_path: None,
                current_target_index: 0,
                stuck_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                last_position: position,
                avoidance_direction: Vec2::ZERO,
                personal_space_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                target_offset: Vec2::ZERO,
            },
            DynamicsZOrder,
            AseAnimation {
                aseprite: sprite,
                animation,
            },
            Sprite::default(),
            RigidBody::Dynamic,
            ZombieTarget,
        ))
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Transform::from_xyz(position.x, position.y, 15.0))
        .with_children(|parent| {
            parent
                .spawn((
                    ZombieCollider,
                    Collider::capsule_y(6., 6.), // Smaller collider to prevent getting stuck
                    CollisionGroups::new(
                        Group::from_bits(ZOMBIE_COLLISION_GROUP).unwrap(),
                        Group::from_bits(PLAYER_COLLISION_GROUP | WALL_COLLISION_GROUP).unwrap(),
                    ),
                    // Add sensor for collision detection
                    ActiveEvents::COLLISION_EVENTS,
                ))
                .insert(Transform::from_xyz(0.0, -16.0, 0.));
        });
}

pub fn despawn_zombies(mut commands: Commands, zombie_query: Query<Entity, With<Zombie>>) {
    for entity in zombie_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn world_to_grid(world_pos: Vec2) -> (i32, i32) {
    let grid_x = ((world_pos.x + (MAP_SIZE.x as f32 * GRID_SIZE) / 2.0) / GRID_SIZE).round() as i32;
    let grid_y = ((world_pos.y + (MAP_SIZE.y as f32 * GRID_SIZE) / 2.0) / GRID_SIZE).round() as i32;
    (grid_x, grid_y)
}

fn is_position_blocked(
    grid_pos: (i32, i32),
    zombie_entities: &[(Vec2, Vec2)],
    self_pos: Option<Vec2>,
) -> bool {
    // Check if position is blocked by another zombie
    let world_pos = grid_to_world(grid_pos);

    for (zombie_pos, _zombie_size) in zombie_entities {
        // Skip checking against self
        if let Some(self_p) = self_pos {
            if (self_p - *zombie_pos).length() < 1.0 {
                continue;
            }
        }

        let distance = (world_pos - *zombie_pos).length();
        if distance < 48.0 {
            // Increased personal space
            return true;
        }
    }

    // Check map boundaries
    if grid_pos.0 < 0
        || grid_pos.0 >= MAP_SIZE.x as i32
        || grid_pos.1 < 0
        || grid_pos.1 >= MAP_SIZE.y as i32
    {
        return true;
    }

    false
}

fn calculate_personal_space_offset(zombie_pos: Vec2, other_zombies: &[(Vec2, Vec2)]) -> Vec2 {
    let mut separation_force = Vec2::ZERO;
    const SEPARATION_RADIUS: f32 = 60.0;
    const SEPARATION_STRENGTH: f32 = 2.0;

    for (other_pos, _size) in other_zombies {
        let distance = (zombie_pos - *other_pos).length();
        if distance > 0.0 && distance < SEPARATION_RADIUS {
            let direction = (zombie_pos - *other_pos).normalize_or_zero();
            let strength = (1.0 - distance / SEPARATION_RADIUS) * SEPARATION_STRENGTH;
            separation_force += direction * strength;
        }
    }

    separation_force
}

fn generate_unique_target_offset(existing_offsets: &[Vec2]) -> Vec2 {
    const MIN_OFFSET_DISTANCE: f32 = 30.0;
    const MAX_ATTEMPTS: usize = 10;

    for _ in 0..MAX_ATTEMPTS {
        let offset = Vec2::new(
            (rand::random::<f32>() - 0.5) * 60.0,
            (rand::random::<f32>() - 0.5) * 60.0,
        );

        let is_unique = existing_offsets
            .iter()
            .all(|&existing| (offset - existing).length() > MIN_OFFSET_DISTANCE);

        if is_unique {
            return offset;
        }
    }

    Vec2::ZERO
}

fn find_alternative_path(
    start: Vec2,
    goal: Vec2,
    zombie_entities: &[(Vec2, Vec2)],
    self_pos: Vec2,
) -> Option<Vec<(i32, i32)>> {
    let start_grid = world_to_grid(start);
    let goal_grid = world_to_grid(goal);

    let result = astar(
        &start_grid,
        |&(x, y)| {
            let mut neighbors = Vec::new();
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    let neighbor_grid = (nx, ny);

                    // Check if neighbor is blocked
                    if !is_position_blocked(neighbor_grid, zombie_entities, Some(self_pos)) {
                        let cost = if dx.abs() + dy.abs() == 2 { 14 } else { 10 };
                        neighbors.push((neighbor_grid, cost));
                    }
                }
            }
            neighbors
        },
        |&(x, y)| ((x - goal_grid.0).abs() + (y - goal_grid.1).abs()) * 10,
        |&(x, y)| x == goal_grid.0 && y == goal_grid.1,
    );

    result.map(|(path, _)| path)
}

fn grid_to_world(grid_pos: (i32, i32)) -> Vec2 {
    let world_x = grid_pos.0 as f32 * GRID_SIZE - (MAP_SIZE.x as f32 * GRID_SIZE) / 2.0;
    let world_y = grid_pos.1 as f32 * GRID_SIZE - (MAP_SIZE.y as f32 * GRID_SIZE) / 2.0;
    Vec2::new(world_x, world_y)
}

// Old find_path function removed - replaced by find_alternative_path

pub fn update_zombie_ai(
    mut zombie_query: Query<(&mut Zombie, &Transform, &mut Velocity, &mut AseAnimation)>,
    zombie_transforms: Query<&Transform, With<Zombie>>,
    thunwa_query: Query<&Transform, (With<Thunwa>, Without<Zombie>)>,
    time: Res<Time>,
) {
    if let Ok(thunwa_transform) = thunwa_query.single() {
        let thunwa_pos = thunwa_transform.translation.xy();

        // Collect zombie positions for collision avoidance
        let zombie_entities: Vec<(Vec2, Vec2)> = zombie_transforms
            .iter()
            .map(|t| (t.translation.xy(), Vec2::new(12.0, 12.0))) // zombie size
            .collect();

        // Collect existing target offsets to ensure uniqueness
        let existing_offsets: Vec<Vec2> = zombie_query
            .iter()
            .map(|(z, _, _, _)| z.target_offset)
            .collect();

        for (mut zombie, zombie_transform, mut velocity, mut animation) in zombie_query.iter_mut() {
            let zombie_pos = zombie_transform.translation.xy();
            let distance_to_player = (thunwa_pos - zombie_pos).length();

            // Update timers
            zombie.path_update_timer.tick(time.delta());
            zombie.stuck_timer.tick(time.delta());
            zombie.personal_space_timer.tick(time.delta());

            // Check if zombie is stuck (not moving much)
            let distance_moved = (zombie_pos - zombie.last_position).length();
            if distance_moved < 5.0 && zombie.stuck_timer.finished() {
                // Zombie is stuck, try to find a new path or use avoidance
                zombie.current_path = None;
                zombie.avoidance_direction = Vec2::new(
                    (rand::random::<f32>() - 0.5) * 2.0,
                    (rand::random::<f32>() - 0.5) * 2.0,
                )
                .normalize_or_zero();
            }
            zombie.last_position = zombie_pos;

            // Generate unique target offset periodically
            if zombie.personal_space_timer.just_finished() || zombie.target_offset == Vec2::ZERO {
                zombie.target_offset = generate_unique_target_offset(&existing_offsets);
            }

            // Calculate personal space separation
            let separation_force = calculate_personal_space_offset(zombie_pos, &zombie_entities);

            // Recalculate path if timer finished or no path exists
            if zombie.path_update_timer.just_finished() || zombie.current_path.is_none() {
                let modified_goal = thunwa_pos + zombie.target_offset;
                if let Some(path) =
                    find_alternative_path(zombie_pos, modified_goal, &zombie_entities, zombie_pos)
                {
                    zombie.current_path = Some(path);
                    zombie.current_target_index = 0;
                    zombie.avoidance_direction = Vec2::ZERO;
                }
            }

            // Move along path
            if let Some(ref path) = zombie.current_path {
                if zombie.current_target_index < path.len() {
                    let target_grid = path[zombie.current_target_index];
                    let target_world = grid_to_world(target_grid);
                    let mut direction = (target_world - zombie_pos).normalize_or_zero();

                    // Add avoidance if stuck
                    if zombie.avoidance_direction != Vec2::ZERO {
                        direction = direction * 0.7 + zombie.avoidance_direction * 0.3;
                        direction = direction.normalize_or_zero();
                    }

                    // Add personal space separation
                    if separation_force != Vec2::ZERO {
                        direction = direction * 0.8 + separation_force * 0.2;
                        direction = direction.normalize_or_zero();
                    }

                    if direction != Vec2::ZERO {
                        velocity.linvel = direction * zombie.speed;
                        animation.animation = Animation::default().with_speed(1.0);

                        // Check if reached target
                        if (target_world - zombie_pos).length() < GRID_SIZE / 2.0 {
                            zombie.current_target_index += 1;
                            zombie.avoidance_direction = Vec2::ZERO;
                        }
                    } else {
                        velocity.linvel = Vec2::ZERO;
                        animation.animation = Animation::default().with_speed(1.0);
                    }
                } else {
                    // Reached end of path, stop
                    velocity.linvel = Vec2::ZERO;
                    animation.animation = Animation::default().with_speed(1.0);
                    zombie.current_path = None;
                }
            } else {
                // No path, use direct movement with avoidance
                if distance_to_player > 50.0 {
                    let mut direction =
                        (thunwa_pos + zombie.target_offset - zombie_pos).normalize_or_zero();

                    // Add avoidance if needed
                    if zombie.avoidance_direction != Vec2::ZERO {
                        direction = direction * 0.7 + zombie.avoidance_direction * 0.3;
                        direction = direction.normalize_or_zero();
                    }

                    // Add personal space separation
                    if separation_force != Vec2::ZERO {
                        direction = direction * 0.8 + separation_force * 0.2;
                        direction = direction.normalize_or_zero();
                    }

                    velocity.linvel = direction * zombie.speed;
                    animation.animation = Animation::default().with_speed(1.0);
                } else {
                    velocity.linvel = Vec2::ZERO;
                    animation.animation = Animation::default().with_speed(1.0);
                }
            }
        }
    }
}

pub fn zombie_attack_system(
    mut zombie_query: Query<(&mut Zombie, &Transform)>,
    mut thunwa_query: Query<(Entity, &mut Thunwa, &Transform), (With<Thunwa>, Without<Zombie>)>,
    mut collision_events: EventReader<CollisionEvent>,
    time: Res<Time>,
) {
    if let Ok((_thunwa_entity, mut thunwa, thunwa_transform)) = thunwa_query.single_mut() {
        let thunwa_pos = thunwa_transform.translation.xy();

        // Update attack cooldowns
        for (mut zombie, zombie_transform) in zombie_query.iter_mut() {
            zombie.attack_cooldown.tick(time.delta());

            let zombie_pos = zombie_transform.translation.xy();
            let distance = (thunwa_pos - zombie_pos).length();

            // Check if zombie is close enough to attack
            if distance < 40.0 && zombie.attack_cooldown.finished() {
                // Deal damage to player
                thunwa.health = (thunwa.health - zombie.damage).max(0.0);
                println!(
                    "Zombie attacks player for {} damage! Player health: {}/{}",
                    zombie.damage, thunwa.health, thunwa.max_health
                );
                zombie.attack_cooldown.reset();

                // You could add a damage flash effect here

                // Check if player is dead
                if thunwa.health <= 0.0 {
                    println!("Player has been defeated by zombies!");
                    // You could trigger game over state here
                }
            }
        }

        // Alternative: Use collision events for attack detection
        for collision_event in collision_events.read() {
            if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
                // Check if collision is between zombie and player
                let (zombie_entity, player_entity) =
                    if zombie_query.contains(*entity1) && thunwa_query.contains(*entity2) {
                        (*entity1, *entity2)
                    } else if zombie_query.contains(*entity2) && thunwa_query.contains(*entity1) {
                        (*entity2, *entity1)
                    } else {
                        continue;
                    };

                if let (Ok((mut zombie, _)), Ok((_, mut thunwa, _))) = (
                    zombie_query.get_mut(zombie_entity),
                    thunwa_query.get_mut(player_entity),
                ) {
                    if zombie.attack_cooldown.finished() {
                        thunwa.health = (thunwa.health - zombie.damage).max(0.0);
                        println!(
                            "Zombie hits player via collision! Player health: {}/{}",
                            thunwa.health, thunwa.max_health
                        );
                        zombie.attack_cooldown.reset();

                        // Check if player is dead
                        if thunwa.health <= 0.0 {
                            println!("Player has been defeated by zombies!");
                            // You could trigger game over state here
                        }
                    }
                }
            }
        }
    }
}

pub fn update_zombie_animation_direction(mut zombie_query: Query<(&Velocity, &mut AseAnimation)>) {
    for (velocity, mut animation) in zombie_query.iter_mut() {
        if velocity.linvel.length() > 0.1 {
            // Update animation based on movement direction
            // Using default animation since that's the only animation available
            animation.animation = Animation::default().with_speed(1.0);
        }
    }
}

pub fn update_player_health_display(
    thunwa_query: Query<&Thunwa, With<PlayerHealth>>,
    mut health_display: ResMut<PlayerHealthDisplay>,
) {
    if let Ok(thunwa) = thunwa_query.single() {
        // Only display health when it changes and is less than max
        if thunwa.health != health_display.last_health && thunwa.health < thunwa.max_health {
            println!("Player Health: {}/{}", thunwa.health, thunwa.max_health);
            health_display.last_health = thunwa.health;
        }
    }
}
