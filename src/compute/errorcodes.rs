// Copyright (c) 2019-2020 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Enumeration of HCS related and common error codes returned by failure paths in the HCS APIs.

use winutils_rs::windefs::HResult;

/// Common result codes and error codes that are specific to virtualization,
/// that can be returned by the HCS APIs.
#[derive(Debug, PartialEq)]
pub enum ResultCode {
    Success,
    OutOfMemory,
    FileNotFound,
    Fail,
    InvalidArgument,
    Unexpected,

    /// The hypervisor does not support the operation because the specified hypercall code is not supported.
    HvInvalidHypercallCode,

    /// The hypervisor does not support the operation because the encoding for the hypercall input register is not supported.
    HvInvalidHypercallInput,

    /// The hypervisor could not perform the operation because a parameter has an invalid alignment.
    HvInvalidAlignment,

    /// The hypervisor could not perform the operation because an invalid parameter was specified.
    HvInvalidParameter,

    /// Access to the specified object was denied.
    HvAccessDenied,

    /// The hypervisor could not perform the operation because the partition is entering or in an invalid state.
    HvInvalidPartitionState,

    /// The operation is not allowed in the current state.
    HvOperationDenied,

    /// The hypervisor does not recognize the specified partition property.
    HvUnknownProperty,

    /// The specified value of a partition property is out of range or violates an invariant.
    HvPropertyValueOutOfRange,

    /// There is not enough memory in the hypervisor pool to complete the operation.
    HvInsufficientMemory,

    /// The maximum partition depth has been exceeded for the partition hierarchy.
    HvPartitionTooDeep,

    /// A partition with the specified partition Id does not exist.
    HvInvalidPartitionId,

    /// The hypervisor could not perform the operation because the specified VP index is invalid.
    HvInvalidVpIndex,

    /// The hypervisor could not perform the operation because the specified port identifier is invalid.
    HvInvalidPortId,

    /// The hypervisor could not perform the operation because the specified connection identifier is invalid.
    HvInvalidConnectionId,

    /// Not enough buffers were supplied to send a message.
    HvInsufficientBuffers,

    /// The previous virtual interrupt has not been acknowledged.
    HvNotAcknowledged,

    /// A virtual processor is not in the correct state for the indicated operation.
    HvInvalidVpState,

    /// The previous virtual interrupt has already been acknowledged.
    HvAcknowledged,

    /// The indicated partition is not in a valid state for saving or restoring.
    HvInvalidSaveRestoreState,

    /// The hypervisor could not complete the operation because a required feature of the synthetic interrupt controller (SynIC) was disabled.
    HvInvalidSynicState,

    /// The hypervisor could not perform the operation because the object or value was either already in use or being used for a purpose that would not permit completing the operation.
    HvObjectInUse,

    /// The proximity domain information is invalid.
    HvInvalidProximityDomainInfo,

    /// An attempt to retrieve debugging data failed because none was available.
    HvNoData,

    /// The physical connection being used for debugging has not recorded any receive activity since the last operation.
    HvInactive,

    /// There are not enough resources to complete the operation.
    HvNoResources,

    /// A hypervisor feature is not available to the user.
    HvFeatureUnavailable,

    /// The specified buffer was too small to contain all of the requested data.
    HvInsufficientBuffer,

    /// The maximum number of domains supported by the platform I/O remapping hardware is currently in use. No domains are available to assign this device to this partition.
    HvInsufficientDeviceDomains,

    /// Validation of CPUID data of the processor failed.
    HvCpuidFeatureValidation,

    /// Validation of XSAVE CPUID data of the processor failed.
    HvCpuidXsaveFeatureValidation,

    /// Processor did not respond within the timeout period.
    HvProcessorStartupTimeout,

    /// SMX has been enabled in the BIOS.
    HvSmxEnabled,

    /// The hypervisor could not perform the operation because the specified LP index is invalid.
    HvInvalidLpIndex,

    /// The supplied register value is invalid.
    HvInvalidRegisterValue,

    /// The supplied virtual trust level is not in the correct state to perform the requested operation.
    HvInvalidVtlState,

    /// No execute feature (NX) is not present or not enabled in the BIOS.
    HvNxNotDetected,

    /// The supplied device ID is invalid.
    HvInvalidDeviceId,

    /// The operation is not allowed in the current device state.
    HvInvalidDeviceState,

    /// The device had pending page requests which were discarded.
    HvPendingPageRequests,

    /// The supplied page request specifies a memory access that the guest does not have permissions to perform.
    HvPageRequestInvalid,

    /// A CPU group with the specified CPU group Id does not exist.
    HvInvalidCpuGroupId,

    /// The hypervisor could not perform the operation because the CPU group is entering or in an invalid state.
    HvInvalidCpuGroupState,

    /// The requested operation failed.
    HvOperationFailed,

    /// The hypervisor could not perform the operation because it is not allowed with nested virtualization active.
    HvNotAllowedWithNestedVirtActive,

    /// There is not enough memory in the root partition's pool to complete the operation.
    HvInsufficientRootMemory,

    /// No hypervisor is present on this system.
    HvNotPresent,

    /// The handler for the virtualization infrastructure driver is already registered. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidDuplicateHandler,

    /// The number of registered handlers for the virtualization infrastructure driver exceeded the maximum. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidTooManyHandlers,

    /// The message queue for the virtualization infrastructure driver is full and cannot accept new messages. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidQueueFull,

    /// No handler exists to handle the message for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidHandlerNotPresent,

    /// The name of the partition or message queue for the virtualization infrastructure driver is invalid. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidInvalidObjectName,

    /// The partition name of the virtualization infrastructure driver exceeds the maximum.
    VidPartitionNameTooLong,

    /// The message queue name of the virtualization infrastructure driver exceeds the maximum.
    VidMessageQueueNameTooLong,

    /// Cannot create the partition for the virtualization infrastructure driver because another partition with the same name already exists.
    VidPartitionAlreadyExists,

    /// The virtualization infrastructure driver has encountered an . The requested partition does not exist. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidPartitionDoesNotExist,

