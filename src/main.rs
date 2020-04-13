// Courtesy https://web.archive.org/web/20200409103725/https://users.rust-lang.org/t/callable-from-python-inside-rust/18866/16

extern crate cpython;
extern crate sciter;

use cpython::{PyResult, Python};

mod python;

const FIBO_PY: &'static str = include_str!("python_scripts/fibo.py");

fn main() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    example(py).unwrap();

    // Sciter demo.
    let mut frame = sciter::Window::new();
    frame.load_file("minimal.htm");
    frame.run_app();
}

fn example(py: Python<'_>) -> PyResult<()> {
    let m = python::module_from_str(py, "fibo", FIBO_PY)?;

    let out: i32 = m.call(py, "fib", (2,), None)?.extract(py)?;
    println!(
        "successfully found fibo.py at compiletime.  Output: {:?}",
        out
    );

    Ok(())
}
