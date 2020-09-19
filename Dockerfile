FROM konstin2/maturin

RUN rustup default nightly-2020-06-09

ENTRYPOINT ["maturin"]
