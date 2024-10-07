// Notes for design and architecture

struct Data {
  id: i32,
  timestamps: Vec<String>,
  values: HashMap<String, Vec<String>>,
}

impl Data {
  fn new(id: i32) -> Self {
    Self {
      id,
      timestamps: Vec::new(),

      // Maybe store some informations about the type of data contained in each column
      // For example, if the column contains temperature data, store the unit of measurement
      // Needed to not give the user the possibility to select a column that doesn't make sense for a graph etc
      values: HashMap::new(),
    }
  }

  fn parse_excel(&self, file_path: &str) -> Result<(), ExcelProcessingError> {
    // Parse the Excel file and populate the timestamps and values fields
    // Return an error if the file is not found or if there is an error parsing the file
    Ok(())
  }

  fn parse_csv(&self, file_path: &str) -> Result<(), ExcelProcessingError> {
    // Parse the CSV file and populate the timestamps and values fields
    // Return an error if the file is not found or if there is an error parsing the file
    Ok(())
  }

  // Used by react to give a list of selectable headers to display on curves
  fn get_headers_list(&self) -> Vec<String> {
    // Return a list of headers present in the data set
    Vec::new()
  }

  // function called by react to get specific data
  fn get_data(&self, headers: &[&str]) -> (Vec<String>, HashMap<String, Vec<String>>) {
    let mut data = HashMap::new();
    for header in headers {
      if let Some(values) = self.values.get(*header) {
        data.insert(header.to_string(), values.clone());
      }
    }
    (self.timestamps.clone(), data)
  }
}