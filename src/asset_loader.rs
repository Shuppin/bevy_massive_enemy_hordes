use std::fs::read_to_string;

use bevy::{prelude::*, utils::hashbrown::HashMap};

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub texture_atlas: Handle<Image>,
    pub texture_atlas_data: HashMap<String, SpriteData>,
}

#[derive(Debug)]
pub struct SpriteData {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    let spritesheet = asset_server.load("spritesheet.png");
    let spritesheet_data =
        read_to_string("assets/spritesheet.txt").expect("cannot find assets/spritesheet.txt");
    let spritesheet_data = parse_spritesheet_data(spritesheet_data);

    *scene_assets = SceneAssets {
        texture_atlas: spritesheet,
        texture_atlas_data: spritesheet_data,
    }
}

fn parse_spritesheet_data(spritesheet_data: String) -> HashMap<String, SpriteData> {
    let mut spritesheet_hashmap = HashMap::new();
    for (i, line) in spritesheet_data.lines().enumerate() {
        let data: Vec<&str> = line.split(' ').collect();
        if data.len() != 5 {
            warn!("assets/spritesheet.txt:{i} has malformed data: '{line}'")
        } else {
            let sprite_name = data[0].to_string();
            let Ok(x) = data[1].parse::<u32>() else {
                warn!("assets/spritesheet.txt:{i} x component has malformed data: '{line}'");
                continue;
            };
            let Ok(y) = data[2].parse::<u32>() else {
                warn!("assets/spritesheet.txt:{i} y component has malformed data: '{line}'");
                continue;
            };
            let Ok(width) = data[3].parse::<u32>() else {
                warn!("assets/spritesheet.txt:{i} width component has malformed data: '{line}'");
                continue;
            };
            let Ok(height) = data[4].parse::<u32>() else {
                warn!("assets/spritesheet.txt:{i} height component has malformed data: '{line}'");
                continue;
            };
            spritesheet_hashmap.insert(
                sprite_name,
                SpriteData {
                    x,
                    y,
                    width,
                    height,
                },
            );
        };
    }
    spritesheet_hashmap
}
