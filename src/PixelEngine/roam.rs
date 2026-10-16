use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};

pub struct roamPattern {
    roamType: String,
}
impl roamPattern {
    pub fn new(roamType: String) -> roamPattern {roamPattern{roamType}}
    pub fn roam<'a>(&mut self, mainCharacter: &mut mainCharacter, meshBox: &mut meshBox, enemy: &'a mut enemy) -> &'a mut enemy {
        match self.roamType.as_str() {
            "instantAttack" =>  instantAttack(enemy),
            _ => return enemy
        }
        enemy
    }
}
fn instantAttack (enemy: &mut enemy) {
    enemy.roam = true;
}