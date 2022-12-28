extern crate itertools;
use datascience::itertools::Itertools;
use crate::{penguin::Penguin, cvsreader::CVSReader, penguinData::PenguinData, geo::Geo};


pub struct DataScience {}

impl DataScience {
    pub fn get_data_by_track_id(vec: Vec<PenguinData>)-> Vec<Penguin>{
        let penguins: Vec<Penguin> = vec
            .iter()
            .group_by(|p| p.get_track_id())
            .into_iter()
            .map(|(track_id, group)| {
                let mut locations: Vec<Geo> = Vec::new();
                for p in group {
                    locations.push(p.geom);
                }
                Penguin::new(locations, track_id.to_string())
            })
            .collect();

        penguins
    }

    pub fn output_penguin_stream() {
        let penguin_data = CVSReader::read_csv().unwrap();
        let mut penguins = DataScience::get_data_by_track_id(penguin_data);
        println!("{:?}", penguins.len());
        for penguin in &mut penguins {
            penguin.to_string_sorted();
        }
    }


    pub fn output_location_range_per_track_id(penguins: Vec<PenguinData>){
        if penguins.is_empty() {return}
        let mut penguins = DataScience::get_data_by_track_id(penguins);
        for penguin in &mut penguins {
            let max_longitude = penguin.get_locations().iter().map(|g| g.get_longitute()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let min_longitude = penguin.get_locations().iter().map(|g| g.get_longitute()).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let max_latitude = penguin.get_locations().iter().map(|g| g.get_latitude()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let min_latitude = penguin.get_locations().iter().map(|g| g.get_latitude()).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let avg_longitude = penguin.get_locations().iter().map(|g| g.get_longitute()).sum::<f64>() / penguin.get_locations().len() as f64;
            let avg_latitude = penguin.get_locations().iter().map(|g| g.get_latitude()).sum::<f64>() / penguin.get_locations().len() as f64;
            println!("Track ID: {}, Max Longitude: {}, Min Longitude: {}, Max Latitude: {}, Min Latitude: {}, Avg Longitude: {}, Avg Latitude: {}", penguin.get_track_id(),  max_longitude, min_longitude, max_latitude, min_latitude, avg_longitude, avg_latitude);
        }
    }
}