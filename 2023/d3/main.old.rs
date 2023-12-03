use std::fs;
use std::fmt;

struct Pos {
    value: u16,
    x: u32,
    y: u32,
    size: u16,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pos {{ value: {}, x: {}, y: {}, size: {} }}", self.value, self.x, self.y, self.size)
    }
}

fn main(){


    let file_path = "input.txt";
    let mut total: u64 = 0;



    match fs::read_to_string(file_path) {
        Ok(contents) => {
            let positions = extract_positions(&contents);
            let reference_table = extract_reference_table(&contents);

            println!("Positions:");
            for pos in &positions {
                let rs = has_a_part_reference(pos, &reference_table);
                if rs {
                    println!("new  value {} toal: {}", pos.value, total);
                    total += pos.value as u64;
                }
                //println!("{} has ref: {}",pos, rs);
            }

        

            println!("toal: {}", total);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }

    println!("done");
}

fn has_a_part_reference(nb:&Pos, parts:&Vec<Pos>)->bool{
    let mut size = (nb.size +1) as u32;
    let mut rel_x = 0;
    if nb.x != 0 {
        size += 1;
        rel_x = 1;
    }

    for _part in parts{

        for _x in 0..size{
            
            //up
            if has_ref(nb.x+_x - rel_x, nb.y +1, &_part) {return true}

            //down
            if nb.y != 0 && has_ref(nb.x+_x - rel_x, nb.y -1, &_part) {return true}

        }

        //before 
        //same level
        if has_ref(nb.x- rel_x, nb.y, &_part) {return true}

        //after 
        //same level
        if has_ref(nb.x + (size-1)- rel_x, nb.y, &_part) {return true}

    }

    return false

}


fn has_ref(x:u32,y:u32, part: &Pos)->bool{
    return part.y == y && part.x == x;
}




fn extract_positions(contents: &str) -> Vec<Pos> {
    let mut positions = Vec::new();

    let mut number = "".to_string();

    for (line_num, line) in contents.lines().enumerate() {
        for (char_num, c) in line.chars().enumerate() {
            if c.is_numeric() {
                number = number + &c.to_string();
            }else if number != ""{
                let value = number.parse::<u16>().unwrap() ;

                positions.push(Pos { value, x: ((char_num as u32) - (number.len() as u32)), y: line_num as u32, size: number.len() as u16});
                number = "".to_string();
            }
        }
        // for number at end of line
        if number != ""{
            let value = number.parse::<u16>().unwrap() ;
            positions.push(Pos { value, x: ((line.len() as u32) - (number.len() as u32)), y: line_num as u32, size: number.len() as u16});
            number = "".to_string();
        }
    }

    positions
}

fn extract_reference_table(contents: &str) -> Vec<Pos> {
    let mut reference_table = Vec::new();

    for (line_num, line) in contents.lines().enumerate() {
        for (char_num, c) in line.chars().enumerate() {
            if c != '.' && !c.is_numeric() {
                reference_table.push(Pos { value: 0, x: char_num as u32, y: line_num as u32, size:0 });
            }
        }
    }

    reference_table
}