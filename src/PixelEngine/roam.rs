use sdl3::render::{FRect, Texture};
use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};

#[derive(Clone)]
pub struct roamPattern {
    roamType: String,
    roamArea: FRect,
    movePoint: i32
}

impl roamPattern {
    pub fn new(roamType: String, roamArea: FRect) -> roamPattern {roamPattern{roamType, roamArea, movePoint: 1}}
    pub fn roam<'a>(&mut self, mainCharacter: &mut mainCharacter, meshBox: &mut meshBox, mut enemy: &mut enemy, backgroundTexture: &Texture) {
        match self.roamType.as_str() {
            "squareRoam" =>  squareRoam(&mut enemy, &self.roamArea),
            _ => {}
        }
    }
}
fn squareRoam (enemy: &mut enemy, roamArea: &FRect, movePoint: &i32) {
    let coordinates = match movePoint {
        1 => (roamArea.x, roamArea.y),
        2 => (roamArea.x, roamArea.y + roamArea.h),
        
        _ => (0.0, 0.0)
    };
}