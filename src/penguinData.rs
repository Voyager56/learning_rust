use geo::Geo;

#[derive(Debug)]
pub struct PenguinData {
    fid: String,
    id: u32,
    datetime: String,
    latitude: f64,
    longitude: f64,
    site: String,
    penguin_track: String,
    track_id: String,
    pub geom: Geo,
}

impl PenguinData {
    pub fn new(fid: String, id: u32, datetime: String, latitude: f64, longitude: f64,
         site: String, penguin_track: String, track_id: String, geom: Geo) -> Self {
        PenguinData {
            fid,
            id,
            datetime,
            latitude,
            longitude,
            site,
            penguin_track,
            track_id,
            geom,
        }
    }

    pub fn get_track_id(&self) -> &String {
         &self.track_id
    }

    pub fn get_location(self) -> Geo {
        self.geom
    }

    fn to_string(&self) -> String {
        format!("PenguinData{{\"fid\": {}, \"id\": {}, \"datetime\": {}, \"latitude\": {}, \"longitude\": {}, \"site\": {}, \"penguin_track\": {}, \"track_id\": {}, \"geom\": {}}}",
                self.fid, self.id, self.datetime, self.latitude, self.longitude, self.site, self.penguin_track, self.track_id, self.geom.to_string())
    }
}