use cosmwasm_std::{Binary, Coin, DepsMut, Env, Event, Response, StdError, StdResult, Uint128};
use router_wasm_bindings::{
    ethabi::{decode, ParamType},
    utils::convert_address_from_bytes_to_string,
    RouterMsg, RouterQuery, SudoMsg,
};

use crate::{
    execution::withdraw_salary,
    state::{
        CHAIN_TYPE_MAPPING, REMOTE_CONTRACT_MAPPING, ROUTER_PAY_STREAM_METADATA_MP,
        TEMP_OUTBOUND_INFO_MP,
    },
};

pub fn handle_sudo(
    deps: DepsMut<RouterQuery>,
    env: Env,
    msg: SudoMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        SudoMsg::HandleIReceive {
            request_sender,
            src_chain_id,
            request_identifier,
            payload,
        } => handle_sudo_request(
            deps,
            env,
            request_sender,
            src_chain_id,
            request_identifier,
            payload,
        ),
        SudoMsg::HandleIAck {
            request_identifier,
            exec_flag,
            exec_data: _,
            refund_amount,
        } => handle_sudo_ack(deps, env, exec_flag, request_identifier, refund_amount),
    }
}

pub fn handle_sudo_request(
    deps: DepsMut<RouterQuery>,
    env: Env,
    request_sender: String,
    src_chain_id: String,
    _request_identifier: u64,
    payload: Binary,
) -> StdResult<Response<RouterMsg>> {
    let r_contract_address = REMOTE_CONTRACT_MAPPING
        .load(deps.storage, src_chain_id.clone())
        .unwrap();
    if r_contract_address != request_sender.to_string().to_lowercase() {
        return Err(StdError::GenericErr {
            msg: "Auth: Invalid Caller".into(),
        });
    }

    let param_vec: Vec<ParamType> = vec![
        ParamType::String,
        ParamType::Bytes,
        ParamType::String,
        ParamType::Uint(64),
        ParamType::Uint(256),
    ];
    let req_res = match decode(&param_vec, &payload.0) {
        Ok(data) => data,
        Err(err) => {
            return Err(StdError::GenericErr {
                msg: format!("{:?}", err),
            })
        }
    };

    let dst_chain_id: String = req_res[0].clone().into_string().unwrap();

    let sender = convert_address_from_bytes_to_string(
        &req_res[1].clone().into_bytes().unwrap(),
        CHAIN_TYPE_MAPPING
            .load(deps.storage, src_chain_id.clone())
            .unwrap(),
    )
    .unwrap();
    let recipient: String = req_res[2].clone().into_string().unwrap().to_lowercase();
    let stream_id: u64 = req_res[3].clone().into_uint().unwrap().as_u64();
    let max_amount: Uint128 = Uint128::from(req_res[4].clone().into_uint().unwrap().as_u128());

    withdraw_salary(
        deps,
        env,
        stream_id,
        Some(max_amount),
        recipient,
        Some(dst_chain_id),
        sender,
        src_chain_id,
    )
}

pub fn handle_sudo_ack(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    exec_flag: bool,
    request_identifier: u64,
    _refund_amount: Coin, //TODO: do have to do anything with refund amount
) -> StdResult<Response<RouterMsg>> {
    let temp_outbound_info = TEMP_OUTBOUND_INFO_MP
        .load(deps.storage, request_identifier)
        .unwrap();
    TEMP_OUTBOUND_INFO_MP.remove(deps.storage, request_identifier);

    let mut router_pay_metadata = ROUTER_PAY_STREAM_METADATA_MP
        .load(deps.storage, temp_outbound_info.stream_id.clone())
        .unwrap();
    router_pay_metadata.is_sending = false;

    if !exec_flag {
        ROUTER_PAY_STREAM_METADATA_MP.save(
            deps.storage,
            temp_outbound_info.stream_id.clone(),
            &router_pay_metadata,
        )?;
        return Ok(Response::new().add_event(
            Event::new("PayTransferFailed")
                .add_attribute("request_identifier", request_identifier.to_string())
                .add_attribute(
                    "stream_id",
                    temp_outbound_info.stream_id.clone().to_string(),
                ),
        ));
    }

    router_pay_metadata.last_withdrawn_at = temp_outbound_info.paid_to_sec;

    ROUTER_PAY_STREAM_METADATA_MP.save(
        deps.storage,
        temp_outbound_info.stream_id.clone(),
        &router_pay_metadata,
    )?;

    Ok(Response::new().add_event(
        Event::new("PayTransferReceived")
            .add_attribute("request_identifier", request_identifier.to_string())
            .add_attribute(
                "stream_id",
                temp_outbound_info.stream_id.clone().to_string(),
            )
            .add_attribute(
                "amount_paid",
                temp_outbound_info
                    .total_amount_to_be_paid
                    .clone()
                    .to_string(),
            ),
    ))
}
