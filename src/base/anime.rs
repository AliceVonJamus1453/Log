use std::path::Path;
use sdl3::pixels::{PixelFormat, PixelFormatEnum};
use sdl3::render::{TextureAccess, TextureCreator, Texture};
use sdl3::video::WindowContext;

pub struct Anime<'a> {
    anime: Vec<Texture<'a>>,
    width: u32,
    height: u32,
}

//构造函数的方法集合
impl<'a> Anime<'a> {
    pub fn from(
        src: &Path,
        length: usize,
        fmt: &str,
        creator: &'a TextureCreator<WindowContext>,
        access: TextureAccess
    ) -> Self {
        let temp = load_anime(
            get_name_list(
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
}

//用于加载序列帧动画
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

//用于将序列帧动画的各图片的路径生成字符串数组
fn get_name_list(src:&Path, length:usize, fmt: &str) -> Vec<String> {
    let mut re = Vec::with_capacity(length);
    for i in 0..length {
        re.push(format!("{}{:0>3}.{fmt}", src.to_str().unwrap() ,i))
    }
    re
}
