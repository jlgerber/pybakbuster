#![feature(specialization)]

extern crate bakbuster;
extern crate chrono;
#[macro_use]extern crate pyo3;

use bakbuster::prelude::*;
use chrono::prelude::*;
use pyo3::{ prelude::*, exceptions::ValueError };
use std::{io::BufReader,fs::File, path::PathBuf};


#[pyfunction]
/// retrieves the file given the full path to the swinstalled file and a
/// string formatted by using datetime.ctime() method
///
/// :param string file:
///     The full path to the installed file
/// :param string datetime:
///     The datetime to lock off on, adopting the datetime.ctime() format
/// (eg 'Sat Feb 113456 2018')
///
/// get_file_on('/dd/facility/etc/packages.xml', 'Sat Dec 23 09:31:46 2017')
fn get_file_on(file: &str, datetime: &str) -> PyResult<String> {
    let mut pb = PathBuf::from(file);
    
    // get filename
    let filename = pb.file_name()
    .ok_or_else(|| ValueError::py_err("Unable to get file name from input"))?
    .to_str()
    .ok_or_else(|| ValueError::py_err("Unable to convert file name from OsStr to &str"))?
    .to_string();

    // pop off file
    pb.pop();

    // push on bak directory
    pb.push("bak");
    
    // push filename as a directory
    pb.push(filename.as_str());

    pb.push(format!("{}_swinstall_stack", filename));
    
    let directory = pb.parent()
    .ok_or_else (|| ValueError::py_err("Unable to get parent path from supplied file"))?
    .to_str()
    .ok_or_else(|| ValueError::py_err("Unable to get parent path from supplied file"))?;

    // parse the datetime passed in by the user
    let dt = NaiveDateTime::parse_from_str(datetime, CTIMEFMT)
    .map_err(|e| ValueError::py_err(format!("error parsing datetime from '{}': {}",datetime, e)))?;
    // open the file
    let filehandle = File::open(pb.as_path())
    .map_err(|e| ValueError::py_err(format!("Error calling File::open with {}: {}",file, e) ))?;
    // get a buffered file handle
    let fileh = BufReader::new(filehandle);
    // pass in to get_file_version_on
    let result = get_file_version_on(fileh, dt)
    .map_err(|e| ValueError::py_err(format!("get_file_version_on error for {:?} and {:?}: {}",file, dt, e)))?;

    Ok(format!("{}/{}/{}", directory, result, filename.as_str()))
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pybakbuster(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_function!(get_file_on))?;
    Ok(())
}