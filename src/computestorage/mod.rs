// Copyright (c) 2019-2020 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust abstractions of the computestorage APIs.
//! These are Rust idiomatic equivalents of the C bindings.

#[cfg(not(feature = "bindings"))]
pub(crate) mod bindings;
#[cfg(feature = "bindings")]
pub mod bindings;

use crate::compute::errorcodes::hresult_to_result_code;
use crate::computestorage::bindings::*;
use crate::HcsResult;
use widestring::WideCString;
use winutils_rs::utilities::CoTaskMemWString;
use winutils_rs::windefs::*;

/// Imports a container layer.
pub fn import_layer(path: &str, source_folder_path: &str, layer_data: &str) -> HcsResult<()> {
    unsafe {
        match HcsImportLayer(
            WideCString::from_str(path).unwrap().as_ptr(),
            WideCString::from_str(source_folder_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Exports a container layer.
pub fn export_layer(
    path: &str,
    export_folder_path: &str,
    layer_data: &str,
    options: &str,
) -> HcsResult<()> {
    unsafe {
        match HcsExportLayer(
            WideCString::from_str(path).unwrap().as_ptr(),
            WideCString::from_str(export_folder_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
            WideCString::from_str(options).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Exports a legacy container writable layer.
pub fn export_legacy_writable_layer(
    mount_path: &str,
    folder_path: &str,
    export_folder_path: &str,
    layer_data: &str,
) -> HcsResult<()> {
    unsafe {
        match HcsExportLegacyWritableLayer(
            WideCString::from_str(mount_path).unwrap().as_ptr(),
            WideCString::from_str(folder_path).unwrap().as_ptr(),
            WideCString::from_str(export_folder_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Deletes a container layer.
///
/// # Note
/// This is a very powerful API call, since it will take high level privilages
/// to delete a directory. Misuse of this API could lead to potential deletion
/// of important files, not revertible.
pub fn destroy_layer(layer_path: &str) -> HcsResult<()> {
    unsafe {
        match HcsDestroyLayer(WideCString::from_str(layer_path).unwrap().as_ptr()) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Sets up a layer that contains a base OS for a container.
pub fn setup_base_os_layer(layer_path: &str, vhd_handle: Handle, options: &str) -> HcsResult<()> {
    unsafe {
        match HcsSetupBaseOSLayer(
            WideCString::from_str(layer_path).unwrap().as_ptr(),
            vhd_handle,
            WideCString::from_str(options).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Initializes a writable layer for a container.
pub fn initialize_writable_layer(
    layer_path: &str,
    layer_data: &str,
    options: &str,
) -> HcsResult<()> {
    unsafe {
        match HcsInitializeWritableLayer(
            WideCString::from_str(layer_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
            WideCString::from_str(options).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Initializes a writable layer for a container using the legacy hive folder format.
pub fn initialize_legacy_writable_layer(
    mount_path: &str,
    folder_path: &str,
    layer_data: &str,
    options: &str,
) -> HcsResult<()> {
    unsafe {
        match HcsInitializeLegacyWritableLayer(
            WideCString::from_str(mount_path).unwrap().as_ptr(),
            WideCString::from_str(folder_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
            WideCString::from_str(options).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Sets up the layer storage filter on a writable container layer.
pub fn attach_layer_storage_filter(layer_path: &str, layer_data: &str) -> HcsResult<()> {
    unsafe {
        match HcsAttachLayerStorageFilter(
            WideCString::from_str(layer_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Detaches the layer storage filter from a writable container layer.
pub fn detach_layer_storage_filter(layer_path: &str) -> HcsResult<()> {
    unsafe {
        match HcsDetachLayerStorageFilter(WideCString::from_str(layer_path).unwrap().as_ptr()) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Formats a virtual disk for the use as a writable container layer.
pub fn format_writable_layer_vhd(vhd_handle: Handle) -> HcsResult<()> {
    unsafe {
        match HcsFormatWritableLayerVhd(vhd_handle) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Returns the volume path for a virtual disk of a writable container layer.
pub fn get_layer_vhd_mount_path(vhd_handle: Handle) -> HcsResult<String> {
    unsafe {
        let mut mount_path = CoTaskMemWString::new();

        match HcsGetLayerVhdMountPath(vhd_handle, mount_path.ptr_mut()) {
            0 => Ok(mount_path.to_string()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}
