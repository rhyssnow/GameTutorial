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
    alive: bool,
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

        if (keyboardState.is_scancode_pressed(Scancode::LShift) || keyboardState.is_scancode_pressed(Scancode::RShift)) && self.mainCharacter.stats.stamina > 0.0 {
            speed *= 1.5;
            self.mainCharacter.stats.stamina -= 0.05;
            usedStamina = true;
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
            if self.meshBox.endeOfDirection.contains(&"up".to_string()) || self.mainCharacter.y > window.size_in_pixels().1 as f32 / 2.0 {
                self.mainCharacter.updatePosition(0.0, -5.0 * speed, window);
            }
            else {
                self.meshBox.updatePosition(0.0, -1.0 * speed, &textureVec[self.meshBox.texturePos]);
            }
        }
        if keyboardState.is_scancode_pressed(Scancode::S) && !keyboardState.is_scancode_pressed(Scancode::W)  {
            walking = true;
            if !arrowKeys {
                self.mainCharacter.direction = "down".to_string();
            }
            if self.meshBox.endeOfDirection.contains(&"down".to_string()) || self.mainCharacter.y > window.size_in_pixels().1 as f32 / 2.0 {
                self.mainCharacter.updatePosition(0.0, 5.0 * speed, window);
            }
            else {
                self.meshBox.updatePosition(0.0, 1.0 * speed, &textureVec[self.meshBox.texturePos]);
            }
        }
        if keyboardState.is_scancode_pressed(Scancode::A) && !keyboardState.is_scancode_pressed(Scancode::D)  {
            walking = true;
            if !arrowKeys {
                self.mainCharacter.direction = "left".to_string();
            }
            if self.meshBox.endeOfDirection.contains(&"left".to_string()) || self.mainCharacter.y > window.size_in_pixels().1 as f32 / 2.0 {
                self.mainCharacter.updatePosition(-5.0 * speed, 0.0, window);
            }
            else {
                self.meshBox.updatePosition(-1.0 * speed, 0.0, &textureVec[self.meshBox.texturePos]);
            }
        }
        if keyboardState.is_scancode_pressed(Scancode::D) && !keyboardState.is_scancode_pressed(Scancode::A)  {
            walking = true;
            if !arrowKeys {
                self.mainCharacter.direction = "right".to_string();
            }
            if self.meshBox.endeOfDirection.contains(&"right".to_string()) || self.mainCharacter.y > window.size_in_pixels().1 as f32 / 2.0 {
                self.mainCharacter.updatePosition(5.0 * speed, 0.0, window);
            }
            else {
                self.meshBox.updatePosition(1.0 * speed, 0.0, &textureVec[self.meshBox.texturePos]);
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
    pub fn update(&mut self,  textureVec: &Vec<Texture>) {
        for enemy in self.enemies.iter_mut() {
            //add enemy movement updates here
            if hitboxDetector(enemy.hitBox, self.mainCharacter.hitBox) && !self.mainCharacter.immune {
                self.mainCharacter.timeSinceHit = Instant::now();
                self.mainCharacter.stats.health -= 2.0;
                if self.mainCharacter.stats.health <= 0.0 {
                    self.alive = false;
                }
            }
            if hitboxDetector(enemy.hitBox, self.mainCharacter.swordAttack.hitBox) {
                let speed = 2.0;
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
        }
        let mut removeEnemy = Vec::new();
        for enemy in 0..self.enemies.len() {
            if self.enemies[enemy].health < 0.0 {
                removeEnemy.push(enemy);
            }
        }
        removeEnemy.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for enemy in removeEnemy {
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
