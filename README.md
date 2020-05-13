# py-bip39-bindings
Python bindings for the tiny-bip39 library: https://crates.io/crates/tiny-bip39

Reference to https://github.com/LocalCoinSwap/kusama-reference-implementation for the initial work 

## Installation

### Install from PyPI

```
pip install py-bip39-bindings
```

### Compile for local development

```
pip install -r requirements.txt
maturin develop
```
### Build wheelhouses
```
pip install -r requirements.txt

# Build local OS wheelhouse
maturin build

# Build manylinux1 wheelhouse
docker build . --tag polkasource/maturin
docker run --rm -i -v $(pwd):/io polkasource/maturin build

```

## License
https://github.com/polkascan/py-bip39-bindings/blob/master/LICENSE