    /// The virtualization infrastructure driver has encountered an . Could not find the requested partition. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidPartitionNameNotFound,

    /// A message queue with the same name already exists for the virtualization infrastructure driver.
    VidMessageQueueAlreadyExists,

    /// The memory block page for the virtualization infrastructure driver cannot be mapped because the page map limit has been reached. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidExceededMbpEntryMapLimit,

    /// The memory block for the virtualization infrastructure driver is still being used and cannot be destroyed.
    VidMbStillReferenced,

    /// Cannot unlock the page array for the guest operating system memory address because it does not match a previous lock request. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidChildGpaPageSetCorrupted,

    /// The non-uniform memory access (NUMA) node settings do not match the system NUMA topology. In order to start the virtual Vidmachine, you will need to modify the NUMA configuration.
    VidInvalidNumaSettings,

    /// The non-uniform memory access (NUMA) node index does not match a valid index in the system NUMA topology.
    VidInvalidNumaNodeIndex,

    /// The memory block for the virtualization infrastructure driver is already associated with a message queue.
    VidNotificationQueueAlreadyAssociated,

    /// The handle is not a valid memory block handle for the virtualization infrastructure driver.
    VidInvalidMemoryBlockHandle,

    /// The request exceeded the memory block page limit for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidPageRangeOverflow,

    /// The handle is not a valid message queue handle for the virtualization infrastructure driver.
    VidInvalidMessageQueueHandle,

    /// The handle is not a valid page range handle for the virtualization infrastructure driver.
    VidInvalidGpaRangeHandle,

    /// Cannot install client notifications because no message queue for the virtualization infrastructure driver is associated with the memory block.
    VidNoMemoryBlockNotificationQueue,

    /// The request to lock or map a memory block page failed because the virtualization infrastructure driver memory block limit has been reached. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidMemoryBlockLockCountExceeded,

    /// The handle is not a valid parent partition mapping handle for the virtualization infrastructure driver.
    VidInvalidPpmHandle,

    /// Notifications cannot be created on the memory block because it is use.
    VidMbpsAreLocked,

    /// The message queue for the virtualization infrastructure driver has been closed. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidMessageQueueClosed,

    /// Cannot add a virtual processor to the partition because the maximum has been reached.
    VidVirtualProcessorLimitExceeded,

    /// Cannot stop the virtual processor immediately because of a pending intercept.
    VidStopPending,

    /// Invalid state for the virtual processor. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidInvalidProcessorState,

    /// The maximum number of kernel mode clients for the virtualization infrastructure driver has been reached. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidExceededKmContextCountLimit,

    /// This kernel mode interface for the virtualization infrastructure driver has already been initialized. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidKmInterfaceAlreadyInitialized,

    /// Cannot set or reset the memory block property more than once for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidMbPropertyAlreadySetReset,

    /// The memory mapped I/O for this page range no longer exists. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidMmioRangeDestroyed,

    /// The lock or unlock request uses an invalid guest operating system memory address. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidInvalidChildGpaPageSet,

    /// Cannot destroy or reuse the reserve page set for the virtualization infrastructure driver because it is in use. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidReservePageSetIsBeingUsed,

    /// The reserve page set for the virtualization infrastructure driver is too small to use in the lock request. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidReservePageSetTooSmall,

    /// Cannot lock or map the memory block page for the virtualization infrastructure driver because it has already been locked using a reserve page set page. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidMbpAlreadyLockedUsingReservedPage,

    /// Cannot create the memory block for the virtualization infrastructure driver because the requested number of pages exceeded the limit. Restarting the virtual machine may fix the problem. If the problem Vidpersists, try restarting the physical computer.
    VidMbpCountExceededLimit,

    /// Cannot restore this virtual machine because the saved state data cannot be read. Delete the saved state data and then try to start the virtual machine.
    VidSavedStateCorrupt,

    /// Cannot restore this virtual machine because an item read from the saved state data is not recognized. Delete the saved state data and then try to start the virtual machine.
    VidSavedStateUnrecognizedItem,

    /// Cannot restore this virtual machine to the saved state because of hypervisor incompatibility. Delete the saved state data and then try to start the virtual machine.
    VidSavedStateIncompatible,

    /// The specified VTL does not have the permission to access the resource.
    VidVtlAccessDenied,

    /// The virtual machine or container exited unexpectedly while starting.
    VmComputeTerminatedDuringStart,

    /// The container operating system does not match the host operating system.
    VmComputeImageMismatch,

    /// The virtual machine could not be started because a required feature is not installed.
    VmComputeHypervNotInstalled,

    /// The call to start an asynchronous operation succeeded and the operation is performed in the background.
    VmComputeOperationPending,

    /// The supported number of notification callbacks has been exceeded.
    VmComputeTooManyNotifications,

    /// The requested virtual machine or container operation is not valid in the current state.
    VmComputeInvalidState,

    /// The virtual machine or container exited unexpectedly.
    VmComputeUnexpectedExit,

    /// The virtual machine or container was forcefully exited.
    VmComputeTerminated,

    /// A connection could not be established with the container or virtual machine.
    VmComputeConnectFailed,

    /// The operation timed out because a response was not received from the virtual machine or container.
    VmComputeTimeout,

    /// The connection with the virtual machine or container was closed.
    VmComputeConnectionClosed,

    /// An unknown internal message was received by the virtual machine or container.
    VmComputeUnknownMessage,

    /// The virtual machine or container does not support an available version of the communication protocol with the host.
    VmComputeUnsupportedProtocolVersion,

    /// The virtual machine or container JSON document is invalid.
    VmComputeInvalidJson,

    /// A virtual machine or container with the specified identifier does not exist.
    VmComputeSystemNotFound,

    /// A virtual machine or container with the specified identifier already exists.
    VmComputeSystemAlreadyExists,

    /// The virtual machine or container with the specified identifier is not running.
    VmComputeSystemAlreadyStopped,

    /// A communication protocol  has occurred between the virtual machine or container and the host.
    VmComputeProtocol,

    /// The container image contains a layer with an unrecognized format.
    VmComputeInvalidLayer,

