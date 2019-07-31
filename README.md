# hcs-rs
Rust wrapper of Host Compute Service API

## Overview

This project is a collection of Rust libraries that wrap functionality exposed by the [Host Compute Service](https://blogs.technet.microsoft.com/virtualization/2017/01/27/introducing-the-host-compute-service-hcs/).

HCS (Host Compute Service for short) APIs are part of the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk).

Public documentation for the HCS is yet to be released.

## Requirements

For this wrapper to build properly, the following requirements need to be met by the building machine:

- Windows 10 SDK version **10.0.17763.132**.
- **amd64** architecture.

## Wrapped Windows 10 SDK APIs

**_Note: This section includes the paths in the Windows SDK for the header and lib files based on the default installation path `c:\Program Files (x86)\Windows Kits\10`._**

The relevant Windows 10 SDK files that this project is wrapping are:
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computecore.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computedefs.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computenetwork.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computestorage.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\hypervdevicevirtualization.h
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computecore.lib
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computenetwork.lib
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computestorage.lib
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\vmdevicehost.lib
- C:\Windows\System32\computecore.dll
- C:\Windows\System32\computenetwork.dll
- C:\Windows\System32\computestorage.dll
- C:\Windows\System32\vmdevicehost.dll

All of the above are serviced through the Windows Service `Host Compute Service`, from executable **C:\Windows\System32\vmcompute.exe**.

The C bindings in this crate will remain private and not public to external code. Only the Rust idiomatic wrappers are exposed.

The following table describes the relevant Windows 10 SDK files that this project is wrapping and how they relate to each module:

| hcs-rs file | Overview | HCS API C Header file | .h path in SDK | .lib path in SDK |
| -- | -- | -- | -- | -- |
| [computedefs](/src/compute/defs.rs) | Types and definitions used by the HCS APIs. | computedefs.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computedefs.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computedefs.lib |
| [computecore](/src/computecore/mod.rs) | APIs for the core HCS functionality to manage compute systems. | computecore.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computecore.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computecore.lib |
| [computenetwork](/src/computenetwork/mod.rs) | Types definitions and APIs to interact with the HCN (Host Compute Network). | computenetwork.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computenetwork.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computenetwork.lib |
| [computestorage](/src/computestorage/mod.rs) | APIs for the HCS storage management. | computestorage.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computestorage.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computestorage.lib |
| [hypervdevicevirtualization](/src/hypervdevicevirtualization/mod.rs) | Types definitions and APIs for device emulation/virtualization. | hypervdevicevirtualization.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\hypervdevicevirtualization.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\vmdevicehost.lib |

## Crates.io version notes

This section briefly describes all published crates.io [versions](https://crates.io/crates/hcs-rs/versions) of this project, ordered from latest to oldest.

- [**0.1.0 Jan 8, 2010**](https://crates.io/crates/virtdisk-rs/0.1.0)
  - First version, with very limited functionality
  - Bindings to the C APIs exposed as thin safe wrappers
  - Lacking a lot of safe abstractions
  - Hardcoded dependency to Windows 10 SDK version 10.0.17763.0
