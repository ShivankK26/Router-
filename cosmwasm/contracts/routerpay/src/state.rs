use std::collections::HashMap;

use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};
use router_pay_stream::routerpay::{OutboundInfo, RouterPayStreamMetadata};

// ADMIN address to perform admin priviledged operations
pub const OWNER: Item<String> = Item::new("router_pay_owner");

// User Router Address -> (stream_id,true)
pub const USER_STREAMS: Map<String, HashMap<u64, bool>> = Map::new("user_streams");

// Stream Id start from 0 onwards
pub const STREAM_INDEXER: Item<u64> = Item::new("stream_indexer");

// stream_id -> it's metadata
pub const ROUTER_PAY_STREAM_METADATA_MP: Map<u64, RouterPayStreamMetadata> =
    Map::new("router_pay_stream_metadata_mp");

// maintains mapping of contract depolyed on other chain to isend and valid upcoming request to this contract
pub const REMOTE_CONTRACT_MAPPING: Map<String, String> = Map::new("remote_contract_mapping");

// it is used to convert address of other chain to router address, e.g chain_Type for evm chain is 1
pub const CHAIN_TYPE_MAPPING: Map<String, u64> = Map::new("chain_type_mapping");

pub const TEMP_OUTBOUND_INFO: Item<OutboundInfo> = Item::new("temp_outbound_info");
pub const TEMP_OUTBOUND_INFO_MP: Map<u64, OutboundInfo> = Map::new("temp_outbound_info_mp");

// while creating isend msg dst_gas_limit, gas required to execute IReceive Fn on dst chain
pub const DST_GAS_LIMIT: Item<u64> = Item::new("dest_gas_limit");

// gas required to execute IAck on this contract
pub const ACK_GAS_LIMIT: Item<u64> = Item::new("ack_gas_limit");

// fee for relayer to relay request
pub const RELAYER_FEE: Item<Uint128> = Item::new("relayer_fee");
