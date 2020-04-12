// Courtesy https://web.archive.org/web/20200409103725/https://users.rust-lang.org/t/callable-from-python-inside-rust/18866/16

extern crate cpython;

use cpython::{Python, PyResult, PyModule};

const FIBO_PY: &'static str = include_str!("python_scripts/fibo.py");

fn main() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    example(py).unwrap();
}

fn example(py: Python<'_>) -> PyResult<()> {
    let m = module_from_str(py, "fibo", FIBO_PY)?;

    let out: i32 = m.call(py, "fib", (2,), None)?.extract(py)?;
    println!("successfully found fibo.py at compiletime.  Output: {:?}", out);

    Ok(())
}

/// Import a module from the given file contents.
///
/// This is a wrapper around `PyModule::new` and `Python::run` which simulates
/// the behavior of the builtin function `exec`. `name` will be used as the
/// module's `__name__`, but is not otherwise important (it does not need
/// to match the file's name).
///
/// Note this compiles and executes the module code each time it is called, as it
/// bypasses the regular import mechanism. No entry is added to the cache in `sys.modules`.
fn module_from_str(py: Python<'_>, name: &str, source: &str) -> PyResult<PyModule> {
    let m = PyModule::new(py, name)?;
    m.add(py, "__builtins__", py.import("builtins")?)?;

    let m_locals = m.get(py, "__dict__")?.extract(py)?;
    py.run(source, Some(&m_locals), None)?;
    Ok(m)
}