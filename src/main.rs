use std::net::{TcpStream, TcpListener};
use std::io::Error;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fs;
use std::env;

mod response;

fn main() {
    let args: Vec<_> = env::args().collect(); // arg1 = site path
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to connect to localhost"); let path = "./site-data/".to_string(); // global ip: 0.0.0.0:80, local ip: 127.0.0.1:8080
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_request(stream, &generate_link_map(), if args.len() > 1 { &args[1] } else { &path });
    }
}

fn handle_request(mut stream: TcpStream, links: &HashMap<String, String>, site_data_path: &String) {
    let buffer = BufReader::new(&mut stream);
    let lines: Vec<String> = buffer.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    println!("{:?}", lines);
    let status = "HTTP/1.1 200 OK";
    let link404 = ("404error.html".to_string(), "./site-data/404error.html".to_string());
    let mut link: (String, String) = match response::find_file(&lines, links) {
        Some(link) => link,
        None => link404.clone()
    };
    let mut count: u8 = 0;
    let split_path: Vec<_> = link.0.split(|c| c == '/' && { count += 1; count } < 3).collect();
    
    let mut template_response: Option<response::ResponseData> = None;
    
    if split_path[0] == "template" {
        match split_path[1] {
            "article" => { 
                template_response = match generate_template_response(&"./site-data/templates/article.html".to_string(), &split_path.iter().map(|line| line.to_string()).collect(), site_data_path) {
                    Ok(response) => Some(response),
                    Err(..) => {
                        link = link404.clone();
                        None
                    }
                };
            },
            "project" => {
                template_response = match generate_template_response(&"./site-data/templates/project.html".to_string(), &split_path.iter().map(|line| line.to_string()).collect(), site_data_path) {
                    Ok(response) => Some(response),
                    Err(..) => {
                        link = link404.clone();
                        None
                    }
                };
            },
            "synthblog" => {
                template_response = match generate_template_response(&"./site-data/templates/synthblog.html".to_string(), &split_path.iter().map(|line| line.to_string()).collect(), site_data_path) {
                    Ok(response) => Some(response),
                    Err(..) => {
                        link = link404.clone();
                        None
                    }
                };
            },
            _ => link = link404.clone() 
        }
    }


    println!("{} {}", link.0, link.1);
    
    let response_data: response::ResponseData = if let Some(response) = template_response {
        response
    } else {
        response::ResponseData::new(fs::read(link.1).expect("The requested file doesn't exists!"), &link.0)
    };
    let formatted_response: String = format!("{status}\r\n Content-Length: {0}\r\n\r\n", response_data.length);
    let mut final_bytes: Vec<u8>= formatted_response.as_bytes().to_vec();
    final_bytes.append(&mut response_data.content.clone());
    stream.write_all(&final_bytes).unwrap();
}

fn generate_link_map() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = Default::default();
    let file = File::open("linksheet.csv").expect("Couldn't find linksheet.csv!");
    let reader = BufReader::new(file);
    reader.lines().map(|result| result.unwrap()).for_each(|line| {
        let skip = || -> bool { 
            match line.chars().next() {
                Some(char) => {
                    if char == '#' {
                        return true;
                    }
                    return false;
                },
                None => false,
            }
        };
        if !skip() {
            let split: Vec<&str> = line.split(",").collect();
            map.insert(split[0].to_string(), split[1].to_string());
        }
    });
    map
}

fn generate_template_response(template_name: &String, split_path: &Vec<String>, site_data_path: &String) -> Result<response::ResponseData, Error> {
    // Fix 404 issues
    let mut file_name = site_data_path.clone().to_string();
    file_name.push_str(&split_path[2].clone());
    let file: File = File::open(&file_name)?;
    let token_set: response::TokenSet = response::TokenSet::new(&BufReader::new(file).lines().map(|line| line.unwrap()).collect());
    let template_file: Vec<String> = response::generate_template_page(&BufReader::new(File::open(template_name)?).lines().map(|line| line.unwrap()).collect(), token_set);
    let mut response_data = response::ResponseData::new(vec![], &"template.html".to_string());
    template_file.iter().for_each(|line| {
        response_data.content.append(&mut line.as_bytes().to_vec());
    });
    Ok(response_data)
}
