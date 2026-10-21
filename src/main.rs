#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod PixelEngine;

use std::path::{Path, PathBuf};
use std::time::Instant;
use sdl3::pixels::Color;
use sdl3::render::{create_renderer, WindowCanvas, FRect, Texture};
use sdl3::surface::Surface;
use sdl3::timer::delay;
use sdl3::video::Window;
use crate::PixelEngine::basic::mainCharacter;
use crate::PixelEngine::pixelEngine;
use crate::PixelEngine::parse::{createEnvironmentFromJson, loadSaveFromJson};

fn main() {
    let videoSubsystem = sdl3::init().unwrap().video().unwrap();
    let mut window = videoSubsystem.window("Game", 1920, 1080).position_centered().resizable().opengl().build().unwrap();
    let mut renderer = create_renderer(window.clone(), None).unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

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

    let save = loadSaveFromJson(&window);
    gameLoop(&mut renderer, &mut window, &mut textureVec, save.0, save.1, save.2, save.3);
}

fn gameLoop(renderer: &mut WindowCanvas, window: &mut Window, mut textureVec: &mut Vec<Texture>, mainCharacter: mainCharacter, environmentFile: String, startX: f32, startY: f32) {
    let environment = createEnvironmentFromJson(PathBuf::from(Path::new(&("src/Metadata/".to_string() + &*environmentFile+ ".json"))));

    for location in environment.0 {
        let surface = Surface::load_png(location).unwrap();
        let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
        texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
        textureVec.push(texture);
    }

    let mut meshBox = environment.1;
    meshBox.x = startX;
    meshBox.y = startY;

    let mut pixelEngine = pixelEngine::new(meshBox, mainCharacter, environment.2);

    let mut running = true;

    while running && pixelEngine.alive {
        let now = Instant::now();

        pixelEngine.input(window, &textureVec, &mut running);
        pixelEngine.update(&mut textureVec, renderer, window);
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

    while running {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();


        let deathMsgRect = FRect::new(
            window.size_in_pixels().0 as f32 / 4.0,
            50.0,
            window.size_in_pixels().0 as f32 / 2.0,
            window.size_in_pixels().1 as f32 / 3.0,
        );

        renderer.copy(&gameOverTexture, None, deathMsgRect).unwrap();

        renderer.present();
        gameOverInput(&mut running);
    }
}

fn gameOverInput (running: &mut bool) {
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
