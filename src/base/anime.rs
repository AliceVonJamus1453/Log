use std::path::Path;
use sdl3::pixels::{PixelFormat, PixelFormatEnum};
use sdl3::render::{Texture, TextureAccess, TextureCreator};
use sdl3::video::WindowContext;
use crate::base::facing::Facing;
use crate::base::file_system;

pub struct Anime<'a> {
    anime: Vec<Texture<'a>>,
    normal_facing: Facing,
    width: u32,
    height: u32,
}

//构造函数的方法集合
impl<'a> Anime<'a> {
    pub fn from_name_list (
        sec: Vec<String>,
        creator: &'a TextureCreator<WindowContext>,
        access: TextureAccess,
        normal_facing: Facing
    ) -> Self {
        let temp = load_anime(
            sec,
            creator,
            access,
        );
        Anime {
            anime: temp.0,
            width: temp.1,
            height: temp.2,
            normal_facing
        }
    }

    pub fn from_detail (
        src: &Path,
        length: usize,
        fmt: &str,
        creator: &'a TextureCreator<WindowContext>,
        access: TextureAccess,
        normal_facing: Facing
    ) -> Self {
        let temp = load_anime(
            file_system::get_name_list(
                src,
                length,
                fmt
            ),
            creator,
            access
        );
        Anime {
            anime: temp.0,
            width: temp.1,
            height: temp.2,
            normal_facing
        }
    }
}

//获取数据的方法集合
impl<'a> Anime<'a> {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn picture(&self, index:usize) -> &Texture<'_> {
        &self.anime[index]
    }
    
    pub fn length(&self) -> usize {
        self.anime.capacity()
    }

    pub fn facing(&self) -> Facing {
        self.normal_facing.clone()
    }
}

//用于加载序列帧动画
fn load_anime(
    src: Vec<String>,
    creator: &TextureCreator<WindowContext>,
    access:TextureAccess
) -> (Vec<Texture<'_>>,u32,u32) {
    let mut re = Vec::with_capacity(src.len());
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
