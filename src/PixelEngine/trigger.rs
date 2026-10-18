use std::fs;
use std::path::Path;
use std::time::Instant;
use sdl3::render::{FRect, Texture, WindowCanvas};
use sdl3::surface::Surface;
use serde_json::Value;
use crate::PixelEngine::attack::attacks;
use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};
use crate::PixelEngine::roam::roamPattern;

#[derive(Clone)]
pub struct trigger {
    triggerType: String,
    area: FRect,
    timeSinceSpawn: Instant
}

impl trigger {
    pub fn new(triggerType: String, mut area: FRect) -> trigger { trigger{triggerType, area, timeSinceSpawn: Instant::now()}}
    pub fn trigger(&mut self, mainCharacter: &mut mainCharacter, mut enemies: &mut Vec<enemy>, textureVec: &mut Vec<Texture>, renderer: &WindowCanvas) {
        let elapsed = self.timeSinceSpawn.elapsed();
        println!("Elapsed: {:.2}s", elapsed.as_secs_f32());
        match self.triggerType.as_str() {
            "spawnWolfAlways" => spawnWolfAlways(&mut enemies, &mut self.timeSinceSpawn, textureVec, renderer),
            _ => {}
        }
    }
}
fn spawnWolfAlways(enemies: &mut Vec<enemy>, timeSinceSpawn: &mut Instant, textureVec: &mut Vec<Texture>, renderer: &WindowCanvas) {
    let content = fs::read_to_string(Path::new("src/Metadata/enemies.json")).unwrap();
    let enemyData: Value = serde_json::from_str(&content).unwrap();

    let mut up = Vec::new();
    let mut down = Vec::new();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for enemyType in enemyData["wolf"].as_array().unwrap() {
        for frect in enemyType["up"].as_array().unwrap() {
            up.push(FRect::new(
                frect["x"].as_f64().unwrap() as f32,
                frect["y"].as_f64().unwrap() as f32,
                frect["w"].as_f64().unwrap() as f32,
                frect["h"].as_f64().unwrap() as f32,
            ))
        }
        for frect in enemyType["down"].as_array().unwrap() {
            down.push(FRect::new(
                frect["x"].as_f64().unwrap() as f32,
                frect["y"].as_f64().unwrap() as f32,
                frect["w"].as_f64().unwrap() as f32,
                frect["h"].as_f64().unwrap() as f32,
            ))
        }
        for frect in enemyType["left"].as_array().unwrap() {
            left.push(FRect::new(
                frect["x"].as_f64().unwrap() as f32,
                frect["y"].as_f64().unwrap() as f32,
                frect["w"].as_f64().unwrap() as f32,
                frect["h"].as_f64().unwrap() as f32,
            ))
        }
        for frect in enemyType["right"].as_array().unwrap() {
            right.push(FRect::new(
                frect["x"].as_f64().unwrap() as f32,
                frect["y"].as_f64().unwrap() as f32,
                frect["w"].as_f64().unwrap() as f32,
                frect["h"].as_f64().unwrap() as f32,
            ))
        }
        if timeSinceSpawn.elapsed().as_secs_f32() > 4.0 {
            *timeSinceSpawn = Instant::now();
            enemies.push(enemy::new(
                rand::random_range(0.0 ..textureVec[3].width() as f32),
                rand::random_range(0.0 .. textureVec[3].height() as f32),
                up,
                down,
                left,
                right,
                attacks::new(enemyType["attackType"].as_str().unwrap().to_string()),
                roamPattern::new(enemyType["roamType"].as_str().unwrap().to_string()),
                textureVec.len()
            ));
            let surface = Surface::load_png("src/Assets/wolf.png").unwrap();
            let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
            texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
            textureVec.push(texture);
        }
        break
    }
}

