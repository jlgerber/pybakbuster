# From the last successful nighly build we have tested
FROM jgerber/rustpyo3:2018-11-12

RUN mkdir /wheels

# copy the src
#COPY src/ /pybakbuster/src/
#COPY ./bakbuster /bakbuster
# copy cargo_slim over as Cargo. Cargo slim only has yaams and yaams_derive
# in it
#COPY Cargo.toml /pybakbuster/Cargo.toml

# copy the lock file over so that we can build the appropriate one.
#COPY Cargo.lock /pybakbuster/Cargo.lock

#RUN pip install pyo3-pack

#WORKDIR /pybakbuster
#ARG PY_VERSION=python2.7
# build the release version of yaams
#RUN cargo build --release
ENTRYPOINT ["pyo3-pack","build"]

