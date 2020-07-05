#![allow(non_snake_case)]
extern crate simple_logging;
extern crate log;
extern crate dirs;

use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::path::Path;
use log::{info, error, LevelFilter};

type PluginHandle = u32;
#[repr(C)]
pub struct SKSEInterface {
    pub skseVersion: u32,
    pub runtimeVersion: u32,
    pub editorVersion: u32,
    pub isEditor: u32,
    pub QueryInterface: fn(KInterface) -> *const c_void,
    pub GetPluginHandle: fn() -> PluginHandle,
    pub GetReleaseIndex: fn() -> u32,
}

#[repr(C)]
pub struct PluginInfo {
    pub infoVersion: u32,
    pub name: *mut c_char,
    pub version: u32,
}

#[repr(C)]
pub struct VMValue;

#[repr(C)]
pub struct VMState;

#[repr(C)]
pub struct NativeFunction {
    InitParams: fn(*const VMClassRegistry) -> (),
    Run: fn(*const VMValue, *const VMClassRegistry, u32, *const VMValue, *const VMState) -> bool,
}

impl NativeFunction {
    fn new() -> NativeFunction {
        NativeFunction {
            InitParams: |registry| {
                info!("NativeFunction InitParams begin");
                info!("registry: {:?}", registry);
                info!("NativeFunction InitParams successful");
            },
            Run: |baseValue, registry, stackId, resultValue, state| {
                info!("NativeFunction Run begin");
                info!("baseValue: {:?}", baseValue);
                info!("registry: {:?}", registry);
                info!("stackId: {:?}", stackId);
                info!("resultValue: {:?}", resultValue);
                info!("state: {:?}", state);
                info!("NativeFunction Run successful");
                true
            },
        }
    }
}


#[repr(C)]
pub struct VMClassInfo;

#[repr(C)]
pub struct VMIdentifier;

#[repr(C)]
pub struct StringCacheRef;

#[repr(C)]
pub struct VMClassRegistry {
    pub Unk_01: fn(c_void) -> c_void,
    pub PrintToDebugLog: fn(*const c_char, u32, u32) -> c_void,
    pub Unk_03: fn(c_void) -> c_void,
    pub Unk_04: fn(c_void) -> c_void,
    pub Unk_05: fn(c_void) -> c_void,
    pub Unk_06: fn(c_void) -> c_void,
    pub Unk_07: fn(c_void) -> c_void,
    pub RegisterForm: fn(u32, *const c_char) -> c_void,
    pub Unk_09: fn(c_void) -> c_void,
    pub GetFormTypeClass: fn(u32, *const VMClassInfo) -> bool,
    pub Unk_0B: fn(c_void) -> c_void,
    pub Unk_0C: fn(c_void) -> c_void,
    pub Unk_0D: fn(*const StringCacheRef, *const u32) -> bool,
    pub Unk_0E: fn(c_void) -> c_void,
    pub Unk_0F: fn(c_void) -> c_void,
    pub Unk_10: fn(c_void) -> c_void,
    pub Unk_11: fn(c_void) -> c_void,
    pub Unk_12: fn(c_void) -> c_void,
    pub Unk_13: fn(c_void) -> c_void,
    pub Unk_14: fn(c_void) -> c_void,
    pub Unk_15: fn(*const StringCacheRef, *const VMIdentifier) -> bool,
    pub CreateArray: fn(*const VMValue, u32, *const VMValue) -> bool,
    pub Unk_17: fn(c_void) -> c_void,
    pub RegisterFunction: fn(*const NativeFunction) -> c_void,
    // more...
}

type RegisterFunctions = fn(*const VMClassRegistry) -> bool;

#[repr(C)]
pub struct SKSEPapyrusInterface {
    pub interfaceVersion: u32,
    pub Register: fn(*const RegisterFunctions) -> bool,
}

#[repr(u32)]
pub enum KInterface {
	Invalid = 0,
	Scaleform,
	Papyrus,
	Serialization,
	Task,
	Messaging,
	Object,
	Max,
}

const fn make_exe_version(major: u32, minor: u32, build: u32, sub: u32) -> u32 {
    (((major) & 0xFF) << 24) | (((minor) & 0xFF) << 16) | (((build) & 0xFFF) << 4) | ((sub) & 0xF)
}

const RUNTIME_VERSION_1_5_97: u32 = make_exe_version(1, 5, 97, 0);

unsafe fn RegisterFuncs(a_registry: *const VMClassRegistry) -> bool {
    info!("RegisterFuncs begin");
    info!("a_registry: {:?}", a_registry);
    let registerFunction = (*a_registry).RegisterFunction;
    info!("registerFunction: {:?}", registerFunction);
    let nativeFunction: *const NativeFunction = &NativeFunction::new();
    info!("nativeFunction: {:?}", nativeFunction);
    // This is as far as I can get. I have no idea how to register a native papyrus function.
    // Doesn't work, throws exception "Access violation executing location"
    // registerFunction(nativeFunction);
    info!("RegisterFuncs successful");
    true
}

#[no_mangle]
pub unsafe extern "C" fn SKSEPlugin_Query(a_skse: *const SKSEInterface, a_info: *mut PluginInfo) -> bool {
    let mut log_dir = dirs::document_dir().expect("could not get Documents directory");
    log_dir.push(Path::new(r#"My Games\Skyrim Special Edition\SKSE\RustSKSEPlugin.log"#));
    simple_logging::log_to_file(log_dir, LevelFilter::Info).unwrap();
    info!("SKSEPlugin_Query begin");

    (*a_info).infoVersion = 1;
    (*a_info).name = CString::new("My Rust SKSE Plugin").expect("could not create CString").into_raw();
    (*a_info).version = 1;

    if (*a_skse).isEditor != 0 {
        error!("Loaded in editor, marking as incompatible!");
        return false;
    } else if (*a_skse).runtimeVersion != RUNTIME_VERSION_1_5_97 {
        error!("Unsupported runtime version {}!", (*a_skse).runtimeVersion);
        return false;
    }
    
    info!("SKSEPlugin_Query successful");
    true
}

#[no_mangle]
pub unsafe extern "C" fn SKSEPlugin_Load(a_skse: *const SKSEInterface) -> bool {
    info!("SKSEPlugin_Load begin");
    
    let queryInterfaceFunc = (*a_skse).QueryInterface;
    info!("queryInterfaceFunc: {:?}", queryInterfaceFunc);
    let queryInterface = queryInterfaceFunc(KInterface::Papyrus);
    info!("queryInterface: {:?}", queryInterface);
    let papyrusInterface: *const SKSEPapyrusInterface = queryInterface as *const SKSEPapyrusInterface;
    info!("papyrusInterface: {:?}", papyrusInterface);
    let papyrusRegister = (*papyrusInterface).Register;
    info!("papyrusRegister: {:?}", papyrusRegister);
    if !papyrusRegister(RegisterFuncs as *const RegisterFunctions) {
        error!("RegisterFuncs returned false!");
        return false;
    }

    info!("SKSEPlugin_Load successful");
    true
}