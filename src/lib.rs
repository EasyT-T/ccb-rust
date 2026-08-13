use std::ffi::{c_char, c_void};

mod common;

#[unsafe(no_mangle)]
pub extern "C" fn load_ccb() -> bool{
    let hostfxr = match netcorehost::nethost::load_hostfxr(){
        Ok(hostfxr) => hostfxr,
        Err(e) => {
            eprintln!("Can't load the hostfxr: {e}");
            return false;
        }
    };

    let context = match hostfxr.initialize_for_runtime_config(common::runtime_config_path()) {
        Ok(context) => context,
        Err(e) => {
            eprintln!("Can't initialize runtime config: {e}");
            return false;
        }
    };

    let fn_loader = match context.get_delegate_loader_for_assembly(common::assembly_path()) {
        Ok(fn_lolader) => fn_lolader,
        Err(e) => {
            eprintln!("Can't get delegate loader for assembly: {e}");
            return false;
        }
    };
    
    let function = fn_loader.get_function_with_unmanaged_callers_only::<fn()>(
        netcorehost::pdcstr!("CCB.Internal.Interop"),
        netcorehost::pdcstr!("Load"))
        .expect("Unable to invoke CCB::Internal::Interop::Load()");
    
    function();

    common::CLR_FUNCTION_LOADER.set(fn_loader).unwrap_or_else(|_| panic!("Unable to set the CLR_FUNCTION_LOADER"));
    
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn register_method(index: i32, class_name_ptr: *const c_char, method_name_ptr: *const c_char, func_ptr: *const c_void){
    let function_loader = common::CLR_FUNCTION_LOADER.get().unwrap_or_else(|| panic!("Unable to get the CLR_FUNCTION_LOADER"));

    let function = function_loader.get_function_with_unmanaged_callers_only::<fn(i32, *const c_char, *const c_char, *const c_void)>(
        netcorehost::pdcstr!("CCB.Internal.Interop, CCB"),
        netcorehost::pdcstr!("RegisterMethod"))
        .expect("Unable to register method");

    function(index, class_name_ptr, method_name_ptr, func_ptr);
}