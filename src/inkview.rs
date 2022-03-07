


//#[repr(C)]
//struct subtaskinfo {
//    pub id: std::os::raw::c_int,
//    pub name: *mut libc::c_char,
//    pub book: *mut libc::c_char,
//    pub fgindex: std::os::raw::c_int,
//    pub order: std::os::raw::c_int,
//    pub rsv_1s: std::os::raw::c_int,
//};
//
//#[repr(C)]
//struct taskinfo {
//    task: std::os::raw::c_int,
//    nsubtasks: std::os::raw::c_int,
//    flags: std::os::raw::c_uint,
//    fbshmkey: std::os::raw::c_int,
//    fbshmsize: std::os::raw::c_int,
//    pid_t mainpid;
//    char *appname;
//    ibitmap *icon;
//    subtaskinfo *subtasks;
//    fbtempkey: std::os::raw::c_int;
//    rsv_2: std::os::raw::c_int;
//    rsv_3: std::os::raw::c_int;
//    rsv_4: std::os::raw::c_int;
//}

use std::ffi::CStr;


extern "C" {
    fn GetCurrentTask() -> std::os::raw::c_int;
    //taskinfo *GetTaskInfo(pid: std::os::raw::c_int);

    fn GetDeviceModel() -> *mut std::os::raw::c_char;
    fn GetHardwareType() -> *mut std::os::raw::c_char;
    fn GetSoftwareVersion() -> *mut std::os::raw::c_char;
}

pub fn current_task() -> i32 {
    unsafe { GetCurrentTask() }
}

pub fn device_model() -> &'static str {
    unsafe { CStr::from_ptr(GetDeviceModel()) }.to_str().unwrap() 
}

pub fn hardware_type() -> &'static str {
    unsafe { CStr::from_ptr(GetHardwareType()) }.to_str().unwrap() 
}

pub fn software_version() -> &'static str {
    unsafe { CStr::from_ptr(GetSoftwareVersion()) }.to_str().unwrap() 
}