use std::fs;
use std::collections::HashMap;



fn main(){
    let long_number: &HashMap<&str, &str>= &HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let file_path = "2023/d1/input.txt";
    let mut total = 0;

    match fs::read_to_string(file_path){
        Ok(contents) => {
            let parts = contents.split("\n");
            for part in parts {
                let finded_number = get_value_from_line(part,long_number) ; 
                total += finded_number.parse::<u16>().unwrap();
                println!("fineded: {}",finded_number);

            }
            println!("total: {}",total);
        }
        Err(err) =>{
            eprintln!("Error reading file: {}", err);
        }
    }

    println!("done");
}

fn get_value_from_line(line: &str, long_number: &HashMap<&str, &str>) -> String{
    let new_line: String = clean_old_value(line, long_number, 1);
    let new_line_rev: String = clean_old_value(line, long_number,0);
    return format!("{}{}",get_first_occurence(&new_line, 1) ,  get_first_occurence(&new_line_rev, 0));

}

fn get_first_occurence(line: &str, order: u16) -> char{
    
    if order == 1{
        for charac in line.chars() {
            if charac.is_numeric() {
                return charac;
            }
        }

    }else{
        for charac in line.chars().rev() {
            if charac.is_numeric() {
                return charac;
            }
        }
    }
    return '0';
}
fn clean_old_value(line: &str, long_number: &HashMap<&str, &str>, order: u16) -> String {
    let mut new_line = String::with_capacity(line.len());

    if order == 1 {
        let mut i = 0;
        while i < line.len() {
            let mut found = false;
            for (k, v) in long_number {
                if line[i..].starts_with(k) {
                    new_line.push_str(v);
                    i += k.len();
                    found = true;
                    break;
                }
            }
            if !found {
                new_line.push_str(&line[i..=i]);
                i += 1;
            }
        }
    } else {
        let mut i = line.len();
        while i > 0 {
            let mut found = false;
            for (k, v) in long_number {
                if line[..i].ends_with(k) {
                    new_line = v.to_string() + &new_line[(v.len() - 1)..];

                    i -= k.len();
                    found = true;
                    break;
                }
            }
            if !found {
                new_line = line[i - 1..=i - 1].to_string() + &new_line;
                i -= 1;
            }
        }
    }

    new_line
}