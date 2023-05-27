use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug)]
struct Value {
    distance: f32,
    duration: f32,
}

fn random_lat_long_string_generation(lat: f64, long: f64, combinations: usize) -> String {
    let lat_upto_two = format!("{:.1}", lat);
    let long_upto_two = format!("{:.1}", long);
    let mut url = String::from("router.project-osrm.org/table/v1/driving/");
    let mut last_three = String::from("000");
    for i in 0..=combinations {
        let mut pair = String::new();
        let random_lat = format!("{}{}", lat_upto_two, last_three);
        let random_long = format!("{}{}", long_upto_two, last_three);
        if i > 0 {
            pair.push(';');
        }
        pair.push_str(&random_long);
        pair.push(',');
        pair.push_str(&random_lat);
        println!("{}", pair);
        url.push_str(&pair);
        if i >= 0 && i <= 9 {
            last_three = increment_last_three(&last_three, 2, i + 1);
        }
        if i >= 10 && i <= 99 {
            last_three = increment_last_three(&last_three, 1, i + 1);
        }
        if i >= 100 && i <= 999 {
            last_three = increment_last_three(&last_three, 0, i + 1);
        }
    }
    url.push_str("?annotations=distance");
    url
}

fn generate_coordinates(lat: f64, long: f64, combinations: usize) -> Vec<[f64; 2]> {
    let mut res = Vec::new();
    let lat_upto_two = format!("{:.1}", lat);
    let long_upto_two = format!("{:.1}", long);
    let mut last_three = String::from("000");
    for i in 0..=combinations {
        let mut pair = [0.0; 2];
        let random_lat = format!("{}{}", lat_upto_two, last_three);
        let random_long = format!("{}{}", long_upto_two, last_three);
        pair[0] = random_lat.parse().unwrap();
        pair[1] = random_long.parse().unwrap();
        res.push(pair);
        if i >= 0 && i <= 9 {
            last_three = increment_last_three(&last_three, 2, i + 1);
        }
        if i >= 10 && i <= 99 {
            last_three = increment_last_three(&last_three, 1, i + 1);
        }
        if i >= 100 && i <= 999 {
            last_three = increment_last_three(&last_three, 0, i + 1);
        }
    }
    res
}

fn increment_last_three(val: &str, index: usize, replacement: usize) -> String {
    if replacement >= 0 && replacement <= 9 {
        let mut new_val = val.to_string();
        new_val.replace_range(index..index + 1, &replacement.to_string());
        new_val
    } else if replacement >= 10 && replacement <= 99 {
        val[..index].to_string() + &replacement.to_string()
    } else if replacement >= 100 && replacement <= 999 {
        replacement.to_string()
    } else {
        String::new()
    }
}

fn generate_url_string(coordinates: Vec<[f64; 2]>) -> String {
    let mut url = String::new();
    url.push_str("http://router.project-osrm.org/table/v1/driving/");
    for (i, coordinate) in coordinates.iter().enumerate() {
        let latitude = coordinate[0];
        let longitude = coordinate[1];
        if i != 0 {
            url.push(';');
        }
        write!(
            &mut url,
            "{},{}",
            longitude,
            latitude
        )
        .unwrap();
    }
    url.push_str("?annotations=distance,duration");
    url
}

fn create_hashmap(locations_list: Vec<[f64; 2]>) {
    let locations_map: HashMap<String, Value> = HashMap::new();
    let locations_list_size = locations_list.len();
    for first in (0..locations_list_size).step_by(50) {
        let end_of_first = first + 50;
        let end_of_first = if end_of_first >= locations_list_size {
            locations_list_size
        } else {
            end_of_first
        };
        for second in (0..locations_list_size).step_by(50) {
            let end_of_second = second + 50;
            let end_of_second = if end_of_second >= locations_list_size {
                locations_list_size
            } else {
                end_of_second
            };
            let first_list = &locations_list[first..end_of_first];
            let second_list = &locations_list[second..end_of_second];
            // Perform operations with first_list and second_list
            // firstChunk and secondChunk are combined in to one list of coordinates
            let mut combined_chunks = Vec::new();
            combined_chunks.extend_from_slice(&first_list);
            combined_chunks.extend_from_slice(&second_list);
            let url = generate_url_string(combined_chunks);
            // Todo:
            // 1. API call
            // 2. Process the response body
            // 3. Push key value pair to locations_map
        }
    }
}

fn main() {
    let locations_list = generate_coordinates(80.0423, 13.3356, 20);
    create_hashmap(locations_list)
}

