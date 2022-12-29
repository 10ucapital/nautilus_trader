// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2022 Nautech Systems Pty Ltd. All rights reserved.
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

use std::fmt::Debug;
use std::str::FromStr;

use pyo3::ffi;
use strum::{Display, EnumString, FromRepr};

use nautilus_core::string::{pystr_to_string, string_to_pystr};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ComponentState {
    PreInitialized = 0,
    PostInitialized = 1,
    Starting = 2,
    Running = 3,
    Stopping = 4,
    Stopped = 5,
    Resuming = 6,
    Resetting = 7,
    Disposing = 8,
    Disposed = 9,
    Degrading = 10,
    Degraded = 11,
    Faulting = 12,
    Faulted = 13,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ComponentTrigger {
    Initialize = 1,
    Start = 2,
    StartCompleted = 3,
    Stop = 4,
    StopCompleted = 5,
    Resume = 6,
    ResumeCompleted = 7,
    Reset = 8,
    ResetCompleted = 9,
    Dispose = 10,
    DisposeCompleted = 11,
    Degrade = 12,
    DegradeCompleted = 13,
    Fault = 14,
    FaultCompleted = 15,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, FromRepr, EnumString)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum LogLevel {
    #[strum(serialize = "DBG", serialize = "DEBUG")]
    Debug = 10,
    #[strum(serialize = "INF", serialize = "INFO")]
    Info = 20,
    #[strum(serialize = "WRN", serialize = "WARNING")]
    Warning = 30,
    #[strum(serialize = "ERR", serialize = "ERROR")]
    Error = 40,
    #[strum(serialize = "CRT", serialize = "CRITICAL")]
    Critical = 50,
}

// Override `strum` implementation
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            LogLevel::Debug => "DBG",
            LogLevel::Info => "INF",
            LogLevel::Warning => "WRN",
            LogLevel::Error => "ERR",
            LogLevel::Critical => "CRT",
        };
        write!(f, "{}", display)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum LogColor {
    #[strum(serialize = "")]
    Normal = 0,
    #[strum(serialize = "\x1b[92m")]
    Green = 1,
    #[strum(serialize = "\x1b[94m")]
    Blue = 2,
    #[strum(serialize = "\x1b[35m")]
    Magenta = 3,
    #[strum(serialize = "\x1b[36m")]
    Cyan = 4,
    #[strum(serialize = "\x1b[1;33m")]
    Yellow = 5,
    #[strum(serialize = "\x1b[1;31m")]
    Red = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum LogFormat {
    #[strum(serialize = "\x1b[95m")]
    Header,
    #[strum(serialize = "\x1b[0m")]
    Endc,
    #[strum(serialize = "\x1b[1m")]
    Bold,
    #[strum(serialize = "\x1b[4m")]
    Underline,
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn component_state_to_pystr(value: ComponentState) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn component_state_from_pystr(ptr: *mut ffi::PyObject) -> ComponentState {
    let value = &pystr_to_string(ptr);
    ComponentState::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn component_trigger_to_pystr(value: ComponentTrigger) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn component_trigger_from_pystr(ptr: *mut ffi::PyObject) -> ComponentTrigger {
    let value = &pystr_to_string(ptr);
    ComponentTrigger::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn log_level_to_pystr(value: LogLevel) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn log_level_from_pystr(ptr: *mut ffi::PyObject) -> LogLevel {
    let value = &pystr_to_string(ptr);
    LogLevel::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn log_color_to_pystr(value: LogColor) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn log_color_from_pystr(ptr: *mut ffi::PyObject) -> LogColor {
    let value = &pystr_to_string(ptr);
    LogColor::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}
