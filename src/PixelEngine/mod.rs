#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod roam;
pub mod attack;
pub mod parse;
pub mod trigger;
pub mod basic;

mod pixelengine {
    use sdl3::render::{Texture};
    use sdl3::keyboard::Scancode;
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

        fn input (&mut self, window: &Window, textureVec: &Vec<Texture>, running: &mut bool) {
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
    }
}