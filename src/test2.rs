// Notes for design and architecture
// using template data structure to store the data

struct ColumnData<T> {
  values: Vec<T>,
  is_graphable: bool,
  unit_of_measurement: Option<String>,
}

impl<T> ColumnData<T> {
  fn new(values: Vec<T>, is_graphable: bool, unit_of_measurement: Option<String>) -> Self {
    Self {
      values,
      is_graphable,
      unit_of_measurement,
    }
  }
}

struct Data {
  id: i32,
  timestamps: Vec<String>,
  values: HashMap<String, ColumnData<String>>,
}

impl Data {
  fn new(id: i32) -> Self {
    Self {
      id,
      timestamps: Vec::new(),
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
  fn get_graphable_headers_list(&self) -> Vec<String> {
    self.values
      .iter()
      .filter(|(_, column_data)| column_data.is_graphable)
      .map(|(header, _)| header.clone())
      .collect()
}

  // function called by react to get specific data
  fn get_data(&self, headers: &[&str]) -> (Vec<String>, HashMap<String, Vec<String>>) {
    let mut data = HashMap::new();
    for header in headers {
      if let Some(column_data) = self.values.get(*header) {
        data.insert(header.to_string(), column_data.values.clone());
      }
    }
    (self.timestamps.clone(), data)
  }
}