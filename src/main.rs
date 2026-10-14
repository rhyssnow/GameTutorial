#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod PixelEngine;

use sdl3::pixels::Color;
use sdl3::render::{create_renderer, WindowCanvas};
use sdl3::surface::Surface;
use sdl3::video::Window;
use crate::PixelEngine::parse::*;

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

    let surfaceMain = Surface::load_png("src/Assets/gameCharacter.png").unwrap(); //here
    let mut textureMain = renderer.create_texture_from_surface(&surfaceMain).unwrap();
    textureMain.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    textureVec.push(textureMain);

    let running = true;

    while running == true {

    }
}
