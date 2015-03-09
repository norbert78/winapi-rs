// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Header file for the Service Control Manager
//80
pub const SERVICE_NO_CHANGE: ::DWORD = 0xffffffff;
pub const SERVICE_ACTIVE: ::DWORD = 0x00000001;
pub const SERVICE_INACTIVE: ::DWORD = 0x00000002;
pub const SERVICE_STATE_ALL: ::DWORD = SERVICE_ACTIVE | SERVICE_INACTIVE;
pub const SERVICE_CONTROL_STOP: ::DWORD = 0x00000001;
pub const SERVICE_CONTROL_PAUSE: ::DWORD = 0x00000002;
pub const SERVICE_CONTROL_CONTINUE: ::DWORD = 0x00000003;
pub const SERVICE_CONTROL_INTERROGATE: ::DWORD = 0x00000004;
pub const SERVICE_CONTROL_SHUTDOWN: ::DWORD = 0x00000005;
pub const SERVICE_CONTROL_PARAMCHANGE: ::DWORD = 0x00000006;
pub const SERVICE_CONTROL_NETBINDADD: ::DWORD = 0x00000007;
pub const SERVICE_CONTROL_NETBINDREMOVE: ::DWORD = 0x00000008;
pub const SERVICE_CONTROL_NETBINDENABLE: ::DWORD = 0x00000009;
pub const SERVICE_CONTROL_NETBINDDISABLE: ::DWORD = 0x0000000A;
pub const SERVICE_CONTROL_DEVICEEVENT: ::DWORD = 0x0000000B;
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: ::DWORD = 0x0000000C;
pub const SERVICE_CONTROL_POWEREVENT: ::DWORD = 0x0000000D;
pub const SERVICE_CONTROL_SESSIONCHANGE: ::DWORD = 0x0000000E;
pub const SERVICE_CONTROL_PRESHUTDOWN: ::DWORD = 0x0000000F;
pub const SERVICE_CONTROL_TIMECHANGE: ::DWORD = 0x00000010;
pub const SERVICE_CONTROL_TRIGGEREVENT: ::DWORD = 0x00000020;
pub const SERVICE_STOPPED: ::DWORD = 0x00000001;
pub const SERVICE_START_PENDING: ::DWORD = 0x00000002;
pub const SERVICE_STOP_PENDING: ::DWORD = 0x00000003;
pub const SERVICE_RUNNING: ::DWORD = 0x00000004;
pub const SERVICE_CONTINUE_PENDING: ::DWORD = 0x00000005;
pub const SERVICE_PAUSE_PENDING: ::DWORD = 0x00000006;
pub const SERVICE_PAUSED: ::DWORD = 0x00000007;
pub const SERVICE_ACCEPT_STOP: ::DWORD = 0x00000001;
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: ::DWORD = 0x00000002;
pub const SERVICE_ACCEPT_SHUTDOWN: ::DWORD = 0x00000004;
pub const SERVICE_ACCEPT_PARAMCHANGE: ::DWORD = 0x00000008;
pub const SERVICE_ACCEPT_NETBINDCHANGE: ::DWORD = 0x00000010;
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: ::DWORD = 0x00000020;
pub const SERVICE_ACCEPT_POWEREVENT: ::DWORD = 0x00000040;
pub const SERVICE_ACCEPT_SESSIONCHANGE: ::DWORD = 0x00000080;
pub const SERVICE_ACCEPT_PRESHUTDOWN: ::DWORD = 0x00000100;
pub const SERVICE_ACCEPT_TIMECHANGE: ::DWORD = 0x00000200;
pub const SERVICE_ACCEPT_TRIGGEREVENT: ::DWORD = 0x00000400;
pub const SC_MANAGER_CONNECT: ::DWORD = 0x0001;
pub const SC_MANAGER_CREATE_SERVICE: ::DWORD = 0x0002;
pub const SC_MANAGER_ENUMERATE_SERVICE: ::DWORD = 0x0004;
pub const SC_MANAGER_LOCK: ::DWORD = 0x0008;
pub const SC_MANAGER_QUERY_LOCK_STATUS: ::DWORD = 0x0010;
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: ::DWORD = 0x0020;
pub const SC_MANAGER_ALL_ACCESS: ::DWORD = ::STANDARD_RIGHTS_REQUIRED | SC_MANAGER_CONNECT
    | SC_MANAGER_CREATE_SERVICE | SC_MANAGER_ENUMERATE_SERVICE | SC_MANAGER_LOCK
    | SC_MANAGER_QUERY_LOCK_STATUS | SC_MANAGER_MODIFY_BOOT_CONFIG;
