use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value;
use sdl3::render::FRect;
use crate::PixelEngine::attack::attacks;
use crate::PixelEngine::basic::{enemy, meshBox};
use crate::PixelEngine::roam::roamPattern;
use crate::PixelEngine::trigger::trigger;

pub fn createEnvironmentFromJson (file: PathBuf) -> (Vec<String>, meshBox, Vec<enemy>) {
    let mut meshBox: meshBox;
    let mut textureVec = Vec::new();
    let mut enemies = Vec::new();

    let content = fs::read_to_string(&file).unwrap();
    let data: Value = serde_json::from_str(&content).unwrap();
    let mut triggerVec = Vec::new();

    for meshBox in data["meshBox"].as_array().unwrap() {
        textureVec.push(meshBox["background"].to_string());
        for trigger in meshBox["triggers"].as_array().unwrap() {
            let triggerArea = FRect::new(
                trigger["x"].as_f64().unwrap() as f32,
                trigger["y"].as_f64().unwrap() as f32,
                trigger["width"].as_f64().unwrap() as f32,
                trigger["height"].as_f64().unwrap() as f32,
            );
            triggerVec.push(trigger::new(trigger["triggerType"].to_string(), triggerArea));
        }
    }
    meshBox = meshBox::new(0.0, 0.0, 0.0, 0.0, triggerVec);

    let content = fs::read_to_string(Path::new("src/Metadata/enemies.json")).unwrap();
    let enemyData: Value = serde_json::from_str(&content).unwrap();
    
    let mut texturePos = 4;
    for enemy in data["enemies"].as_array().unwrap() {
        for enemyType in enemyData[enemy["enemyType"].to_string()].as_array().unwrap() {
            let mut up = Vec::new();
            let mut down = Vec::new();
            let mut left = Vec::new();
            let mut right = Vec::new();

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
            enemies.push(enemy::new(
                enemy["x"].as_f64().unwrap() as f32,
                enemy["y"].as_f64().unwrap() as f32,
                up,
                down,
                left,
                right,
                attacks::new(enemyType["attackType"].to_string()),
                roamPattern::new(enemyType["roamType"].to_string()),
                texturePos.clone()
            ));
        }
        texturePos += 1;
    }

    (textureVec, meshBox, enemies) //the meshbox needs its width, height and x, y coordinates set before it is usable.
}