use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use sdl3::keyboard::{KeyboardState, Scancode};
use sdl3::render::{FRect, Texture, WindowCanvas};
use sdl3::video::Window;
use serde_json::{json, Value};
use crate::PixelEngine::basic::{mainCharacter, meshBox};
use crate::PixelEngine::hitboxDetector;

pub struct object{
    pub(crate) area: FRect,
    areaOfEffect: FRect,
    objectType: String,
    textureRect: Vec<FRect>,
    texturePosition: usize,
    animationState: usize,
    timeSinceStateChange: Instant,
}

impl object{
    pub fn new(area: FRect, areaOfEffect: FRect, objectType: String, textureRect: Vec<FRect>, texturePosition: usize) -> object{object{area, areaOfEffect, objectType, textureRect, texturePosition, animationState: 0, timeSinceStateChange: Instant::now()}}
    pub fn render(&mut self, renderer: &mut WindowCanvas, meshBox: &meshBox, textureVec: &Vec<Texture>, window: &Window){
        let renderArea = FRect::new((self.area.x - meshBox.x) * 6.0, (self.area.y - meshBox.y) * 6.0, self.area.w * 6.0, self.area.h * 6.0);

        renderer.copy(&textureVec[self.texturePosition], self.textureRect[self.animationState], renderArea).unwrap();
        match self.objectType.as_str(){
            "litBeacon" => litBeaconRender(self.areaOfEffect, window, renderer),
            _ => {}
        }
    }
    pub fn updateInput(&mut self, mainCharacter: &mainCharacter, window: &Window, keyboardState: &KeyboardState, meshBox: &meshBox, currentLocation: &String){
        if self.timeSinceStateChange.elapsed().as_millis() > 500 {
            self.timeSinceStateChange = Instant::now();
            self.animationState += 1;
            if self.animationState > self.textureRect.len() - 1{
                self.animationState = 0;
            }
        }
        match self.objectType.as_str(){
            "litBeacon" => litBeaconInput(self.areaOfEffect, mainCharacter, window, keyboardState, meshBox, currentLocation),
            _ => {}
        }
    }
}

fn litBeaconInput (areaOfEffect: FRect, mainCharacter: &mainCharacter, window: &Window, keyboardState: &KeyboardState, meshBox: &meshBox, currentLocation: &String) {
    let hitbox = FRect::new((areaOfEffect.x - meshBox.x) * 6.0, (areaOfEffect.y - meshBox.y) * 6.0, areaOfEffect.w * 6.0, areaOfEffect.h * 6.0);
    if hitboxDetector(areaOfEffect, hitbox) && keyboardState.is_scancode_pressed(Scancode::F) {
        let content = fs::read_to_string(&"src/Metadata/save.json").unwrap();
        let mut data: Value = serde_json::from_str(&content).unwrap();

        data["currentArea"] = json!(currentLocation);
        data["maxHealth"] = json!(mainCharacter.stats.maxHealth);
        data["damage"] = json!(mainCharacter.stats.damage);
        data["x"] = json!(meshBox.x);
        data["y"] = json!(meshBox.y);
        data["mainX"] = json!(mainCharacter.x);
        data["mainY"] = json!(mainCharacter.y);

        let mut file = File::create("src/Metadata/save.json").unwrap();
        file.write_all(serde_json::to_string(&data).unwrap().as_bytes()).unwrap();
    }
}

fn litBeaconRender (areaOfEffect: FRect, window: &Window, renderer: &mut WindowCanvas) {

}