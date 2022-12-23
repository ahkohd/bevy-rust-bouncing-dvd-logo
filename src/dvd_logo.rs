use bevy::{prelude::*, sprite::Anchor};
use math::round::floor;
use rand::Rng;

const SPEED: f32 = 3.5;
const LOGO_RATIO: f32 = 5.5;

#[derive(Component)]
struct DvdLogo;

#[derive(PartialEq)]
enum HorizontalDirection {
    Left,
    Right,
}

#[derive(PartialEq)]
enum VerticalDirection {
    Up,
    Down,
}

#[derive(Component)]
struct DvdLogoDirection {
    horizontal: HorizontalDirection,
    vertical: VerticalDirection,
}

#[derive(Component)]
struct DvdLogoCollison(Collision);

#[derive(Debug, Clone, PartialEq)]
enum Collision {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

struct DvdLogoCollisonEvent(Collision);

pub struct DVDLogoPlugin;

impl Plugin for DVDLogoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(ClearColor(Color::BLACK))
            .add_event::<DvdLogoCollisonEvent>()
            .add_system(detect_logo_win_collision)
            .add_system(translate_logo)
            .add_system(logo_color_change);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    commands.spawn(Camera2dBundle::default());
    let window = windows.primary();

    let w = window.width() / LOGO_RATIO;
    let h = window.height() / LOGO_RATIO;
    let x = ((window.width() as f64 / window.scale_factor()) as f32 / 2.0) - w / 2.0;
    let y = ((window.height() as f64 / window.scale_factor()) as f32 / 2.0) - h / 2.0;

    commands.spawn((
        DvdLogo,
        DvdLogoDirection {
            horizontal: HorizontalDirection::Right,
            vertical: VerticalDirection::Down,
        },
        DvdLogoCollison(Collision::None),
        SpriteBundle {
            texture: asset_server.load("imgs/dvd.png"),
            sprite: Sprite {
                anchor: Anchor::Center,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-x, y, 0.0),
            ..Default::default()
        },
    ));
}

fn translate_logo(
    mut query: Query<(&mut Transform, &mut DvdLogoDirection), With<DvdLogo>>,
    mut ev_dvd_logo_collision: EventReader<DvdLogoCollisonEvent>,
) {
    let (mut transform, mut direction) = query.single_mut();

    let x = transform.translation.x
        + if let HorizontalDirection::Left = direction.horizontal {
            -SPEED
        } else {
            SPEED
        };

    let y = transform.translation.y
        + if let VerticalDirection::Down = direction.vertical {
            -SPEED
        } else {
            SPEED
        };

    transform.translation = Vec3::new(x, y, 0.0);

    for ev in ev_dvd_logo_collision.iter() {
        match ev.0 {
            Collision::Left => direction.horizontal = HorizontalDirection::Right,
            Collision::Right => direction.horizontal = HorizontalDirection::Left,
            Collision::Top => direction.vertical = VerticalDirection::Down,
            Collision::Bottom => direction.vertical = VerticalDirection::Up,
            Collision::None => {}
        }
    }
}

fn detect_logo_win_collision(
    windows: Res<Windows>,
    query: Query<(&Sprite, &Transform), With<DvdLogo>>,
    mut logo_collision_query: Query<&mut DvdLogoCollison, With<DvdLogo>>,
    cam_query: Query<&Transform, With<Camera>>,
    mut ev_dvd_logo_collision: EventWriter<DvdLogoCollisonEvent>,
) {
    let (sprite, transform) = query.single();
    let mut logo_collision = logo_collision_query.single_mut();
    let cam_transform = cam_query.single();

    let window = windows.primary();

    let win_rect = window_to_rect(window, cam_transform);
    let logo_size = sprite.custom_size.unwrap();
    let half_width = logo_size.x / 2.0;
    let half_height = logo_size.y / 2.0;

    let contains_bottom_left = win_rect.contains(Vec2::new(
        transform.translation.x - half_width,
        transform.translation.y - half_height,
    ));

    let contains_bottom_right = win_rect.contains(Vec2::new(
        transform.translation.x + half_width,
        transform.translation.y - half_height,
    ));

    let contains_top_left = win_rect.contains(Vec2::new(
        transform.translation.x - half_width,
        transform.translation.y + half_height,
    ));

    let contains_top_right = win_rect.contains(Vec2::new(
        transform.translation.x + half_width,
        transform.translation.y + half_height,
    ));

    let collision = if !contains_top_right && !contains_bottom_right {
        Collision::Right
    } else if !contains_top_left && !contains_top_right {
        Collision::Top
    } else if !contains_top_left && !contains_bottom_left {
        Collision::Left
    } else if !contains_bottom_left && !contains_bottom_right {
        Collision::Bottom
    } else {
        Collision::None
    };

    let prev_collision = logo_collision.0.clone();
    if prev_collision != collision {
        logo_collision.0 = collision.clone();
        ev_dvd_logo_collision.send(DvdLogoCollisonEvent(collision));
    }
}

fn window_to_rect(window: &Window, camera: &Transform) -> Rect {
    let center = camera.translation.truncate();
    let half_width = (window.width() / 2.0) * camera.scale.x;
    let half_height = (window.height() / 2.0) * camera.scale.y;

    let left = center.x - half_width;
    let top = half_height + center.y;

    Rect::new(left, top, center.x + half_width, center.y - half_height)
}

fn logo_color_change(
    mut query: Query<&mut Sprite, With<DvdLogo>>,
    mut ev_dvd_logo_collision: EventReader<DvdLogoCollisonEvent>,
) {
    let mut sprite = query.single_mut();
    let last_ev = ev_dvd_logo_collision.iter().last();

    if let Some(ev) = last_ev {
        if Collision::None != ev.0 {
            let mut rng = rand::thread_rng();
            let h = rng.gen::<f64>() * 360.0;
            let color = Color::hsl(floor(h, 1) as f32, 100.0, 50.0);

            sprite.color = color;
        }
    }
}
