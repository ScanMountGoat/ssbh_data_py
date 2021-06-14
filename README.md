# ssbh_data_py
Python bindings to the [ssbh_data](https://github.com/ultimate-research/ssbh_lib) Rust library. ssbh_data_py provides a higher level abstraction over 
the binary data stored in SSBH files such as a "model.numshb". SSBH files can be created from scratch or modified from existing files entirely in Python. 
Currently supported formats are mesh, skel, and modl. 

Report bugs and request new features in [issues](https://github.com/ScanMountGoat/ssbh_data_py/issues). Check the requirements before reporting on any installation issues.

## Installing
Installing: `pip install ssbh_data_py`  
Updating: `pip install ssbh_data_py --upgrade`.  
The minimum supported pip version is 20.3. 

## Requirements
The package is available on [PyPi](https://pypi.org/project/ssbh_data_py/) for Python 3.6, 3.7, 3.8, and 3.9 for the latest versions of Windows, Linux, and Mac OS. The supported Linux distributions are Debian 11+, Fedora 34+, Mageia 8+, Photon OS 3.0 with updates, and Ubuntu 21.04+. See the [manylinux](https://github.com/pypa/manylinux) repo under the `many_linux_x_y` section for details on supported Linux distributions.

## Getting Started
Import the package after installing with Pip.
```python
import ssbh_data_py
```
Each supported SSBH type has an associated data struct that can be created reading from a file.
```python
mesh = ssbh_data_py.mesh_data.read_mesh("model.numshb")
modl = ssbh_data_py.modl_data.read_modl("model.numdlb")
skel = ssbh_data_py.skel_data.read_skel("model.nusktb")
```
It's also possible to create the objects from scratch.  
```python
# It's possible to specify the version in the constructor or change it later.
# Leave the versions at their default values to avoid errors when saving.
mesh = ssbh_data_py.mesh_data.MeshData(major_version=1, minor_version=8)
modl = ssbh_data_py.modl_data.ModlData()
skel = ssbh_data_py.skel_data.SkelData()
```
After making any changes, save the results. Using the same path used to read the files will overwrite the file.
```python
mesh.save("model_new.numshb")
modl.save("model_new.numdlb")
skel.save("model_new.nusktb")
```
## Documentation
The Python API matches the underlying Rust types and functions as closely as possible. See the [ssbh_data docs.rs](https://docs.rs/ssbh_data) on [![docs.rs](https://docs.rs/ssbh_data/badge.svg)](https://docs.rs/ssbh_data)

For additional documentation and more advanced sample scripts, see the [examples](https://github.com/ScanMountGoat/ssbh_data_py/tree/main/examples).

## Building
`cargo build --release` builds the `ssbh_data_py` module that can be imported into Python. Depending on the platform, it may be necessary to rename the file. See the [PyO3 page](https://github.com/PyO3/pyo3) for details. The builds published to PyPi are built using [Maturin](https://github.com/PyO3/maturin).
