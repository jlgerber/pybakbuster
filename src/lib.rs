#![feature(specialization)]

#[macro_use]
extern crate pyo3;
extern crate bakbuster;
extern crate chrono;
use std::boxed::Box;
use pyo3::prelude::*;
use pyo3::exceptions;
use bakbuster::prelude::*;
use std::{io::BufReader,fs::File, path::PathBuf};
use chrono::prelude::*;


#[pyfunction]
/// retrieves the file given the basename and a datetime string of
/// the form returned by python's ctime
fn get_file_on(file: String, datetime: String) -> PyResult<String> {
    let pb = PathBuf::from(file.as_str());

    let directory = match pb.parent() {
        Some(d) => d,
        None => return Err(exceptions::ValueError::py_err("Unable to get parent path from supplied file")),
    };
    //.ok_or(|e| exceptions::ValueError::py_err("Unable to get parent path from supplied file")) ?;

    let dt = NaiveDateTime::parse_from_str(datetime.as_str(), CTIMEFMT)
    .map_err(|e| exceptions::ValueError::py_err(format!("error parsing datetime from '{}': {}",datetime.as_str(), e)))?;

    let filehandle = File::open(file.as_str())
    .map_err(|e| exceptions::ValueError::py_err(format!("error calling File::open with {}: {}",file.as_str(), e) ))?;

    let fileh = BufReader::new(filehandle);
    let result = get_file_version_on(fileh, dt)
    .map_err(|e| exceptions::ValueError::py_err(format!("get_file_version_on error for {:?} and {:?}: {}",file, dt, e)))?;

    // convert to string
    let directory = match directory.to_str() {
        Some(d) => d,
        None => return Err(exceptions::ValueError::py_err("Unable to convert directory to str")),
    };
    //.ok_or(|e|  )?;

    Ok(format!("{}/{}",directory, result))
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pybakbuster(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_function!(get_file_on))?;
    Ok(())
}