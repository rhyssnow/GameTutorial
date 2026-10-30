#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{fs, io};
use std::fs::File;
use std::io::Write;
use sdl3::render::{FRect, Texture, WindowCanvas};
use std::time::Instant;
use sdl3::keyboard::{KeyboardState, Scancode};
use sdl3::pixels::Color;
use sdl3::video::Window;
use serde_json::{json, Value};
use crate::PixelEngine::attack::attacks;
use crate::PixelEngine::hitboxDetector;
use crate::PixelEngine::roam::roamPattern;

pub struct meshBox {
    pub x: f32,
    pub y: f32,
    width: f32,
    height: f32,
    pub endOfDirection: Vec<String>,
    pub texturePos: usize,
    pub respawnCoords: (f32, f32),
}

impl meshBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> meshBox {meshBox{x, y, width, height, endOfDirection: Vec::new(), texturePos: 4, respawnCoords: (x, y)}}

    pub fn render(&mut self, textureVec: &Vec<Texture>, renderer: &mut WindowCanvas, renderArea: FRect) {
        let textureArea = FRect::new(self.x, self.y, renderArea.w / 6.0, renderArea.h / 6.0 );

        self.width = renderArea.w /6.0;
        self.height = renderArea.h /6.0;

        renderer.copy(&textureVec[self.texturePos], textureArea, renderArea).unwrap();
    }
    pub fn renderForeground(&mut self, textureVec: &Vec<Texture>, renderer: &mut WindowCanvas, renderArea: FRect) {
        let textureArea = FRect::new(self.x, self.y, renderArea.w / 6.0, renderArea.h / 6.0 );

        self.width = renderArea.w /6.0;
        self.height = renderArea.h /6.0;

        renderer.copy(&textureVec[self.texturePos - 1], textureArea, renderArea).unwrap();
    }
    pub fn updatePosition(&mut self, mx: f32, my: f32, texture: &Texture) {
        self.x += mx;
        self.y += my;
        self.endOfDirection.clear();

        if self.x <= 0.0 {
            self.x = 0.0;
            self.endOfDirection.push("left".to_string());
        }

        if self.x + self.width >= texture.width() as f32 {
            self.x = texture.width() as f32 - self.width;
            self.endOfDirection.push("right".to_string());
        }

        if self.y <= 0.0 {
            self.y = 0.0;
            self.endOfDirection.push("up".to_string());
        }

        if self.y + self.height >= texture.height() as f32 {
            self.y = texture.height() as f32 - self.height;
            self.endOfDirection.push("down".to_string());
        }
    }
}

pub struct hazard {
    pub damage: f32,
    pub area: FRect,
}

impl hazard {
    pub fn new(damage: f32, area: FRect) -> hazard {hazard{damage, area}}
    pub fn update(&mut self, mainCharacter: &mut mainCharacter, meshBox: &mut meshBox, alive: &mut bool) {
        let hitbox = FRect::new((self.area.x - meshBox.x) * 6.0, (self.area.y - meshBox.y) * 6.0, self.area.w * 6.0 , self.area.h * 6.0);
        let mainHitbox = FRect::new(
            mainCharacter.x + 3.5,
            mainCharacter.y + 110.0,
            mainCharacter.hitBox.w - 7.0,
            mainCharacter.hitBox.h - 110.0,
        );

        if hitboxDetector(hitbox, mainHitbox) && !mainCharacter.immune {
            mainCharacter.timeSinceHit = Instant::now();
            mainCharacter.stats.health -= 2.0;
            mainCharacter.immune = true;
            if mainCharacter.stats.health < 0.0 {
                *alive = false;
            }
            meshBox.x = meshBox.respawnCoords.0;
            meshBox.y = meshBox.respawnCoords.1;
            mainCharacter.x = mainCharacter.respawnCoords.0;
            mainCharacter.y = mainCharacter.respawnCoords.1;
        }
    }
}

pub struct object{
    pub(crate) area: FRect,
    areaOfEffect: FRect,
    objectType: String,
    textureRect: Vec<FRect>,
    texturePosition: usize,
    animationState: usize,
    timeSinceStateChange: Instant,
}

