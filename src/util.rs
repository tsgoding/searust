pub mod utils {

    use serde_json;
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::{Path, PathBuf};
    use xml::reader::{EventReader, XmlEvent};

    const JSON_FILE_PATH: &str = "index.json";

    pub type TermFreq = HashMap<String, usize>;
    pub type TermFreqIndex = HashMap<PathBuf, TermFreq>;

    pub fn read_entire_file<P: AsRef<Path>>(file_path: P) -> Result<String, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let er = EventReader::new(file);
        let mut content = String::new();

        for event in er.into_iter() {
            if let XmlEvent::Characters(event) = event? {
                content.push_str(&event);
            }
        }
        Ok(content)
    }

    pub fn write_tf_to_file(tf_index: TermFreqIndex) -> Result<(), Box<dyn Error>> {
        println!("Saving to JSON");

        let result = serde_json::to_string(&tf_index)?;
        let file = File::create(JSON_FILE_PATH)?;
        serde_json::to_writer(file, &result)?;

        Ok(())
    }

    pub fn read_tf_from_json() -> Result<TermFreqIndex, Box<dyn Error>> {
        let json_file = File::open(JSON_FILE_PATH)?;
        let reader = BufReader::new(json_file);

        let u: TermFreqIndex = serde_json::from_reader(reader)?;

        Ok(u)
    }
}
