use std::fs::*;

struct Game {
    id: i32,
    pulls: Vec::<Pull>
}

struct Pull {
    green: i32,
    blue: i32,
    red: i32
}

const GREEN: &str = "green";
const RED: &str = "red";
const BLUE: &str = "blue";

const BLUE_CUBES_MAX: i32 = 14;
const GREEN_CUBES_MAX: i32 = 13;
const RED_CUBES_MAX: i32 = 12;

fn main() {

    let input = read_to_string("./res/input.txt").unwrap();
    let mut games = Vec::<Game>::new();
    
    for line in input.lines() {
        games.push(fit_line_into_struct(line));
    }

    println!("{}", sum_valid_game_ids(games));
}

fn fit_line_into_struct(line: &str) -> Game {

    let collon_split = line.split(": ").collect::<Vec<&str>>();
    let game_str = collon_split.get(0);
    let tail = collon_split.get(1);

    let pulls_string = tail.unwrap().split(";").collect::<Vec<&str>>();
    let mut pulls = Vec::<Pull>::new();

    let game_split = game_str.unwrap().trim().split(" ").collect::<Vec<&str>>();
    let game_id = game_split.get(1).unwrap().parse::<i32>().unwrap();

    for pull in pulls_string {
        let cubes = pull.trim().split(", ").collect::<Vec<&str>>();

        let mut pull_green = 0;
        let mut pull_red = 0;
        let mut pull_blue = 0;

        for cube in cubes {
            let cube_split = cube.trim().split(" ").collect::<Vec<&str>>();
            let cube_cnt = cube_split.get(0).unwrap();
            let color = cube_split.get(1);

            match *color.unwrap() {
                GREEN => pull_green = cube_cnt.parse::<i32>().unwrap(),
                RED => pull_red = cube_cnt.parse::<i32>().unwrap(),
                BLUE => pull_blue = cube_cnt.parse::<i32>().unwrap(),
                _ => (),
            };
        };

        pulls.push(Pull {
            green: pull_green,
            blue: pull_blue,
            red: pull_red,
        });
    }

    return Game {
        id : game_id,
        pulls : pulls,
    };
}

fn sum_valid_game_ids(games: Vec<Game>) -> i32 {
    let mut game_id_sum = 0;
    
    for game in games {
        if game_is_valid(&game) {
            game_id_sum += &game.id;
        }
    }

    return game_id_sum;
}

fn game_is_valid(game: &Game) -> bool {
    for pull in &game.pulls {
        if pull.green > GREEN_CUBES_MAX || pull.red > RED_CUBES_MAX || pull.blue > BLUE_CUBES_MAX {
            return false;
        }
    }
    return true;
}
