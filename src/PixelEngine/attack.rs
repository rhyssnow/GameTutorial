use std::time::Instant;
use sdl3::render::Texture;
use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};

#[derive(Clone)]
pub struct attacks {
    attackType: String,
}

impl attacks {
    pub fn new(attackType: String) -> attacks {attacks{attackType}}
    pub fn attack(&mut self, mainCharacter: &mut mainCharacter, meshBox: &mut meshBox, mut enemy: &mut enemy, backGroundTexture: &Texture) {
        match self.attackType.as_str() {
            "missile" =>  missileAttack(mainCharacter, backGroundTexture, meshBox, enemy),
            _ => return
        }
    }
}
fn missileAttack (mainCharacter: &mainCharacter, backgroundTexture: &Texture, meshBox: &meshBox, enemy: &mut enemy) {
    if enemy.lastPhaseChange.elapsed().as_millis() >= 200 {
        enemy.walkingPhase += 1;
        if enemy.walkingPhase > 1 {
            enemy.walkingPhase = 0;
        }
        enemy.lastPhaseChange = Instant::now();
    }

    let speed = 1.0;
    let mainPosX = mainCharacter.x + meshBox.x * 6.0;
    let mainPosY = mainCharacter.y + meshBox.y * 6.0;

    let dx = mainPosX - enemy.x * 6.0;
    let dy = mainPosY - enemy.y * 6.0;

    let mut vertical = false;

    let mut dxRatio = dx / dy;
    if dxRatio < 0.0{
        dxRatio = -dxRatio;
    }
    dxRatio += 1.0;

    let mut my = speed / dxRatio;
    let mut mx = speed - my;
    if my > speed {
        mx = speed / dxRatio;
        my = speed - my;
    }
    if mx < my {
        vertical = true;
    }

    if dx < 0.0 {
        mx = -mx;
    }
    if dy < 0.0 {
        my = -my;
    }
    if vertical == true {
        if my > 0.0 {
            enemy.direction = String::from("down");
        }
        else {
            enemy.direction = String::from("up");
        }
    }
    else {
        if mx > 0.0 {
            enemy.direction = String::from("right");
        }
        else {
            enemy.direction = String::from("left");
        }
    }

    enemy.updatePosition(mx, my, backgroundTexture);
}