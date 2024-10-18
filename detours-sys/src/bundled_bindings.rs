use std::os::raw::c_void;

use windows_sys::{
    core::{GUID, PCSTR, PCWSTR, PSTR, PWSTR},
    Win32::{
        Foundation::{BOOL, HANDLE, HINSTANCE, HMODULE, HWND},
        Security::SECURITY_ATTRIBUTES,
        System::Threading::{PROCESS_INFORMATION, STARTUPINFOA, STARTUPINFOW},
    },
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DETOUR_TRAMPOLINE {
    _unused: [u8; 0],
}
pub type PDETOUR_TRAMPOLINE = *mut _DETOUR_TRAMPOLINE;
#[doc = " Binary Typedefs."]
pub type PF_DETOUR_BINARY_BYWAY_CALLBACK = ::std::option::Option<
    unsafe extern "system" fn(
        pContext: *mut c_void,
        pszFile: PCSTR,
        ppszOutFile: *mut PCSTR,
    ) -> BOOL,
>;
pub type PF_DETOUR_BINARY_FILE_CALLBACK = ::std::option::Option<
    unsafe extern "system" fn(
        pContext: *mut c_void,
        pszOrigFile: PCSTR,
        pszFile: PCSTR,
        ppszOutFile: *mut PCSTR,
    ) -> BOOL,
>;
pub type PF_DETOUR_BINARY_SYMBOL_CALLBACK = ::std::option::Option<
    unsafe extern "system" fn(
        pContext: *mut c_void,
        nOrigOrdinal: u32,
        nOrdinal: u32,
        pnOutOrdinal: *mut u32,
        pszOrigSymbol: PCSTR,
        pszSymbol: PCSTR,
        ppszOutSymbol: *mut PCSTR,
    ) -> BOOL,
>;
pub type PF_DETOUR_BINARY_COMMIT_CALLBACK =
    ::std::option::Option<unsafe extern "system" fn(pContext: *mut c_void) -> BOOL>;
pub type PF_DETOUR_ENUMERATE_EXPORT_CALLBACK = ::std::option::Option<
    unsafe extern "system" fn(
        pContext: *mut c_void,
        nOrdinal: u32,
        pszName: PCSTR,
        pCode: *mut c_void,
    ) -> BOOL,
>;
pub type PF_DETOUR_IMPORT_FILE_CALLBACK = ::std::option::Option<
    unsafe extern "system" fn(pContext: *mut c_void, hModule: HMODULE, pszFile: PCSTR) -> BOOL,
>;
pub type PF_DETOUR_IMPORT_FUNC_CALLBACK = ::std::option::Option<
    unsafe extern "system" fn(
        pContext: *mut c_void,
        nOrdinal: u32,
        pszFunc: PCSTR,
        pvFunc: *mut c_void,
    ) -> BOOL,
>;
pub type PF_DETOUR_IMPORT_FUNC_CALLBACK_EX = ::std::option::Option<
    unsafe extern "system" fn(
        pContext: *mut c_void,
        nOrdinal: u32,
        pszFunc: PCSTR,
        ppvFunc: *mut *mut c_void,
    ) -> BOOL,
>;
pub type PDETOUR_BINARY = *mut ::std::os::raw::c_void;
extern "system" {
    #[doc = " Transaction APIs."]
    pub fn DetourTransactionBegin() -> i32;
}
extern "system" {
    pub fn DetourTransactionAbort() -> i32;
}
extern "system" {
    pub fn DetourTransactionCommit() -> i32;
}
extern "system" {
    pub fn DetourTransactionCommitEx(pppFailedPointer: *mut *mut *mut c_void) -> i32;
}
extern "system" {
    pub fn DetourUpdateThread(hThread: HANDLE) -> i32;
}
extern "system" {
    pub fn DetourAttach(ppPointer: *mut *mut c_void, pDetour: *mut c_void) -> i32;
}
extern "system" {
    pub fn DetourAttachEx(
        ppPointer: *mut *mut c_void,
        pDetour: *mut c_void,
        ppRealTrampoline: *mut PDETOUR_TRAMPOLINE,
        ppRealTarget: *mut *mut c_void,
        ppRealDetour: *mut *mut c_void,
    ) -> i32;
}
extern "system" {
    pub fn DetourDetach(ppPointer: *mut *mut c_void, pDetour: *mut c_void) -> i32;
}
extern "system" {
    pub fn DetourSetIgnoreTooSmall(fIgnore: BOOL) -> BOOL;
}
extern "system" {
    pub fn DetourSetRetainRegions(fRetain: BOOL) -> BOOL;
}
extern "system" {
    pub fn DetourSetSystemRegionLowerBound(pSystemRegionLowerBound: *mut c_void) -> *mut c_void;
}
extern "system" {
    pub fn DetourSetSystemRegionUpperBound(pSystemRegionUpperBound: *mut c_void) -> *mut c_void;
}
extern "system" {
    #[doc = " Code Functions."]
    pub fn DetourFindFunction(pszModule: PCSTR, pszFunction: PCSTR) -> *mut c_void;
}
extern "system" {
    pub fn DetourCodeFromPointer(pPointer: *mut c_void, ppGlobals: *mut *mut c_void)
        -> *mut c_void;
}
extern "system" {
    #[doc = " Loaded Binary Functions."]
    pub fn DetourGetContainingModule(pvAddr: *mut c_void) -> HMODULE;
}
extern "system" {
    pub fn DetourEnumerateModules(hModuleLast: HMODULE) -> HMODULE;
}
extern "system" {
    pub fn DetourGetEntryPoint(hModule: HMODULE) -> *mut c_void;
}
extern "system" {
    pub fn DetourGetModuleSize(hModule: HMODULE) -> u32;
}
extern "system" {
    pub fn DetourEnumerateExports(
        hModule: HMODULE,
        pContext: *mut c_void,
        pfExport: PF_DETOUR_ENUMERATE_EXPORT_CALLBACK,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourEnumerateImports(
        hModule: HMODULE,
        pContext: *mut c_void,
        pfImportFile: PF_DETOUR_IMPORT_FILE_CALLBACK,
        pfImportFunc: PF_DETOUR_IMPORT_FUNC_CALLBACK,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourEnumerateImportsEx(
        hModule: HMODULE,
        pContext: *mut c_void,
        pfImportFile: PF_DETOUR_IMPORT_FILE_CALLBACK,
        pfImportFuncEx: PF_DETOUR_IMPORT_FUNC_CALLBACK_EX,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourFindPayload(
        hModule: HMODULE,
        rguid: *const GUID,
        pcbData: *mut u32,
    ) -> *mut c_void;
}
extern "system" {
    pub fn DetourGetSizeOfPayloads(hModule: HMODULE) -> u32;
}
extern "system" {
    #[doc = " Persistent Binary Functions."]
    pub fn DetourBinaryOpen(hFile: HANDLE) -> PDETOUR_BINARY;
}
extern "system" {
    pub fn DetourBinaryEnumeratePayloads(
        pBinary: PDETOUR_BINARY,
        pGuid: *mut GUID,
        pcbData: *mut u32,
        pnIterator: *mut u32,
    ) -> *mut c_void;
}
extern "system" {
    pub fn DetourBinaryFindPayload(
        pBinary: PDETOUR_BINARY,
        rguid: *const GUID,
        pcbData: *mut u32,
    ) -> *mut c_void;
}
extern "system" {
    pub fn DetourBinarySetPayload(
        pBinary: PDETOUR_BINARY,
        rguid: *const GUID,
        pData: *mut c_void,
        cbData: u32,
    ) -> *mut c_void;
}
extern "system" {
    pub fn DetourBinaryDeletePayload(pBinary: PDETOUR_BINARY, rguid: *const GUID) -> BOOL;
}
extern "system" {
    pub fn DetourBinaryPurgePayloads(pBinary: PDETOUR_BINARY) -> BOOL;
}
extern "system" {
    pub fn DetourBinaryResetImports(pBinary: PDETOUR_BINARY) -> BOOL;
}
extern "system" {
    pub fn DetourBinaryEditImports(
        pBinary: PDETOUR_BINARY,
        pContext: *mut c_void,
        pfByway: PF_DETOUR_BINARY_BYWAY_CALLBACK,
        pfFile: PF_DETOUR_BINARY_FILE_CALLBACK,
        pfSymbol: PF_DETOUR_BINARY_SYMBOL_CALLBACK,
        pfCommit: PF_DETOUR_BINARY_COMMIT_CALLBACK,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourBinaryWrite(pBinary: PDETOUR_BINARY, hFile: HANDLE) -> BOOL;
}
extern "system" {
    pub fn DetourBinaryClose(pBinary: PDETOUR_BINARY) -> BOOL;
}
pub type PDETOUR_CREATE_PROCESS_ROUTINEA = ::std::option::Option<
    unsafe extern "system" fn(
        lpApplicationName: PCSTR,
        lpCommandLine: PSTR,
        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
        lpThreadAttributes: *mut SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: u32,
        lpEnvironment: *mut c_void,
        lpCurrentDirectory: PCSTR,
        lpStartupInfo: *mut STARTUPINFOA,
        lpProcessInformation: *mut PROCESS_INFORMATION,
    ) -> BOOL,
>;
pub type PDETOUR_CREATE_PROCESS_ROUTINEW = ::std::option::Option<
    unsafe extern "system" fn(
        lpApplicationName: PCWSTR,
        lpCommandLine: PWSTR,
        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
        lpThreadAttributes: *mut SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: u32,
        lpEnvironment: *mut c_void,
        lpCurrentDirectory: PCWSTR,
        lpStartupInfo: *mut STARTUPINFOW,
        lpProcessInformation: *mut PROCESS_INFORMATION,
    ) -> BOOL,
>;
extern "system" {
    pub fn DetourCreateProcessWithDllA(
        lpApplicationName: PCSTR,
        lpCommandLine: PSTR,
        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
        lpThreadAttributes: *mut SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: u32,
        lpEnvironment: *mut c_void,
        lpCurrentDirectory: PCSTR,
        lpStartupInfo: *mut STARTUPINFOA,
        lpProcessInformation: *mut PROCESS_INFORMATION,
        lpDllName: PCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourCreateProcessWithDllW(
        lpApplicationName: PCWSTR,
        lpCommandLine: PWSTR,
        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
        lpThreadAttributes: *mut SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: u32,
        lpEnvironment: *mut c_void,
        lpCurrentDirectory: PCWSTR,
        lpStartupInfo: *mut STARTUPINFOW,
        lpProcessInformation: *mut PROCESS_INFORMATION,
        lpDllName: PCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourCreateProcessWithDllExA(
        lpApplicationName: PCSTR,
        lpCommandLine: PSTR,
        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
        lpThreadAttributes: *mut SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: u32,
        lpEnvironment: *mut c_void,
        lpCurrentDirectory: PCSTR,
        lpStartupInfo: *mut STARTUPINFOA,
        lpProcessInformation: *mut PROCESS_INFORMATION,
        lpDllName: PCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourCreateProcessWithDllExW(
        lpApplicationName: PCWSTR,
        lpCommandLine: PWSTR,
        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
        lpThreadAttributes: *mut SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: u32,
        lpEnvironment: *mut c_void,
        lpCurrentDirectory: PCWSTR,
        lpStartupInfo: *mut STARTUPINFOW,
        lpProcessInformation: *mut PROCESS_INFORMATION,
        lpDllName: PCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourCreateProcessWithDllsA(
        lpApplicationName: PCSTR,
        lpCommandLine: PSTR,
        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
        lpThreadAttributes: *mut SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: u32,
        lpEnvironment: *mut c_void,
        lpCurrentDirectory: PCSTR,
        lpStartupInfo: *mut STARTUPINFOA,
        lpProcessInformation: *mut PROCESS_INFORMATION,
        nDlls: u32,
        rlpDlls: *mut PCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourCreateProcessWithDllsW(
        lpApplicationName: PCWSTR,
        lpCommandLine: PWSTR,
        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
        lpThreadAttributes: *mut SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: u32,
        lpEnvironment: *mut c_void,
        lpCurrentDirectory: PCWSTR,
        lpStartupInfo: *mut STARTUPINFOW,
        lpProcessInformation: *mut PROCESS_INFORMATION,
        nDlls: u32,
        rlpDlls: *mut PCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourCopyPayloadToProcess(
        hProcess: HANDLE,
        rguid: *const GUID,
        pvData: *const c_void,
        cbData: u32,
    ) -> BOOL;
}
extern "system" {
    pub fn DetourRestoreAfterWith() -> BOOL;
}
extern "system" {
    pub fn DetourIsHelperProcess() -> BOOL;
}
extern "system" {
    pub fn DetourFinishHelperProcess(arg1: HWND, arg2: HINSTANCE, arg3: PSTR, arg4: i32);
}
