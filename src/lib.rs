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
/// retrieves the file given the full path to the swinstalled file and a
/// string formatted by using datetime.ctime() method
///
/// # example
///
/// get_file_on(/dd/facility/etc/packages.xml, Sat Dec 23 09:31:46 2017)
fn get_file_on(file: &str, datetime: &str) -> PyResult<String> {
    let mut pb = PathBuf::from(file);

    // get filename
    // for some reason ok_or(...) does not work here. I have to use the more verbose
    // match...
    let filename = match pb.file_name() {
        Some (fname) => match fname.to_str() {
            Some(fstr) => fstr.to_string(), // need to allocate to deal with immutable borrow in the midst of mut borrow
            None => return Err(exceptions::ValueError::py_err("Unable to convert file name from OsStr to &str")),
        },
        None => return Err(exceptions::ValueError::py_err("Unable to get file name from input")),
    };
    // pop off file
    pb.pop();
    // push on bak directory
    pb.push("bak");
    // push filename as a directory
    pb.push(filename.as_str());
    // build the swinstall stack file name and push it on the pathbuf
    pb.push(format!("{}_swinstall_stack", filename));
    // grab the directory from the path. Ideally we would do this before the
    // former call, but we run into mut vs non-mut reference scope issues so...
    let directory = match pb.parent() {
        Some(d) => match d.to_str() {
            Some(d) => d,
            None => return Err(exceptions::ValueError::py_err("Unable to get parent path from supplied file")),
        },
        None => return Err(exceptions::ValueError::py_err("Unable to get parent path from supplied file")),
    };
    // parse the datetime passed in by the user
    let dt = NaiveDateTime::parse_from_str(datetime, CTIMEFMT)
    .map_err(|e| exceptions::ValueError::py_err(format!("error parsing datetime from '{}': {}",datetime, e)))?;
    // open the file
    let filehandle = File::open(pb.as_path())
    .map_err(|e| exceptions::ValueError::py_err(format!("Error calling File::open with {}: {}",file, e) ))?;
    // get a buffered file handle
    let fileh = BufReader::new(filehandle);
    // pass in to get_file_version_on
    let result = get_file_version_on(fileh, dt)
    .map_err(|e| exceptions::ValueError::py_err(format!("get_file_version_on error for {:?} and {:?}: {}",file, dt, e)))?;

    Ok(format!("{}/{}",directory, result))
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pybakbuster(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_function!(get_file_on))?;
    Ok(())
}