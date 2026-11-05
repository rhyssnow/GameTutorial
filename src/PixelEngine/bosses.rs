use std::time::Instant;
use sdl3::render::{FRect, Texture, WindowCanvas};
use sdl3::video::Window;
use rand::random;
use crate::PixelEngine::basic::{mainCharacter, meshBox, swordAttack};
use crate::PixelEngine::{asPositive, damageMaincharacter, hitboxDetector, movementCalculator};

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
    alive: bool,
    pub(crate) dead: bool,
    texturePos: usize,
    damage: f32,
    sword: swordAttack,
    mode: i32,
    time: Instant,
    bool: bool,
    bool2: bool,
    float: f32,
    float2: f32
}

impl boss {
    pub fn new(boss: String, x: f32, y: f32, tx: f32, ty: f32, tw: f32, th: f32, bossUp: Vec<FRect>, bossDown: Vec<FRect>, bossLeft: Vec<FRect>, bossRight: Vec<FRect>, damage: f32, health: f32, texturePos: usize) -> boss {boss{
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
        damage,
        health,
        lastPhaseChange: Instant::now(),
        render: true,
        timeSinceHit: Instant::now(),
        activated: false,
        alive: true,
        dead: false,
        texturePos,
        sword: swordAttack::new("up".to_string()),
        mode: 0,
        time: Instant::now(),
        bool: false,
        bool2: false,
        float: 0.0,
        float2: 0.0
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

        self.sword.render(renderer, self.hitbox, textureVec)
    }
    pub fn update(&mut self, mainCharacter: &mut mainCharacter, meshBox: &mut meshBox, textureVec: &Vec<Texture>, window: &Window, alive: &mut bool) {
        if !self.activated {
            match self.boss.as_str() {
                "protectorKnight" => protectorKnightUnactivated(self, mainCharacter, meshBox),
                _ => {}
            }
        }
        else {
            match self.boss.as_str() {
                "protectorKnight" => protectorKnightActivated(self, mainCharacter, meshBox, &textureVec[4], window, alive),
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
        if self.x + self.hitbox.w > backgroundTexture.width() as f32 {
            self.x = backgroundTexture.width() as f32 - self.hitbox.w;
        }
        if self.y + self.hitbox.h > backgroundTexture.height() as f32{
            self.y = backgroundTexture.height() as f32 - self.hitbox.h;
        }
    }
}

fn protectorKnightUnactivated (boss: &mut boss, mainCharacter: &mut mainCharacter, meshBox: &meshBox) {
    let hitboxTrigger = FRect::new(
        (boss.triggerArea.x - meshBox.x) * 6.0,
        (boss.triggerArea.y - meshBox.y) * 6.0,
        boss.triggerArea.w * 6.0,
        boss.triggerArea.h * 6.0
    );
    if hitboxDetector(hitboxTrigger, mainCharacter.hitBox) {
        boss.activated = true;
    }
}
fn protectorKnightActivated (boss: &mut boss, mainCharacter: &mut mainCharacter, meshBox: &meshBox, backgroundTexture: &Texture, window: &Window, alive: &mut bool) {
    if boss.timeSinceHit.elapsed().as_millis() <= 250 {
        boss.render = !boss.render;
    }
    else {
        boss.render = true;
    }
    if boss.lastPhaseChange.elapsed().as_millis() >= 300 {
        boss.walkingPhase += 1;
        if boss.walkingPhase > 1 {
            boss.walkingPhase = 0;
        }
        boss.lastPhaseChange = Instant::now();
    }

    let mainPosX = (mainCharacter.x + mainCharacter.hitBox.w / 2.0) + meshBox.x * 6.0;
    let mainPosY = (mainCharacter.y + mainCharacter.hitBox.h / 2.0) + meshBox.y * 6.0;

    let dx = mainPosX - boss.x * 6.0;
    let dy = mainPosY - boss.y * 6.0;

    if hitboxDetector(boss.hitbox, mainCharacter.swordAttack.hitBox) {
        let speed = 0.5;
        let mainPosX = mainCharacter.x + meshBox.x * 6.0;
        let mainPosY = mainCharacter.y + meshBox.y * 6.0;

        let dx = mainPosX - boss.x * 6.0;
        let dy = mainPosY - boss.y * 6.0;

        let m = movementCalculator(dx, dy, speed);

        boss.health -= mainCharacter.stats.damage;
        boss.updatePosition(-m.0, -m.1, backgroundTexture);
        boss.timeSinceHit = Instant::now();
    }

    if boss.health <= 0.0 {
        boss.dead = true;
    }
    boss.mode = match boss.mode {
        0 => {
            let speed = 0.5;

            let m = movementCalculator(dx, dy, speed);
            let mut vertical = true;
            if asPositive(m.0) > asPositive(m.1) {
                vertical = false;
            }
            if vertical == true {
                if m.1 > 0.0 {
                    boss.direction = String::from("down");
                }
                else {
                    boss.direction = String::from("up");
                }
            }
            else {
                if m.0 > 0.0 {
                    boss.direction = String::from("right");
                }
                else {
                    boss.direction = String::from("left");
                }
            }

            boss.updatePosition(m.0, m.1, backgroundTexture);

            let mut number = 0;
            if boss.time.elapsed().as_millis() > 1000 {
                number = rand::random_range(0..4);
                boss.time = Instant::now();
                if (dx*dx + dy*dy).sqrt() > 1000.0 {
                    number = 3;
                }
            }
            number
        },
        1 => {
            let mut number = 1;
            if boss.bool {
                if boss.sword.swingPhase == 6 && boss.bool2 {
                    number = 0;
                    boss.bool = false;
                    boss.bool2 = false;
                    boss.time = Instant::now();
                } else if boss.bool2{
                    damageMaincharacter(mainCharacter, window, boss.sword.hitBox, boss.damage * 1.25, alive);
                }
                else {
                    if boss.sword.swingCooldown.elapsed().as_millis() > 400 {
                        boss.sword.swingPhase = 0;
                        boss.sword.swingPhaseFloat = 0.0;
                        boss.sword.direction = boss.direction.clone();
                        boss.bool2 = true;
                    }
                }
            }
            if !boss.bool {
                 if (dx * dx + dy * dy).sqrt() < 300.0 && boss.sword.swingPhase == 6 {
                    boss.sword.swingCooldown = Instant::now();
                    boss.bool = true;
                     boss.bool2 = false;
                } else {
                    number = 0;
                }
            }
            number
        }
        2 => {
            let mut number = 2;

            if boss.bool {
                if boss.sword.swingCooldown.elapsed().as_millis() < 700 && boss.bool2 {
                    boss.sword.swingPhase = 0;
                    boss.sword.swingPhaseFloat = 0.0;
                    boss.sword.direction = boss.direction.clone();
                    boss.float = dx;
                    boss.float2 = dy;
                } else if boss.bool2{
                    let speed = 10.0;

                    let m = movementCalculator(boss.float, boss.float2, speed);
                    let mut vertical = true;
                    if asPositive(m.0) > asPositive(m.1) {
                        vertical = false;
                    }
                    if vertical == true {
                        if m.1 > 0.0 {
                            boss.direction = String::from("down");
                        }
                        else {
                            boss.direction = String::from("up");
                        }
                    }
                    else {
                        if m.0 > 0.0 {
                            boss.direction = String::from("right");
                        }
                        else {
                            boss.direction = String::from("left");
                        }
                    }

                    boss.updatePosition(m.0, m.1, backgroundTexture);

                    damageMaincharacter(mainCharacter, window, boss.hitbox, boss.damage * 2.0, alive);
                }
                else {
                    let speed = 25.0;

                    let m = movementCalculator(dx, dy, speed);
                    let mut vertical = true;
                    if asPositive(m.0) > asPositive(m.1) {
                        vertical = false;
                    }
                    if vertical == true {
                        if m.1 > 0.0 {
                            boss.direction = String::from("down");
                        }
                        else {
                            boss.direction = String::from("up");
                        }
                    }
                    else {
                        if m.0 > 0.0 {
                            boss.direction = String::from("right");
                        }
                        else {
                            boss.direction = String::from("left");
                        }
                    }

                    boss.updatePosition(-m.0, -m.1, backgroundTexture);
                    if (dx*dx + dy*dy).sqrt() >= 600.0 {
                        boss.bool2 = true;
                    }
                }
            }
            if !boss.bool {
                if (dx * dx + dy * dy).sqrt() < 500.0 && (dx * dx + dy * dy).sqrt() > 100.0 && boss.sword.swingPhase == 6 {
                    boss.sword.swingCooldown = Instant::now();
                    boss.bool = true;
                    boss.bool2 = false;
                } else {
                    number = 0;
                }
            }
            if boss.sword.swingCooldown.elapsed().as_millis() > 1500 && boss.bool {
                boss.bool2 = false;
                boss.bool = false;
                number = 0;
                boss.time = Instant::now();
            }

            number
        }
        3 => {
            let mut number = 3;

            if boss.bool {
                if (dx * dx + dy * dy).sqrt() < 200.0 && !boss.bool2{
                    boss.sword.swingPhase = 0;
                    boss.sword.swingPhaseFloat = 0.0;
                    boss.sword.direction = boss.direction.clone();
                    boss.bool2 = true;                }
                if boss.bool2 {
                    if boss.sword.swingPhase == 6 {
                        number = 0;
                        boss.bool = false;
                        boss.bool2 = false;
                        boss.time = Instant::now();
                    }
                    else {
                        damageMaincharacter(mainCharacter, window, boss.sword.hitBox, boss.damage * 1.25, alive);
                    }
                }
                else {
                    let speed = 2.5;

                    let m = movementCalculator(dx, dy, speed);
                    let mut vertical = true;
                    if asPositive(m.0) > asPositive(m.1) {
                        vertical = false;
                    }
                    if vertical == true {
                        if m.1 > 0.0 {
                            boss.direction = String::from("down");
                        }
                        else {
                            boss.direction = String::from("up");
                        }
                    }
                    else {
                        if m.0 > 0.0 {
                            boss.direction = String::from("right");
                        }
                        else {
                            boss.direction = String::from("left");
                        }
                    }

                    boss.updatePosition(m.0, m.1, backgroundTexture);
                }
            }
            if !boss.bool {
                if (dx * dx + dy * dy).sqrt() > 550.0 {
                    boss.bool = true;
                    boss.bool2 = false;
                } else {
                    number = 0;
                }
            }

            number
        }
        _ => 0
    };
    damageMaincharacter(mainCharacter, window, boss.hitbox, boss.damage, alive);
}