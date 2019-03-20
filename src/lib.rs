extern crate libc;
use libc::{c_int, size_t};

const MODULE_NAME: &'static str = "rusthello";

const REDISMODULE_OK: c_int = 0;
const REDISMODULE_ERR: c_int = 1;

const REDISMODULE_APIVER_1: c_int = 1;

pub enum RedisModuleCtx {}
pub enum RedisModuleString {}

extern "C" {
    #[allow(improper_ctypes)]
    #[link(name = "redismodule", kind = "static")]
    pub fn Export_RedisModule_Init(
        ctx: *mut RedisModuleCtx,
        modulename: *const u8,
        module_version: c_int,
        api_version: c_int,
    ) -> c_int;
}

#[no_mangle]
pub extern "C" fn RedisModule_OnLoad(
    ctx: *mut RedisModuleCtx,
    argv: *mut *mut RedisModuleString,
    argc: c_int,
) -> c_int {
    unsafe {
        Export_RedisModule_Init(
            ctx,
            format!("{}\0", MODULE_NAME).as_ptr(),
            1,
            REDISMODULE_APIVER_1,
        );
    }
    return REDISMODULE_OK;
}
