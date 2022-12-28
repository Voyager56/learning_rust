use PenguinData;
use geo::Geo;
use std::{error::Error};

#[derive(Debug)]
pub struct CVSReader{}

impl CVSReader {
    pub fn read_csv() -> Result<Vec<PenguinData>, Box<dyn Error>> {
        let mut rdr = 
                csv::ReaderBuilder::new()
                .from_path("data.csv")
                .expect("Could not open file");
        let instance = rdr.records().map(|line| {
                let line = line.unwrap();
                CVSReader::map_penguin_data(line)
            }).collect();
        Ok(instance)
    }

    fn map_penguin_data(line: csv::StringRecord) -> PenguinData {        
        let fid = line.get(0).unwrap().to_string();
        let id = line.get(1).unwrap().parse::<u32>().unwrap();
        let datetime = line.get(2).unwrap().to_string();
        let latitude = line.get(3).unwrap().parse::<f64>().unwrap();
        let longitude = line.get(4).unwrap().parse::<f64>().unwrap();
        let site = line.get(5).unwrap().to_string();
        let penguin_track = line.get(6).unwrap().to_string();
        let track_id = line.get(7).unwrap().to_string();
        let geom = Geo::new_from_string(CVSReader::string_between(line.get(8).unwrap(), "(", ")").to_string());
        PenguinData::new(fid, id, datetime, latitude, longitude, site, penguin_track, track_id, geom)
    }

    fn string_between<'a>(s: &'a str, start: &str, end: &str) -> &'a str {
        let start = s.find(start).unwrap() + start.len();
        let end = s.find(end).unwrap();
        &s[start..end]
    }
}


