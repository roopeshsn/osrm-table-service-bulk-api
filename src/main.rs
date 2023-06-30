use serde::{Deserialize, Serialize};
use std::fmt::Write;

#[derive(Debug)]
struct Data {
    distances: Vec<Vec<f64>>,
    durations: Vec<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TableResponse {
    code: String,
    distances: Vec<Vec<f64>>,
    destinations: Vec<Destination>,
    durations: Vec<Vec<f64>>,
    sources: Vec<Source>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Destination {
    hint: String,
    distance: f64,
    name: String,
    location: Vec<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Source {
    hint: String,
    distance: f64,
    name: String,
    location: Vec<f64>,
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
        write!(&mut url, "{},{}", longitude, latitude).unwrap();
    }
    url.push_str("?annotations=distance,duration");
    url
}

fn create_hashmap(locations_list: Vec<[f64; 2]>) {
    // let locations_map: HashMap<String, Value> = HashMap::new();
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
            println!("{}", url);

            // Todo:
            // 1. API call
            // 2. Process the response body
            // 3. Push key value pair to locations_map

            let distances = get_data(url);
            println!("{:#?}", distances)
        }
    }
}

fn get_data(url: String) -> Result<Data, reqwest::Error> {
    let resp: TableResponse = reqwest::blocking::get(url)?.json()?;
    let data = Data {
        distances: resp.distances,
        durations: resp.durations,
    };
    Ok(data)
}

fn generate_array_of_coordinates(latitude: f64, longitude: f64, times: i32) -> Vec<[f64; 2]> {
    let mut array: Vec<[f64; 2]> = vec![];
    for i in 0..times {
        let lat = latitude + (i as f64) * 0.001;
        let lon = longitude + (i as f64) * 0.001;
        array.push([lon, lat]);
    }

    array
}

fn main() {
    let locations_list = generate_array_of_coordinates(80.0, 13.0, 5);
    let _ = create_hashmap(locations_list);
}
