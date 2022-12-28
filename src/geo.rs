
#[derive(Debug, Clone, Copy)]
pub struct Geo {
    latitude: f64,
    longitute: f64,
}

impl Geo {
    pub fn new(longitute: f64, latitude: f64) -> Self {
        Geo {
            longitute,
            latitude,
        }
    }

    pub fn new_from_string(geom: String) -> Self{
        let mut geom = geom.split_whitespace();
        let longitute = geom.next().unwrap().parse::<f64>().unwrap();
        let latitude = geom.next().unwrap().parse::<f64>().unwrap();
        Self {
            longitute,
            latitude,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Geo{{\"longitute\": {}, \"latitude\": {}}}", self.longitute, self.latitude)
    }

    pub fn get_longitute(&self) -> f64 {
        self.longitute
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }
}
