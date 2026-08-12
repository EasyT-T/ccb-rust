use std::sync::OnceLock;
use netcorehost::hostfxr::AssemblyDelegateLoader;
use netcorehost::pdcstring::PdCStr;

pub static CLR_FUNCTION_LOADER: OnceLock<AssemblyDelegateLoader> = OnceLock::new();

pub fn runtime_config_path() -> &'static PdCStr{
    netcorehost::pdcstr!("CCB.runtimeconfig.json")
}
pub fn assembly_path() -> &'static PdCStr{
    netcorehost::pdcstr!("CCB.dll")
}