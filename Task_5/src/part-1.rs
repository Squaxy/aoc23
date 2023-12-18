use std::fs;

const INPUT_PATH: &str = "./res/input.txt";

#[derive(Copy, Clone)]
struct GardenersEntry {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Copy, Clone)]
enum GardenersMapNames {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
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

    assert_eq!(gardeners_maps.len(), 7);
}

