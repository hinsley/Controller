// python.rs
// Provides Python module loading and execution utilities.
//
// Copyright (C) 2020 Carter Hinsley
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

extern crate cpython;

use cpython::{PyModule, PyResult, Python};

/// Courtesy https://web.archive.org/web/20200409103725/https://users.rust-lang.org/t/callable-from-python-inside-rust/18866/16
///
/// Import a module from the given file contents.
///
/// This is a wrapper around `PyModule::new` and `Python::run` which simulates
/// the behavior of the builtin function `exec`. `name` will be used as the
/// module's `__name__`, but is not otherwise important (it does not need
/// to match the file's name).
///
/// Note this compiles and executes the module code each time it is called, as it
/// bypasses the regular import mechanism. No entry is added to the cache in
/// `sys.modules`.
pub fn module_from_str(py: Python<'_>, name: &str, source: &str) -> PyResult<PyModule> {
    let m = PyModule::new(py, name)?;
    m.add(py, "__builtins__", py.import("builtins")?)?;

    let m_locals = m.get(py, "__dict__")?.extract(py)?;
    py.run(source, Some(&m_locals), None)?;
    Ok(m)
}
