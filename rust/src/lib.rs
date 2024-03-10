mod context;
mod cursor;
mod error;
mod guard;
mod indent;
mod parser;
mod schema;
mod utils;
use crate::context::Context;
use crate::cursor::Cursor;
use crate::error::ParseError;
use crate::parser::docstring::parse_docstring;
use crate::schema::{Argument, Docstring, DocstringParagraph};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass]
struct PyArgument {
    inner: Argument,
}

#[pymethods]
impl PyArgument {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Argument({:?})", self.inner))
    }
}

impl IntoPy<PyArgument> for Argument {
    fn into_py(self, _py: Python) -> PyArgument {
        PyArgument { inner: self }
    }
}

#[pyclass]
enum PyDocstringParagraphType {
    Args,
    Returns,
    Note,
    Warning,
    Examples,
    Raw,
}
#[pyclass]
struct PyDocstringParagraph {
    inner: DocstringParagraph,
}

impl IntoPy<PyDocstringParagraph> for DocstringParagraph {
    fn into_py(self, _py: Python) -> PyDocstringParagraph {
        PyDocstringParagraph { inner: self }
    }
}

#[pymethods]
impl PyDocstringParagraph {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("DocstringParagraph({:?})", self.inner))
    }

    #[pyo3(name = "r#type")]
    fn type_(&self) -> PyResult<PyDocstringParagraphType> {
        match self.inner {
            DocstringParagraph::Args(_) => Ok(PyDocstringParagraphType::Args),
            DocstringParagraph::Returns(_) => Ok(PyDocstringParagraphType::Returns),
            DocstringParagraph::Note(_) => Ok(PyDocstringParagraphType::Note),
            DocstringParagraph::Warning(_) => Ok(PyDocstringParagraphType::Warning),
            DocstringParagraph::Examples(_) => Ok(PyDocstringParagraphType::Examples),
            DocstringParagraph::Raw(_) => Ok(PyDocstringParagraphType::Raw),
        }
    }
}

#[pyclass]
struct PyDocstring {
    inner: Docstring,
}

impl IntoPy<PyDocstring> for Docstring {
    fn into_py(self, _py: Python) -> PyDocstring {
        PyDocstring { inner: self }
    }
}

#[pymethods]
impl PyDocstring {
    fn __getitem__(&self, index: usize) -> PyResult<PyDocstringParagraph> {
        Python::with_gil(|py| {
            self.inner
                .get(index)
                .map_or(Err(PyValueError::new_err("Index out of range")), |s| {
                    Ok(s.clone().into_py(py))
                })
        })
    }

    // fn __iter__(&self) -> PyResult<PyObject> {
    //     Python::with_gil(|py| {
    //         let list = PyList::empty(py);
    //         for item in &self.inner {
    //             let item: PyObject = item.clone().into_py(py).into();
    //             list.append(item)?;
    //         }
    //         Ok(list.into())
    //     })
    // }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Docstring({:?})", self.inner))
    }
}

impl std::convert::From<ParseError> for PyErr {
    fn from(err: ParseError) -> PyErr {
        PyValueError::new_err(err.format())
    }
}

#[pyfunction]
fn parse(input: &str) -> PyResult<PyDocstring> {
    Python::with_gil(|py| {
        let mut cursor = Cursor::new(input);
        let mut ctx = Context::new(0);

        let parsed = parse_docstring(&mut cursor, &mut ctx)?;
        Ok(parsed.into_py(py))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_core")]
fn docstring_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
