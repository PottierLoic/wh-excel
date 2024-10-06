use calamine::{open_workbook_auto, DataType, Reader};
use chrono::{Duration, NaiveDate};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct SensorData {
    sensor_id: i32,
    data: HashMap<String, Vec<(String, String)>>,
}

#[derive(Debug)]
enum ExcelProcessingError {
    InvalidDateTimeValue(DataType),
    InvalidHeaderValue,
    FileNotFoundError(String),
    SheetNotFoundError,
    CellParsingError,
    MissingDateTime,
    InvalidSensorIdFormat,
}

impl fmt::Display for ExcelProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExcelProcessingError::InvalidDateTimeValue(value) => {
                write!(f, "Invalid date/time value encountered: {:?}", value)
            }
            ExcelProcessingError::InvalidHeaderValue => {
                write!(f, "Invalid header value encountered")
            }
            ExcelProcessingError::FileNotFoundError(path) => {
                write!(f, "File not found: {}", path)
            }
            ExcelProcessingError::SheetNotFoundError => {
                write!(f, "Could not read the second sheet.")
            }
            ExcelProcessingError::CellParsingError => {
                write!(f, "Cell parsing error occurred.")
            }
            ExcelProcessingError::MissingDateTime => {
                write!(f, "Missing valid date/time value in row.")
            }
            ExcelProcessingError::InvalidSensorIdFormat => {
                write!(f, "Sensor ID format is invalid or cannot be parsed.")
            }
        }
    }
}

impl Error for ExcelProcessingError {}

type ExcelResult = SensorData;

fn excel_datetime_to_string(excel_float: f64) -> (String, String) {
    let base_date = NaiveDate::from_ymd_opt(1899, 12, 30).expect("Invalid base date");
    let whole_days = excel_float.trunc() as i64;
    let fractional_day = excel_float.fract();
    let date = base_date + Duration::days(whole_days);
    let date_string = date.format("%Y-%m-%d").to_string();
    let seconds_in_day = (fractional_day * 86400.0).round() as i64;
    let hours = seconds_in_day / 3600;
    let minutes = (seconds_in_day % 3600) / 60;
    let seconds = seconds_in_day % 60;
    let time_string = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    (date_string, time_string)
}

fn process_excel_file(file_path: &str) -> Result<ExcelResult, Box<dyn Error>> {
    let mut data: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut workbook = open_workbook_auto(file_path)
        .map_err(|_| ExcelProcessingError::FileNotFoundError(file_path.to_string()))?;

    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        let mut rows = range.rows();
        let sensor_id = if let Some(sensor_id_row) = rows.next() {
            if let Some(DataType::String(sensor_id_str)) = sensor_id_row.first() {
                if let Some(id_str) = sensor_id_str.strip_prefix("Sensor ID: ") {
                    id_str
                        .parse::<i32>()
                        .map_err(|_| ExcelProcessingError::InvalidSensorIdFormat)?
                } else {
                    return Err(Box::new(ExcelProcessingError::InvalidSensorIdFormat));
                }
            } else {
                return Err(Box::new(ExcelProcessingError::InvalidSensorIdFormat));
            }
        } else {
            return Err(Box::new(ExcelProcessingError::SheetNotFoundError));
        };
        // Skipping the useless designation field
        rows.next();
        if let Some(header_row) = rows.next() {
            for (_col_idx, header_cell) in header_row.iter().enumerate().skip(2) {
                if let DataType::String(header_str) = header_cell {
                    data.insert(header_str.clone(), Vec::new());
                } else {
                    return Err(Box::new(ExcelProcessingError::InvalidHeaderValue));
                }
            }
            for row in rows {
                let date_time_cell = row.first().or(row.get(1));
                if let Some(date_time_value) = date_time_cell {
                    let date_time_float = match date_time_value {
                        DataType::DateTime(dt) | DataType::Float(dt) => *dt,
                        _ => {
                            return Err(Box::new(ExcelProcessingError::InvalidDateTimeValue(
                                date_time_value.clone(),
                            )));
                        }
                    };
                    let (date_str, time_str) = excel_datetime_to_string(date_time_float);
                    let timestamp = format!("{} {}", date_str, time_str);
                    for (col_idx, value_cell) in row.iter().enumerate().skip(2) {
                        if let Some(DataType::String(header)) = header_row.get(col_idx) {
                            let entry = data
                                .get_mut(header)
                                .ok_or(ExcelProcessingError::CellParsingError)?;

                            match value_cell {
                                DataType::String(val_str) => {
                                    entry.push((timestamp.clone(), val_str.clone()));
                                }
                                DataType::Float(val_float) => {
                                    entry.push((timestamp.clone(), val_float.to_string()));
                                }
                                DataType::Empty => {
                                    continue;
                                }
                                _ => {
                                    return Err(Box::new(ExcelProcessingError::CellParsingError));
                                }
                            }
                        }
                    }
                } else {
                    return Err(Box::new(ExcelProcessingError::MissingDateTime));
                }
            }
        }
        Ok(SensorData { sensor_id, data })
    } else {
        Err(Box::new(ExcelProcessingError::SheetNotFoundError))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data.xlsx";
    let result = process_excel_file(file_path)?;
    println!("Sensor ID: {}", result.sensor_id);
    for (header, entries) in result.data {
        println!("Header: {}", header);
        for (timestamp, value) in entries {
            println!("Timestamp: {}, Value: {}", timestamp, value);
        }
        println!();
    }

    Ok(())
}
