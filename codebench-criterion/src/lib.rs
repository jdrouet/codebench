use codebench_reader::Reader;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CriterionBenchmark {
    group_id: String,
    function_id: String,
    value_str: Option<String>,
    throughput: serde_json::Value,
    full_id: String,
    directory_name: String,
    title: String,
}

impl CriterionBenchmark {
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        Self::from_reader(&mut file)
    }

    fn from_reader<R: Read>(input: R) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_reader(input)?)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CriterionRawLine {
    group: String,
    function: String,
    value: Option<String>,
    throughput_num: Option<usize>,
    throughput_type: Option<String>,
    sample_measured_value: f64,
    unit: String,
    iteration_count: usize,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CriterionRaw(Vec<CriterionRawLine>);

impl CriterionRaw {
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        Self::from_reader(&mut file)
    }

    fn from_reader<R: Read>(input: R) -> Result<Self, Box<dyn Error>> {
        let mut reader = csv::Reader::from_reader(input);
        let result: Result<Vec<CriterionRawLine>, _> = reader.deserialize().collect();
        Ok(Self(result?))
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CriterionResult {
    benchmark: CriterionBenchmark,
    raw: CriterionRaw,
}

impl CriterionResult {
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let bench_path = path.join("new").join("benchmark.json");
        let raw_path = path.join("new").join("raw.csv");

        Ok(Self {
            benchmark: CriterionBenchmark::from_path(&bench_path)?,
            raw: CriterionRaw::from_path(&raw_path)?,
        })
    }

    pub fn is_valid(path: &Path) -> bool {
        path.join("new").join("raw.csv").exists()
            && path.join("new").join("benchmark.json").exists()
    }
}

#[derive(Debug, Default, Clone)]
pub struct CriterionReader;

impl CriterionReader {
    fn visit_dir(
        &self,
        path: &Path,
        result: &mut Vec<CriterionResult>,
    ) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry = entry.path();
            if entry.is_dir() {
                if CriterionResult::is_valid(&entry) {
                    result.push(CriterionResult::from_path(&entry)?);
                } else {
                    self.visit_dir(&entry, result)?;
                }
            }
        }
        Ok(())
    }
}

impl Reader for CriterionReader {
    type Output = Vec<CriterionResult>;

    fn evaluate(&self, path: &Path) -> Result<Self::Output, Box<dyn Error>> {
        let mut result = Vec::new();
        self.visit_dir(path, &mut result)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::CriterionReader;
    use codebench_reader::Reader;
    use std::path::PathBuf;

    #[test]
    fn parse_vector_example() {
        let path = PathBuf::from(".")
            .join("..")
            .join("resources")
            .join("vector-criterion");
        let reader = CriterionReader::default();
        let result = reader.evaluate(&path).unwrap();
        assert_eq!(result.len(), 40);
    }
}
