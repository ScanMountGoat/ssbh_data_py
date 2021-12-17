# ssbh_data_py
Python bindings to the [ssbh_data](https://github.com/ultimate-research/ssbh_lib) Rust library. ssbh_data_py provides a higher level abstraction over 
the binary data stored in SSBH files such as a "model.numshb". SSBH files can be created from scratch or modified from existing files entirely in Python. Currently supported formats are mesh, skel, and modl. 

Report bugs and request new features in [issues](https://github.com/ScanMountGoat/ssbh_data_py/issues). Check the requirements before reporting on any installation issues.

## Supported Formats
| Format | Supported Versions (major.minor) | Read | Save |
| --- | --- | --- | --- |
| Modl (`.numdlb`, `.nusrcmdlb`) | 1.7 | :heavy_check_mark: | :heavy_check_mark: |
| Mesh (`.numshb`) | 1.8, 1.9, 1.10 | :heavy_check_mark: | :heavy_check_mark: |
| Skel (`.nusktb`) | 1.0 | :heavy_check_mark: | :heavy_check_mark: |
| Anim (`.nuanmb`) | 2.0, 2.1 | :heavy_check_mark: | :heavy_check_mark: (2.0 only) |
| Matl (`.numatb`) | 1.6 | :heavy_check_mark: | :heavy_check_mark: |
| Adj (`.adjb`) |  | :heavy_check_mark: | :heavy_check_mark: |

## Installing
The package can be installed for a supported python version using `pip` on the latest version of Windows, Linux, or MacOS. The prebuilt wheels (`.whl` files) are included only for situations where `pip` might not be available such as for plugin development for applications. 

Installing: `pip install ssbh_data_py`
Updating: `pip install ssbh_data_py --upgrade`

The minimum supported pip version is 20.3. 

## Requirements
The package is available on [PyPi](https://pypi.org/project/ssbh_data_py/) for Python 3.7, 3.8, 3.9, and 3.10 for the latest versions of Windows, Linux, and Mac OS. The supported Linux distributions are Debian 11+, Fedora 34+, Mageia 8+, Photon OS 3.0 with updates, and Ubuntu 21.04+. See the [manylinux](https://github.com/pypa/manylinux) repo under the `many_linux_x_y` section for details on supported Linux distributions.

## Getting Started
Each supported SSBH type has an associated data struct that can be created reading from a file.
```python
import ssbh_data_py

mesh = ssbh_data_py.mesh_data.read_mesh("model.numshb")
modl = ssbh_data_py.modl_data.read_modl("model.numdlb")
skel = ssbh_data_py.skel_data.read_skel("model.nusktb")
```

It's also possible to construct new objects. Specify the major and minor version to use a particular file format revision. Note that this only impacts the binary output when calling the save method. Not all versions are supported, so it's recommended to use the default values.  
```python

mesh = ssbh_data_py.mesh_data.MeshData(major_version=1, minor_version=8)
modl = ssbh_data_py.modl_data.ModlData()
skel = ssbh_data_py.skel_data.SkelData()
```

ssbh_data_py uses standard Python types whenever possible. Conversion to the appropriate binary format is handled automatically on saving. For example, the 4x4 transformation matrix for bone data is simply a list of lists of floats. 
```python
for bone in skel.bones:
    bone.transform[2][1] = 0.5
```
Standard Python operations will work, but lists should always have the same type for each element.  
```python
for bone in skel.bones:
    # Create a 4x4 matrix of all 0's.
    bone.transform = [[0.0] * 4] * 4

    # Python allows this, but this will cause an exception when saving.
    bone.transform = [0, 'abc', []]

# ssbh_data_py found an unexpected type, so this line will fail.
skel.save("skel.nustkb")
```

After making any changes, the results can be saved back to a file. Using the same path used to read the files will overwrite the file. Even if no edits are made, the resulting file will likely not be binary identical with the original due to floating point rounding errors or the use of different algorithms.
```python
mesh.save("model_new.numshb")
modl.save("model_new.numdlb")
skel.save("model_new.nusktb")
```

## Documentation
The Python API matches the underlying Rust types and functions as closely as possible. The Rust documentation can be found at https://docs.rs/ssbh_data. For additional documentation and more advanced sample scripts, see the [examples](https://github.com/ScanMountGoat/ssbh_data_py/tree/main/examples).

## Building
The builds published to PyPi are built using [Maturin](https://github.com/PyO3/maturin). Aftering succesfully installing Maturin, running `maturin develop` or `maturin develop --release` will build and install the module into the current virtual environment. 

`cargo build --release` builds the `ssbh_data_py` module that can be imported into Python. The resulting binary can only be used with the current Python version, processor type, and operating system. Depending on the platform, it may be necessary to rename the file. See the [PyO3 builds page](https://pyo3.rs/v0.14.2/building_and_distribution.html#manual-builds) for details.

If the import fails, check that the `.pyd` or `.so` file exists in the appropriate location. Importing may also fail if `ssbh_data_py` was built from source for an operating system or 
Python interpreter version that isn't compatible with the current Python environment.
