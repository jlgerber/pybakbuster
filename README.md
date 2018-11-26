# pybakbuster
Pybakbuster uses bakbuster locally via pyo3 to create a python wheel.

# building

The process relies on pyo3-pack to generate a python wheel file. Then we use pip install to install the local wheel.

```bash
pyo3-pack build
pip install /Users/jonathangerber/src/rust/tests/pyo3test/target/wheels/pybakbuster-0.3.4-cp27-cp27m-macosx_10_7_x86_64.whl
```
