use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};

use crate::user::User;

fn parse_line(line: String) -> Result<User, String> {
    let split_result: Vec<&str> = line.split(";").collect();

    if split_result.len() != 2 {
        return Err("Record didn't contain 2 elements".to_string());
    }

    let usr_name = split_result[0].to_string();

    let usr_age = match split_result[1].parse::<u8>() {
        Ok(val) => val,
        Err(_) => return Err("Age not of type u8".to_string()),
    };

    Ok(User {
        name: usr_name,
        age: usr_age,
    })
}

pub fn read_users_from_file(path: String) -> Vec<User> {
    let file_path = Path::new(&path);

    let file = match File::open(file_path) {
        Err(_) => return vec![],
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();

    let mut users: Vec<User> = vec![];
    for line in lines {
        if let Ok(ip) = line {
            match parse_line(ip) {
                Ok(val) => users.push(val),
                Err(_) => continue,
            };
        } else {
            continue;
        }
    }

    users
}

pub fn write_users_to_file(user_list: &Vec<User>) -> Result<String, String> {
    let path = Path::new("users");

    let mut file = match File::create(&path) {
        Err(_) => return Err("Não foi possível criar o arquivo".to_string()),
        Ok(file) => file,
    };

    let user_list_data = user_list
        .iter()
        .map(|user| format!("{};{}", user.name, user.age))
        .collect::<Vec<String>>()
        .join("\n");

    match file.write_all(user_list_data.as_bytes()) {
        Err(_) => Err("Não foi possível criar o arquivo".to_string()),
        Ok(_) => Ok("Arquvo criado com sucesso".to_string()),
    }
}
