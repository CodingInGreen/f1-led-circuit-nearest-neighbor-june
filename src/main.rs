use csv::ReaderBuilder;
use csv::WriterBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct RaceData {
    x: f64,
    y: f64,
    date: String,
    driver_number: i32,
}

#[derive(Debug, Deserialize)]
struct LedData {
    x_led: f64,
    y_led: f64,
    led_num: i32,
}

fn read_csv<T: for<'de> serde::Deserialize<'de>>(path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }
    Ok(records)
}

fn write_csv(path: &str, records: &[(f64, f64, String, i32, i32)]) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut wtr = WriterBuilder::new().from_writer(file);
    wtr.write_record(&["x", "y", "date", "driver_number", "led_num"])?;
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn nearest_neighbor(
    race_data: &[RaceData],
    led_data: &[LedData],
) -> Vec<(f64, f64, String, i32, i32)> {
    let mut results = Vec::new();
    for race in race_data {
        let mut min_distance = f64::MAX;
        let mut closest_led = None;
        for led in led_data {
            let distance = ((race.x - led.x_led).powi(2) + (race.y - led.y_led).powi(2)).sqrt();
            if distance < min_distance {
                min_distance = distance;
                closest_led = Some(led.led_num);
            }
        }
        if let Some(led_num) = closest_led {
            results.push((race.x, race.y, race.date.clone(), race.driver_number, led_num));
        }
    }
    results
}

fn main() -> Result<(), Box<dyn Error>> {
    let race_data_path = "sorted_api_race_data_all.csv";
    let led_data_path = "board_led_coords_with_numbers.csv";
    let output_path = "processed_race_data_all.csv";

    let race_data: Vec<RaceData> = read_csv(race_data_path)?;
    let led_data: Vec<LedData> = read_csv(led_data_path)?;

    let mapped_data = nearest_neighbor(&race_data, &led_data);
    
    write_csv(output_path, &mapped_data)?;

    Ok(())
}
