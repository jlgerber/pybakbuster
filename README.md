# pybakbuster
Pybakbuster uses bakbuster locally via pyo3 to create a python wheel.

# building

The process relies on pyo3-pack to generate a python wheel file. Then we use pip install to install the local wheel.

```bash
pyo3-pack build
pip install /Users/jonathangerber/src/rust/tests/pyo3test/target/wheels/pybakbuster-0.3.4-cp27-cp27m-macosx_10_7_x86_64.whl
```
# installing requirements
Set the version of rust. I am using a nightly build, which is needed.
```
rustup override set nightly-2018-11-10
```
Install pyo3-pack

```
pip install pyo3-pack
```
or
```
cargo build pyo3-pack
```
## Dependencies
pybakbuster depends on bakbuster, which I have included as a git [submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules).

This is primarily to get around the limitation with Dockerfiles not being able to reference resources outside of the directory in which they appear.

### pulling this project
In order to get the contents of bakbuster when pulling the project, do the following:

```
git clone clone --recurse-submodules https://jlgerber/pybakbuster
```
### Fetching updates to the submodule
```
git submodule update --remote bakbuster
```

## COMPLETE BUILD INSTRUCTIONS
- git clone https://github.com/jlgerber/rustydocker 
- cd rustup
- sudo docker build -t jgerber/rustup:nightly-2018-11-12 .
- cd ../withpython 
- sudo docker build -t jgerber/pyo3-pack:nightly-2018-11-12
- git clone --recurse-submodules https://github.com/jlgerber/pybakbuster 
- sudo docker run --rm -v "${PWD}":/pybakbuster -w /pybakbuster jgerber/pyo3-pack:nightly-2018-11-12  