    /// To use this container image, you must join the Windows Insider Program. Please see https://go.microsoft.com/fwlink/?linkid=850659 for more information.
    VmComputeWindowsInsiderRequired,

    /// The virtual machine or container exited unexpectedly while starting.
    HcsTerminatedDuringStart,

    /// The container operating system does not match the host operating system.
    HcsImageMismatch,

    /// The virtual machine could not be started because a required feature is not installed.
    HcsHypervNotInstalled,

    /// The requested virtual machine or container operation is not valid in the current state.
    HcsInvalidState,

    /// The virtual machine or container exited unexpectedly.
    HcsUnexpectedExit,

    /// The virtual machine or container was forcefully exited.
    HcsTerminated,

    /// A connection could not be established with the container or virtual machine.
    HcsConnectFailed,

    /// The operation timed out because a response was not received from the virtual machine or container.
    HcsConnectionTimeout,

    /// The connection with the virtual machine or container was closed.
    HcsConnectionClosed,

    /// An unknown internal message was received by the virtual machine or container.
    HcsUnknownMessage,

    /// The virtual machine or container does not support an available version of the communication protocol with the host.
    HcsUnsupportedProtocolVersion,

    /// The virtual machine or container JSON document is invalid.
    HcsInvalidJson,

    /// A virtual machine or container with the specified identifier does not exist.
    HcsSystemNotFound,

    /// A virtual machine or container with the specified identifier already exists.
    HcsSystemAlreadyExists,

    /// The virtual machine or container with the specified identifier is not running.
    HcsSystemAlreadyStopped,

    /// A communication protocol  has occurred between the virtual machine or container and the host.
    HcsProtocol,

    /// The container image contains a layer with an unrecognized format.
    HcsInvalidLayer,

    /// To use this container image, you must join the Windows Insider Program. Please see https://go.microsoft.com/fwlink/?linkid=850659 for more information.
    HcsWindowsInsiderRequired,

    /// The operation could not be started because a required feature is not installed.
    HcsServiceNotAvailable,

    /// The operation has not started.
    HcsOperationNotStarted,

    /// The operation is already running.
    HcsOperationAlreadyStarted,

    /// The operation is still running.
    HcsOperationPending,

    /// The operation did not complete in time.
    HcsOperationTimeout,

    /// An event callback has already been registered on this handle.
    HcsOperationSystemCallbackAlreadySet,

    /// Not enough memory available to return the result of the operation.
    HcsOperationResultAllocationFailed,

    /// Insufficient privileges. Only administrators or users that are members of the Hyper-V Administrators user group are permitted to access virtual machines or containers. To add yourself to the Hyper-V Administrators user group, please see https://aka.ms/admin for more information.
    HcsAccessDenied,

    /// The virtual machine or container reported a critical  and was stopped or restarted.
    HcsGuestCritical,

    /// A virtual switch with the given name was not found.
    ErrorVnetVirtualSwitchNameNotFound,

    /// A virtual machine is running with its memory allocated across multiple NUMA nodes. This does not indicate a problem unless the performance of your virtual machine is unusually slow. If you are experiencing performance problems, you may need to modify the NUMA configuration.
    ErrorVidRemoteNodeParentGpaPagesUsed,

    /// The specified capability does not exist.
    WhvUnknownCapability,

    /// The specified buffer is too small for the requested data.
    WhvInsufficientBuffer,

    /// The specified property does not exist.
    WhvUnknownProperty,

    /// The configuration of the hypervisor on this system is not supported.
    WhvUnsupportedHypervisorConfig,

    /// The configuration of the partition is not valid.
    WhvInvalidPartitionConfig,

    /// The specified GPA range was not found.
    WhvGpaRangeNotFound,

    /// A virtual processor with the specified index already exists.
    WhvVpAlreadyExists,

    /// A virtual processor with the specified index does not exist.
    WhvVpDoesNotExist,

    /// The virtual processor is not in the correct state to perform the requested operation.
    WhvInvalidVpState,

    /// A virtual processor register with the specified name does not exist.
    WhvInvalidVpRegisterName,

    /// Cannot restore this virtual machine because a file read from the vSMB saved state data could not be found. Delete the saved state data and then try to start the virtual machine.
    ErrorVsmbSavedStateFileNotFound,

    /// Cannot restore this virtual machine because the vSMB saved state data cannot be read. Delete the saved state data and then try to start the virtual machine.
    ErrorVsmbSavedStateCorrupt,

    /// The network was not found.
    HcnNetworkNotFound,

    /// The endpoint was not found.
    HcnEndpointNotFound,

    /// The network's underlying layer was not found.
    HcnLayerNotFound,

    /// The virtual switch was not found.
    HcnSwitchNotFound,

    /// The network does not have a subnet for this endpoint.
    HcnSubnetNotFound,

    /// An adapter was not found.
    HcnAdapterNotFound,

    /// The switch-port was not found.
    HcnPortNotFound,

    /// An expected policy was not found.
    HcnPolicyNotFound,

    /// A required VFP port setting was not found.
    HcnVfpPortsettingNotFound,

    /// The provided network configuration is invalid or missing parameters.
    HcnInvalidNetwork,

    /// Invalid network type.
    HcnInvalidNetworkType,

    /// The provided endpoint configuration is invalid or missing parameters.
    HcnInvalidEndpoint,

    /// The provided policy configuration is invalid or missing parameters.
    HcnInvalidPolicy,

    /// Invalid policy type.
    HcnInvalidPolicyType,

    /// This requested operation is invalid for a remote endpoint.
    HcnInvalidRemoteEndpointOperation,

    /// A network with this name already exists.
    HcnNetworkAlreadyExists,

    /// A network with this name already exists.
    HcnLayerAlreadyExists,

    /// Policy information already exists on this object.
    HcnPolicyAlreadyExists,

    /// The specified port already exists.
    HcnPortAlreadyExists,

    /// This endpoint is already attached to the switch.
    HcnEndpointAlreadyAttached,

    /// The specified request is unsupported.
    HcnRequestUnsupported,

