use std::collections::HashMap;

use crate::{Deserialize, Serialize};
use cosmwasm_std::Uint128;
use schemars::JsonSchema;

pub const CREATE_OUTBOUND_REPLY_ID: u64 = 1;
pub const SEPARATOR: &str = "_&_";

// Define struct pub struct RouterPayStreamMetdata
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RouterPayStreamMetadata {
    pub recipient_owner: String,
    pub created_at: u64,
    pub start_time: u64,
    pub pay_per_sec: Uint128,
    pub reason: String,
    pub last_withdrawn_at: u64,
    pub is_sending: bool,
    pub whitelisted_addresses: HashMap<String, bool>, // (chainid+_+address -> true)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CrossChainMetadata {
    pub relayer_fee: Uint128,
    pub ack_gas_limit: u64,
    pub dst_gas_limit: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OutboundInfo {
    pub stream_id: u64,
    pub total_amount_to_be_paid: Uint128,
    pub paid_to_sec: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub dst_gas_limit: u64,
    pub ack_gas_limit: u64,
    pub relayer_fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateStream {
        whitelisted_addresses: Option<Vec<(String, String)>>, // chainId -> address
        start_time: u64,
        pay_per_month: Uint128,
        recipient: String,
        remarks: Option<String>,
    },
    CancelStream {
        stream_id: u64,
        remarks: Option<String>,
    },
    DepositRoute {},
    WithdrawSalary {
        stream_id: u64,
        recipient: String,
        dst_chain_id: Option<String>,
        max_amount: Option<Uint128>,
    },
    EnrollRemoteContract {
        chain_id: String,
        remote_contract: String,
    },
    MapChainType {
        chain_id: String,
        chain_type: u64,
    },
    WithdrawFunds {
        recipient: String,
        amount: Uint128,
    },
    UpdateWhiteListAddress {
        stream_id: u64,
        address: String,
        chain_id: String,
        to: bool, // to where, true -> add and false -> remove
    },
    UpdateCrossChainMetadata {
        dst_gas_limit: Option<u64>,
        ack_gas_limit: Option<u64>,
        relayer_fee: Option<Uint128>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // fetch contract version
    GetContractVersion {},
    GetRemoteContract {
        chain_id: String,
    },
    GetRouterPayMetadata {
        stream_id: u64,
    },
    GetOwner {},
    GetStreams {
        from: u64,
        to: Option<u64>,
    },
    GetStreamWhiteListAddress {
        stream_id: u64,
    },
    GetUserStreamIds {
        address: String,
    },
    GetUserStreamsInfo {
        address: String,
    },
    IsWhiteListed {
        stream_id: u64,
        chain_id: String,
        address: String,
    },
    GetCrossChainMetadata {},
    GetAccumulatedAmount {
        stream_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WithDrawResponse {
    pub total_amount_to_be_paid: Uint128,
    pub paid_from_sec: u64,
    pub paid_to_sec: u64,
}