impl object{
    pub fn new(area: FRect, areaOfEffect: FRect, objectType: String, textureRect: Vec<FRect>, texturePosition: usize) -> object{object{area, areaOfEffect, objectType, textureRect, texturePosition, animationState: 0, timeSinceStateChange: Instant::now()}}
    pub fn render(&mut self, renderer: &mut WindowCanvas, meshBox: &meshBox, textureVec: &Vec<Texture>, window: &Window){
        let renderArea = FRect::new((self.area.x - meshBox.x) * 6.0, (self.area.y - meshBox.y) * 6.0, self.area.w * 6.0, self.area.h * 6.0);

        renderer.copy(&textureVec[self.texturePosition], self.textureRect[self.animationState], renderArea).unwrap();
        match self.objectType.as_str(){
            "litBeacon" => litBeaconRender(self.areaOfEffect, window, renderer),
            _ => {}
        }
    }
    pub fn updateInput(&mut self, mainCharacter: &mainCharacter, window: &Window, keyboardState: &KeyboardState, meshBox: &meshBox){
        if self.timeSinceStateChange.elapsed().as_millis() > 500 {
            self.timeSinceStateChange = Instant::now();
            self.animationState += 1;
            if self.animationState > self.textureRect.len() - 1{
                self.animationState = 0;
            }
            println!("{}", self.animationState);
        }
        match self.objectType.as_str(){
            "litBeacon" => litBeaconInput(self.areaOfEffect, mainCharacter, window, keyboardState, meshBox),
            _ => {}
        }
    }
}

fn litBeaconInput (areaOfEffect: FRect, mainCharacter: &mainCharacter, window: &Window, keyboardState: &KeyboardState, meshBox: &meshBox) {
    let hitbox = FRect::new((areaOfEffect.x - meshBox.x) * 6.0, (areaOfEffect.y - meshBox.y) * 6.0, areaOfEffect.w * 6.0, areaOfEffect.h * 6.0);
    if hitboxDetector(areaOfEffect, hitbox) && keyboardState.is_scancode_pressed(Scancode::F) {
        let content = fs::read_to_string(&"src/Metadata/save.json").unwrap();
        let mut data: Value = serde_json::from_str(&content).unwrap();

        data["maxHealth"] = json!(mainCharacter.stats.maxHealth);
        data["x"] = json!(meshBox.x);
        data["y"] = json!(meshBox.y);
        data["mainX"] = json!(mainCharacter.x);
        data["mainY"] = json!(mainCharacter.y);

        let mut file = File::create("src/Metadata/save.json").unwrap();
        file.write_all(serde_json::to_string(&data).unwrap().as_bytes()).unwrap();
    }
}

fn litBeaconRender (areaOfEffect: FRect, window: &Window, renderer: &mut WindowCanvas) {

}

