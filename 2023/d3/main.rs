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
            for gear in &reference_table {
                let rs = has_a_part_reference(gear, &positions);
        
                total += rs as u64;
            }

            println!("total: {}", total);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }

    println!("done");
}

fn has_a_part_reference(nb:&Pos, parts:&Vec<Pos>)->u64{
    let mut finded: u16 = 0;

    let mut one: &Pos = &Pos {value: 0, x:0, y:0, size:0};

    for _part in parts{
        let mut yes = false;
        // left
        if has_ref(nb.x +1, nb.y, &_part){
           yes = true; 
        }

        if has_ref(nb.x, nb.y+1, &_part){
            yes = true; 
         }

        if has_ref(nb.x +1, nb.y+1, &_part){
            yes = true; 
        }

        if nb.y != 0 && has_ref(nb.x +1, nb.y-1, &_part){
            yes = true; 
        }

        if nb.y != 0 && has_ref(nb.x, nb.y-1, &_part){
            yes = true; 
        }

        if nb.y != 0 && 0 != nb.x && has_ref(nb.x-1, nb.y-1, &_part){
            yes = true; 
        }

        if 0 != nb.x && has_ref(nb.x-1, nb.y, &_part){
            yes = true; 
        }

        if 0 != nb.x && has_ref(nb.x-1, nb.y+1, &_part){
            yes = true; 
        }

        if yes {
            if finded == 0{
                one = _part;
                finded +=1; 
            }else{
                return (one.value as u64 * _part.value as u64).into();
            }
        }
    }

    return 0;

}


fn has_ref(x:u32,y:u32, part: &Pos)->bool{
    if y ==  7|| y ==  9{
    }
    return part.y == y && part.x <= x && x <= part.x + (part.size-1) as u32;
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
            if c == '*' {
                reference_table.push(Pos { value: 0, x: char_num as u32, y: line_num as u32, size:0 });
            }
        }
    }

    reference_table
}