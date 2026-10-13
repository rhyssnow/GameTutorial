#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod roam;
pub mod attack;
pub mod parse;
pub mod trigger;

mod pixelengine {
    use sdl3::render::{FRect, Texture, WindowCanvas};
    use std::time::Instant;
    use sdl3::pixels::Color;
    use sdl3::video::Window;
    use crate::PixelEngine::trigger::trigger;

    pub struct meshBox {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        endeOfDirection: Vec<String>,
        triggers: trigger
    }

    impl meshBox {
        fn new(x: f32, y: f32, width: f32, height: f32, triggers: Vec) -> meshBox {meshBox{x, y, width, height, endeOfDirection: Vec::new(), triggers: Vec::new()}}
        fn render(&mut self, texture: &Texture, renderer: &mut WindowCanvas, renderArea: FRect) {
            let textureArea = FRect::new(self.x, self.y, renderArea.w / 6.0, renderArea.h / 6.0 );

            self.width = renderArea.w /6.0;
            self.height = renderArea.h /6.0;

            renderer.copy(texture, textureArea, renderArea).unwrap();
        }
        fn updatePosition(&mut self, mx: f32, my: f32, texture: &Texture) {
            self.x += mx;
            self.y += my;
            self.endeOfDirection.clear();

            if self.x <= 0.0{
                self.x = 0.0;
                self.endeOfDirection.push("left".to_string());
            }

            if self.x + self.width >= texture.width() as f32{
                self.x = texture.width() as f32 - self.width;
                self.endeOfDirection.push("right".to_string());
            }

            if self.y <= 0.0{
                self.y = 0.0;
                self.endeOfDirection.push("up".to_string());
            }

            if self.y + self.height >= texture.height() as f32{
                self.y = texture.height() as f32 - self.height;
                self.endeOfDirection.push("down".to_string());
            }
        }
    }

    pub struct mainCharacter{
        x: f32,
        y: f32,
        direction: String,
        walkingPhase: usize,
        characterUp: [FRect; 3],
        characterDown: [FRect; 3],
        characterRight: [FRect; 3],
        characterLeft: [FRect; 3],
        hitBox: FRect,
        stats: stats,
        swordAttack: swordAttack,
        lastPhaseChange: Instant,
        immune: bool,
        render: bool,
        timeSinceHit: Instant,
    }
    impl mainCharacter {
        fn new(x: f32, y: f32, direction: String, health: f32) -> mainCharacter {mainCharacter{
            x,
            y,
            direction,
            walkingPhase: 0,
            characterUp: [FRect::new(2.0, 33.0, 9.0, 16.0), FRect::new(12.0, 33.0, 9.0, 16.0), FRect::new(22.0, 33.0, 9.0, 16.0)],
            characterDown: [FRect::new(2.0, 49.0, 9.0, 16.0), FRect::new(12.0, 49.0, 9.0, 16.0), FRect::new(22.0, 49.0, 9.0, 16.0)] ,
            characterRight: [FRect::new(1.0, 17.0, 10.0, 16.0), FRect::new(12.0, 17.0, 10.0, 16.0), FRect::new(23.0, 17.0, 10.0, 16.0)],
            characterLeft: [FRect::new(1.0, 1.0, 10.0, 16.0), FRect::new(12.0, 1.0, 10.0, 16.0), FRect::new(23.0, 1.0, 10.0, 16.0)],
            hitBox: FRect{x, y, w: 70.0, h: 112.0},
            stats: stats::new(health),
            swordAttack: swordAttack::new("up".to_string()),
            lastPhaseChange: Instant::now(),
            immune: false,
            render: true,
            timeSinceHit: Instant::now(),
        }}
        fn walkPhaseChange(&mut self) {
            if self.lastPhaseChange.elapsed().as_millis() >= 200 {
                self.walkingPhase += 1;
                if self.walkingPhase > 2 {
                    self.walkingPhase = 1;
                }
                self.lastPhaseChange = Instant::now();
            }
        }
        fn update (&mut self) {
            if self.timeSinceHit.elapsed().as_millis() <= 250 {
                self.render = !self.render;
                self.immune = true;
            }
            else {
                self.immune = false;
                self.render = true;
            }
        }
        fn render(&mut self, texture: &Texture, renderer: &mut WindowCanvas) {
            self.hitBox.x = self.x;
            self.hitBox.y = self.y;

            let rectSet = match self.direction.as_str() {
                "up" => self.characterUp,
                "down" => self.characterDown,
                "right" => self.characterRight,
                "left" => self.characterLeft,
                _ => self.characterDown
            };

            self.hitBox.w = rectSet[self.walkingPhase].w * 7.0;
            self.hitBox.h = rectSet[self.walkingPhase].h * 7.0;

            renderer.copy(texture, rectSet[self.walkingPhase], self.hitBox).unwrap();
        }
        fn updatePosition(&mut self, x: f32, y: f32, window: &Window) {
            self.x += x;
            self.y += y;

            if self.x < 0.0 {
                self.x = 0.0;
            }
            if self.y < 0.0 {
                self.y = 0.0;
            }
            if self.x + 70.0 > 0.0 + window.size_in_pixels().0 as f32 {
                self.x = window.size_in_pixels().0 as f32 - 70.0;
            }
            if self.y + 105.0 > 0.0 + window.size_in_pixels().1 as f32 {
                self.y = window.size_in_pixels().1 as f32 - 105.0;
            }
        }
    }
    struct stats {
        health: f32,
        maxHealth: f32,
        stamina: f32,
        maxStamina: f32,
        mana: f32,
        maxMana: f32,
    }
    impl stats {
        fn new(health: f32) -> stats {stats{health, maxHealth: health, stamina:10.0, maxStamina:10.0, mana:10.0, maxMana:10.0}}
        fn render(&self, renderer: &mut WindowCanvas) {
            let maxHealthRenderBox = FRect::new(
                105.0,
                10.0,
                self.maxHealth * 25.0,
                25.0,
            );
            let healthRenderBox = FRect::new(
                108.0,
                13.0,
                self.health * 25.0 - 6.0,
                19.0,
            );
            renderer.set_draw_color(Color::RGB(180, 180, 180));
            renderer.fill_rect(maxHealthRenderBox).unwrap();
            renderer.set_draw_color(Color::RGB(130, 20, 20));
            renderer.fill_rect(healthRenderBox).unwrap();

            let maxStaminaRenderBox = FRect::new(
                105.0,
                47.0,
                self.maxStamina * 50.0,
                25.0,
            );
            let staminaRenderBox = FRect::new(
                108.0,
                50.0,
                self.stamina * 50.0 - 8.0,
                19.0,
            );
            renderer.set_draw_color(Color::RGB(180, 180, 180));
            renderer.fill_rect(maxStaminaRenderBox).unwrap();
            renderer.set_draw_color(Color::RGB(30, 130, 30));
            renderer.fill_rect(staminaRenderBox).unwrap();

            let maxManaRenderBox = FRect::new(
                105.0,
                85.0,
                self.maxMana * 50.0,
                25.0,
            );
            let manaRenderBox = FRect::new(
                108.0,
                88.0,
                self.mana * 50.0 - 6.0,
                19.0,
            );
            renderer.set_draw_color(Color::RGB(180, 180, 180));
            renderer.fill_rect(maxManaRenderBox).unwrap();
            renderer.set_draw_color(Color::RGB(80, 0, 140));
            renderer.fill_rect(manaRenderBox).unwrap();
        }
    }

