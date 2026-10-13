use sdl3::render::FRect;

pub struct trigger {
    area: FRect,
}

impl trigger {
    pub fn new(area: FRect) -> trigger {trigger{area}}
}