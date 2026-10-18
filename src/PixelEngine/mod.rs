#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod roam;
pub mod attack;
pub mod parse;
pub mod trigger;
pub mod basic;

use std::time::Instant;
use sdl3::render::{FRect, Texture, WindowCanvas};
use sdl3::keyboard::Scancode;
use sdl3::pixels::Color;
use sdl3::video::Window;
use crate::PixelEngine::basic::{enemy, meshBox, mainCharacter};

pub struct pixelEngine {
    meshBox: meshBox,
    mainCharacter: mainCharacter,
    enemies: Vec<enemy>,
    pub(crate) alive: bool,
}

impl pixelEngine {
    pub fn new(meshBox: meshBox, mainCharacter: mainCharacter, enemies: Vec<enemy>) -> pixelEngine {pixelEngine{ meshBox, mainCharacter, enemies, alive: true}}

    pub fn input (&mut self, window: &Window, textureVec: &Vec<Texture>, running: &mut bool) {
        let mut eventPump = sdl3::init().unwrap().event_pump().unwrap();
        let keyboardState = eventPump.keyboard_state();
        let mut speed = 1.0;
        let mut walking = false;
        let mut arrowKeys = false;
        let mut usedStamina = false;

        let mut moveBackInDirection = Vec::new();
        let mut moveMainInDirection = Vec::new();

        if keyboardState.is_scancode_pressed(Scancode::Space) && !self.mainCharacter.jumping && self.mainCharacter.stats.stamina > 1.0 && self.mainCharacter.timeSinceJumped.elapsed().as_millis() > 100 {
            self.mainCharacter.velocity = 10.0;
            self.mainCharacter.stats.stamina -= 2.5;
            usedStamina = true;
        }
        if self.mainCharacter.jumping {
            usedStamina = true;
            speed *= 1.2;
        }

        if !keyboardState.is_scancode_pressed(Scancode::Space) && self.mainCharacter.jumping && self.mainCharacter.velocity > 0.1 {
            self.mainCharacter.velocity = 0.1;
        }

        if keyboardState.is_scancode_pressed(Scancode::Up){
            self.mainCharacter.direction = "up".to_string();
            arrowKeys = true;
        }
        if keyboardState.is_scancode_pressed(Scancode::Down){
            self.mainCharacter.direction = "down".to_string();
            arrowKeys = true;
        }
        if keyboardState.is_scancode_pressed(Scancode::Left){
            self.mainCharacter.direction = "left".to_string();
            arrowKeys = true;
        }
        if keyboardState.is_scancode_pressed(Scancode::Right){
            self.mainCharacter.direction = "right".to_string();
            arrowKeys = true;
        }
        if self.mainCharacter.swordAttack.swingPhase != 6 {
            self.mainCharacter.direction = self.mainCharacter.swordAttack.direction.clone();
            arrowKeys = true;
        }

        if keyboardState.is_scancode_pressed(Scancode::W) && !keyboardState.is_scancode_pressed(Scancode::S)  {
            walking = true;
            if !arrowKeys {
                self.mainCharacter.direction = "up".to_string();
            }
            if self.meshBox.endOfDirection.contains(&"up".to_string()) || self.mainCharacter.y > window.size_in_pixels().1 as f32 / 2.0 {
                moveMainInDirection.push("up".to_string());
            }
            else {
                moveBackInDirection.push("up".to_string());
            }
        }
        if keyboardState.is_scancode_pressed(Scancode::S) && !keyboardState.is_scancode_pressed(Scancode::W)  {
            walking = true;
            if !arrowKeys {
                self.mainCharacter.direction = "down".to_string();
            }
            if self.meshBox.endOfDirection.contains(&"down".to_string()) || self.mainCharacter.y < window.size_in_pixels().1 as f32 / 2.0 {
                moveMainInDirection.push("down".to_string());
            }
            else {
                moveBackInDirection.push("down".to_string());
            }
        }
        if keyboardState.is_scancode_pressed(Scancode::A) && !keyboardState.is_scancode_pressed(Scancode::D)  {
            walking = true;
            if !arrowKeys {
                self.mainCharacter.direction = "left".to_string();
            }
            if self.meshBox.endOfDirection.contains(&"left".to_string()) || self.mainCharacter.x > window.size_in_pixels().0 as f32 / 2.0 -35.0 {
                moveMainInDirection.push("left".to_string());
            }
            else {
                moveBackInDirection.push("left".to_string());
            }
        }
        if keyboardState.is_scancode_pressed(Scancode::D) && !keyboardState.is_scancode_pressed(Scancode::A)  {
            walking = true;
            if !arrowKeys {
                self.mainCharacter.direction = "right".to_string();
            }
            if self.meshBox.endOfDirection.contains(&"right".to_string()) || self.mainCharacter.x < window.size_in_pixels().0 as f32 / 2.0 -35.0 {
                moveMainInDirection.push("right".to_string());
            }
            else {
                moveBackInDirection.push("right".to_string());
            }
        }

        if (keyboardState.is_scancode_pressed(Scancode::LShift) || keyboardState.is_scancode_pressed(Scancode::RShift)) && walking == true {
            usedStamina = true;
            if self.mainCharacter.stats.stamina > 0.0 {
                speed *= 1.5;
                self.mainCharacter.stats.stamina -= 0.05;
            }
        }

        if keyboardState.is_scancode_pressed(Scancode::X) && self.mainCharacter.swordAttack.swingCooldown.elapsed().as_millis() > 350 && self.mainCharacter.stats.stamina > 1.0 && !self.mainCharacter.jumping{
            self.mainCharacter.swordAttack.swingPhase = 0;
            self.mainCharacter.swordAttack.swingPhaseFloat = 0.0;
            self.mainCharacter.swordAttack.direction = self.mainCharacter.direction.clone();
            self.mainCharacter.stats.stamina -= 3.0;
            self.mainCharacter.swordAttack.swingCooldown = Instant::now();
            usedStamina = true;
        }

        let directions = [moveBackInDirection.clone(), moveMainInDirection.clone()].concat();
        if !directions.contains(&self.mainCharacter.direction){
            speed /= 3.0;
        }
        if (directions.contains(&"up".to_string()) || directions.contains(&"down".to_string())) && (directions.contains(&"left".to_string())  || directions.contains(&"right".to_string())) {
            speed /= 1.5;
        }

        for direction in moveMainInDirection {
            match direction.as_str() {
                "up" => self.mainCharacter.updatePosition(0.0, -5.0 * speed, window),
                "down" => self.mainCharacter.updatePosition(0.0, 5.0 * speed, window),
                "left" => self.mainCharacter.updatePosition(-5.0 * speed, 0.0, window),
                "right" => self.mainCharacter.updatePosition(5.0 * speed, 0.0, window),
                &_ => {}
            }
        }
        for direction in moveBackInDirection {
            match direction.as_str() {
                "up" => self.meshBox.updatePosition(0.0, -1.0 * speed, &textureVec[self.meshBox.texturePos]),
                "down" => self.meshBox.updatePosition(0.0, 1.0 * speed, &textureVec[self.meshBox.texturePos]),
                "left" => self.meshBox.updatePosition(-1.0 * speed, 0.0, &textureVec[self.meshBox.texturePos]),
                "right" => self.meshBox.updatePosition(1.0 * speed, 0.0, &textureVec[self.meshBox.texturePos]),
                &_ => {}
            }
        }

        if walking {
            self.mainCharacter.walkPhaseChange();

            if !usedStamina &&self.mainCharacter.stats.stamina < self.mainCharacter.stats.maxStamina {
                self.mainCharacter.stats.stamina += 0.04;
            }
        }
        else {
            self.mainCharacter.walkingPhase = 0;
            if self.mainCharacter.stats.stamina < self.mainCharacter.stats.maxStamina - 0.1 {
                self.mainCharacter.stats.stamina += 0.05;
            }
        }

        if self.mainCharacter.stats.stamina > self.mainCharacter.stats.maxStamina {
            self.mainCharacter.stats.stamina = self.mainCharacter.stats.maxStamina;
        }
        if self.mainCharacter.stats.mana > self.mainCharacter.stats.maxMana {
            self.mainCharacter.stats.mana = self.mainCharacter.stats.maxMana;
        }
        if self.mainCharacter.stats.health > self.mainCharacter.stats.maxHealth {
            self.mainCharacter.stats.health = self.mainCharacter.stats.maxHealth;
        }

        for event in eventPump.poll_iter() {
            use sdl3::event::Event;
            match event {
                Event::Quit { .. } =>  {
                    *running = false;
                }
                _ => {}
            }
        }
    }
    pub fn update(&mut self, mut textureVec: &mut Vec<Texture>, renderer: &WindowCanvas, score: &mut i32) {
        self.mainCharacter.update();
        for mut enemy in self.enemies.iter_mut() {
            let enemyUsedHitbox = FRect::new(
                enemy.hitBox.x + enemy.hitBox.w * 0.1,
                enemy.hitBox.y + enemy.hitBox.h * 0.1,
                enemy.hitBox.w * 0.8,
                enemy.hitBox.h * 0.8
            );
            if hitboxDetector(enemyUsedHitbox, self.mainCharacter.hitBox) && !self.mainCharacter.immune {
                self.mainCharacter.timeSinceHit = Instant::now();
                self.mainCharacter.stats.health -= 2.0;
                self.mainCharacter.immune = true;
                if self.mainCharacter.stats.health <= 0.0 {
                    self.alive = false;
                }
            }
            if hitboxDetector(enemy.hitBox, self.mainCharacter.swordAttack.hitBox) {
                let speed = 1.0;
                let mainPosX = self.mainCharacter.x + self.meshBox.x * 6.0;
                let mainPosY = self.mainCharacter.y + self.meshBox.y * 6.0;

                let dx = mainPosX - enemy.x * 6.0;
                let dy = mainPosY - enemy.y * 6.0;

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

                enemy.health -= 0.1;
                enemy.updatePosition(-mx, -my, &textureVec[self.meshBox.texturePos]);
                enemy.timeSinceHit = Instant::now();
            }
            enemy.update();
        }
        let mut enemiesClone=self.enemies.clone();
        let texturePos = self.meshBox.texturePos;
        for enemyPos in 0..self.enemies.len() {
            if self.enemies[enemyPos].roam {
                enemiesClone[enemyPos].roamPattern.roam(&mut self.mainCharacter, &mut self.meshBox, &mut self.enemies[enemyPos], &textureVec[texturePos]);
            }
            else {
                enemiesClone[enemyPos].attacks.attack(&mut self.mainCharacter, &mut self.meshBox, &mut self.enemies[enemyPos], &textureVec[texturePos]);
            }
        }
        for mut trigger in self.meshBox.triggers.iter_mut() {
            trigger.trigger(&mut self.mainCharacter, &mut self.enemies, &mut textureVec, renderer);
        }
        let mut removeEnemy = Vec::new();
        for enemy in 0..self.enemies.len() {
            if self.enemies[enemy].health < 0.0 {
                removeEnemy.push(enemy);
            }
        }
        removeEnemy.sort_by(|a, b| b.cmp(a));
        for enemy in removeEnemy {
            *score +=1;
            self.enemies.remove(enemy);
        }
    }
    pub fn render(&mut self, textureVec: &Vec<Texture>, renderer: &mut WindowCanvas, window: &Window) {
        renderer.set_draw_color(Color::RGB(50, 155, 50));
        renderer.clear();

        let backgroundRenderArea = FRect::new(0.0, 0.0, window.size_in_pixels().0 as f32, window.size_in_pixels().1 as f32);
        let crestRenderArea = FRect::new(0.0, 0.0, 104.0, 115.0);

        self.meshBox.render(textureVec, renderer, backgroundRenderArea);
        if self.mainCharacter.render {
            self.mainCharacter.render(textureVec, renderer);
        }
        for enemy in self.enemies.iter_mut() {
            if enemy.render {
                enemy.render(textureVec, renderer, &mut self.meshBox);
            }
        }
        self.mainCharacter.swordAttack.render(renderer, self.mainCharacter.x, self.mainCharacter.y, textureVec);
        self.mainCharacter.stats.render(renderer);

        renderer.copy(&textureVec[1], None, crestRenderArea).unwrap();

        renderer.present();
    }
}
fn hitboxDetector(hitbox1: FRect, hitbox2: FRect) -> bool {
    let mut touching = true;

    let top1 = hitbox1.y;
    let bottom1 = hitbox1.y + hitbox1.h;
    let left1 = hitbox1.x;
    let right1 = hitbox1.x + hitbox1.w;

    let top2 = hitbox2.y;
    let bottom2 = hitbox2.y + hitbox2.h;
    let left2 = hitbox2.x;
    let right2 = hitbox2.x + hitbox2.w;

    if bottom1 < top2{
        touching = false;
    }
    if top1 > bottom2{
        touching = false;
    }
    if left1 > right2{
        touching = false;
    }
    if right1 < left2{
        touching = false;
    }

    touching
}
