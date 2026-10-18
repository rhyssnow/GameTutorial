#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod PixelEngine;

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use sdl3::pixels::Color;
use sdl3::render::{create_renderer, WindowCanvas, FRect};
use sdl3::surface::Surface;
use sdl3::timer::delay;
use sdl3::video::Window;

use crate::PixelEngine::pixelEngine;
use crate::PixelEngine::parse::{createEnvironmentFromJson, loadSaveFromJson};

fn main() {
    let videoSubsystem = sdl3::init().unwrap().video().unwrap();
    let window = videoSubsystem.window("Game", 1920, 1080).position_centered().resizable().opengl().build().unwrap();
    let mut renderer = create_renderer(window.clone(), None).unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    gameLoop(&mut renderer, window);
}

fn gameLoop(renderer: &mut WindowCanvas, window: Window) {
    let mut textureVec = Vec::new();

    let surfaceMain = Surface::load_png("src/Assets/gameCharacter.png").unwrap();
    let mut textureMain = renderer.create_texture_from_surface(&surfaceMain).unwrap();
    textureMain.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    textureVec.push(textureMain);

    let surfaceCrest = Surface::load_png("src/Assets/crest.png").unwrap();
    let mut textureCrest = renderer.create_texture_from_surface(&surfaceCrest).unwrap();
    textureCrest.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    textureVec.push(textureCrest);

    let surfaceSword = Surface::load_png("src/Assets/mainSword.png").unwrap();
    let mut textureSword = renderer.create_texture_from_surface(&surfaceSword).unwrap();
    textureSword.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    textureVec.push(textureSword);

    let mut score = 0;
    let save = loadSaveFromJson(&window);
    let environment = createEnvironmentFromJson(PathBuf::from(Path::new(&("src/Metadata/".to_string() + &*save.1 + ".json"))));

    for location in environment.0 {
        let surface = Surface::load_png(location).unwrap();
        let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
        texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
        textureVec.push(texture);
    }

    let mut meshBox = environment.1;
    meshBox.x = save.2;
    meshBox.y = save.3;

    let mut pixelEngine = pixelEngine::new(meshBox, save.0, environment.2);

    let mut running = true;

    while running && pixelEngine.alive {
        let now = Instant::now();

        pixelEngine.input(&window, &textureVec, &mut running);
        pixelEngine.update(&mut textureVec, renderer, &mut score);
        pixelEngine.render(&textureVec, renderer, &window);

        let mut elapsed = now.elapsed().as_millis();
        if elapsed > 16 {
            elapsed = 16;
        }
        let extraTime = 16 - elapsed;
        delay(extraTime as u32);
    }
    delay(250);

    let path = Path::new("src/Assets/DovesTypeText-Regular.ttf"); //here
    let ttfContext = sdl3::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttfContext.load_font(path, 100.0).expect("SDL_TTF failed to load font.");

    let gameOverSurface = font.render("You Died").blended(Color::RGB(150, 50, 50)).unwrap();
    let gameOverTexture = renderer.create_texture_from_surface(&gameOverSurface).unwrap();

    let scoreSurface = font.render(&*("Score: ".to_string() + &*score.to_string())).blended(Color::RGB(200, 200, 200)).unwrap();
    let scoreTexture = renderer.create_texture_from_surface(&scoreSurface).unwrap();

    let mut file = File::open("src/Assets/highscore.txt").unwrap(); //here
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let highScoreSurface = font.render(&*("High Score: ".to_string() + &buffer)).blended(Color::RGB(200, 200, 200)).unwrap();
    let highScoreTexture = renderer.create_texture_from_surface(&highScoreSurface).unwrap();


    while running {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();


        let deathMsgRect = FRect::new(
            window.size_in_pixels().0 as f32 / 4.0,
            50.0,
            window.size_in_pixels().0 as f32 / 2.0,
            window.size_in_pixels().1 as f32 / 3.0,
        );
        let mut highScoreMsgRect = FRect::new(
            deathMsgRect.x + deathMsgRect.w / 3.0,
            deathMsgRect.y + deathMsgRect.h + 50.0,
            deathMsgRect.w / 3.0,
            deathMsgRect.h / 3.0,
        );

        renderer.copy(&gameOverTexture, None, deathMsgRect).unwrap();
        renderer.copy(&scoreTexture, None, highScoreMsgRect).unwrap();
        highScoreMsgRect.y += highScoreMsgRect.h + 50.0;
        highScoreMsgRect.h /= 1.5;
        renderer.copy(&highScoreTexture, None, highScoreMsgRect).unwrap();

        renderer.present();
        gameoverInput(&mut running);
    }
    if score > buffer.parse::<i32>().unwrap() {
        let mut newFile = File::create("src/Assets/highscore.txt").unwrap(); //here
        newFile.write_all(&score.to_string().as_bytes()).unwrap();
    }
}

fn gameoverInput (running: &mut bool) {
    let mut eventPump = sdl3::init().unwrap().event_pump().unwrap();

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
