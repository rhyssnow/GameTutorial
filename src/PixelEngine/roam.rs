use std::time::Instant;
use sdl3::render::{FRect, Texture};
use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};
use crate::PixelEngine::hitboxDetector;

#[derive(Clone)]
pub struct roamPattern {
    roamType: String,
    roamArea: FRect,
    movePoint: i32
}

impl roamPattern {
    pub fn new(roamType: String, roamArea: FRect) -> roamPattern {roamPattern{roamType, roamArea, movePoint: 1}}
    pub fn roam(&mut self, mainCharacter: &mut mainCharacter, meshBox: &mut meshBox, mut enemy: &mut enemy, backgroundTexture: &Texture) {
        match self.roamType.as_str() {
            "squareRoam" =>  squareRoam(&mut enemy, &self.roamArea, &mut self.movePoint, backgroundTexture, mainCharacter, meshBox),
            "squarePace" =>  {squareRoam(&mut enemy, &self.roamArea, &mut self.movePoint, backgroundTexture, mainCharacter, meshBox); enemy.roam = true},
            _ => {}
        }
    }
}

fn squareRoam (enemy: &mut enemy, roamArea: &FRect, mut movePoint: &mut i32, backgroundTexture: &Texture, mainCharacter: &mainCharacter, meshBox: &meshBox) {
    let coordinates = match movePoint {
        1 => (roamArea.x, roamArea.y),
        2 => (roamArea.x + roamArea.w, roamArea.y),
        3 => (roamArea.x + roamArea.w, roamArea.y + roamArea.h),
        4 => (roamArea.x, roamArea.y + roamArea.h),
        _ => (0.0, 0.0)
    };
    if enemy.lastPhaseChange.elapsed().as_millis() >= 200 {
        enemy.walkingPhase += 1;
        if enemy.walkingPhase > 1 {
            enemy.walkingPhase = 0;
        }
        enemy.lastPhaseChange = Instant::now();
    }

    let speed = 0.5;
    let mainPosX = coordinates.0;
    let mainPosY = coordinates.1;

    let mut dx = mainPosX - enemy.x;
    let mut dy = mainPosY - enemy.y;

    if dx == 0.0 {
        dx = 1.0;
    }
    if dy == 0.0 {
        dy = 1.0;
    }

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

    if dx < 1.0 && dx > -1.0 && dy < 1.0 && dy > -1.0 {
        *movePoint += 1;
        if movePoint > &mut 4 {
            *movePoint = 1;
        }
    }
    let hitbox = FRect::new(
        (roamArea.x - 25.0 - meshBox.x) * 6.0,
        (roamArea.y - 25.0 - meshBox.y) * 6.0,
        (roamArea.w + 50.0) * 6.0,
        (roamArea.h + 50.0) * 6.0,
    );
    if hitboxDetector(hitbox, mainCharacter.hitBox) {
        //enemy.roam = false;
    }
}