    /// Port mapping is not supported on the given network.
    HcnMappingNotSupported,

    /// There was an operation attempted on a degraded object.
    HcnDegradedOperation,

    /// Cannot modify a switch shared by multiple networks.
    HcnSharedSwitchModification,

    /// Failed to interpret a parameter as a GUID.
    HcnGuidConversionFailure,

    /// Failed to process registry key.
    HcnRegkeyFailure,

    /// Invalid JSON document string.
    HcnInvalidJson,

    /// The reference is invalid in the JSON document.
    HcnInvalidJsonReference,

    /// Endpoint sharing is disabled.
    HcnEndpointSharingDisabled,

    /// IP address is either invalid or not part of any configured subnet(s).
    HcnInvalidIp,

    /// The specified switch extension does not exist on this switch.
    HcnSwitchExtensionNotFound,

    /// Operation cannot be performed while service is stopping.
    HcnManagerStopped,

    /// Operation cannot be performed while service module not found.
    GcnModuleNotFound,

    /// Request Handlers not present to handle the JSON request.
    GcnNoRequestHandlers,

    /// The specified request is unsupported.
    GcnRequestUnsupported,

    /// Add runtime keys to container failed.
    GcnRuntimekeysFailed,

    /// Timeout while waiting for network adapter with the given instance id
    GcnNetadapterTimeout,

    /// Network adapter not found for the given instance id
    GcnNetadapterNotFound,

    /// Network compartment not found for the given  id
    GcnNetcompartmentNotFound,

    /// Network interface not found for the given  id
    GcnNetinterfaceNotFound,

    /// Default Namespace already exists
    GcnDefaultnamespaceExists,

    UnknownHResult(HResult),
}

