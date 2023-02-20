#![windows_subsystem = "windows"]

mod config;
use config::*;

use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::text::Text2dBounds;
use bevy::window::PresentMode;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Bevy 2048".to_string(),
				position: WindowPosition::Centered,
				width: WINDOW_WIDTH,
				height: WINDOW_HEIGHT,
				present_mode: PresentMode::AutoNoVsync,
				resizable: false,
				..default()
			},
			..default()
		}))
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_startup_system(setup)
		.run();
}

fn cell_color(cell_value: u32) -> bevy::render::color::Color {
	match cell_value {
		2 => COLOR_CELL_2,
		4 => COLOR_CELL_4,
		8 => COLOR_CELL_8,
		16 => COLOR_CELL_16,
		32 => COLOR_CELL_32,
		64 => COLOR_CELL_64,
		128 => COLOR_CELL_128,
		256 => COLOR_CELL_256,
		512 => COLOR_CELL_512,
		1024 => COLOR_CELL_1024,
		2048 => COLOR_CELL_2048,
		_ => COLOR_CELL_NULL
	}
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>
) {
	commands.spawn(Camera2dBundle::default());

	let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
	let mut x_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	let mut y_offset = (side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	x_offset = 2.0 * x_offset - (-1.0) * (WINDOW_WIDTH / 2.0 - CELL_SPACE) - side_length / 2.0;

	commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(shape::Box::new(WINDOW_HEIGHT, WINDOW_HEIGHT, 0.0).into()).into(),
		material: materials.add(ColorMaterial::from(COLOR_BACKGROUND)),
		transform: Transform::from_xyz((WINDOW_WIDTH - WINDOW_HEIGHT) / 2.0, 0.0, 0.0),
		..default()
	});

	for i in 0..CELL_SIDE_NUM {
		for j in 0..CELL_SIDE_NUM {
			commands.spawn(MaterialMesh2dBundle {
				mesh: meshes.add(shape::Box::new(side_length, side_length, 0.0).into()).into(),
				material: materials.add(ColorMaterial::from(COLOR_CELL_NULL)),
				transform: Transform::from_xyz(x_offset + (j as f32) * (side_length + CELL_SPACE), y_offset - (i as f32) * (side_length + CELL_SPACE), 0.0),
				..default()
			});
		}
	}

	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
		font,
		font_size: 50.0,
		color: Color::BLACK,
	};
	let box_size = Vec2::new(side_length, side_length);
	commands.spawn(Text2dBundle {
		text: Text::from_section("SCORE:", text_style),
		text_2d_bounds: Text2dBounds {
			// Wrap text in the rectangle
			size: box_size,
		},
		transform: Transform::from_xyz(
			-WINDOW_WIDTH / 2.0,
			WINDOW_HEIGHT / 2.0,
			0.0,
		),
		..default()
	});
}











