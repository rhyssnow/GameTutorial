use sdl3::render::Texture;
use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};

#[derive(Clone)]
pub struct roamPattern {
    roamType: String,
}
impl roamPattern {
    pub fn new(roamType: String) -> roamPattern {roamPattern{roamType}}
    pub fn roam<'a>(&mut self, mainCharacter: &mut mainCharacter, meshBox: &mut meshBox, mut enemy: &mut enemy, backgroundTexture: &Texture) {
        match self.roamType.as_str() {
            "instantAttack" =>  instantAttack(&mut enemy),
            _ => {}
        }
    }
}
fn instantAttack (enemy: &mut enemy) {
    enemy.roam = false;
}