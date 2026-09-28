use bevy::{
    asset::Handle,
    math::primitives::Rectangle,
    prelude::{
        AssetServer, Assets, Changed, Children, Click, Color, ColorMaterial, Commands, Component,
        Entity, Image, Mesh, Mesh2d, MeshMaterial2d, On, Pickable, Pointer, Query, Res, ResMut,
        Resource, Sprite, SpriteImageMode, SpriteScalingMode, Transform, Vec2, Visibility, With,
    },
    time::{Time, Timer, TimerMode},
};
use rand::seq::SliceRandom;

#[derive(Resource)]
pub struct MismatchTimer {
    timer: Timer,
    active: bool,
}

#[derive(Resource, Default)]
pub struct GameOver {
    pub visible: bool,
}

impl Default for MismatchTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            active: false,
        }
    }
}

#[derive(Component)]
pub enum CardPart {
    Label,
    MatchedBorder,
    WrongBorder,
}

#[derive(Debug, Component)]
pub struct ItemGrid {
    key: String,
    is_revealed: bool,
    is_matched: bool,
    is_mismatched: bool,
}

const CELL_SIZE: f32 = 120.0;
const SPACING_SIZE: f32 = 20.0;
const BORDER_SIZE: f32 = 5.0;

const STEP_SIZE: f32 = CELL_SIZE + 2.0 * BORDER_SIZE + SPACING_SIZE;

const PAIRS: usize = 12;

pub fn create(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let card_mesh = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));
    let card_material = materials.add(Color::hsl(144., 0.14, 0.79));
    let border_mesh = meshes.add(Rectangle::new(
        CELL_SIZE + 2.0 * BORDER_SIZE,
        CELL_SIZE + 2.0 * BORDER_SIZE,
    ));
    let matched_material = materials.add(Color::hsl(144.6, 0.81, 0.61));
    let wrong_material = materials.add(Color::hsl(345., 0.81, 0.61));

    let (cols, rows) = grid_size(PAIRS);

    let mut cards: Vec<(String, String)> = (0..PAIRS)
        .flat_map(|i| {
            let key = format!("pair-{i}");
            let img = format!("imgs/{}.png", i + 1);
            [(key.clone(), img.clone()), (key, img)]
        })
        .collect();
    cards.shuffle(&mut rand::rng());

    for x in 0..cols {
        for y in 0..rows {
            let pos_x = (x as f32 - (cols as f32 - 1.0) / 2.0) * STEP_SIZE;
            let pos_y = (y as f32 - (rows as f32 - 1.0) / 2.0) * STEP_SIZE;
            let (key, img_path) = cards[x * rows + y].clone();
            let image: Handle<Image> = asset_server.load(img_path.clone());

            spawn_grid_item(
                &mut commands,
                key,
                image,
                card_mesh.clone(),
                card_material.clone(),
                border_mesh.clone(),
                matched_material.clone(),
                wrong_material.clone(),
                pos_x,
                pos_y,
            );
        }
    }
}

