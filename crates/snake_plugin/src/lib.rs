#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod plugin {
    use bevy::prelude::*;
    use snake_game:: {
        snake::snake_functions::*,
        snake::snake_data::*,
        game::game_data::*,
        game::game_functions::*,
        fruit::fruit_logic::*,
        ui::game_ui::*,
    };
    impl Plugin for SnakeGame {
        fn build(&self, app: &mut AppBuilder) {
            app.add_resource(GameState { 
                    // difficulty: 25.0,
                    score: 0,
                    playing: true, 
                    play_area: 600.0,
                    cell_size: 25.0,
                    prev_scores: Vec::new()
                })
                .add_resource(GameTimer(Timer::from_seconds(0.50, true)))
                // .add_resource( Grid {
                //     cells: Vec::new()
                // })
                .add_startup_system(setup.system())
                .add_startup_system(init_ui.system())
                // .add_startup_system(grid_init.system())
                .add_system(restart.system())
                .add_system(game_over.system())
                .add_system(fruit_spawner.system())
                .add_system(snake_movement.system())
                .add_system(snake_collision.system())
                .add_system(grow_tail_listener.system())
                .add_system(move_tail_listener.system())
                .add_system(update_score.system())
                .add_system(update_high_scores.system())
                .add_event::<EventGrowTail>()
                .add_event::<EventMoveTail>()
                .add_event::<EventGameOver>()
                .add_event::<EventRestart>();
        }
    }
    pub struct SnakeGame;

    fn setup(
        mut commands: Commands,
        mut game: ResMut<GameState>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ){
        let cell_size = game.cell_size as f32;
        let snake1_pos = Vec2::new(0.0, 6.0);
        let last1_pos = Vec2::new(-1.0, 6.0);
        let snake_pos = Vec2::new(0.0, -6.0);
        let last_pos = Vec2::new(-1.0, -6.0);
        game.prev_scores.push(0);
        game.prev_scores.push(0);
        game.prev_scores.push(0);
        
        commands
            .spawn(Camera2dComponents::default())
            .spawn(UiCameraComponents::default())
            .spawn(SpriteComponents {
                material: materials.add(get_player_color(Players::Player1).into()),
                transform: Transform::from_translation(Vec3::new(0.0, snake1_pos.y() * game.cell_size as f32, 0.0)),
                sprite: Sprite::new(Vec2::new(cell_size - 2.0, cell_size - 2.0)),
                ..Default::default()
            })
            .with(Snake { 
                player: Players::Player1,
                direction: SnakeDirection::RIGHT,
                position: snake1_pos,
                last_position: last1_pos,
                movement_locked: false,
                next_move: SnakeDirection::RIGHT
            })
            .with(Collider::Snake);
            let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
            let wall_thickness = cell_size;
            let bounds = Vec2::new(game.play_area, game.play_area);
        

        commands
            .spawn(Camera2dComponents::default())
            .spawn(UiCameraComponents::default())
            .spawn(SpriteComponents {
                material: materials.add(get_player_color(Players::Player2).into()),
                transform: Transform::from_translation(Vec3::new(0.0, snake_pos.y() * game.cell_size as f32, 0.0)),
                sprite: Sprite::new(Vec2::new(cell_size - 2.0, cell_size - 2.0)),
                ..Default::default()
            })
            .with(Snake { 
                player: Players::Player2,
                direction: SnakeDirection::RIGHT,
                position: snake_pos,
                last_position: last_pos,
                movement_locked: false,
                next_move: SnakeDirection::RIGHT
            })
            .with(Collider::Snake);
            let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
            let wall_thickness = cell_size;
            let bounds = Vec2::new(game.play_area, game.play_area);
        
        commands
            // left
            .spawn(SpriteComponents {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(-bounds.x() / 2.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
                ..Default::default()
            })
            .with(Collider::Solid)
            // right
            .spawn(SpriteComponents {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
                ..Default::default()
            })
            .with(Collider::Solid)
            // bottom
            .spawn(SpriteComponents {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -bounds.y() / 2.0, 0.0)),
                sprite: Sprite::new(Vec2::new(bounds.x() + wall_thickness, wall_thickness)),
                ..Default::default()
            })
            .with(Collider::Solid)
            // top
            .spawn(SpriteComponents {
                material: wall_material,
                transform: Transform::from_translation(Vec3::new(0.0, bounds.y() / 2.0, 0.0)),
                sprite: Sprite::new(Vec2::new(bounds.x() + wall_thickness, wall_thickness)),
                ..Default::default()
            })
            .with(Collider::Solid);
        println!("SNAKE!");
    }
}

