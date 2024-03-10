mod context;
mod cursor;
mod error;
mod guard;
mod indent;
mod parser;
mod schema;
mod utils;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fmt;

struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

impl std::convert::From<ParseError> for PyErr {
    fn from(err: ParseError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

// impl<E> std::convert::From<winnow::Err<E>> for ParseError {
//     fn from(err: winnow::Err<E>) -> ParseError {
//         match err {
//             winnow::Err::Incomplete(_) => ParseError {
//                 message: "Incomplete".to_string(),
//             },
//             winnow::Err::Error(e) => ParseError {
//                 message: "Error".to_string(),
//             },
//             winnow::Err::Failure(e) => ParseError {
//                 message: "Failure".to_string(),
//             },
//         }
//     }
// }

impl<E> std::convert::From<winnow::error::ErrMode<E>> for ParseError {
    fn from(err: winnow::error::ErrMode<E>) -> ParseError {
        match err {
            winnow::error::ErrMode::Incomplete(_) => ParseError {
                message: "Incomplete".to_string(),
            },
            winnow::error::ErrMode::Backtrack(e) => ParseError {
                message: "Backtrack".to_string(),
            },
            winnow::error::ErrMode::Cut(e) => ParseError {
                message: "Cut".to_string(),
            },
        }
    }
}

#[pyfunction]
fn parse_args(input: &str) -> PyResult<String> {
    // let input = input.to_string();
    // let parsed = args::args_parser(&mut input.as_ref()).map_err(|e| {
    //     let err: ParseError = e.into();
    //     err
    // })?;
    let parsed = "";
    // let parsed = args::args_parser(input);
    // println!("remaining: {}", remaining);
    println!("parsed: {}", parsed);
    Ok(parsed.to_string())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_core")]
fn docstring_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_args, m)?)?;
    Ok(())
}
