use std::time::Instant;
use sdl3::render::{FRect, Texture, WindowCanvas};
use sdl3::video::Window;
use crate::gameLoop;
use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};
use crate::PixelEngine::hitboxDetector;

#[derive(Clone)]
pub struct trigger {
    triggerType: String,
    area: FRect
}

impl trigger {
    pub fn new(triggerType: String, mut area: FRect) -> trigger { trigger{triggerType, area}}
    pub fn trigger(&mut self, mainCharacter: &mut mainCharacter, mut enemies: &mut Vec<enemy>, textureVec: &mut Vec<Texture>, renderer: &mut WindowCanvas, window: &mut Window, meshBoxX: f32, meshBoxY: f32, running: &mut bool) {
        let mut newArea = false;
        let mut file = String::new();
        let mut index = 0;
        for letter in self.triggerType.chars() {
            if !newArea {
                match index {
                    0 => if letter == 'n' { index += 1; continue },
                    1 => if letter == 'e' { index += 1; continue },
                    2 => if letter == 'w' { index += 1; continue },
                    3 => if letter == 'A' { index += 1; continue },
                    4 => if letter == 'r' { index += 1; continue },
                    5 => if letter == 'e' { index += 1; continue },
                    6 => if letter == 'a' {  },
                    _ => {}
                }
            }
            else {
                file.push(letter);
            }
            if index == 6 {
                newArea = true;
            }
            if !newArea {
                break
            }
            index += 1;
        }

        let hitbox = FRect::new(
            (self.area.x - meshBoxX) * 6.0,
            (self.area.y - meshBoxY) * 6.0,
            self.area.w *6.0,
            self.area.h*6.0
        );

        if newArea {
            loadNewArea(file, &hitbox, renderer, window, mainCharacter, textureVec, running);
        }

        match self.triggerType.as_str() {
            "impassable" => impassable(mainCharacter, hitbox, window),
            _ => {}
        }
    }
}

fn impassable (mainCharacter: &mut mainCharacter, hitBox: FRect, window: &Window) {

    if hitboxDetector(hitBox, mainCharacter.hitBox) {
        let speed = 6.0;

        let hitboxX = hitBox.x + hitBox.w /2.0;
        let hitboxY = hitBox.y + hitBox.h /2.0;

        let dx = mainCharacter.x - hitboxX;
        let dy = mainCharacter.y - hitboxY;

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

        if dx < 0.0 {
            mx = -mx;
        }
        if dy < 0.0 {
            my = -my;
        }

        mainCharacter.updatePosition(mx, my, window);
    }
    // add this for enemies. also knockback for when the main character gets hit
}

fn loadNewArea (file: String, area: &FRect, renderer: &mut WindowCanvas, window: &mut Window, mainCharacter: &mainCharacter, textureVec: &mut Vec<Texture>, running: &mut bool) {
    if hitboxDetector(mainCharacter.hitBox, *area) {
        while textureVec.len() > 3 {
            textureVec.pop().unwrap();
        }

        gameLoop(renderer, window, textureVec, mainCharacter.clone(), file, 0.0, 0.0, running);
    }
}