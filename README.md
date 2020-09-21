# py-bip39-bindings
Python bindings for the tiny-bip39 library: https://crates.io/crates/tiny-bip39

Reference to Reference to https://github.com/LocalCoinSwap/kusama-reference-implementation/tree/improve-trading-tests/bindings for the initial work 

## Documentation

https://docs.rs/py-bip39-bindings/

## Installation

### Install from PyPI

```shell script
pip install py-bip39-bindings
```

### Compile for local development

```shell script
pip install -r requirements.txt
maturin develop
```
### Build wheelhouses
```shell script
pip install -r requirements.txt

# Build local OS wheelhouse
maturin build

# Build manylinux1 wheelhouse
docker build . --tag polkasource/maturin
docker run --rm -i -v $(pwd):/io polkasource/maturin build

```

## Examples

```python
import binascii
from bip39 import bip39_to_mini_secret, bip39_generate, bip39_validate

mnemonic = bip39_generate(12)
bip39_validate(mnemonic)

seed_array = bip39_to_mini_secret(mnemonic, "")
seed_hex = binascii.hexlify(bytearray(seed_array)).decode("ascii")

```

## License
https://github.com/polkascan/py-bip39-bindings/blob/master/LICENSE
