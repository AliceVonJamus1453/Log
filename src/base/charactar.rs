use sdl3::rect::Rect;
use crate::base::anime::Anime;

pub struct Charactar<'a> {
    entity: Rect,
    anime: Anime<'a>,
}

impl<'a> Charactar<'a> {
    pub fn new(entity:Rect, anime: Anime<'a>) -> Self {
        Charactar {
            entity,
            anime
        }
    }
}