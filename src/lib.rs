#![feature(specialization)]

extern crate bakbuster;
extern crate chrono;
#[macro_use]extern crate pyo3;

use bakbuster::prelude::*;
use chrono::prelude::*;
use pyo3::{ prelude::*, exceptions };
use std::{io::BufReader,fs::File, path::PathBuf, path::Path};


#[pyfunction]
/// retrieves the file given the full path to the swinstalled file and a
/// &str formatted by using datetime.ctime() method
///
/// :param string file:
///     The full path to the installed file
/// :param string datetime:
///     The datetime to lock off on, adopting the datetime.ctime() format
/// (eg 'Sat Feb 113456 2018')
///
/// get_file_on('/dd/facility/etc/packages.xml', 'Sat Dec 23 09:31:46 2017')
fn get_file_on(file: &str, datetime: &str) -> PyResult<String> {

    let file_path = Path::new(file);

    let filename = get_filename_from_path(&file_path)?;

    let swinstall_stack = build_path_to_swinstall_stack(&file_path, filename);

    let file_at_datetime = choose_file_from_swinstall_stack(datetime, &swinstall_stack, filename)?;

    Ok(file_at_datetime)
}

fn get_filename_from_path(path: &Path) -> PyResult<&str> {
    let filename = match path.file_name() {
        Some (fname) => match fname.to_str() {
            Some(fstr) => fstr, // need to allocate to deal with immutable borrow in the midst of mut borrow
            None => return Err(exceptions::ValueError::py_err("Unable to convert file name from OsStr to &str")),
        },
        None => return Err(exceptions::ValueError::py_err("Unable to get file name from input")),
    };
    Ok(filename)
}

fn build_path_to_swinstall_stack(file: &Path, filename: &str) -> PathBuf  {
    // construct a PathBuf from the full path to the versionless file.
    // We will manipulate the pathbuf to point at the swinstall_stack for
    // the original versionless file.
    let mut swinstall_stack = PathBuf::from(file);
    // pop off file name from the
    swinstall_stack.pop();
    // push on bak directory
    swinstall_stack.push("bak");
    // push filename as a directory
    swinstall_stack.push(filename);
    // build the swinstall stack file name and push it on the pathbuf
    swinstall_stack.push(format!("{}_swinstall_stack", filename));
    swinstall_stack
}

fn get_directory_from_path(path: &PathBuf) -> PyResult<&str> {
    let directory = match path.parent() {
        Some(d) => match d.to_str() {
            Some(d) => d,
            None => return Err(exceptions::ValueError::py_err("Unable to get parent path from supplied file")),
        },
        None => return Err(exceptions::ValueError::py_err("Unable to get parent path from supplied file")),
    };
    Ok(directory)
}

fn choose_file_from_swinstall_stack(datetime: &str, pathbuf: &PathBuf, filename: &str) -> PyResult<String> {

    let directory = get_directory_from_path(pathbuf)?;

    // parse the datetime passed in by the user
    let dt =
         NaiveDateTime::parse_from_str(datetime, CTIMEFMT)
        .map_err(|e| exceptions::ValueError::py_err(
            format!("error parsing datetime from '{}': {}",datetime, e)))?;

    // open the file
    let path = pathbuf.as_path();
    let filehandle =
        File::open(path)
        .map_err(|e| exceptions::ValueError::py_err(
            format!("Error calling File::open with {:?}: {}",path, e) ))?;

    // get a buffered file handle
    let buffered_filehandle = BufReader::new(filehandle);

    // pass in to get_file_version_on
    let result =
        get_file_version_on(buffered_filehandle, dt)
        .map_err(|e| exceptions::ValueError::py_err(
            format!("get_file_version_on error for {:?} and {:?}: {}",path, dt, e)))?;

    Ok(format!("{}/{}/{}", directory, result, filename))
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pybakbuster(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_function!(get_file_on))?;
    Ok(())
}