use cosmwasm_std::{to_binary, Binary, Deps, Env, StdError, StdResult, Uint128};
use cw2::get_contract_version;
use router_pay_stream::routerpay::{
    CrossChainMetadata, QueryMsg, RouterPayStreamMetadata, SEPARATOR,
};
use router_wasm_bindings::RouterQuery;

use crate::{
    execution::get_id,
    state::{
        ACK_GAS_LIMIT, DST_GAS_LIMIT, OWNER, RELAYER_FEE, REMOTE_CONTRACT_MAPPING,
        ROUTER_PAY_STREAM_METADATA_MP, USER_STREAMS,
    },
};

pub fn handle_query(deps: Deps<RouterQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::GetOwner {} => to_binary(&get_owner(deps)?),
        QueryMsg::GetRemoteContract { chain_id } => {
            to_binary(&get_remote_contract(deps, chain_id)?)
        }
        QueryMsg::IsWhiteListed {
            stream_id,
            address,
            chain_id,
        } => to_binary(&is_white_listed(deps, stream_id, chain_id, address)?),
        QueryMsg::GetCrossChainMetadata {} => to_binary(&get_crosschain_metadata(deps)?),
        QueryMsg::GetRouterPayMetadata { stream_id } => {
            to_binary(&get_routerpay_metadata(deps, stream_id)?)
        }
        QueryMsg::GetStreamWhiteListAddress { stream_id } => {
            to_binary(&get_white_listed_addresses(deps, stream_id)?)
        }
        QueryMsg::GetAccumulatedAmount { stream_id } => {
            to_binary(&get_accumulated_amount(deps, env, stream_id)?)
        }
        QueryMsg::GetStreams { from, to } => to_binary(&get_streams(deps, from, to)?),
        QueryMsg::GetUserStreamIds { address } => to_binary(&get_user_stream_ids(deps, address)?),
        QueryMsg::GetUserStreamsInfo { address } => {
            to_binary(&get_user_streams_info(deps, address)?)
        }
    }
}

pub fn get_owner(deps: Deps<RouterQuery>) -> StdResult<String> {
    OWNER.load(deps.storage)
}

pub fn get_accumulated_amount(
    deps: Deps<RouterQuery>,
    env: Env,
    stream_id: u64,
) -> StdResult<Uint128> {
    match ROUTER_PAY_STREAM_METADATA_MP.load(deps.storage, stream_id) {
        Ok(router_pay_metadata) => {
            let delta = env.block.time.seconds() - router_pay_metadata.last_withdrawn_at;
            Ok(Uint128::from(delta) * router_pay_metadata.pay_per_sec)
        }
        Err(_) => Err(StdError::GenericErr {
            msg: "Not_Found".to_string(),
        }),
    }
}

pub fn get_crosschain_metadata(deps: Deps<RouterQuery>) -> StdResult<CrossChainMetadata> {
    Ok(CrossChainMetadata {
        ack_gas_limit: ACK_GAS_LIMIT.load(deps.storage).unwrap(),
        relayer_fee: RELAYER_FEE.load(deps.storage).unwrap(),
        dst_gas_limit: DST_GAS_LIMIT.load(deps.storage).unwrap(),
    })
}

pub fn is_white_listed(
    deps: Deps<RouterQuery>,
    stream_id: u64,
    chain_id: String,
    address: String,
) -> StdResult<bool> {
    match ROUTER_PAY_STREAM_METADATA_MP.load(deps.storage, stream_id) {
        Ok(router_metadata) => {
            if router_metadata
                .whitelisted_addresses
                .contains_key(&get_id(chain_id, address))
            {
                return Ok(true);
            }
            Ok(false)
        }
        Err(_) => Err(cosmwasm_std::StdError::GenericErr {
            msg: "Stream_Not_Found".to_string(),
        }),
    }
}

fn get_remote_contract(deps: Deps<RouterQuery>, chain_id: String) -> StdResult<String> {
    REMOTE_CONTRACT_MAPPING.load(deps.storage, chain_id)
}

fn get_user_stream_ids(deps: Deps<RouterQuery>, address: String) -> StdResult<Vec<u64>> {
    deps.api.addr_validate(&address)?;
    match USER_STREAMS.load(deps.storage, address) {
        Ok(streams_info) => Ok(streams_info.keys().cloned().collect()),
        Err(_) => Ok(vec![]),
    }
}

fn get_user_streams_info(
    deps: Deps<RouterQuery>,
    address: String,
) -> StdResult<Vec<RouterPayStreamMetadata>> {
    deps.api.addr_validate(&address)?;
    let stream_ids = get_user_stream_ids(deps, address).unwrap();
    let mut streams_info: Vec<RouterPayStreamMetadata> = vec![];
    for stream_id in stream_ids {
        streams_info.push(
            ROUTER_PAY_STREAM_METADATA_MP
                .load(deps.storage, stream_id)
                .unwrap(),
        )
    }
    Ok(streams_info)
}

fn get_routerpay_metadata(
    deps: Deps<RouterQuery>,
    stream_id: u64,
) -> StdResult<RouterPayStreamMetadata> {
    ROUTER_PAY_STREAM_METADATA_MP.load(deps.storage, stream_id)
}

fn get_white_listed_addresses(
    deps: Deps<RouterQuery>,
    stream_id: u64,
) -> StdResult<Vec<(String, String)>> {
    match get_routerpay_metadata(deps, stream_id.clone()) {
        Ok(router_metadata) => {
            let mut result = Vec::new();
            for key in router_metadata.whitelisted_addresses.keys() {
                let parts: Vec<&str> = key.split(SEPARATOR).collect();
                if parts.len() == 2 {
                    result.push((parts[0].to_owned(), parts[1].to_owned()));
                }
            }
            Ok(result)
        }

        Err(err) => Err(err),
    }
}

fn get_streams(
    deps: Deps<RouterQuery>,
    from: u64,
    to: Option<u64>,
) -> StdResult<Vec<RouterPayStreamMetadata>> {
    let mut to = to.unwrap_or(from + 10u64);
    if to < from {
        to = from + 10u64;
    }
    let mut all_streams_info = Vec::<RouterPayStreamMetadata>::new();
    for stream_id in from..=to {
        if let Ok(stream_info) = ROUTER_PAY_STREAM_METADATA_MP.load(deps.storage, stream_id) {
            all_streams_info.push(stream_info);
        }
    }
    Ok(all_streams_info)
}