#[allow(overflowing_literals)]
pub(crate) fn hresult_to_result_code(hresult: &HResult) -> ResultCode {
    match hresult {
        0 => ResultCode::Success,
        0x8007000E => ResultCode::OutOfMemory,
        0x80070002 => ResultCode::FileNotFound,
        0x80004005 => ResultCode::Fail,
        0x80070057 => ResultCode::InvalidArgument,
        0x8000FFFF => ResultCode::Unexpected,
        0xC0350002 => ResultCode::HvInvalidHypercallCode,
        0xC0350003 => ResultCode::HvInvalidHypercallInput,
        0xC0350004 => ResultCode::HvInvalidAlignment,
        0xC0350005 => ResultCode::HvInvalidParameter,
        0xC0350006 => ResultCode::HvAccessDenied,
        0xC0350007 => ResultCode::HvInvalidPartitionState,
        0xC0350008 => ResultCode::HvOperationDenied,
        0xC0350009 => ResultCode::HvUnknownProperty,
        0xC035000A => ResultCode::HvPropertyValueOutOfRange,
        0xC035000B => ResultCode::HvInsufficientMemory,
        0xC035000C => ResultCode::HvPartitionTooDeep,
        0xC035000D => ResultCode::HvInvalidPartitionId,
        0xC035000E => ResultCode::HvInvalidVpIndex,
        0xC0350011 => ResultCode::HvInvalidPortId,
        0xC0350012 => ResultCode::HvInvalidConnectionId,
        0xC0350013 => ResultCode::HvInsufficientBuffers,
        0xC0350014 => ResultCode::HvNotAcknowledged,
        0xC0350015 => ResultCode::HvInvalidVpState,
        0xC0350016 => ResultCode::HvAcknowledged,
        0xC0350017 => ResultCode::HvInvalidSaveRestoreState,
        0xC0350018 => ResultCode::HvInvalidSynicState,
        0xC0350019 => ResultCode::HvObjectInUse,
        0xC035001A => ResultCode::HvInvalidProximityDomainInfo,
        0xC035001B => ResultCode::HvNoData,
        0xC035001C => ResultCode::HvInactive,
        0xC035001D => ResultCode::HvNoResources,
        0xC035001E => ResultCode::HvFeatureUnavailable,
        0xC0350033 => ResultCode::HvInsufficientBuffer,
        0xC0350038 => ResultCode::HvInsufficientDeviceDomains,
        0xC035003C => ResultCode::HvCpuidFeatureValidation,
        0xC035003D => ResultCode::HvCpuidXsaveFeatureValidation,
        0xC035003E => ResultCode::HvProcessorStartupTimeout,
        0xC035003F => ResultCode::HvSmxEnabled,
        0xC0350041 => ResultCode::HvInvalidLpIndex,
        0xC0350050 => ResultCode::HvInvalidRegisterValue,
        0xC0350051 => ResultCode::HvInvalidVtlState,
        0xC0350055 => ResultCode::HvNxNotDetected,
        0xC0350057 => ResultCode::HvInvalidDeviceId,
        0xC0350058 => ResultCode::HvInvalidDeviceState,
        0x00350059 => ResultCode::HvPendingPageRequests,
        0xC0350060 => ResultCode::HvPageRequestInvalid,
        0xC035006F => ResultCode::HvInvalidCpuGroupId,
        0xC0350070 => ResultCode::HvInvalidCpuGroupState,
        0xC0350071 => ResultCode::HvOperationFailed,
        0xC0350072 => ResultCode::HvNotAllowedWithNestedVirtActive,
        0xC0350073 => ResultCode::HvInsufficientRootMemory,
        0xC0351000 => ResultCode::HvNotPresent,
        0xC0370001 => ResultCode::VidDuplicateHandler,
        0xC0370002 => ResultCode::VidTooManyHandlers,
        0xC0370003 => ResultCode::VidQueueFull,
        0xC0370004 => ResultCode::VidHandlerNotPresent,
        0xC0370005 => ResultCode::VidInvalidObjectName,
        0xC0370006 => ResultCode::VidPartitionNameTooLong,
        0xC0370007 => ResultCode::VidMessageQueueNameTooLong,
        0xC0370008 => ResultCode::VidPartitionAlreadyExists,
        0xC0370009 => ResultCode::VidPartitionDoesNotExist,
        0xC037000A => ResultCode::VidPartitionNameNotFound,
        0xC037000B => ResultCode::VidMessageQueueAlreadyExists,
        0xC037000C => ResultCode::VidExceededMbpEntryMapLimit,
        0xC037000D => ResultCode::VidMbStillReferenced,
        0xC037000E => ResultCode::VidChildGpaPageSetCorrupted,
        0xC037000F => ResultCode::VidInvalidNumaSettings,
        0xC0370010 => ResultCode::VidInvalidNumaNodeIndex,
        0xC0370011 => ResultCode::VidNotificationQueueAlreadyAssociated,
        0xC0370012 => ResultCode::VidInvalidMemoryBlockHandle,
        0xC0370013 => ResultCode::VidPageRangeOverflow,
        0xC0370014 => ResultCode::VidInvalidMessageQueueHandle,
        0xC0370015 => ResultCode::VidInvalidGpaRangeHandle,
        0xC0370016 => ResultCode::VidNoMemoryBlockNotificationQueue,
        0xC0370017 => ResultCode::VidMemoryBlockLockCountExceeded,
        0xC0370018 => ResultCode::VidInvalidPpmHandle,
        0xC0370019 => ResultCode::VidMbpsAreLocked,
        0xC037001A => ResultCode::VidMessageQueueClosed,
        0xC037001B => ResultCode::VidVirtualProcessorLimitExceeded,
        0xC037001C => ResultCode::VidStopPending,
        0xC037001D => ResultCode::VidInvalidProcessorState,
        0xC037001E => ResultCode::VidExceededKmContextCountLimit,
        0xC037001F => ResultCode::VidKmInterfaceAlreadyInitialized,
        0xC0370020 => ResultCode::VidMbPropertyAlreadySetReset,
        0xC0370021 => ResultCode::VidMmioRangeDestroyed,
        0xC0370022 => ResultCode::VidInvalidChildGpaPageSet,
        0xC0370023 => ResultCode::VidReservePageSetIsBeingUsed,
        0xC0370024 => ResultCode::VidReservePageSetTooSmall,
        0xC0370025 => ResultCode::VidMbpAlreadyLockedUsingReservedPage,
        0xC0370026 => ResultCode::VidMbpCountExceededLimit,
        0xC0370027 => ResultCode::VidSavedStateCorrupt,
        0xC0370028 => ResultCode::VidSavedStateUnrecognizedItem,
        0xC0370029 => ResultCode::VidSavedStateIncompatible,
        0xC037002A => ResultCode::VidVtlAccessDenied,
        0xC0370100 => ResultCode::VmComputeTerminatedDuringStart,
        0xC0370101 => ResultCode::VmComputeImageMismatch,
        0xC0370102 => ResultCode::VmComputeHypervNotInstalled,
        0xC0370103 => ResultCode::VmComputeOperationPending,
        0xC0370104 => ResultCode::VmComputeTooManyNotifications,
        0xC0370105 => ResultCode::VmComputeInvalidState,
        0xC0370106 => ResultCode::VmComputeUnexpectedExit,
        0xC0370107 => ResultCode::VmComputeTerminated,
        0xC0370108 => ResultCode::VmComputeConnectFailed,
        0xC0370109 => ResultCode::VmComputeTimeout,
        0xC037010A => ResultCode::VmComputeConnectionClosed,
        0xC037010B => ResultCode::VmComputeUnknownMessage,
        0xC037010C => ResultCode::VmComputeUnsupportedProtocolVersion,
        0xC037010D => ResultCode::VmComputeInvalidJson,
        0xC037010E => ResultCode::VmComputeSystemNotFound,
        0xC037010F => ResultCode::VmComputeSystemAlreadyExists,
        0xC0370110 => ResultCode::VmComputeSystemAlreadyStopped,
        0xC0370111 => ResultCode::VmComputeProtocol,
        0xC0370112 => ResultCode::VmComputeInvalidLayer,
        0xC0370113 => ResultCode::VmComputeWindowsInsiderRequired,
        0x80370100 => ResultCode::HcsTerminatedDuringStart,
        0x80370101 => ResultCode::HcsImageMismatch,
        0x80370102 => ResultCode::HcsHypervNotInstalled,
        0x80370105 => ResultCode::HcsInvalidState,
        0x80370106 => ResultCode::HcsUnexpectedExit,
        0x80370107 => ResultCode::HcsTerminated,
        0x80370108 => ResultCode::HcsConnectFailed,
        0x80370109 => ResultCode::HcsConnectionTimeout,
        0x8037010A => ResultCode::HcsConnectionClosed,
        0x8037010B => ResultCode::HcsUnknownMessage,
        0x8037010C => ResultCode::HcsUnsupportedProtocolVersion,
        0x8037010D => ResultCode::HcsInvalidJson,
        0x8037010E => ResultCode::HcsSystemNotFound,
        0x8037010F => ResultCode::HcsSystemAlreadyExists,
        0x80370110 => ResultCode::HcsSystemAlreadyStopped,
        0x80370111 => ResultCode::HcsProtocol,
        0x80370112 => ResultCode::HcsInvalidLayer,
        0x80370113 => ResultCode::HcsWindowsInsiderRequired,
        0x80370114 => ResultCode::HcsServiceNotAvailable,
        0x80370115 => ResultCode::HcsOperationNotStarted,
        0x80370116 => ResultCode::HcsOperationAlreadyStarted,
        0x80370117 => ResultCode::HcsOperationPending,
        0x80370118 => ResultCode::HcsOperationTimeout,
        0x80370119 => ResultCode::HcsOperationSystemCallbackAlreadySet,
        0x8037011A => ResultCode::HcsOperationResultAllocationFailed,
        0x8037011B => ResultCode::HcsAccessDenied,
        0x8037011C => ResultCode::HcsGuestCritical,
        0xC0370200 => ResultCode::ErrorVnetVirtualSwitchNameNotFound,
        0x80370001 => ResultCode::ErrorVidRemoteNodeParentGpaPagesUsed,
        0x80370300 => ResultCode::WhvUnknownCapability,
        0x80370301 => ResultCode::WhvInsufficientBuffer,
        0x80370302 => ResultCode::WhvUnknownProperty,
        0x80370303 => ResultCode::WhvUnsupportedHypervisorConfig,
        0x80370304 => ResultCode::WhvInvalidPartitionConfig,
        0x80370305 => ResultCode::WhvGpaRangeNotFound,
        0x80370306 => ResultCode::WhvVpAlreadyExists,
        0x80370307 => ResultCode::WhvVpDoesNotExist,
        0x80370308 => ResultCode::WhvInvalidVpState,
        0x80370309 => ResultCode::WhvInvalidVpRegisterName,
        0xC0370400 => ResultCode::ErrorVsmbSavedStateFileNotFound,
        0xC0370401 => ResultCode::ErrorVsmbSavedStateCorrupt,
        0x803B0001 => ResultCode::HcnNetworkNotFound,
        0x803B0002 => ResultCode::HcnEndpointNotFound,
        0x803B0003 => ResultCode::HcnLayerNotFound,
        0x803B0004 => ResultCode::HcnSwitchNotFound,
        0x803B0005 => ResultCode::HcnSubnetNotFound,
        0x803B0006 => ResultCode::HcnAdapterNotFound,
        0x803B0007 => ResultCode::HcnPortNotFound,
        0x803B0008 => ResultCode::HcnPolicyNotFound,
        0x803B0009 => ResultCode::HcnVfpPortsettingNotFound,
        0x803B000A => ResultCode::HcnInvalidNetwork,
        0x803B000B => ResultCode::HcnInvalidNetworkType,
        0x803B000C => ResultCode::HcnInvalidEndpoint,
        0x803B000D => ResultCode::HcnInvalidPolicy,
        0x803B000E => ResultCode::HcnInvalidPolicyType,
        0x803B000F => ResultCode::HcnInvalidRemoteEndpointOperation,
        0x803B0010 => ResultCode::HcnNetworkAlreadyExists,
        0x803B0011 => ResultCode::HcnLayerAlreadyExists,
        0x803B0012 => ResultCode::HcnPolicyAlreadyExists,
        0x803B0013 => ResultCode::HcnPortAlreadyExists,
        0x803B0014 => ResultCode::HcnEndpointAlreadyAttached,
        0x803B0015 => ResultCode::HcnRequestUnsupported,
        0x803B0016 => ResultCode::HcnMappingNotSupported,
        0x803B0017 => ResultCode::HcnDegradedOperation,
        0x803B0018 => ResultCode::HcnSharedSwitchModification,
        0x803B0019 => ResultCode::HcnGuidConversionFailure,
        0x803B001A => ResultCode::HcnRegkeyFailure,
        0x803B001B => ResultCode::HcnInvalidJson,
        0x803B001C => ResultCode::HcnInvalidJsonReference,
        0x803B001D => ResultCode::HcnEndpointSharingDisabled,
        0x803B001E => ResultCode::HcnInvalidIp,
        0x803B001F => ResultCode::HcnSwitchExtensionNotFound,
        0x803B0020 => ResultCode::HcnManagerStopped,
        0x803B0021 => ResultCode::GcnModuleNotFound,
        0x803B0022 => ResultCode::GcnNoRequestHandlers,
        0x803B0023 => ResultCode::GcnRequestUnsupported,
        0x803B0024 => ResultCode::GcnRuntimekeysFailed,
        0x803B0025 => ResultCode::GcnNetadapterTimeout,
        0x803B0026 => ResultCode::GcnNetadapterNotFound,
        0x803B0027 => ResultCode::GcnNetcompartmentNotFound,
        0x803B0028 => ResultCode::GcnNetinterfaceNotFound,
        0x803B0029 => ResultCode::GcnDefaultnamespaceExists,
        other => ResultCode::UnknownHResult(*other),
    }
}

