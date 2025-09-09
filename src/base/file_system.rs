use std::collections::HashMap;
use std::fs;
use std::path::Path;

//递归遍历目录并生成字符串数组的映射表
pub fn ls_dir_to_name_map(src:&Path, target:&HashMap<String,(usize,String)>) {
    for entry in fs::read_dir(src).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            ls_dir_to_name_map(&path,target);
        }
        else {
        }
    }
}

//用于将序列帧动画的各图片的路径生成字符串数组
pub fn get_name_list(src:&Path, length:usize, fmt: &str) -> Vec<String> {
    let mut re = Vec::with_capacity(length);
    for i in 0..length{
        re.push(format!("{}{:0>3}.{fmt}", src.to_str().unwrap() ,i))
    }
    re
}