// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust types that provide convenient functionality built on top of the computecore APIs.

use crate::compute::defs::*;
use crate::computecore;
use crate::HcsResult;
use winutils_rs::windefs::*;

pub struct HcsSystem {
    handle: HcsSystemHandle,
}

impl std::ops::Drop for HcsSystem {
    fn drop(&mut self) {
        if self.handle != std::ptr::null_mut() {
            computecore::close_compute_system(self.handle).expect("Failed to close handle");
        }
    }
}

pub struct HcsProcess {
    handle: HcsProcessHandle,
}

impl std::ops::Drop for HcsProcess {
    fn drop(&mut self) {
        if self.handle != std::ptr::null_mut() {
            computecore::close_process(self.handle).expect("Failed to close handle");
        }
    }
}

pub struct HcsOperation {
    handle: HcsOperationHandle,
}

impl std::ops::Drop for HcsOperation {
    fn drop(&mut self) {
        if self.handle != std::ptr::null_mut() {
            computecore::close_operation(self.handle).expect("Failed to close handle");
        }
    }
}

impl HcsOperation {
    pub fn wrap_handle(handle: Handle) -> HcsOperation {
        HcsOperation { handle }
    }

    pub fn get_handle(&self) -> Handle {
        self.handle
    }

    pub unsafe fn release_handle(&mut self) {
        self.handle = std::ptr::null_mut();
    }

    pub fn create<T>(context: *mut T, callback: HcsOperationCompletion) -> HcsResult<HcsOperation> {
        Ok(HcsOperation {
            handle: computecore::create_operation(context as *mut T as *mut Void, callback)?,
        })
    }

    pub fn set_context<T>(&self, context: *mut T) -> HcsResult<()> {
        computecore::set_operation_context(self.handle, context as *mut T as *mut Void)
    }

    pub fn get_context<T>(&self) -> HcsResult<*mut T> {
        Ok(computecore::get_operation_context(self.handle)? as *mut T)
    }

    pub fn get_compute_system(&self) -> HcsResult<HcsSystem> {
        Ok(HcsSystem {
            handle: computecore::get_compute_system_from_operation(self.handle)?,
        })
    }

    pub fn get_process(&self) -> HcsResult<HcsProcess> {
        Ok(HcsProcess {
            handle: computecore::get_process_from_operation(self.handle)?,
        })
    }

    pub fn get_type(&self) -> HcsResult<HcsOperationType> {
        computecore::get_operation_type(self.handle)
    }

    pub fn get_id(&self) -> HcsResult<u64> {
        computecore::get_operation_id(self.handle)
    }

    pub fn get_result(&self) -> HcsResult<String> {
        computecore::get_operation_result(self.handle)
    }

    pub fn get_result_and_process_info(&self) -> HcsResult<(HcsProcessInformation, String)> {
        computecore::get_operation_result_and_process_info(self.handle)
    }

    pub fn wait_for_result(&self, timeout_ms: DWord) -> HcsResult<String> {
        computecore::wait_for_operation_result(self.handle, timeout_ms)
    }

    pub fn wait_for_result_and_process_info(
        &self,
        timeout_ms: DWord,
    ) -> HcsResult<(HcsProcessInformation, String)> {
        computecore::wait_for_operation_result_and_process_info(self.handle, timeout_ms)
    }

    pub fn set_callback<T>(
        &self,
        context: &mut T,
        callback: HcsOperationCompletion,
    ) -> HcsResult<()> {
        computecore::set_operation_callback(self.handle, context as *mut T as *mut Void, callback)
    }

    pub fn cancel(&self) -> HcsResult<()> {
        computecore::cancel_operation(self.handle)
    }
}
