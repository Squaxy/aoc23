use std::fs::*;

struct Game {
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

fn main() {

    let input = read_to_string("./res/input2.txt").unwrap();
    let mut games = Vec::<Game>::new();
    
    for line in input.lines() {
        games.push(fit_line_into_struct(line));
    }

    println!("{}", sum_power_of_games(games));
}

fn fit_line_into_struct(line: &str) -> Game {

    let collon_split = line.split(": ").collect::<Vec<&str>>();
    let tail = collon_split.get(1);

    let pulls_string = tail.unwrap().split(";").collect::<Vec<&str>>();
    let mut pulls = Vec::<Pull>::new();

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
        pulls : pulls,
    };
}

fn sum_power_of_games(games: Vec<Game>) -> i32 {
    let mut game_power_sum = 0;
    
    for game in games {
        game_power_sum += get_power_of_game(&game);
    }

    return game_power_sum;
}

// assumption: there cant be 0 cubes for a color
fn get_power_of_game(game: &Game) -> i32 {
    let mut green_max = 1;
    let mut red_max = 1;
    let mut blue_max = 1;
    
    for pull in &game.pulls {
        green_max = if pull.green > green_max {pull.green} else {green_max};
        red_max = if pull.red > red_max {pull.red} else {red_max};
        blue_max = if pull.blue > blue_max {pull.blue} else {blue_max};
    }

    return green_max * red_max * blue_max;
}
