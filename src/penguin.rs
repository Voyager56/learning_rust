use crate::geo::Geo;


#[derive(Debug)]
pub struct Penguin {
    locations: Vec<Geo>,
    track_id: String,
}

impl Penguin {
    pub fn new(locations: Vec<Geo>, track_id: String) -> Self {
        Penguin {
            locations,
            track_id,
        }
    }

    pub fn get_track_id(&self) -> &String {
        &self.track_id
    }

    pub fn to_string(&self) -> String {
        format!("Penguin{{\"locations\": {}, \"track_id\": {}}}",
                self.locations.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","),
                self.track_id)
    }

    pub fn to_string_sorted(&mut self) {
         self.locations.sort_by(
            |a, b| {
                if a.get_latitude() > b.get_latitude() { std::cmp::Ordering::Less }
                else if a.get_latitude() < b.get_latitude() { std::cmp::Ordering::Greater }
                else { 
                    if a.get_longitute() > b.get_longitute() { std::cmp::Ordering::Less }
                    else if a.get_longitute() < b.get_longitute() {  std::cmp::Ordering::Greater }
                    else { std::cmp::Ordering::Equal }
                }
            });

        println!("{:?}, trackID='{}' ", self.locations, self.track_id);
           
    }

    pub fn get_locations(&mut self) -> &Vec<Geo> {
        &self.locations
    }
}