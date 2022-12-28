pub mod penguinData;
pub mod geo;
pub mod cvsreader;
pub mod penguin;
pub mod datascience;

use cvsreader::CVSReader;
use penguinData::PenguinData;





fn main(){
    let penguin_data = CVSReader::read_csv().unwrap();
    datascience::DataScience::output_location_range_per_track_id(penguin_data);
}