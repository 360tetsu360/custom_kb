use kernel32::FreeLibraryAndExitThread;
use memory::force_cast;
use minhook::{
    MH_CreateHook, MH_EnableHook, MH_Initialize, MH_Uninitialize, MH_STATUS, MH_STATUS_MH_OK,
};
use winapi::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};

pub mod memory;

const DLL_PROCESS_DETACH: DWORD = 0;
const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_THREAD_ATTACH: DWORD = 2;
const DLL_THREAD_DETACH: DWORD = 3;

static mut DLL_INST: HINSTANCE = std::ptr::null_mut();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hinst: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    match reason {
        DLL_PROCESS_DETACH => {}
        DLL_PROCESS_ATTACH => unsafe {
            DLL_INST = hinst;
            start();
            kernel32::DisableThreadLibraryCalls(hinst);
        },
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => {}
    };

    TRUE
}

#[repr(C)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

type Hook = unsafe extern "fastcall" fn(addr: u64, val: u64);

static mut BACK_TO_FUNCTION: u64 = 0;

unsafe extern "fastcall" fn callback(addr: u64, val: u64) {
    let velocity = force_cast::<Vec3>(val);
    (*velocity).x *= 2.;
    (*velocity).y *= 2.;
    (*velocity).z *= 2.;

    let function1 = std::mem::transmute::<u64, Hook>(BACK_TO_FUNCTION);
    function1(addr, val);
}

unsafe fn start() {
    let status: MH_STATUS = MH_Initialize();
    if status != MH_STATUS_MH_OK {
        unload();
    }

    let base = memory::module_base_addres();
    let fn_pointer: LPVOID = std::mem::transmute::<u64, LPVOID>(base + 0x1DCAB50);

    let cc = std::mem::transmute::<*mut u64, *mut LPVOID>(&mut BACK_TO_FUNCTION);

    let status: MH_STATUS = MH_CreateHook(fn_pointer, callback as *mut winapi::c_void, cc);
    if status != MH_STATUS_MH_OK {
        unload();
    }

    let status = MH_EnableHook(fn_pointer);
    if status != MH_STATUS_MH_OK {
        unload();
    }
}

unsafe fn unload() {
    MH_Uninitialize();
    FreeLibraryAndExitThread(DLL_INST, 1)
}
