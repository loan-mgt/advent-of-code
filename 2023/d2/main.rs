use std::fs;

struct Result {
    id: u32,
    blue: u16,
    red: u16,
    green: u16
}

struct Pick{
    nb: u16,
    color: String,
}

fn main(){


    let file_path = "2023/d2/input.txt";
    let mut total = 0;

    let max_allowed: Result = Result {
        id: 0,
        blue: 14,
        red: 12,
        green: 13
    };

    match fs::read_to_string(file_path){
        Ok(contents) => {
            let parts = contents.split("\n");
            for part in parts {
                let game_result = get_value_from_line(part) ; 
                // part 1
                // if is_possible(&max_allowed, &game_result){
                //     total += game_result.id;
                // }
                total += game_result.id;
                println!("fineded: id: {} red: {} blue: {} green: {}",
                game_result.id, game_result.red, game_result.blue, game_result.green);

            }
            println!("total: {}",total);
        }
        Err(err) =>{
            eprintln!("Error reading file: {}", err);
        }
    }

    println!("done");
}

fn is_possible(max: &Result, game_result: &Result) -> bool{
    if max.blue < game_result.blue{
        return false;
    }
    if max.red < game_result.red{
        return false;
    }
    if max.green < game_result.green{
        return false;
    }
    return true;
}



fn get_value_from_line(line: &str) -> Result{
    let game_id:u32 = get_game_id(line);

    let mut max_score = Result{id:game_id, red:0, green:0, blue: 0};

    let right_part = line.split(':').collect::<Vec<_>>()[1];
    for turn in right_part.split(';'){
        let res  = get_game_result(turn, game_id);

        if max_score.red < res.red{
            max_score.red = res.red;
        }

        if max_score.blue < res.blue{
            max_score.blue = res.blue;
        }

        if max_score.green < res.green{
            max_score.green = res.green;
        }
    }

    max_score.id = u32::from(max_score.red) *  u32::from(max_score.blue) * u32::from(max_score.green);


    return max_score;

}

fn get_game_id(line: &str) -> u32 {
    let left_part: Vec<&str> = line.split(':').collect();
    if let Some(substring) = left_part.get(0) {
        if substring.len() >= 5 {
            let substring_str = &substring[5..];
            return substring_str.parse::<u32>().unwrap_or_else(|e| {
                eprintln!("Error parsing {}: {}", substring_str, e);
                0
            });
        }
    }
    0
}


fn get_game_result(line: &str, id: u32) -> Result{

    

    let picks = line.split(',');

    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    for _pick in picks{
        let pick_res: Pick = get_pick_res(_pick);

        if pick_res.color == "red"{
            red += pick_res.nb;
        }else if pick_res.color == "blue"{
            blue += pick_res.nb;
        }else if pick_res.color == "green"{
            green += pick_res.nb;
        }
    }

    return Result { id: id, red: red, blue: blue, green: green};
}
    
fn get_pick_res(pick: &str) -> Pick{
    
    let mut color = "";

    let or_size = pick.len();

    let mut local_pick = pick.to_string();

    local_pick = local_pick.replace("red","");

    if or_size != local_pick.len() {
        color = "red";
    }

    local_pick = local_pick.replace("blue","");

    if or_size != local_pick.len() && color == "" {
        color = "blue";
    }

    local_pick = local_pick.replace("green","");

    if or_size != local_pick.len() && color == "" {
        color = "green";
    }

    local_pick = local_pick.trim().to_string();

    return Pick { nb: local_pick.parse::<u16>().unwrap(), color:color.to_string()}


}