#[allow(overflowing_literals)]
#[allow(dead_code)]
pub(crate) fn result_code_to_hresult(result_code: ResultCode) -> HResult {
    match result_code {
        ResultCode::Success => 0,
        ResultCode::OutOfMemory => 0x8007000E,
        ResultCode::FileNotFound => 0x80070002,
        ResultCode::Fail => 0x80004005,
        ResultCode::InvalidArgument => 0x80070057,
        ResultCode::Unexpected => 0x8000FFFF,
        ResultCode::HvInvalidHypercallCode => 0xC0350002,
        ResultCode::HvInvalidHypercallInput => 0xC0350003,
        ResultCode::HvInvalidAlignment => 0xC0350004,
        ResultCode::HvInvalidParameter => 0xC0350005,
        ResultCode::HvAccessDenied => 0xC0350006,
        ResultCode::HvInvalidPartitionState => 0xC0350007,
        ResultCode::HvOperationDenied => 0xC0350008,
        ResultCode::HvUnknownProperty => 0xC0350009,
        ResultCode::HvPropertyValueOutOfRange => 0xC035000A,
        ResultCode::HvInsufficientMemory => 0xC035000B,
        ResultCode::HvPartitionTooDeep => 0xC035000C,
        ResultCode::HvInvalidPartitionId => 0xC035000D,
        ResultCode::HvInvalidVpIndex => 0xC035000E,
        ResultCode::HvInvalidPortId => 0xC0350011,
        ResultCode::HvInvalidConnectionId => 0xC0350012,
        ResultCode::HvInsufficientBuffers => 0xC0350013,
        ResultCode::HvNotAcknowledged => 0xC0350014,
        ResultCode::HvInvalidVpState => 0xC0350015,
        ResultCode::HvAcknowledged => 0xC0350016,
        ResultCode::HvInvalidSaveRestoreState => 0xC0350017,
        ResultCode::HvInvalidSynicState => 0xC0350018,
        ResultCode::HvObjectInUse => 0xC0350019,
        ResultCode::HvInvalidProximityDomainInfo => 0xC035001A,
        ResultCode::HvNoData => 0xC035001B,
        ResultCode::HvInactive => 0xC035001C,
        ResultCode::HvNoResources => 0xC035001D,
        ResultCode::HvFeatureUnavailable => 0xC035001E,
        ResultCode::HvInsufficientBuffer => 0xC0350033,
        ResultCode::HvInsufficientDeviceDomains => 0xC0350038,
        ResultCode::HvCpuidFeatureValidation => 0xC035003C,
        ResultCode::HvCpuidXsaveFeatureValidation => 0xC035003D,
        ResultCode::HvProcessorStartupTimeout => 0xC035003E,
        ResultCode::HvSmxEnabled => 0xC035003F,
        ResultCode::HvInvalidLpIndex => 0xC0350041,
        ResultCode::HvInvalidRegisterValue => 0xC0350050,
        ResultCode::HvInvalidVtlState => 0xC0350051,
        ResultCode::HvNxNotDetected => 0xC0350055,
        ResultCode::HvInvalidDeviceId => 0xC0350057,
        ResultCode::HvInvalidDeviceState => 0xC0350058,
        ResultCode::HvPendingPageRequests => 0x00350059,
        ResultCode::HvPageRequestInvalid => 0xC0350060,
        ResultCode::HvInvalidCpuGroupId => 0xC035006F,
        ResultCode::HvInvalidCpuGroupState => 0xC0350070,
        ResultCode::HvOperationFailed => 0xC0350071,
        ResultCode::HvNotAllowedWithNestedVirtActive => 0xC0350072,
        ResultCode::HvInsufficientRootMemory => 0xC0350073,
        ResultCode::HvNotPresent => 0xC0351000,
        ResultCode::VidDuplicateHandler => 0xC0370001,
        ResultCode::VidTooManyHandlers => 0xC0370002,
        ResultCode::VidQueueFull => 0xC0370003,
        ResultCode::VidHandlerNotPresent => 0xC0370004,
        ResultCode::VidInvalidObjectName => 0xC0370005,
        ResultCode::VidPartitionNameTooLong => 0xC0370006,
        ResultCode::VidMessageQueueNameTooLong => 0xC0370007,
        ResultCode::VidPartitionAlreadyExists => 0xC0370008,
        ResultCode::VidPartitionDoesNotExist => 0xC0370009,
        ResultCode::VidPartitionNameNotFound => 0xC037000A,
        ResultCode::VidMessageQueueAlreadyExists => 0xC037000B,
        ResultCode::VidExceededMbpEntryMapLimit => 0xC037000C,
        ResultCode::VidMbStillReferenced => 0xC037000D,
        ResultCode::VidChildGpaPageSetCorrupted => 0xC037000E,
        ResultCode::VidInvalidNumaSettings => 0xC037000F,
        ResultCode::VidInvalidNumaNodeIndex => 0xC0370010,
        ResultCode::VidNotificationQueueAlreadyAssociated => 0xC0370011,
        ResultCode::VidInvalidMemoryBlockHandle => 0xC0370012,
        ResultCode::VidPageRangeOverflow => 0xC0370013,
        ResultCode::VidInvalidMessageQueueHandle => 0xC0370014,
        ResultCode::VidInvalidGpaRangeHandle => 0xC0370015,
        ResultCode::VidNoMemoryBlockNotificationQueue => 0xC0370016,
        ResultCode::VidMemoryBlockLockCountExceeded => 0xC0370017,
        ResultCode::VidInvalidPpmHandle => 0xC0370018,
        ResultCode::VidMbpsAreLocked => 0xC0370019,
        ResultCode::VidMessageQueueClosed => 0xC037001A,
        ResultCode::VidVirtualProcessorLimitExceeded => 0xC037001B,
        ResultCode::VidStopPending => 0xC037001C,
        ResultCode::VidInvalidProcessorState => 0xC037001D,
        ResultCode::VidExceededKmContextCountLimit => 0xC037001E,
        ResultCode::VidKmInterfaceAlreadyInitialized => 0xC037001F,
        ResultCode::VidMbPropertyAlreadySetReset => 0xC0370020,
        ResultCode::VidMmioRangeDestroyed => 0xC0370021,
        ResultCode::VidInvalidChildGpaPageSet => 0xC0370022,
        ResultCode::VidReservePageSetIsBeingUsed => 0xC0370023,
        ResultCode::VidReservePageSetTooSmall => 0xC0370024,
        ResultCode::VidMbpAlreadyLockedUsingReservedPage => 0xC0370025,
        ResultCode::VidMbpCountExceededLimit => 0xC0370026,
        ResultCode::VidSavedStateCorrupt => 0xC0370027,
        ResultCode::VidSavedStateUnrecognizedItem => 0xC0370028,
        ResultCode::VidSavedStateIncompatible => 0xC0370029,
        ResultCode::VidVtlAccessDenied => 0xC037002A,
        ResultCode::VmComputeTerminatedDuringStart => 0xC0370100,
        ResultCode::VmComputeImageMismatch => 0xC0370101,
        ResultCode::VmComputeHypervNotInstalled => 0xC0370102,
        ResultCode::VmComputeOperationPending => 0xC0370103,
        ResultCode::VmComputeTooManyNotifications => 0xC0370104,
        ResultCode::VmComputeInvalidState => 0xC0370105,
        ResultCode::VmComputeUnexpectedExit => 0xC0370106,
        ResultCode::VmComputeTerminated => 0xC0370107,
        ResultCode::VmComputeConnectFailed => 0xC0370108,
        ResultCode::VmComputeTimeout => 0xC0370109,
        ResultCode::VmComputeConnectionClosed => 0xC037010A,
        ResultCode::VmComputeUnknownMessage => 0xC037010B,
        ResultCode::VmComputeUnsupportedProtocolVersion => 0xC037010C,
        ResultCode::VmComputeInvalidJson => 0xC037010D,
        ResultCode::VmComputeSystemNotFound => 0xC037010E,
        ResultCode::VmComputeSystemAlreadyExists => 0xC037010F,
        ResultCode::VmComputeSystemAlreadyStopped => 0xC0370110,
        ResultCode::VmComputeProtocol => 0xC0370111,
        ResultCode::VmComputeInvalidLayer => 0xC0370112,
        ResultCode::VmComputeWindowsInsiderRequired => 0xC0370113,
        ResultCode::HcsTerminatedDuringStart => 0x80370100,
        ResultCode::HcsImageMismatch => 0x80370101,
        ResultCode::HcsHypervNotInstalled => 0x80370102,
        ResultCode::HcsInvalidState => 0x80370105,
        ResultCode::HcsUnexpectedExit => 0x80370106,
        ResultCode::HcsTerminated => 0x80370107,
        ResultCode::HcsConnectFailed => 0x80370108,
        ResultCode::HcsConnectionTimeout => 0x80370109,
        ResultCode::HcsConnectionClosed => 0x8037010A,
        ResultCode::HcsUnknownMessage => 0x8037010B,
        ResultCode::HcsUnsupportedProtocolVersion => 0x8037010C,
        ResultCode::HcsInvalidJson => 0x8037010D,
        ResultCode::HcsSystemNotFound => 0x8037010E,
        ResultCode::HcsSystemAlreadyExists => 0x8037010F,
        ResultCode::HcsSystemAlreadyStopped => 0x80370110,
        ResultCode::HcsProtocol => 0x80370111,
        ResultCode::HcsInvalidLayer => 0x80370112,
        ResultCode::HcsWindowsInsiderRequired => 0x80370113,
        ResultCode::HcsServiceNotAvailable => 0x80370114,
        ResultCode::HcsOperationNotStarted => 0x80370115,
        ResultCode::HcsOperationAlreadyStarted => 0x80370116,
        ResultCode::HcsOperationPending => 0x80370117,
        ResultCode::HcsOperationTimeout => 0x80370118,
        ResultCode::HcsOperationSystemCallbackAlreadySet => 0x80370119,
        ResultCode::HcsOperationResultAllocationFailed => 0x8037011A,
        ResultCode::HcsAccessDenied => 0x8037011B,
        ResultCode::HcsGuestCritical => 0x8037011C,
        ResultCode::ErrorVnetVirtualSwitchNameNotFound => 0xC0370200,
        ResultCode::ErrorVidRemoteNodeParentGpaPagesUsed => 0x80370001,
        ResultCode::WhvUnknownCapability => 0x80370300,
        ResultCode::WhvInsufficientBuffer => 0x80370301,
        ResultCode::WhvUnknownProperty => 0x80370302,
        ResultCode::WhvUnsupportedHypervisorConfig => 0x80370303,
        ResultCode::WhvInvalidPartitionConfig => 0x80370304,
        ResultCode::WhvGpaRangeNotFound => 0x80370305,
        ResultCode::WhvVpAlreadyExists => 0x80370306,
        ResultCode::WhvVpDoesNotExist => 0x80370307,
        ResultCode::WhvInvalidVpState => 0x80370308,
        ResultCode::WhvInvalidVpRegisterName => 0x80370309,
        ResultCode::ErrorVsmbSavedStateFileNotFound => 0xC0370400,
        ResultCode::ErrorVsmbSavedStateCorrupt => 0xC0370401,
        ResultCode::HcnNetworkNotFound => 0x803B0001,
        ResultCode::HcnEndpointNotFound => 0x803B0002,
        ResultCode::HcnLayerNotFound => 0x803B0003,
        ResultCode::HcnSwitchNotFound => 0x803B0004,
        ResultCode::HcnSubnetNotFound => 0x803B0005,
        ResultCode::HcnAdapterNotFound => 0x803B0006,
        ResultCode::HcnPortNotFound => 0x803B0007,
        ResultCode::HcnPolicyNotFound => 0x803B0008,
        ResultCode::HcnVfpPortsettingNotFound => 0x803B0009,
        ResultCode::HcnInvalidNetwork => 0x803B000A,
        ResultCode::HcnInvalidNetworkType => 0x803B000B,
        ResultCode::HcnInvalidEndpoint => 0x803B000C,
        ResultCode::HcnInvalidPolicy => 0x803B000D,
        ResultCode::HcnInvalidPolicyType => 0x803B000E,
        ResultCode::HcnInvalidRemoteEndpointOperation => 0x803B000F,
        ResultCode::HcnNetworkAlreadyExists => 0x803B0010,
        ResultCode::HcnLayerAlreadyExists => 0x803B0011,
        ResultCode::HcnPolicyAlreadyExists => 0x803B0012,
        ResultCode::HcnPortAlreadyExists => 0x803B0013,
        ResultCode::HcnEndpointAlreadyAttached => 0x803B0014,
        ResultCode::HcnRequestUnsupported => 0x803B0015,
        ResultCode::HcnMappingNotSupported => 0x803B0016,
        ResultCode::HcnDegradedOperation => 0x803B0017,
        ResultCode::HcnSharedSwitchModification => 0x803B0018,
        ResultCode::HcnGuidConversionFailure => 0x803B0019,
        ResultCode::HcnRegkeyFailure => 0x803B001A,
        ResultCode::HcnInvalidJson => 0x803B001B,
        ResultCode::HcnInvalidJsonReference => 0x803B001C,
        ResultCode::HcnEndpointSharingDisabled => 0x803B001D,
        ResultCode::HcnInvalidIp => 0x803B001E,
        ResultCode::HcnSwitchExtensionNotFound => 0x803B001F,
        ResultCode::HcnManagerStopped => 0x803B0020,
        ResultCode::GcnModuleNotFound => 0x803B0021,
        ResultCode::GcnNoRequestHandlers => 0x803B0022,
        ResultCode::GcnRequestUnsupported => 0x803B0023,
        ResultCode::GcnRuntimekeysFailed => 0x803B0024,
        ResultCode::GcnNetadapterTimeout => 0x803B0025,
        ResultCode::GcnNetadapterNotFound => 0x803B0026,
        ResultCode::GcnNetcompartmentNotFound => 0x803B0027,
        ResultCode::GcnNetinterfaceNotFound => 0x803B0028,
        ResultCode::GcnDefaultnamespaceExists => 0x803B0029,
        ResultCode::UnknownHResult(other) => other,
    }
}