pub const SERVICE_QUERY_CONFIG: ::DWORD = 0x0001;
pub const SERVICE_CHANGE_CONFIG: ::DWORD = 0x0002;
pub const SERVICE_QUERY_STATUS: ::DWORD = 0x0004;
pub const SERVICE_ENUMERATE_DEPENDENTS: ::DWORD = 0x0008;
pub const SERVICE_START: ::DWORD = 0x0010;
pub const SERVICE_STOP: ::DWORD = 0x0020;
pub const SERVICE_PAUSE_CONTINUE: ::DWORD = 0x0040;
pub const SERVICE_INTERROGATE: ::DWORD = 0x0080;
pub const SERVICE_USER_DEFINED_CONTROL: ::DWORD = 0x0100;
pub const SERVICE_ALL_ACCESS: ::DWORD = ::STANDARD_RIGHTS_REQUIRED | SERVICE_QUERY_CONFIG
    | SERVICE_CHANGE_CONFIG | SERVICE_QUERY_STATUS | SERVICE_ENUMERATE_DEPENDENTS | SERVICE_START
    | SERVICE_STOP | SERVICE_PAUSE_CONTINUE | SERVICE_INTERROGATE | SERVICE_USER_DEFINED_CONTROL;
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: ::DWORD = 0x00000001;
pub const SERVICE_CONFIG_DESCRIPTION: ::DWORD = 1;
pub const SERVICE_CONFIG_FAILURE_ACTIONS: ::DWORD = 2;
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: ::DWORD = 3;
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: ::DWORD = 4;
pub const SERVICE_CONFIG_SERVICE_SID_INFO: ::DWORD = 5;
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: ::DWORD = 6;
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: ::DWORD = 7;
pub const SERVICE_CONFIG_TRIGGER_INFO: ::DWORD = 8;
pub const SERVICE_CONFIG_PREFERRED_NODE: ::DWORD = 9;
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: ::DWORD = 12;
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: ::DWORD = 1;
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: ::DWORD = 2;
pub const SERVICE_NOTIFY_STATUS_CHANGE: ::DWORD = SERVICE_NOTIFY_STATUS_CHANGE_2;
pub const SERVICE_NOTIFY_STOPPED: ::DWORD = 0x00000001;
pub const SERVICE_NOTIFY_START_PENDING: ::DWORD = 0x00000002;
pub const SERVICE_NOTIFY_STOP_PENDING: ::DWORD = 0x00000004;
pub const SERVICE_NOTIFY_RUNNING: ::DWORD = 0x00000008;
pub const SERVICE_NOTIFY_CONTINUE_PENDING: ::DWORD = 0x00000010;
pub const SERVICE_NOTIFY_PAUSE_PENDING: ::DWORD = 0x00000020;
pub const SERVICE_NOTIFY_PAUSED: ::DWORD = 0x00000040;
pub const SERVICE_NOTIFY_CREATED: ::DWORD = 0x00000080;
pub const SERVICE_NOTIFY_DELETED: ::DWORD = 0x00000100;
pub const SERVICE_NOTIFY_DELETE_PENDING: ::DWORD = 0x00000200;
pub const SERVICE_STOP_REASON_FLAG_MIN: ::DWORD = 0x00000000;
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: ::DWORD = 0x10000000;
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: ::DWORD = 0x20000000;
pub const SERVICE_STOP_REASON_FLAG_PLANNED: ::DWORD = 0x40000000;
pub const SERVICE_STOP_REASON_FLAG_MAX: ::DWORD = 0x80000000;
pub const SERVICE_STOP_REASON_MAJOR_MIN: ::DWORD = 0x00000000;
pub const SERVICE_STOP_REASON_MAJOR_OTHER: ::DWORD = 0x00010000;
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: ::DWORD = 0x00020000;
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: ::DWORD = 0x00030000;
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: ::DWORD = 0x00040000;
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: ::DWORD = 0x00050000;
pub const SERVICE_STOP_REASON_MAJOR_NONE: ::DWORD = 0x00060000;
pub const SERVICE_STOP_REASON_MAJOR_MAX: ::DWORD = 0x00070000;
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: ::DWORD = 0x00400000;
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: ::DWORD = 0x00ff0000;
pub const SERVICE_STOP_REASON_MINOR_MIN: ::DWORD = 0x00000000;
pub const SERVICE_STOP_REASON_MINOR_OTHER: ::DWORD = 0x00000001;
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: ::DWORD = 0x00000002;
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: ::DWORD = 0x00000003;
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: ::DWORD = 0x00000004;
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: ::DWORD = 0x00000005;
pub const SERVICE_STOP_REASON_MINOR_HUNG: ::DWORD = 0x00000006;
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: ::DWORD = 0x00000007;
pub const SERVICE_STOP_REASON_MINOR_DISK: ::DWORD = 0x00000008;
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: ::DWORD = 0x00000009;
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: ::DWORD = 0x0000000a;
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: ::DWORD = 0x0000000b;
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: ::DWORD = 0x0000000c;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: ::DWORD = 0x0000000d;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: ::DWORD = 0x0000000e;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: ::DWORD = 0x0000000f;
pub const SERVICE_STOP_REASON_MINOR_SECURITY: ::DWORD = 0x00000010;
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: ::DWORD = 0x00000011;
pub const SERVICE_STOP_REASON_MINOR_WMI: ::DWORD = 0x00000012;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: ::DWORD = 0x00000013;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: ::DWORD = 0x00000014;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: ::DWORD = 0x00000015;
pub const SERVICE_STOP_REASON_MINOR_MMC: ::DWORD = 0x00000016;
pub const SERVICE_STOP_REASON_MINOR_NONE: ::DWORD = 0x00000017;
pub const SERVICE_STOP_REASON_MINOR_MAX: ::DWORD = 0x00000018;
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: ::DWORD = 0x00000100;
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: ::DWORD = 0x0000FFFF;
pub const SERVICE_CONTROL_STATUS_REASON_INFO: ::DWORD = 1;
pub const SERVICE_SID_TYPE_NONE: ::DWORD = 0x00000000;
pub const SERVICE_SID_TYPE_UNRESTRICTED: ::DWORD = 0x00000001;
pub const SERVICE_SID_TYPE_RESTRICTED: ::DWORD = 0x00000002 | SERVICE_SID_TYPE_UNRESTRICTED;
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: ::DWORD = 1;
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: ::DWORD = 2;
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: ::DWORD = 3;
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: ::DWORD = 4;
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: ::DWORD = 5;
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: ::DWORD = 6;
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: ::DWORD = 7;
pub const SERVICE_TRIGGER_TYPE_CUSTOM: ::DWORD = 20;
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: ::DWORD = 1;
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: ::DWORD = 2;
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: ::DWORD = 3;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: ::DWORD = 4;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: ::DWORD = 5;
pub const SERVICE_START_REASON_DEMAND: ::DWORD = 0x00000001;
pub const SERVICE_START_REASON_AUTO: ::DWORD = 0x00000002;
pub const SERVICE_START_REASON_TRIGGER: ::DWORD = 0x00000004;
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: ::DWORD = 0x00000008;
pub const SERVICE_START_REASON_DELAYEDAUTO: ::DWORD = 0x00000010;
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: ::DWORD = 1;
pub const SERVICE_LAUNCH_PROTECTED_NONE: ::DWORD = 0;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: ::DWORD = 1;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: ::DWORD = 2;
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: ::DWORD = 3;
//678
DECLARE_HANDLE!(SC_HANDLE, SC_HANDLE__);
pub type LPSC_HANDLE = *mut SC_HANDLE;
DECLARE_HANDLE!(SERVICE_STATUS_HANDLE, SERVICE_STATUS_HANDLE__);
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum SC_STATUS_TYPE {
    SC_STATUS_PROCESS_INFO = 0,
    __, // FIXME - Univariant enums
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum _SC_ENUM_TYPE {
    SC_ENUM_PROCESS_INFO = 0,
    __, // FIXME - Univariant enums
}
//700
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SERVICE_STATUS {
    pub dwServiceType: ::DWORD,
    pub dwCurrentState: ::DWORD,
    pub dwControlsAccepted: ::DWORD,
    pub dwWin32ExitCode: ::DWORD,
    pub dwServiceSpecificExitCode: ::DWORD,
    pub dwCheckPoint: ::DWORD,
    pub dwWaitHint: ::DWORD,
}
pub type LPSERVICE_STATUS = *mut SERVICE_STATUS;
//848
pub type LPSERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(
    dwNumServicesArgs: ::DWORD, lpServiceArgVectors: *mut ::LPWSTR,
)>;
pub type LPSERVICE_MAIN_FUNCTIONA = Option<unsafe extern "system" fn(
    dwNumServicesArgs: ::DWORD, lpServiceArgVectors: *mut ::LPSTR,
)>;
#[repr(C)] #[derive(Copy)]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: ::LPCSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}
pub type LPSERVICE_TABLE_ENTRYA = *mut SERVICE_TABLE_ENTRYA;
#[repr(C)] #[derive(Copy)]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: ::LPCWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}
pub type LPSERVICE_TABLE_ENTRYW = *mut SERVICE_TABLE_ENTRYW;
//900
pub type LPHANDLER_FUNCTION = Option<unsafe extern "system" fn(dwControl: ::DWORD)>;
pub type LPHANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(
    dwControl: ::DWORD, dwEventType: ::DWORD, lpEventData: ::LPVOID, lpContext: ::LPVOID,
) -> ::DWORD>;
