use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

//用于将序列帧动画的各图片的路径生成字符串数组
pub fn get_name_list(src:&Path, length:usize, fmt: &str) -> Vec<String> {
    let mut re = Vec::with_capacity(length);
    for i in 0..length{
        re.push(format!("{}{:0>3}.{fmt}", src.to_str().unwrap() ,i))
    }
    re
}

pub fn ls_dir(src: &Path, doing:&mut dyn FnMut(&Path)) {
    if src.is_dir() {
        for entry in fs::read_dir(src).unwrap() {
            let path_buf = entry.unwrap().path();
            let path = path_buf.as_path();
            if path.is_dir() {
                ls_dir(path, doing);
            }
            else {
                doing(path);
            }
        }
    }
}

pub fn remove_last(mut src: String) -> String {
    match src.find("0") {
        Some(index) => {
            src.truncate(index)
        }
        None => {
            src.truncate(src.len() - 4)
        }
    }
    src
}