#[derive(Clone)]
pub struct mainCharacter{
    pub x: f32,
    pub y: f32,
    pub direction: String,
    pub walkingPhase: usize,
    characterUp: [FRect; 3],
    characterDown: [FRect; 3],
    characterRight: [FRect; 3],
    characterLeft: [FRect; 3],
    pub hitBox: FRect,
    pub stats: stats,
    pub swordAttack: swordAttack,
    lastPhaseChange: Instant,
    pub immune: bool,
    pub render: bool,
    pub timeSinceHit: Instant,
    pub timeSinceJumped: Instant,
    texturePos: usize,
    pub jumping: bool,
    height: f32,
    pub velocity: f32,
    pub respawnCoords: (f32, f32),
}
impl mainCharacter {
    pub fn new(x: f32, y: f32, health: f32) -> mainCharacter {mainCharacter{
        x,
        y,
        direction: "down".to_string(),
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
        texturePos: 0,
        timeSinceJumped: Instant::now(),
        jumping: false,
        height: 0.0,
        velocity: 0.0,
        respawnCoords: (x, y),
    }}
    pub fn walkPhaseChange(&mut self) {
        if self.lastPhaseChange.elapsed().as_millis() >= 200 {
            self.walkingPhase += 1;
            if self.walkingPhase > 2 {
                self.walkingPhase = 1;
            }
            self.lastPhaseChange = Instant::now();
        }
    }
    pub fn update (&mut self) {
        if self.timeSinceHit.elapsed().as_millis() <= 250 {
            self.render = !self.render;
            self.immune = true;
        }
        else {
            self.immune = false;
            self.render = true;
        }
        if self.velocity != 0.0 {
            self.jumping = true;
        }
        else {
            self.jumping = false;
        }
        if self.jumping {
            self.immune = true;
            self.velocity -= 0.2;
            self.height += self.velocity;
            if self.height <= 0.0 {
                self.jumping = false;
                self.velocity = 0.0;
                self.height = 0.0;
                self.timeSinceJumped = Instant::now();
            }
        }
    }
    pub fn render(&mut self, textureVec: &Vec<Texture>, renderer: &mut WindowCanvas) {
        self.hitBox.x = self.x;
        self.hitBox.y = self.y;

        if !self.jumping {
            let rectSet = match self.direction.as_str() {
                "up" => self.characterUp,
                "down" => self.characterDown,
                "right" => self.characterRight,
                "left" => self.characterLeft,
                _ => self.characterDown
            };

            self.hitBox.w = rectSet[self.walkingPhase].w * 7.0;
            self.hitBox.h = rectSet[self.walkingPhase].h * 7.0;
            renderer.copy(&textureVec[self.texturePos], rectSet[self.walkingPhase], self.hitBox).unwrap();
        }

        if self.jumping {

            let mut mainCharacterRect = FRect::new(15.0, 66.0, 10.0, 15.0);
            mainCharacterRect.y += match self.direction.as_str() {
                "up" => 32.0,
                "down" => 48.0,
                "right" => 16.0,
                "left" => 0.0,
                _ => 2.0
            };

            if self.direction == "up" || self.direction == "down"{
                mainCharacterRect.w -= 1.0;
            }

            self.hitBox.w = mainCharacterRect.w * 7.0;
            self.hitBox.h = mainCharacterRect.h * 7.0;

            let mut mainCharacterRenderRect = self.hitBox;
            mainCharacterRenderRect.y -= self.height;

            let mut shadowRect = FRect::new(0.0, 66.0, 10.0, 15.0);

            shadowRect.x = match self.direction.as_str() {
                "up" => 2.0,
                "down" => 2.0,
                "right" => 0.0,
                "left" => 3.0,
                _ => 2.0
            };
            renderer.copy(&textureVec[self.texturePos], shadowRect, self.hitBox).unwrap();

            renderer.copy(&textureVec[self.texturePos], mainCharacterRect, mainCharacterRenderRect).unwrap();
        }
    }
    pub fn updatePosition(&mut self, x: f32, y: f32, window: &Window) {
        self.x += x;
        self.y += y;

        if self.x < 0.0 - self.hitBox.w / 2.0 {
            self.x = 0.0;
        }
        if self.y < 0.0 - self.hitBox.h / 2.0 {
            self.y = 0.0;
        }
        if self.x + 70.0 > window.size_in_pixels().0 as f32 + self.hitBox.w / 2.0 {
            self.x = window.size_in_pixels().0 as f32 - 70.0;
        }
        if self.y + 105.0 > 0.0 + window.size_in_pixels().1 as f32 + self.hitBox.h / 2.0 {
            self.y = window.size_in_pixels().1 as f32 - 105.0;
        }
    }
}

#[derive(Clone)]
pub struct stats {
    pub health: f32,
    pub(crate) maxHealth: f32,
    pub stamina: f32,
    pub maxStamina: f32,
    pub(crate) mana: f32,
    pub(crate) maxMana: f32,
    texturePos: usize
}
impl stats {
    fn new(health: f32) -> stats {stats{health, maxHealth: health, stamina:10.0, maxStamina:10.0, mana:10.0, maxMana:10.0, texturePos: 1}}
    pub fn render(&self, renderer: &mut WindowCanvas) {
        let maxHealthRenderBox = FRect::new(
            104.0,
            10.0,
            self.maxHealth * 25.0,
            25.0,
        );
        let healthRenderBox = FRect::new(
            107.0,
            13.0,
            self.health * 25.0 - 6.0,
            19.0,
        );


        renderer.set_draw_color(Color::RGB(180, 180, 180));
        renderer.fill_rect(maxHealthRenderBox).unwrap();
        renderer.set_draw_color(Color::RGB(130, 20, 20));
        renderer.fill_rect(healthRenderBox).unwrap();

        let maxStaminaRenderBox = FRect::new(
            104.0,
            47.0,
            self.maxStamina * 50.0,
            25.0,
        );
        let staminaRenderBox = FRect::new(
            107.0,
            50.0,
            self.stamina * 50.0 - 6.0,
            19.0,
        );
        renderer.set_draw_color(Color::RGB(180, 180, 180));
        renderer.fill_rect(maxStaminaRenderBox).unwrap();
        if self.stamina > 0.0 {
            renderer.set_draw_color(Color::RGB(30, 130, 30));
            renderer.fill_rect(staminaRenderBox).unwrap();
        }

        let maxManaRenderBox = FRect::new(
            104.0,
            85.0,
            self.maxMana * 50.0,
            25.0,
        );
        let manaRenderBox = FRect::new(
            107.0,
            88.0,
            self.mana * 50.0 - 6.0,
            19.0,
        );
        renderer.set_draw_color(Color::RGB(180, 180, 180));
        renderer.fill_rect(maxManaRenderBox).unwrap();
        if self.mana > 0.0 {
            renderer.set_draw_color(Color::RGB(80, 0, 140));
            renderer.fill_rect(manaRenderBox).unwrap();
        }
    }
}

