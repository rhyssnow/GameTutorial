use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value;
use sdl3::render::FRect;
use sdl3::video::Window;
use crate::PixelEngine::attack::attacks;
use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};
use crate::PixelEngine::roam::roamPattern;
use crate::PixelEngine::trigger::trigger;

pub fn loadSaveFromJson (window: &Window) -> (mainCharacter, String, f32, f32) {
    let content = fs::read_to_string(&"src/Metadata/save.json").unwrap();
    let data: Value = serde_json::from_str(&content).unwrap();

    let mainCharacter = mainCharacter::new(window.size_in_pixels().0 as f32 /2.0 - 35.0, window.size_in_pixels().1 as f32 /2.0, data["maxHealth"].as_f64().unwrap() as f32);
    let path = data["currentArea"].as_str().unwrap();

    (mainCharacter, path.to_string(), data["x"].as_f64().unwrap() as f32, data["y"].as_f64().unwrap() as f32)
}

pub fn createEnvironmentFromJson (file: PathBuf) -> (Vec<String>, meshBox, Vec<enemy>) {
    let mut meshBox: meshBox;
    let mut textureVec = Vec::new();
    let mut enemies = Vec::new();

    let content = fs::read_to_string(&file).unwrap();
    let data: Value = serde_json::from_str(&content).unwrap();
    let mut triggerVec = Vec::new();

    for meshBox in data["meshBox"].as_array().unwrap() {
        textureVec.push(meshBox["background"].as_str().unwrap().to_string());
        for trigger in meshBox["triggers"].as_array().unwrap() {
            let triggerArea = FRect::new(
                trigger["x"].as_f64().unwrap() as f32,
                trigger["y"].as_f64().unwrap() as f32,
                trigger["w"].as_f64().unwrap() as f32,
                trigger["h"].as_f64().unwrap() as f32,
            );
            triggerVec.push(trigger::new(trigger["triggerType"].as_str().unwrap().to_string(), triggerArea));
        }
    }
    meshBox = meshBox::new(0.0, 0.0, 0.0, 0.0, triggerVec);

    let content = fs::read_to_string(Path::new("src/Metadata/enemies.json")).unwrap();
    let enemyData: Value = serde_json::from_str(&content).unwrap();
    
    let mut texturePos = 4;
    for enemy in data["enemies"].as_array().unwrap() {
        for enemyType in enemyData[enemy["enemyType"].as_str().unwrap().to_string()].as_array().unwrap() {
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
                attacks::new(enemyType["attackType"].as_str().unwrap().to_string()),
                roamPattern::new(enemyType["roamType"].as_str().unwrap().to_string()),
                texturePos.clone()
            ));
            textureVec.push("src/Assets/".to_string() + enemyType["textureFile"].as_str().unwrap());
        }
        texturePos += 1;
    }
    (textureVec, meshBox, enemies) //the meshbox needs its width, height and x, y coordinates set before it is usable.
}