    struct swordAttack {
        swingUp: [FRect; 6],
        swingDown: [FRect; 6],
        swingLeft: [FRect; 6],
        swingRight: [FRect; 6],
        hitBox: FRect,
        swingPhase: usize,
        swingPhaseFloat: f32,
        direction: String,
        swingCooldown: Instant,
    }

    impl swordAttack{
        fn new(direction: String) -> swordAttack {swordAttack{
            swingUp: [FRect::new(1.0, 1.0, 3.0, 3.0), FRect::new(5.0, 1.0, 13.0, 11.0), FRect::new(19.0, 1.0, 25.0, 13.0), FRect::new(47.0, 1.0, 31.0, 11.0), FRect::new(79.0, 1.0, 33.0, 10.0), FRect::new(113.0, 1.0, 32.0, 14.0)],
            swingDown: [FRect::new(1.0, 16.0, 3.0, 3.0), FRect::new(5.0, 16.0, 13.0, 11.0), FRect::new(19.0, 16.0, 25.0, 13.0), FRect::new(47.0, 16.0, 31.0, 11.0), FRect::new(79.0, 16.0, 33.0, 11.0), FRect::new(113.0, 16.0, 32.0, 14.0)],
            swingLeft: [FRect::new(1.0, 30.0, 3.0, 2.0), FRect::new(5.0, 30.0, 12.0, 9.0), FRect::new(17.0, 30.0, 12.0, 15.0), FRect::new(30.0, 30.0, 11.0, 20.0), FRect::new(42.0, 30.0, 10.0, 21.0), FRect::new(53.0, 30.0, 14.0, 20.0)],
            swingRight: [FRect::new(1.0, 53.0, 3.0, 2.0), FRect::new(5.0, 53.0, 12.0, 8.0), FRect::new(17.0, 53.0, 12.0, 15.0), FRect::new(30.0, 53.0, 11.0, 20.0), FRect::new(42.0, 53.0, 10.0, 21.0), FRect::new(53.0, 53.0, 14.0, 20.0)],
            hitBox: FRect{x: 0.0, y: 0.0, w: 0.0, h: 0.0},
            swingPhase: 6,
            swingPhaseFloat: 6.0,
            direction,
            swingCooldown: Instant::now()}}
        fn render (&mut self, renderer: &mut WindowCanvas, characterX: f32, characterY: f32, texture: &Texture) {
            if self.swingPhase == 6 {
                self.hitBox.w = 0.0;
                self.hitBox.h = 0.0;
                self.hitBox.x = 0.0;
                self.hitBox.y = 0.0;
                return;
            }
            let renderX:f32 = match self.direction.as_str() {
                "up" => match self.swingPhase as f32 {
                    0.0 | 1.0 | 2.0 => characterX - 19.5 * 7.0 + 35.0,
                    3.0 => characterX - 17.5 * 7.0 + 35.0,
                    4.0 => characterX - 14.5 * 7.0 + 35.0,
                    5.0 => characterX - 11.5 * 7.0 + 35.0,
                    _ => 0.0,
                }
                "down" => match self.swingPhase as f32 {
                    0.0 => characterX + 16.5 * 7.0 + 35.0,
                    1.0 => characterX + 6.5 * 7.0 + 35.0,
                    2.0 => characterX + -5.5 * 7.0 + 35.0,
                    3.0 => characterX + -11.5 * 7.0 + 35.0,
                    4.0 => characterX + -16.5 * 7.0 + 35.0,
                    5.0 => characterX + -18.5 * 7.0 + 35.0,
                    _ => 0.0,
                },
                "left" => match self.swingPhase as f32 {
                    0.0 => characterX - 3.0 * 7.9 - 30.0,
                    1.0  => characterX - 11.0 * 7.0 - 30.0,
                    2.0 |3.0 |4.0 | 5.0 => characterX - 12.0 * 7.0 - 30.0,

                    _ => 0.0,
                },
                "right" => match self.swingPhase as f32 {
                    0.0 | 1.0 | 2.0  => characterX + 90.0,
                    3.0 => characterX + 1.0 * 7.0 + 100.0,
                    4.0 => characterX + 2.0 * 7.0 + 100.0,
                    5.0 => characterX - 2.0 * 7.0 + 100.0,
                    _ => 0.0,
                },
                _ => 0.0,
            };
            let renderY:f32 = match self.direction.as_str() {
                "up" => match self.swingPhase as f32 {
                    0.0 => characterY,
                    1.0 => characterY - 8.0 * 7.0,
                    2.0 | 3.0 | 4.0 | 5.0 => characterY - 10.0 * 7.0,
                    _ => 0.0
                }
                "down" => match self.swingPhase as f32 {
                    0.0 | 1.0 | 2.0 | 5.0 => characterY + 95.0,
                    3.0 |4.0 => characterY + 109.0,
                    _ => 0.0
                }
                "left" => match self.swingPhase as f32 {
                    0.0 => characterY - 3.0 * 7.0 + 140.0,
                    1.0 => characterY - 10.0 * 7.0 + 140.0,
                    2.0 => characterY - 16.0 * 7.0 + 140.0,
                    3.0 => characterY - 22.0 * 7.0 + 140.0,
                    4.0 => characterY - 25.0 * 7.0 + 140.0,
                    5.0 => characterY - 26.0 * 7.0 + 140.0,
                    _ => 0.0
                }
                "right" => match self.swingPhase as f32 {
                    0.0 | 1.0 | 2.0 | 3.0 => characterY - 35.0,
                    4.0 => characterY + 3.0 * 7.0 - 35.0,
                    5.0 => characterY + 5.0 * 7.0 - 35.0,
                    _ => 0.0,
                }
                _ => 0.0,
            };

            self.hitBox = match self.direction.as_str() {
                "up" => FRect { x: renderX, y: renderY, w: self.swingUp[self.swingPhase].w * 7.0, h: self.swingUp[self.swingPhase].h * 7.0 },
                "down" => FRect { x: renderX, y: renderY, w: self.swingDown[self.swingPhase].w * 7.0, h: self.swingDown[self.swingPhase].h * 7.0 },
                "left" => FRect { x: renderX, y: renderY, w: self.swingLeft[self.swingPhase].w * 7.0, h: self.swingLeft[self.swingPhase].h * 7.0 },
                "right" => FRect { x: renderX, y: renderY, w: self.swingRight[self.swingPhase].w * 7.0, h: self.swingRight[self.swingPhase].h * 7.0 },
                _ => FRect::new(0.0, 0.0, 0.0, 0.0),
            };

            match self.direction.as_str() {
                "up" => renderer.copy(texture, self.swingUp[self.swingPhase], self.hitBox).unwrap(),
                "down" => renderer.copy(texture, self.swingDown[self.swingPhase], self.hitBox).unwrap(),
                "left" => renderer.copy(texture, self.swingLeft[self.swingPhase], self.hitBox).unwrap(),
                "right" => renderer.copy(texture, self.swingRight[self.swingPhase], self.hitBox).unwrap(),
                _ => return
            };

            self.swingPhaseFloat += 0.5;
            self.swingPhase = self.swingPhaseFloat.floor() as usize;
        }
    }
}