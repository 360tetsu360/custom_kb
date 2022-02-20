use kernel32::GetModuleHandleA;

pub unsafe fn module_base_addres() -> u64 {
    GetModuleHandleA(std::ptr::null()) as u64
}

pub unsafe fn force_cast<T>(addr: u64) -> *mut T {
    std::mem::transmute::<u64, *mut T>(addr)
}
