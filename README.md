# ssbh_data_py [![PyPI](https://img.shields.io/pypi/v/ssbh_data_py)](https://pypi.org/project/ssbh-data-py/)
Python bindings to the [ssbh_data](https://github.com/ultimate-research/ssbh_lib) Rust library. ssbh_data_py provides a higher level abstraction over 
the binary data stored in SSBH files such as a "model.numshb". SSBH files can be created from scratch or modified from existing files entirely in Python.  

Report bugs and request new features in [issues](https://github.com/ScanMountGoat/ssbh_data_py/issues). Check the requirements before reporting on any installation issues.

## Supported Formats
| Format | Supported Versions (major.minor) | Read | Save |
| --- | --- | --- | --- |
| Modl (`.numdlb`, `.nusrcmdlb`) | 1.7 | :heavy_check_mark: | :heavy_check_mark: |
| Mesh (`.numshb`) | 1.8, 1.9, 1.10 | :heavy_check_mark: | :heavy_check_mark: |
| Skel (`.nusktb`) | 1.0 | :heavy_check_mark: | :heavy_check_mark: |
| Anim (`.nuanmb`) | 2.0, 2.1 | :heavy_check_mark: | :heavy_check_mark: (2.0 only) |
| Matl (`.numatb`) | 1.5, 1.6 | :heavy_check_mark: | :heavy_check_mark: |
| Hlpb (`.nuhlpb`) | 1.1 | :heavy_check_mark: | :heavy_check_mark: |
| Adj (`.adjb`) |  | :heavy_check_mark: | :heavy_check_mark: |
| MeshEx (`.numshexb`) |  | :heavy_check_mark: | :heavy_check_mark: |

## Installing
The package can be installed for a supported python version using `pip` on the latest version of Windows, Linux, or MacOS. The prebuilt wheels (`.whl` files) are included only for situations where `pip` might not be available such as for plugin development for applications. 

Installing: `pip install ssbh_data_py`  
Updating: `pip install ssbh_data_py --upgrade`

The minimum supported pip version is 20.3. 

## Requirements
The package is available on [PyPi](https://pypi.org/project/ssbh_data_py/) for Python 3.7, 3.8, 3.9, and 3.10 for newer versions of Windows, Linux, and Mac OS. Apple Silicon support is currently only available for Python 3.9 and 3.10. For other Python versions, build ssbh_data_py from source.

## Getting Started
Each supported SSBH type has an associated data struct that can be created reading from a file.
Some files like meshes support reading the data as numpy arrays. Enabling numpy support in the read functions substantially reduces the overhead of converting the file data to Python. This requires the `numpy` package to be installed in the current Python environment when enabled.
```python
import ssbh_data_py

mesh = ssbh_data_py.mesh_data.read_mesh("model.numshb", use_numpy=True)
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
Numpy's `ndarray` type is supported for fields and arguments that expect matrices or lists of floats or integers.
```python
import numpy as np

# Assign the identity transform to each bone.
for bone in skel.bones:
    bone.transform = np.eye((4))

# Convert the positions to a numpy array.
for o in mesh.objects:
    o.positions[0].data = np.array(o.positions[0].data)
```

After making any changes, the results can be saved back to a file. Using the same path used to read the files will overwrite the file. Even if no edits are made, the resulting file will likely not be binary identical with the original due to floating point rounding errors or the use of different algorithms. Numpy arrays use a faster exporting code path compared to Python lists or tuples. Using numpy arrays mainly benefits export time for larger lists like mesh vertex attributes. This works the best when the data is already a numpy array since converting from lists may be slow.
```python
mesh.save("model_new.numshb")
modl.save("model_new.numdlb")
skel.save("model_new.nusktb")
```

## Documentation
The Python API is best summarized by the type stub (.pyi) files, which can be found [here](https://github.com/ScanMountGoat/ssbh_data_py/tree/main/ssbh_data_py/ssbh_data_py). The Python API matches the underlying Rust types and functions as closely as possible. The Rust documentation has much more detailed documentation and can be found at https://docs.rs/ssbh_data. For more advanced sample Python scripts, see the [examples](https://github.com/ScanMountGoat/ssbh_data_py/tree/main/examples).

## Building
Requires a recent version of Rust. The builds published to PyPi are built using [Maturin](https://github.com/PyO3/maturin). Aftering succesfully installing Maturin, running `maturin develop` or `maturin develop --release` will build and install the module into the current virtual environment. 

`maturin build --release` builds the `ssbh_data_py` module that can be imported into Python. The resulting binary can only be used with the current Python version, processor type, and operating system. Rename the `.dll` to `.pyd` on Windows and the `.dylib` to `.so` on Linux or MacOS.

If the import fails, check that the `.pyd` or `.so` file exists in the appropriate location. Importing may also fail if `ssbh_data_py` was built from source for an operating system or Python interpreter version that isn't compatible with the current Python environment.
