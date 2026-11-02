use std::time::Instant;
use sdl3::render::{FRect, Texture, WindowCanvas};
use crate::PixelEngine::basic::{ mainCharacter, meshBox};
use crate::PixelEngine::hitboxDetector;

pub struct boss {
    x: f32,
    pub(crate) y: f32,
    pub(crate) hitbox: FRect,
    triggerArea: FRect,
    boss: String,
    pub(crate) direction: String,
    pub(crate) walkingPhase: usize,
    bossUp: Vec<FRect>,
    bossDown: Vec<FRect>,
    bossRight: Vec<FRect>,
    bossLeft: Vec<FRect>,
    pub health: f32,
    pub(crate) lastPhaseChange: Instant,
    pub render: bool,
    pub timeSinceHit: Instant,
    pub(crate) activated: bool,
    texturePos: usize
}

impl boss {
    pub fn new(boss: String, x: f32, y: f32, tx: f32, ty: f32, tw: f32, th: f32, bossUp: Vec<FRect>, bossDown: Vec<FRect>, bossLeft: Vec<FRect>, bossRight: Vec<FRect>, texturePos: usize) -> boss {boss{
        x,
        y,
        hitbox: FRect::new(x, y, 0.0, 0.0),
        triggerArea: FRect{x: tx,y:  ty, w: tw, h: th},
        boss,
        direction: "up".to_string(),
        walkingPhase: 0,
        bossUp,
        bossDown,
        bossLeft,
        bossRight,
        health: 4.0,
        lastPhaseChange: Instant::now(),
        render: true,
        timeSinceHit: Instant::now(),
        activated: false,
        texturePos
    }}
    pub fn render(&mut self, textureVec: &Vec<Texture>, renderer: &mut WindowCanvas, meshBox: &mut meshBox) {
        if !self.render{
            return;
        }
        self.hitbox = FRect::new(
            (self.x - meshBox.x) * 6.0,
            (self.y - meshBox.y) * 6.0,
            0.0,
            0.0
        );

        let rectSet = match self.direction.as_str() {
            "up" => &self.bossUp,
            "down" => &self.bossDown,
            "right" => &self.bossRight,
            "left" => &self.bossLeft,
            _ => &self.bossDown
        };

        self.hitbox.w = rectSet[self.walkingPhase].w * 6.0;
        self.hitbox.h = rectSet[self.walkingPhase].h * 6.0;

        self.hitbox.x -= self.hitbox.w / 2.0;
        self.hitbox.y -= self.hitbox.h / 2.0;

        renderer.copy(&textureVec[self.texturePos], rectSet[self.walkingPhase], self.hitbox).unwrap();
    }
    pub fn update(&mut self, mainCharacter: &mut mainCharacter) {
        if !self.activated {
            match self.boss.as_str() {
                "protectorKnight" => protectorKnightUnactivated(self, mainCharacter),
                _ => {}
            }
        }
        else {
            match self.boss.as_str() {
                "protectorKnight" => protectorKnightActivated(self),
                _ => {}
            }
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
        if self.x + self.hitbox.w > backgroundTexture.width() as f32 * 6.0 {
            self.x = backgroundTexture.width() as f32 - self.hitbox.w;
        }
        if self.y + self.hitbox.h > backgroundTexture.height() as f32 * 6.0 {
            self.y = backgroundTexture.height() as f32 - self.hitbox.h;
        }
    }
}

fn protectorKnightUnactivated (boss: &mut boss, mainCharacter: &mut mainCharacter) {
    if hitboxDetector(boss.hitbox, mainCharacter.hitBox) {
        boss.activated == true;
    }
}
fn protectorKnightActivated (boss: &mut boss) {
    if boss.timeSinceHit.elapsed().as_millis() <= 250 {
        boss.render = !boss.render;
    }
    else {
        boss.render = true;
    }
}