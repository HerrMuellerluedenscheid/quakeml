mod quakeml;

use std::path::Path;


fn main() {
    let filename = Path::new("sample.quakeml");
    let data = quakeml::read_quakeml(filename);
    quakeml::deserialize_quakeml(data);
}
