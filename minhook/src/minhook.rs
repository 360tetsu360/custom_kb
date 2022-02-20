use winapi::{LPCSTR, LPCWSTR, LPVOID};

pub const MH_STATUS_MH_UNKNOWN: MH_STATUS = -1;
pub const MH_STATUS_MH_OK: MH_STATUS = 0;
pub const MH_STATUS_MH_ERROR_ALREADY_INITIALIZED: MH_STATUS = 1;
pub const MH_STATUS_MH_ERROR_NOT_INITIALIZED: MH_STATUS = 2;
pub const MH_STATUS_MH_ERROR_ALREADY_CREATED: MH_STATUS = 3;
pub const MH_STATUS_MH_ERROR_NOT_CREATED: MH_STATUS = 4;
pub const MH_STATUS_MH_ERROR_ENABLED: MH_STATUS = 5;
pub const MH_STATUS_MH_ERROR_DISABLED: MH_STATUS = 6;
pub const MH_STATUS_MH_ERROR_NOT_EXECUTABLE: MH_STATUS = 7;
pub const MH_STATUS_MH_ERROR_UNSUPPORTED_FUNCTION: MH_STATUS = 8;
pub const MH_STATUS_MH_ERROR_MEMORY_ALLOC: MH_STATUS = 9;
pub const MH_STATUS_MH_ERROR_MEMORY_PROTECT: MH_STATUS = 10;
pub const MH_STATUS_MH_ERROR_MODULE_NOT_FOUND: MH_STATUS = 11;
pub const MH_STATUS_MH_ERROR_FUNCTION_NOT_FOUND: MH_STATUS = 12;

#[allow(non_camel_case_types)]
pub type MH_STATUS = ::std::os::raw::c_int;

#[link(name = "libMinHook", kind = "static")]
extern "C" {
    pub fn MH_Initialize() -> MH_STATUS;

    pub fn MH_Uninitialize() -> MH_STATUS;

    pub fn MH_CreateHook(pTarget: LPVOID, pDetour: LPVOID, ppOriginal: *mut LPVOID) -> MH_STATUS;

    pub fn MH_CreateHookApi(
        pszModule: LPCWSTR,
        pszProcName: LPCSTR,
        pDetour: LPVOID,
        ppOriginal: *mut LPVOID,
    ) -> MH_STATUS;

    pub fn MH_CreateHookApiEx(
        pszModule: LPCWSTR,
        pszProcName: LPCSTR,
        pDetour: LPVOID,
        ppOriginal: *mut LPVOID,
        ppTarget: *mut LPVOID,
    ) -> MH_STATUS;

    pub fn MH_RemoveHook(pTarget: LPVOID) -> MH_STATUS;

    pub fn MH_EnableHook(pTarget: LPVOID) -> MH_STATUS;

    pub fn MH_DisableHook(pTarget: LPVOID) -> MH_STATUS;

    pub fn MH_QueueEnableHook(pTarget: LPVOID) -> MH_STATUS;

    pub fn MH_QueueDisableHook(pTarget: LPVOID) -> MH_STATUS;

    pub fn MH_ApplyQueued() -> MH_STATUS;

    pub fn MH_StatusToString(status: MH_STATUS) -> *const ::std::os::raw::c_char;
}
