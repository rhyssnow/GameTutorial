#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod PixelEngine;

use std::path::{Path, PathBuf};
use std::time::Instant;
use sdl3::pixels::Color;
use sdl3::render::{create_renderer, WindowCanvas};
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

    while running == true {
        let now = Instant::now();

        pixelEngine.input(&window, &textureVec, &mut running);
        pixelEngine.update(&textureVec);
        pixelEngine.render(&textureVec, renderer, &window);

        let mut elapsed = now.elapsed().as_millis();
        if elapsed > 16 {
            elapsed = 0;
        }
        let extraTime = 16 - elapsed;
        delay(extraTime as u32);
    }
}
