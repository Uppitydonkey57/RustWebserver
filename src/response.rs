use std::path::Path;
use std::collections::HashMap;

const WEB_PATH: &str = "./site-data/";

pub fn find_file(request: &Vec<String>, link_map: &HashMap<String, String>) -> Option<(String, String)> {
    if request.len() <= 0 { return None; }
    let mut file_name = request[0].split(" ").collect::<Vec<&str>>()[1];
    match link_map.get(file_name) {
        Some(link) => file_name = link,
        None => {},
    }
    let mut path_string: String = String::from(WEB_PATH);
    path_string.push_str(&file_name);
    if file_name.split("/").collect::<Vec<&str>>()[0] == "template" {
        return Some(((&file_name).to_string(), path_string));
    }
    let path: &Path = &Path::new(&path_string);
    match path.try_exists() {
        Ok(exists) => {
            if exists {
                return Some(((&file_name).to_string(), path_string));
            }
            return None;
        }
        Err(..) => {
            println!("the path {} is inaccessible", path_string);
            return None;
        }
    }
}

pub struct TokenSet {
    pub tokens: HashMap<String, Vec<String>>,
}

impl TokenSet {
    pub fn new(post: &Vec<String>) -> TokenSet {
        let mut post_tokens: HashMap<String, Vec<String>> = HashMap::new();
        let mut current_token: Option<String> = None;

        post.into_iter().for_each(|line| {
            if line.chars().collect::<Vec<_>>().len() > 0 && line.chars().nth(0).unwrap() == '#' {
                let mut line_buffer = line.clone();
                line_buffer.remove(0);
                let command: Vec<&str> = line_buffer.split(" ").collect::<Vec<&str>>();
                match command[0] {
                    "segment" => {
                        current_token = Some(command[1].to_string());
                        post_tokens.insert(command[1].to_string(), vec![]);
                    },
                    "end" => {current_token = None},
                    _ => {
                        let error_message: String = format!("the command {} doesn't exist", command[0]);
                        println!("{error_message}");
                    }
                }
            }
            else {
                match &current_token {
                    Some(token) => {
                        //maybe dont use a box
                        post_tokens.get_mut(token).expect("token names not matching!").push(line.clone());
                    }
                    None => {}
                }
            }
        });
        TokenSet {
            tokens: post_tokens,
        }
    }
    //pub fn to_json(&self) {}
}

pub struct ProcessedTokenFile {
    token_file: String,
    token: TokenSet,
    template_file: String,
    template: Vec<String>,
}

pub enum Request {
    Get,
    Post,
    Ok(u16)
}

pub fn generate_template_page(page_data: &Vec<String>, tokens: TokenSet) -> Vec<String> {
    page_data.clone().iter().map(|line| 
    {
        let mut final_line: String = String::from("");
        let mut current_token: Option<String> = None;
        for char in line.chars().collect::<Vec<_>>() {
            if char == '{' {
                current_token = Some(String::from(""));
            }
            else if char == '}' {
                if let Some(token) = &current_token {
                    match tokens.tokens.get(token) {
                        Some(found_token) => found_token.iter().for_each(|line| final_line.push_str(line)),
                        None => final_line.push_str("<h1>required token doesn't exist</h1>")
                    }
                    current_token = None;
                }
                
            } else {
                match &mut current_token {
                    Some(token) => {
                        token.push(char);
                    }
                    None => {}
                }
                if current_token == None {
                    final_line.push(char);
                }
            }
        }
        final_line
    }).collect()
}

#[derive(Debug)]
pub struct ResponseData {
    pub content_type: String,
    pub content: Vec<u8>,
    pub length: usize,
}

impl ResponseData {
    pub fn new(content: Vec<u8>, file_name: &String) -> ResponseData {
        let content_length = content.len();
        let extension: &str = file_name.split(".").collect::<Vec<&str>>()[1];
        let content_type = match extension {
            "html"  => "text/html",
            "css"  => "text/css",
            "js"  => "text/js",
            "ico"  => "image/x-icon",
            "jpg"  => "image/jpeg",
            "jpeg"  => "image/jpeg",
            "png"  => "image/png",
            _       => { panic!("the file extension {}, isn't registered.", extension); }
        }.to_string();
        ResponseData {
            content_type,
            content,
            length: content_length,
        }
    }
}

