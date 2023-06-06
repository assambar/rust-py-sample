use pyo3::prelude::*;

mod sample;
use sample::sample_module;

pub fn main() -> PyResult<()> {
    pyo3::append_to_inittab!(sample_module);

    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| -> PyResult<()> {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def f(image_path):
                import sample
                tensor = sample.image_to_tensor('/data/input.jpg', 4, 4)
                print(f'Transformed {image_path} to tensor: {tensor}')",
            "",
            "",
        )?
        .getattr("f")?
        .into();

        fun.call1(py, ("/data/input.jpg",))?;
        Ok(())
    })
}