#[derive(Clone)]
pub struct swordAttack {
    swingUp: [FRect; 6],
    swingDown: [FRect; 6],
    swingLeft: [FRect; 6],
    swingRight: [FRect; 6],
    pub hitBox: FRect,
    pub swingPhase: usize,
    pub(crate) swingPhaseFloat: f32,
    pub direction: String,
    pub(crate) swingCooldown: Instant,
    texturePos: usize,
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
        swingCooldown: Instant::now(),
        texturePos: 2,
    }}
    pub fn render (&mut self, renderer: &mut WindowCanvas, characterX: f32, characterY: f32, textureVec: &Vec<Texture>) {
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
            "up" => renderer.copy(&textureVec[self.texturePos], self.swingUp[self.swingPhase], self.hitBox).unwrap(),
            "down" => renderer.copy(&textureVec[self.texturePos], self.swingDown[self.swingPhase], self.hitBox).unwrap(),
            "left" => renderer.copy(&textureVec[self.texturePos], self.swingLeft[self.swingPhase], self.hitBox).unwrap(),
            "right" => renderer.copy(&textureVec[self.texturePos], self.swingRight[self.swingPhase], self.hitBox).unwrap(),
            _ => return
        };

        self.swingPhaseFloat += 0.5;
        self.swingPhase = self.swingPhaseFloat.floor() as usize;
    }
}

#[derive(Clone)]
pub struct enemy {
    pub x: f32,
    pub y: f32,
    pub(crate) direction: String,
    pub(crate) walkingPhase: usize,
    enemyUp: Vec<FRect>,
    enemyDown: Vec<FRect>,
    enemyRight: Vec<FRect>,
    enemyLeft: Vec<FRect>,
    pub hitBox: FRect,
    pub health: f32,
    pub(crate) lastPhaseChange: Instant,
    pub render: bool,
    pub timeSinceHit: Instant,
    pub(crate) roam: bool,
    pub(crate) attacks: attacks,
    pub roamPattern: roamPattern,
    texturePos: usize
}
impl enemy {
    pub fn new(x: f32, y: f32, enemyUp: Vec<FRect>, enemyDown: Vec<FRect>, enemyLeft: Vec<FRect>, enemyRight: Vec<FRect>, attacks: attacks, roamPattern: roamPattern, texturePos: usize) -> enemy {enemy{
        x,
        y,
        direction: "up".to_string(),
        walkingPhase: 0,
        enemyUp,
        enemyDown,
        enemyLeft,
        enemyRight,
        hitBox: FRect{x, y, w: 0.0, h: 0.0},
        health: 4.0,
        lastPhaseChange: Instant::now(),
        render: true,
        timeSinceHit: Instant::now(),
        roam: true,
        attacks,
        roamPattern,
        texturePos
    }}
    pub fn render(&mut self, textureVec: &Vec<Texture>, renderer: &mut WindowCanvas, meshBox: &mut meshBox) {
        if !self.render{
            return;
        }
        self.hitBox.x = (self.x - meshBox.x) * 6.0;
        self.hitBox.y = (self.y - meshBox.y) * 6.0;

        let rectSet = match self.direction.as_str() {
            "up" => &self.enemyUp,
            "down" => &self.enemyDown,
            "right" => &self.enemyRight,
            "left" => &self.enemyLeft,
            _ => &self.enemyDown
        };

        self.hitBox.w = rectSet[self.walkingPhase].w * 6.0;
        self.hitBox.h = rectSet[self.walkingPhase].h * 6.0;

        self.hitBox.x -= self.hitBox.w / 2.0;
        self.hitBox.y -= self.hitBox.h / 2.0;

        renderer.copy(&textureVec[self.texturePos], rectSet[self.walkingPhase], self.hitBox).unwrap();
    }
    pub fn update(&mut self) {
        if self.timeSinceHit.elapsed().as_millis() <= 250 {
            self.render = !self.render;
        }
        else {
            self.render = true;
        }
    }
    pub fn updatePosition(&mut self, x: f32, y: f32, backgroundTexture: &Texture) {
        self.x += x;
        self.y += y;

        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.y < 0.0 {
            self.y = 0.0;
        }
        if self.x + self.hitBox.w > backgroundTexture.width() as f32 * 6.0 {
            self.x = backgroundTexture.width() as f32 - self.hitBox.w;
        }
        if self.y + self.hitBox.h > backgroundTexture.height() as f32 * 6.0 {
            self.y = backgroundTexture.height() as f32 - self.hitBox.h;
        }
    }
}