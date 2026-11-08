use sdl3::render::{FRect, Texture, WindowCanvas};
use sdl3::video::Window;
use crate::gameLoop;
use crate::PixelEngine::basic::{enemy, mainCharacter, meshBox};
use crate::PixelEngine::{hitboxDetector, movementCalculator};
use crate::PixelEngine::bosses::boss;

#[derive(Clone)]
pub struct trigger {
    triggerType: String,
    area: FRect,
    strings: Vec<String>
}

impl trigger {
    pub fn new(triggerType: String, area: FRect, strings: Vec<String>) -> trigger { trigger{triggerType, area, strings}}
    pub fn trigger(&mut self, meshBox: &mut meshBox, mainCharacter: &mut mainCharacter, enemies: &mut Vec<enemy>, bosses: &mut Vec<boss>, textureVec: &mut Vec<Texture>, renderer: &mut WindowCanvas, window: &mut Window, running: &mut bool, alive: &mut bool) {
        let hitbox = FRect::new(
            (self.area.x - meshBox.x) * 6.0,
            (self.area.y - meshBox.y) * 6.0,
            self.area.w *6.0,
            self.area.h*6.0
        );

        match self.triggerType.as_str() {
            "impassable" => impassable(mainCharacter, hitbox, enemies, bosses, meshBox, &self.strings),
            "setNewRespawn" => setNewRespawn(hitbox, mainCharacter, meshBox),
            "newArea" => loadNewArea(self.strings[0].clone(), hitbox, meshBox, renderer, window, mainCharacter, textureVec, running, alive),
            _ => {}
        }
    }
}

fn impassable (mainCharacter: &mut mainCharacter, hitBox: FRect, enemies: &mut Vec<enemy>, bosses: &mut Vec<boss>, meshBox: &meshBox, strings: &Vec<String>) {
    if strings.contains(&"character".to_string()) {
        if hitboxDetector(hitBox, mainCharacter.hitBox) {
            let topMain = mainCharacter.hitBox.y;
            let bottomMain = mainCharacter.hitBox.y + mainCharacter.hitBox.h;
            let leftMain = mainCharacter.hitBox.x;
            let rightMain = mainCharacter.hitBox.x + mainCharacter.hitBox.w;

            let top = hitBox.y;
            let bottom = hitBox.y + hitBox.h;
            let left = hitBox.x;
            let right = hitBox.x + hitBox.w;

            if leftMain > left - (mainCharacter.hitBox.w - 7.0) && rightMain < right + (mainCharacter.hitBox.w - 7.0) {
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
    }
    // add this for enemies. also knockback for when the main character gets hit
    for enemy in enemies.iter_mut() {
        if hitboxDetector(hitBox, enemy.hitBox) {
            let topMain = enemy.hitBox.y;
            let bottomMain = enemy.hitBox.y + enemy.hitBox.h;
            let leftMain = enemy.hitBox.x;
            let rightMain = enemy.hitBox.x + enemy.hitBox.w;

            let top = hitBox.y;
            let bottom = hitBox.y + hitBox.h;
            let left = hitBox.x;
            let right = hitBox.x + hitBox.w;

            if leftMain > left - (enemy.hitBox.w - 18.0) && rightMain < right + (enemy.hitBox.w - 18.0) {
                if bottomMain > top && bottomMain < bottom {
                    enemy.y = ((hitBox.y - enemy.hitBox.h/2.0) + meshBox.y * 6.0) / 6.0;
                }
                if topMain < bottom && topMain > top {
                    enemy.y = ((hitBox.y + hitBox.h + enemy.hitBox.h/2.0) + meshBox.y * 6.0) / 6.0;
                }
            }
            else if topMain > top - (enemy.hitBox.h - 6.0)  && bottomMain < bottom + (enemy.hitBox.h - 6.0) {
                if leftMain > left && leftMain < right {
                    enemy.x = ((hitBox.x + hitBox.w + enemy.hitBox.h/2.0) + meshBox.x * 6.0) / 6.0;
                }
                if rightMain < right && rightMain > left {
                    enemy.x = ((hitBox.x - enemy.hitBox.w/2.0) + meshBox.x * 6.0) / 6.0;
                }
            }
        }
    }
    for boss in bosses.iter_mut() {
        if hitboxDetector(hitBox, boss.hitbox) {
            let topMain = boss.hitbox.y;
            let bottomMain = boss.hitbox.y + boss.hitbox.h;
            let leftMain = boss.hitbox.x;
            let rightMain = boss.hitbox.x + boss.hitbox.w;

            let top = hitBox.y;
            let bottom = hitBox.y + hitBox.h;
            let left = hitBox.x;
            let right = hitBox.x + hitBox.w;

            if leftMain > left - (boss.hitbox.w - 18.0) && rightMain < right + (boss.hitbox.w - 18.0) {
                if bottomMain > top && bottomMain < bottom {
                    boss.y = ((hitBox.y - boss.hitbox.h/2.0) + meshBox.y * 6.0) / 6.0;
                }
                if topMain < bottom && topMain > top {
                    boss.y = ((hitBox.y + hitBox.h + boss.hitbox.h/2.0) + meshBox.y * 6.0) / 6.0;
                }
            }
            else if topMain > top - (boss.hitbox.h - 6.0)  && bottomMain < bottom + (boss.hitbox.h - 6.0) {
                if leftMain > left && leftMain < right {
                    boss.x = ((hitBox.x + hitBox.w + boss.hitbox.h/2.0) + meshBox.x * 6.0) / 6.0;
                }
                if rightMain < right && rightMain > left {
                    boss.x = ((hitBox.x - boss.hitbox.w/2.0) + meshBox.x * 6.0) / 6.0;
                }
            }
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

fn loadNewArea (file: String, area: FRect, meshBox: &meshBox, renderer: &mut WindowCanvas, window: &mut Window, mainCharacter: &mut mainCharacter, textureVec: &mut Vec<Texture>, running: &mut bool, parentFunc: &mut bool) {
    if hitboxDetector(mainCharacter.hitBox, area) {
        let startX: f32;
        if area.x > textureVec[3].width() as f32 - 10.0 {
            startX = 0.0;
            mainCharacter.x = -12.0;
        } else if area.x < 10.0 {
            startX = -1.0;
            mainCharacter.x = window.size_in_pixels().0 as f32 - mainCharacter.hitBox.w + 12.0;
        } else {
            startX = meshBox.x;
        }

        let startY: f32;
        if area.y > textureVec[3].height() as f32 - 10.0 {
            startY = 0.0;
            mainCharacter.y = -40.0;
        } else if area.y < 10.0 {
            startY = -1.0;
            mainCharacter.y = window.size_in_pixels().1 as f32 - mainCharacter.hitBox.h + 40.0;
        } else {
            startY = meshBox.y;
        }
        
        while textureVec.len() > 3 {
            textureVec.pop().unwrap();
        }

        gameLoop(renderer, window, textureVec, mainCharacter.clone(), file, startX, startY, running);
        *parentFunc = true;
    }
}