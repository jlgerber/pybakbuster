# From the last successful nighly build we have tested
FROM jgerber/rustpy

RUN mkdir /pybakbuster && mkdir /wheels && mkdir /output

# copy the src
COPY src/ /pybakbuster/src/
COPY ./bakbuster /bakbuster
# copy cargo_slim over as Cargo. Cargo slim only has yaams and yaams_derive
# in it
COPY Cargo.toml /pybakbuster/Cargo.toml

# copy the lock file over so that we can build the appropriate one.
COPY Cargo.lock /pybakbuster/Cargo.lock

RUN pip install pyo3-pack

WORKDIR /pybakbuster
ARG PY_VERSION=python2.7

#RUN cargo build --release
RUN pyo3-pack build -i ${PY_VERSION} -o /wheels
RUN find /wheels/* -exec pip install '{}' -t ${INSTALL} \;

