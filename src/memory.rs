use kernel32::GetModuleHandleA;

#[allow(clippy::missing_safety_doc)]
pub unsafe fn module_base_addres() -> u64 {
    GetModuleHandleA(std::ptr::null()) as u64
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn force_cast<T>(addr: u64) -> *mut T {
    std::mem::transmute::<u64, *mut T>(addr)
}
