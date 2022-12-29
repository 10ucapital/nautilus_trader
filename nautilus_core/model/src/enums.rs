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
pub enum AccountType {
    Cash = 1,
    Margin = 2,
    Betting = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum AggregationSource {
    External = 1,
    Internal = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum AggressorSide {
    NoAggressor = 0, // Will be replaced by `Option`
    Buyer = 1,
    Seller = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum AssetClass {
    FX = 1,
    Equity = 2,
    Commodity = 3,
    Metal = 4,
    Energy = 5,
    Bond = 6,
    Index = 7,
    Cryptocurrency = 8,
    SportsBetting = 9,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetType {
    Spot = 1,
    Swap = 2,
    Future = 3,
    Forward = 4,
    Cfd = 5,
    Option = 6,
    Warrant = 7,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum BarAggregation {
    Tick = 1,
    TickImbalance = 2,
    TickRuns = 3,
    Volume = 4,
    VolumeImbalance = 5,
    VolumeRuns = 6,
    Value = 7,
    ValueImbalance = 8,
    ValueRuns = 9,
    Millisecond = 10,
    Second = 11,
    Minute = 12,
    Hour = 13,
    Day = 14,
    Week = 15,
    Month = 16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum BookAction {
    Add = 1,
    Update = 2,
    Delete = 3,
    Clear = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum BookType {
    /// Top-of-book best bid/offer.
    L1_TBBO = 1,
    /// Market by price.
    L2_MBP = 2,
    /// Market by order.
    L3_MBO = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ContingencyType {
    NoContingency = 0, // Will be replaced by `Option`
    Oco = 1,
    Oto = 2,
    Ouo = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CurrencyType {
    Crypto = 1,
    Fiat = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum DepthType {
    Volume = 1,
    Exposure = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentCloseType {
    EndOfSession = 1,
    ContractExpired = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum LiquiditySide {
    NoLiquiditySide = 0, // Will be replaced by `Option`
    Maker = 1,
    Taker = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketStatus {
    Closed = 1,
    PreOpen = 2,
    Open = 3,
    Pause = 4,
    PreClose = 5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OmsType {
    Unspecified = 0, // Will be replaced by `Option`
    Netting = 1,
    Hedging = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionKind {
    Call = 1,
    Put = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum OrderSide {
    NoOrderSide = 0, // Will be replaced by `Option`
    Buy = 1,
    Sell = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Initialized = 1,
    Denied = 2,
    Submitted = 3,
    Accepted = 4,
    Rejected = 5,
    Canceled = 6,
    Expired = 7,
    Triggered = 8,
    PendingUpdate = 9,
    PendingCancel = 10,
    PartiallyFilled = 11,
    Filled = 12,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Market = 1,
    Limit = 2,
    StopMarket = 3,
    StopLimit = 4,
    MarketToLimit = 5,
    MarketIfTouched = 6,
    LimitIfTouched = 7,
    TrailingStopMarket = 8,
    TrailingStopLimit = 9,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum PositionSide {
    NoPositionSide = 0, // Will be replaced by `Option`
    Flat = 1,
    Long = 2,
    Short = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceType {
    Bid = 1,
    Ask = 2,
    Mid = 3,
    Last = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    Gtc = 1,
    Ioc = 2,
    Fok = 3,
    Gtd = 4,
    Day = 5,
    AtTheOpen = 6,
    AtTheClose = 7,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TradingState {
    Active = 1,
    Halted = 2,
    Reducing = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TrailingOffsetType {
    NoTrailingOffset = 0, // Will be replaced by `Option`
    Price = 1,
    BasisPoints = 2,
    Ticks = 3,
    PriceTier = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TriggerType {
    NoTrigger = 0, // Will be replaced by `Option`
    Default = 1,
    BidAsk = 2,
    LastTrade = 3,
    DoubleLast = 4,
    DoubleBidAsk = 5,
    LastOrBidAsk = 6,
    MidPoint = 7,
    MarkPrice = 8,
    IndexPrice = 9,
}

// TODO(cs): These should be macros

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn account_type_to_pystr(value: AccountType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn account_type_from_pystr(ptr: *mut ffi::PyObject) -> AccountType {
    let value = &pystr_to_string(ptr);
    AccountType::from_str(value)
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn aggregation_source_to_pystr(
    value: AggregationSource,
) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn aggregation_source_from_pystr(
    ptr: *mut ffi::PyObject,
) -> AggregationSource {
    let value = &pystr_to_string(ptr);
    AggregationSource::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn aggressor_side_to_pystr(value: AggressorSide) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn aggressor_side_from_pystr(ptr: *mut ffi::PyObject) -> AggressorSide {
    let value = &pystr_to_string(ptr);
    AggressorSide::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn asset_class_to_pystr(value: AssetClass) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn asset_class_from_pystr(ptr: *mut ffi::PyObject) -> AssetClass {
    let value = &pystr_to_string(ptr);
    AssetClass::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn asset_type_to_pystr(value: AssetType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn asset_type_from_pystr(ptr: *mut ffi::PyObject) -> AssetType {
    let value = &pystr_to_string(ptr);
    AssetType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn bar_aggregation_to_pystr(value: BarAggregation) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn bar_aggregation_from_pystr(ptr: *mut ffi::PyObject) -> BarAggregation {
    let value = &pystr_to_string(ptr);
    BarAggregation::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn book_action_to_pystr(value: BookAction) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn book_action_from_pystr(ptr: *mut ffi::PyObject) -> BookAction {
    let value = &pystr_to_string(ptr);
    BookAction::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn book_type_to_pystr(value: BookType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn book_type_from_pystr(ptr: *mut ffi::PyObject) -> BookType {
    let value = &pystr_to_string(ptr);
    BookType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn contingency_type_to_pystr(value: ContingencyType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn contingency_type_from_pystr(ptr: *mut ffi::PyObject) -> ContingencyType {
    let value = &pystr_to_string(ptr);
    ContingencyType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn currency_type_to_pystr(value: CurrencyType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn currency_type_from_pystr(ptr: *mut ffi::PyObject) -> CurrencyType {
    let value = &pystr_to_string(ptr);
    CurrencyType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn depth_type_to_pystr(value: DepthType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn instrument_close_type_from_pystr(
    ptr: *mut ffi::PyObject,
) -> InstrumentCloseType {
    let value = &pystr_to_string(ptr);
    InstrumentCloseType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn instrument_close_type_to_pystr(
    value: InstrumentCloseType,
) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn depth_type_from_pystr(ptr: *mut ffi::PyObject) -> DepthType {
    let value = &pystr_to_string(ptr);
    DepthType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn liquidity_side_to_pystr(value: LiquiditySide) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn liquidity_side_from_pystr(ptr: *mut ffi::PyObject) -> LiquiditySide {
    let value = &pystr_to_string(ptr);
    LiquiditySide::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn market_status_to_pystr(value: MarketStatus) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn market_status_from_pystr(ptr: *mut ffi::PyObject) -> MarketStatus {
    let value = &pystr_to_string(ptr);
    MarketStatus::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn oms_type_to_pystr(value: OmsType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn oms_type_from_pystr(ptr: *mut ffi::PyObject) -> OmsType {
    let value = &pystr_to_string(ptr);
    OmsType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn option_kind_to_pystr(value: OptionKind) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn option_kind_from_pystr(ptr: *mut ffi::PyObject) -> OptionKind {
    let value = &pystr_to_string(ptr);
    OptionKind::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn order_side_to_pystr(value: OrderSide) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn order_side_from_pystr(ptr: *mut ffi::PyObject) -> OrderSide {
    let value = &pystr_to_string(ptr);
    OrderSide::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn order_status_to_pystr(value: OrderStatus) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn order_status_from_pystr(ptr: *mut ffi::PyObject) -> OrderStatus {
    let value = &pystr_to_string(ptr);
    OrderStatus::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn order_type_to_pystr(value: OrderType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn order_type_from_pystr(ptr: *mut ffi::PyObject) -> OrderType {
    let value = &pystr_to_string(ptr);
    OrderType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn position_side_to_pystr(value: PositionSide) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn position_side_from_pystr(ptr: *mut ffi::PyObject) -> PositionSide {
    let value = &pystr_to_string(ptr);
    PositionSide::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn price_type_to_pystr(value: PriceType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn price_type_from_pystr(ptr: *mut ffi::PyObject) -> PriceType {
    let value = &pystr_to_string(ptr);
    PriceType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn time_in_force_to_pystr(value: TimeInForce) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn time_in_force_from_pystr(ptr: *mut ffi::PyObject) -> TimeInForce {
    let value = &pystr_to_string(ptr);
    TimeInForce::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn trading_state_to_pystr(value: TradingState) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn trading_state_from_pystr(ptr: *mut ffi::PyObject) -> TradingState {
    let value = &pystr_to_string(ptr);
    TradingState::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn trailing_offset_type_to_pystr(
    value: TrailingOffsetType,
) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn trailing_offset_type_from_pystr(
    ptr: *mut ffi::PyObject,
) -> TrailingOffsetType {
    let value = &pystr_to_string(ptr);
    TrailingOffsetType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}

/// Returns a pointer to a valid Python UTF-8 string.
///
/// # Safety
/// - Assumes that since the data is originating from Rust, the GIL does not need
/// to be acquired.
/// - Assumes you are immediately returning this pointer to Python.
#[no_mangle]
pub unsafe extern "C" fn trigger_type_to_pystr(value: TriggerType) -> *mut ffi::PyObject {
    string_to_pystr(&value.to_string())
}

/// Returns an enum from a Python string.
///
/// # Safety
/// - Assumes `ptr` is borrowed from a valid Python UTF-8 `str`.
#[no_mangle]
pub unsafe extern "C" fn trigger_type_from_pystr(ptr: *mut ffi::PyObject) -> TriggerType {
    let value = &pystr_to_string(ptr);
    TriggerType::from_str(&pystr_to_string(ptr))
        .unwrap_or_else(|_| panic!("Invalid enum string value, was '{value}'"))
}
