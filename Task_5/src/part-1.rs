use std::fs;
use std::collections::HashMap;

const INPUT_PATH: &str = "./res/input.txt";

#[derive(Copy, Clone, Debug)]
struct GardenersEntry {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl GardenersEntry {
    fn get_dest_val(&self, source_val: u64) -> Option<u64> {
        if (self.source_range_start..(self.source_range_start + self.range_length)).contains(&source_val) {
            let offset: u64 = source_val - self.source_range_start;
            return Option::Some(self.destination_range_start + offset);
        }
        return Option::None;
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).expect("Failed to read file!");
    let segments: Vec<&str> = input.split("\n\n").collect();


    let mut seeds = Vec::<u64>::new();
    let mut gardeners_maps = Vec::<Vec::<GardenersEntry>>::new();

    segments.get(0).unwrap().split(" ")
    .skip(1)
    .for_each(|e| {
        seeds.push(
            e.parse::<u64>()
            .expect("couldn't convert seed str to u64!")
        )
    });

    segments.iter().skip(1).for_each(|seg| {
        let mut gardeners_map = Vec::<GardenersEntry>::new();
        seg.split("\n").skip(1).for_each(|chunk| {
            let values = chunk.split(" ")
            .map(|n| {n.parse::<u64>().unwrap()})
            .collect::<Vec<u64>>();

            gardeners_map.push(GardenersEntry {
                destination_range_start: *values.get(0).expect("could unwrap val 1!"),
                source_range_start: *values.get(1).expect("could unwrap val 2!"),
                range_length: *values.get(2).expect("could unwrap val 3!"),
            });
        });
        gardeners_maps.push(gardeners_map);
    });

    let seeds_to_location_map: HashMap<u64, u64> = seeds.iter()
    .map(|&seed| (seed, propagate_seed(seed, &gardeners_maps)))
    .collect();

    let smallest_location = seeds_to_location_map.values().min().unwrap();
    // println!("{:?}", seeds_to_location_map);
    println!("{}", smallest_location);
}

fn propagate_seed(seed: u64, gardeners_maps: &Vec::<Vec::<GardenersEntry>>) -> u64 {
    return propagate_seed_recurse(seed, gardeners_maps, 0);
}

fn propagate_seed_recurse(seed: u64, gardeners_maps: &Vec::<Vec::<GardenersEntry>>, counter: usize) -> u64{
    if counter == gardeners_maps.len() {
        return seed;
    }
    let new_counter: usize = counter + 1;
    let gardeners_map = gardeners_maps.get(counter).unwrap();
    for i in 0..gardeners_map.len() {
        let next = gardeners_map.get(i).unwrap().get_dest_val(seed);
        if next.is_some() {
            return propagate_seed_recurse(next.unwrap(), gardeners_maps, new_counter);
        }
    }
    return propagate_seed_recurse(seed, gardeners_maps, new_counter);
}
