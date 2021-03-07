use std::io;
use rand::{Rng};

fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let splited_person = guess.split(' ').collect::<Vec<&str>>();
    let random_num = rand::thread_rng().gen_range(0..(splited_person.len()));
    // nambuだったらprintlnする
    let mut index = 0;
    let is_nambu = loop {
        if splited_person[index] == "nambu" {
            break true
        } else {
            index += 1;
            if index > splited_person.len() - 1 {
                break false
            }
        }
    };
    if is_nambu == true {
        println!("nambu")
    } else {
        println!("{}", splited_person[random_num])
    }
    
    // println!("{:?}", splited_person)
}