pub fn remove(mut commands: Commands, query: Query<Entity, With<ItemGrid>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_item_change(
    items: Query<(&ItemGrid, &Children), Changed<ItemGrid>>,
    mut parts: Query<(&mut Visibility, &CardPart)>,
) {
    for (item, children) in &items {
        for child in children.iter() {
            let Ok((mut visibility, part)) = parts.get_mut(*child) else {
                continue;
            };
            let visible = match part {
                CardPart::Label => item.is_revealed,
                CardPart::MatchedBorder => item.is_matched,
                CardPart::WrongBorder => item.is_mismatched,
            };
            *visibility = if visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
pub fn check_pair(
    time: Res<Time>,
    mut mismatch: ResMut<MismatchTimer>,
    mut items: Query<(Entity, &mut ItemGrid)>,
) {
    if mismatch.active {
        mismatch.timer.tick(time.delta());

        if mismatch.timer.just_finished() {
            for (_, mut item) in &mut items {
                if item.is_revealed && !item.is_matched {
                    item.is_revealed = false;
                    item.is_mismatched = false;
                }
            }
            mismatch.active = false;
        }
        return;
    }

    // Фаза проверки: собрать открытые, но ещё не найденные карточки
    let open: Vec<Entity> = items
        .iter()
        .filter(|(_, item)| item.is_revealed && !item.is_matched)
        .map(|(entity, _)| entity)
        .collect();

    if open.len() != 2 {
        return;
    }

    let (Ok((_, a)), Ok((_, b))) = (items.get(open[0]), items.get(open[1])) else {
        return;
    };
    let is_match = a.key == b.key;

    if is_match {
        for entity in open {
            if let Ok((_, mut item)) = items.get_mut(entity) {
                item.is_matched = true;
            }
        }
    } else {
        for entity in open {
            if let Ok((_, mut item)) = items.get_mut(entity) {
                item.is_mismatched = true;
            }
        }
        mismatch.timer.reset();
        mismatch.active = true;
    }
}

pub fn reset_mismatch_timer(mut mismatch: ResMut<MismatchTimer>) {
    mismatch.timer.reset();
    mismatch.active = false;
}
pub fn check_game_over(
    changed: Query<&ItemGrid, Changed<ItemGrid>>,
    all_items: Query<&ItemGrid>,
    mut game_over: ResMut<GameOver>,
) {
    if changed.is_empty() || all_items.is_empty() || game_over.visible {
        return;
    }

    if all_items.iter().all(|item| item.is_matched) {
        game_over.visible = true;
    }
}

pub fn reset_game_over(mut game_over: ResMut<GameOver>) {
    game_over.visible = false;
}

fn grid_size(pairs: usize) -> (usize, usize) {
    let size = match pairs {
        6 => (4, 3),
        8 => (4, 4),
        10 => (5, 4),
        12 => (6, 4),
        _ => panic!("Неподдерживаемое число пар: {pairs}"),
    };
    assert_eq!(size.0 * size.1, pairs * 2);
    size
}

fn on_grid_item_click(
    click: On<Pointer<Click>>,
    mut mismatch: ResMut<MismatchTimer>,
    mut query: Query<&mut ItemGrid>,
) {
    let Ok(item) = query.get(click.entity) else {
        return;
    };

    if item.is_revealed {
        return;
    }

    if mismatch.active {
        for mut other in &mut query {
            if other.is_revealed && !other.is_matched {
                other.is_revealed = false;
                other.is_mismatched = false;
            }
        }
        mismatch.active = false;
    }

    if let Ok(mut item) = query.get_mut(click.entity) {
        item.is_revealed = true;
    }
}

fn spawn_grid_item(
    commands: &mut Commands,
    key: String,
    image: Handle<Image>,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
    border_mesh: Handle<Mesh>,
    matched_material: Handle<ColorMaterial>,
    wrong_material: Handle<ColorMaterial>,
    pos_x: f32,
    pos_y: f32,
) {
    commands
        .spawn((
            ItemGrid {
                key,
                is_revealed: false,
                is_matched: false,
                is_mismatched: false,
            },
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_xyz(pos_x, pos_y, 0.0),
            Pickable::default(),
        ))
        .with_children(|parent| {
            // Картинка на лицевой стороне, скрыта до открытия
            parent.spawn((
                CardPart::Label,
                Sprite {
                    image,
                    custom_size: Some(Vec2::splat(CELL_SIZE)),
                    image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),

                    ..Default::default()
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
                Visibility::Hidden,
                Pickable::IGNORE,
            ));

            // Зелёная рамка (найденная пара), позади карточки
            parent.spawn((
                CardPart::MatchedBorder,
                Mesh2d(border_mesh.clone()),
                MeshMaterial2d(matched_material),
                Transform::from_xyz(0.0, 0.0, -0.1),
                Visibility::Hidden,
                Pickable::IGNORE,
            ));

            // Красная рамка (несовпадение), тоже позади
            parent.spawn((
                CardPart::WrongBorder,
                Mesh2d(border_mesh),
                MeshMaterial2d(wrong_material),
                Transform::from_xyz(0.0, 0.0, -0.1),
                Visibility::Hidden,
                Pickable::IGNORE,
            ));
        })
        .observe(on_grid_item_click);
}
