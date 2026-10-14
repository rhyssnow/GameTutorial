use sdl3::render::FRect;

pub struct trigger {
    triggerType: String,
    area: FRect,
}

impl trigger {
    pub fn new(triggerType: String, area: FRect) -> trigger {trigger{triggerType, area}}
}