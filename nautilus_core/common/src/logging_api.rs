// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2023 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

use std::ffi::c_char;
use std::ops::{Deref, DerefMut};

use nautilus_core::parsing::optional_bytes_to_json;
use nautilus_core::string::{cstr_to_string, optional_cstr_to_string, str_to_cstr};
use nautilus_core::uuid::UUID4;
use nautilus_model::identifiers::trader_id::TraderId;

use crate::enums::{LogColor, LogLevel};
use crate::logging::Logger;

/// Logger is not C FFI safe, so we box and pass it as an opaque pointer.
/// This works because Logger fields don't need to be accessed, only functions
/// are called.
#[repr(C)]
pub struct CLogger(Box<Logger>);

impl Deref for CLogger {
    type Target = Logger;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CLogger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Creates a new logger.
///
/// # Safety
///
/// - Assumes `trader_id_ptr` is a valid C string pointer.
/// - Assumes `machine_id_ptr` is a valid C string pointer.
/// - Assumes `instance_id_ptr` is a valid C string pointer.
#[no_mangle]
pub unsafe extern "C" fn logger_new(
    trader_id_ptr: *const c_char,
    machine_id_ptr: *const c_char,
    instance_id_ptr: *const c_char,
    level_stdout: LogLevel,
    level_file: LogLevel,
    file_logging: u8,
    directory_ptr: *const c_char,
    file_name_ptr: *const c_char,
    file_format_ptr: *const c_char,
    component_levels_ptr: *const c_char,
    is_bypassed: u8,
) -> CLogger {
    CLogger(Box::new(Logger::new(
        TraderId::new(&cstr_to_string(trader_id_ptr)),
        String::from(&cstr_to_string(machine_id_ptr)),
        UUID4::from(cstr_to_string(instance_id_ptr).as_str()),
        level_stdout,
        if file_logging != 0 {
            Some(level_file)
        } else {
            None
        },
        optional_cstr_to_string(directory_ptr),
        optional_cstr_to_string(file_name_ptr),
        optional_cstr_to_string(file_format_ptr),
        optional_bytes_to_json(component_levels_ptr),
        is_bypassed != 0,
    )))
}

#[no_mangle]
pub extern "C" fn logger_drop(logger: CLogger) {
    drop(logger); // Memory freed here
}

#[no_mangle]
pub extern "C" fn logger_get_trader_id_cstr(logger: &CLogger) -> *const c_char {
    str_to_cstr(&logger.trader_id.to_string())
}

#[no_mangle]
pub extern "C" fn logger_get_machine_id_cstr(logger: &CLogger) -> *const c_char {
    str_to_cstr(&logger.machine_id)
}

#[no_mangle]
pub extern "C" fn logger_get_instance_id(logger: &CLogger) -> UUID4 {
    logger.instance_id.clone()
}

#[no_mangle]
pub extern "C" fn logger_is_bypassed(logger: &CLogger) -> u8 {
    logger.is_bypassed as u8
}

/// Create a new log event.
///
/// # Safety
///
/// - Assumes `component_ptr` is a valid C string pointer.
/// - Assumes `message_ptr` is a valid C string pointer.
#[no_mangle]
pub unsafe extern "C" fn logger_log(
    logger: &mut CLogger,
    timestamp_ns: u64,
    level: LogLevel,
    color: LogColor,
    component_ptr: *const c_char,
    message_ptr: *const c_char,
) {
    let component = cstr_to_string(component_ptr);
    let message = cstr_to_string(message_ptr);
    logger.send(timestamp_ns, level, color, component, message);
}
