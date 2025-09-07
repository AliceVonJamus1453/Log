use std::path::Path;
use sdl3::pixels::{PixelFormat, PixelFormatEnum};
use sdl3::render::{TextureAccess, TextureCreator, Texture};
use sdl3::video::WindowContext;

pub struct Anime<'a> {
    anime: Vec<Texture<'a>>,
    width: u32,
    height: u32,
}

impl<'a> Anime<'a> {
    pub fn from(
        src:&Path,
        length:usize,
        fmt: &str,
        creator: &'a TextureCreator<WindowContext>,
        access:TextureAccess
    ) -> Self {
        let temp = Self::load_anime(
            Self::get_name_list(
                src,
                length,
                fmt
            ),
            creator,
            access
        );
        Anime {
            anime:temp.0,
            width:temp.1,
            height:temp.2,
        }
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }

    fn load_anime(
        src: Vec<String>,
        creator: &TextureCreator<WindowContext>,
        access:TextureAccess
    ) -> (Vec<Texture<'_>>,u32,u32) {
        let mut re = Vec::with_capacity(src.capacity());
        let mut img = image::open(&src[0]).unwrap().to_rgba8();
        let (x,y) = img.dimensions();
        for src in src.iter() {
            re.push({
                img = image::open(src).unwrap().to_rgba8();
                let mut re = creator.create_texture(
                    PixelFormat::from(PixelFormatEnum::ABGR8888),
                    access,
                    x,y
                ).unwrap();
                re.update(None, &img, x as usize * 4).unwrap();
                re
            })
        }
        (re,x,y)
    }

    fn get_name_list(src:&Path, length:usize, fmt: &str) -> Vec<String> {
        let mut re = Vec::with_capacity(length);
        for i in 0..length {
            re.push(format!("{}{:0>3}.{fmt}", src.to_str().unwrap() ,i))
        }
        re
    }
}
