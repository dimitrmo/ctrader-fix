use serde::Deserialize;
use std::str::FromStr;

use async_trait::async_trait;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const DELIMITER: &str = "\u{1}";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // reponse send error
    // #[error(transparent)]
    // SendError(#[from] async_std::channel::SendError<ResponseMessage>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub username: String,
    pub password: String,
    pub broker: String,
    pub heart_beat: u32,
}

impl Config {
    pub fn new(
        host: String,
        username: String,
        password: String,
        broker: String,
        heart_beat: u32,
    ) -> Self {
        Self {
            host,
            username,
            password,
            broker,
            heart_beat,
        }
    }
}

#[async_trait]
pub trait ConnectionHandler {
    async fn on_connect(&self);
    async fn on_logon(&self);
    async fn on_disconnect(&self);
}

#[repr(u32)]
#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Eq, Hash, Copy)]
pub enum Field {
    AvgPx = 6,
    BeginSeqNo = 7,
    BeginString = 8,
    BodyLength = 9,
    CheckSum = 10,
    ClOrdId = 11,
    CumQty = 14,
    EndSeqNo = 16,
    OrdQty = 32,
    MsgSeqNum = 34,
    MsgType = 35,
    NewSeqNo = 36,
    OrderID = 37,
    OrderQty = 38,
    OrdStatus = 39,
    OrdType = 40,
    OrigClOrdID = 41,
    Price = 44,
    RefSeqNum = 45,
    SenderCompID = 49,
    SenderSubID = 50,
    SendingTime = 52,
    Side = 54,
    Symbol = 55,
    TargetCompID = 56,
    TargetSubID = 57,
    Text = 58,
    TimeInForce = 59,
    TransactTime = 60,
    EncryptMethod = 98,
    StopPx = 99,
    OrdRejReason = 103,
    HeartBtInt = 108,
    TestReqID = 112,
    GapFillFlag = 123,
    ExpireTime = 126,
    ResetSeqNumFlag = 141,
    NoRelatedSym = 146,
    ExecType = 150,
    LeavesQty = 151,
    IssueDate = 225,
    MDReqID = 262,
    SubscriptionRequestType = 263,
    MarketDepth = 264,
    MDUpdateType = 265,
    NoMDEntryTypes = 267,
    NoMDEntries = 268,
    MDEntryType = 269,
    MDEntryPx = 270,
    MDEntrySize = 271,
    MDEntryID = 278,
    MDUpdateAction = 279,
    SecurityReqID = 320,
    SecurityResponseID = 322,
    EncodedTextLen = 354,
    EncodedText = 355,
    RefTagID = 371,
    RefMsgType = 372,
    SessionRejectReason = 373,
    BusinessRejectRefID = 379,
    BusinessRejectReason = 380,
    CxlRejResponseTo = 434,
    Designation = 494,
    Username = 553,
    Password = 554,
    SecurityListRequestType = 559,
    SecurityRequestResult = 560,
    MassStatusReqID = 584,
    MassStatusReqType = 585,
    NoPositions = 702,
    LongQty = 704,
    ShortQty = 705,
    PosReqID = 710,
    PosMaintRptID = 721,
    TotalNumPosReports = 727,
    PosReqResult = 728,
    SettlPrice = 730,
    TotNumReports = 911,
    AbsoluteTP = 1000,
    RelativeTP = 1001,
    AbsoluteSL = 1002,
    RelativeSL = 1003,
    TrailingSL = 1004,
    TriggerMethodSL = 1005,
    GuaranteedSL = 1006,
    SymbolName = 1007,
    SymbolDigits = 1008,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SubID {
    QUOTE,
    TRADE,
}

impl std::fmt::Display for SubID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SubID::QUOTE => "QUOTE",
            SubID::TRADE => "TRADE",
        };
        f.write_str(s)
    }
}

impl FromStr for SubID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "QUOTE" => Ok(SubID::QUOTE),
            "TRADE" => Ok(SubID::TRADE),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, TryFromPrimitive, Clone, Copy)]
pub enum Side {
    BUY = 1,
    SELL = 2,
}

impl Default for Side {
    fn default() -> Self {
        Side::BUY
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, TryFromPrimitive, Clone, Copy)]
pub enum OrderType {
    MARKET = 1,
    LIMIT = 2,
    STOP = 3,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::MARKET
    }
}