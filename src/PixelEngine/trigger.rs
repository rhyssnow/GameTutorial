use sdl3::pixels::Color;
use sdl3::render::{FRect, Texture, WindowCanvas};
use sdl3::video::Window;
use crate::gameLoop;
use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};
use crate::PixelEngine::{hitboxDetector, movementCalculator};

#[derive(Clone)]
pub struct trigger {
    triggerType: String,
    area: FRect
}

impl trigger {
    pub fn new(triggerType: String, mut area: FRect) -> trigger { trigger{triggerType, area}}
    pub fn trigger(&mut self, meshBox: &mut meshBox, mainCharacter: &mut mainCharacter, enemies: &mut Vec<enemy>, textureVec: &mut Vec<Texture>, renderer: &mut WindowCanvas, window: &mut Window, running: &mut bool) {
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
            (self.area.x - meshBox.x) * 6.0,
            (self.area.y - meshBox.y) * 6.0,
            self.area.w *6.0,
            self.area.h*6.0
        );

        if newArea {
            loadNewArea(file, &hitbox, renderer, window, mainCharacter, textureVec, running);
        }

        match self.triggerType.as_str() {
            "impassable" => impassable(mainCharacter, hitbox, window, enemies, textureVec),
            "setNewRespawn" => setNewRespawn(hitbox, mainCharacter, meshBox),
            _ => {}
        }
    }
}

fn impassable (mainCharacter: &mut mainCharacter, hitBox: FRect, window: &Window, enemies: &mut Vec<enemy>, textureVec: &Vec<Texture>) {

    if hitboxDetector(hitBox, mainCharacter.hitBox) { 
        let topMain = mainCharacter.hitBox.y;
        let bottomMain = mainCharacter.hitBox.y + mainCharacter.hitBox.h;
        let leftMain = mainCharacter.hitBox.x;
        let rightMain = mainCharacter.hitBox.x + mainCharacter.hitBox.w;

        let top = hitBox.y;
        let bottom = hitBox.y + hitBox.h;
        let left = hitBox.x;
        let right = hitBox.x + hitBox.w;

        if leftMain > left - (mainCharacter.hitBox.w - 7.0) && rightMain < right + (mainCharacter.hitBox.w - 7.0)  {
            if bottomMain > top && bottomMain < bottom {
                mainCharacter.y = hitBox.y - mainCharacter.hitBox.h;
            }
            if topMain < bottom && topMain > top {
                mainCharacter.y = hitBox.y + hitBox.h;

            }
        }
        else if topMain > top - (mainCharacter.hitBox.h - 7.0) && bottomMain < bottom + (mainCharacter.hitBox.h - 7.0) {
            if leftMain > left && leftMain < right {
                mainCharacter.x = hitBox.x + hitBox.w;
            }
            if rightMain < right && rightMain > left {
                mainCharacter.x = hitBox.x - mainCharacter.hitBox.w;
            }
        }
    }
    // add this for enemies. also knockback for when the main character gets hit
    for enemy in enemies.iter_mut() {
        if hitboxDetector(enemy.hitBox, hitBox){
            let speed = 1.0;

            let dx = hitBox.x - enemy.x * 6.0;
            let dy = hitBox.y - enemy.y * 6.0;

            let m = movementCalculator(dx, dy, speed);

            enemy.updatePosition(-m.0, -m.1, &textureVec[3]);
        }
    }
}

fn setNewRespawn (area: FRect, mainCharacter: &mut mainCharacter, meshBox: &mut meshBox) {
    let mainHitbox = FRect::new(
        mainCharacter.x + 3.5,
        mainCharacter.y + 77.0,
        mainCharacter.hitBox.w - 7.0,
        mainCharacter.hitBox.h - 77.0,
    );
    if hitboxDetector(area, mainHitbox) {
        meshBox.respawnCoords = (meshBox.x, meshBox.y);
        mainCharacter.respawnCoords = (mainCharacter.x, mainCharacter.y);
    }
}

fn loadNewArea (file: String, area: &FRect, renderer: &mut WindowCanvas, window: &mut Window, mainCharacter: &mainCharacter, textureVec: &mut Vec<Texture>, running: &mut bool) {
    if hitboxDetector(mainCharacter.hitBox, *area) {
        while textureVec.len() > 3 {
            textureVec.pop().unwrap();
        }

        gameLoop(renderer, window, textureVec, mainCharacter.clone(), file, 0.0, 0.0, running);
    }
}