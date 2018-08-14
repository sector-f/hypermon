use libvirt_sys::virErrorPtr;
use std::os::raw::c_void;

pub unsafe extern "C" fn do_nothing(_user_data: *mut c_void, _error: virErrorPtr) {
    // Do nothing, successfully
}
