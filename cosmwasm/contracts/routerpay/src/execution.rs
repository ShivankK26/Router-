use std::collections::HashMap;

use cosmwasm_std::{
    BankMsg, Coin, Deps, DepsMut, Env, Event, MessageInfo, ReplyOn, Response, StdError, StdResult,
    SubMsg, Uint128,
};
use router_pay_stream::routerpay::{
    ExecuteMsg, OutboundInfo, RouterPayStreamMetadata, WithDrawResponse, CREATE_OUTBOUND_REPLY_ID,
    SEPARATOR,
};
use router_wasm_bindings::{
    ethabi::{encode, ethereum_types::U256, Token},
    types::{AckType, GasPriceResponse, RequestMetaData},
    Bytes, RouterMsg, RouterQuerier, RouterQuery,
};

use crate::{
    modifiers::is_owner,
    state::{
        ACK_GAS_LIMIT, CHAIN_TYPE_MAPPING, DST_GAS_LIMIT, RELAYER_FEE, REMOTE_CONTRACT_MAPPING,
        ROUTER_PAY_STREAM_METADATA_MP, STREAM_INDEXER, TEMP_OUTBOUND_INFO, USER_STREAMS,
    },
};

pub fn handle_execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        ExecuteMsg::EnrollRemoteContract {
            chain_id,
            remote_contract,
        } => enroll_remote_contract(deps, env, info, chain_id, remote_contract),
        ExecuteMsg::MapChainType {
            chain_id,
            chain_type,
        } => map_chain_type(deps, info, chain_id, chain_type),
        ExecuteMsg::CreateStream {
            whitelisted_addresses,
            start_time,
            pay_per_month,
            recipient,
            remarks,
        } => create_stream(
            deps,
            env,
            info,
            whitelisted_addresses,
            start_time,
            pay_per_month,
            recipient,
            remarks,
        ),
        ExecuteMsg::CancelStream { stream_id, remarks } => {
            cancel_stream(deps, env, info, stream_id, remarks)
        }
        ExecuteMsg::WithdrawFunds { recipient, amount } => {
            withdraw_funds(deps, &env, &info, recipient, amount)
        }
        ExecuteMsg::DepositRoute {} => deposit_route(deps, env, info),
        ExecuteMsg::WithdrawSalary {
            stream_id,
            max_amount,
            recipient,
            dst_chain_id,
        } => withdraw_salary(
            deps,
            env.clone(),
            stream_id,
            max_amount,
            recipient,
            dst_chain_id,
            info.sender.to_string(),
            env.block.chain_id,
        ),
        ExecuteMsg::UpdateWhiteListAddress {
            stream_id,
            address,
            chain_id,
            to,
        } => update_whitelist_address(deps, env, info, stream_id, address, chain_id, to),
        ExecuteMsg::UpdateCrossChainMetadata {
            dst_gas_limit,
            ack_gas_limit,
            relayer_fee,
        } => update_crosschain_metadata(deps, env, info, dst_gas_limit, ack_gas_limit, relayer_fee),
    }
}

pub fn check_valid_route_fund(info: MessageInfo) -> StdResult<Uint128> {
    if info.funds.len() == 1
        && info.funds[0].denom == "route"
        && info.funds[0].amount > Uint128::from(0u32)
    {
        return Ok(info.funds[0].amount);
    }
    Err(StdError::GenericErr {
        msg: "Invalid_Fund".to_string(),
    })
}

pub fn get_route_balance(deps: Deps<RouterQuery>, address: String) -> StdResult<Uint128> {
    match deps.querier.query_balance(&address, "route") {
        Ok(balance) => Ok(balance.amount),
        Err(_) => Ok(Uint128::from(0u32)),
    }
}

pub fn get_oracle_gas_price(
    deps: Deps<RouterQuery>,
    chain_id: String,
) -> StdResult<GasPriceResponse> {
    let router_querier: RouterQuerier = RouterQuerier::new(&deps.querier);
    router_querier.gas_price(chain_id)

    // NOTE: Uncomment this for testing
    // Ok(GasPriceResponse { gas_price: 0 })
}

pub fn enroll_remote_contract(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    chain_id: String,
    remote_address: String,
) -> StdResult<Response<RouterMsg>> {
    is_owner(deps.as_ref(), &info)?;
    REMOTE_CONTRACT_MAPPING.save(
        deps.storage,
        chain_id.clone(),
        &remote_address.to_lowercase(),
    )?;
    Ok(Response::new().add_event(
        Event::new("EnrolledRemoteContract")
            .add_attribute("chain_id", chain_id)
            .add_attribute("contract_address", remote_address.to_lowercase()),
    ))
}

pub fn map_chain_type(
    deps: DepsMut<RouterQuery>,
    info: MessageInfo,
    chain_id: String,
    chain_type: u64,
) -> StdResult<Response<RouterMsg>> {
    is_owner(deps.as_ref(), &info)?;
    CHAIN_TYPE_MAPPING.save(deps.storage, chain_id.clone(), &chain_type)?;
    Ok(Response::new().add_event(
        Event::new("MappedChainType")
            .add_attribute("chain_id", chain_id)
            .add_attribute("chain_type", chain_type.to_string()),
    ))
}

pub fn deposit_route(
    _deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
) -> StdResult<Response<RouterMsg>> {
    check_valid_route_fund(info.clone())?;
    let response = Response::new().add_event(
        Event::new("FundDeposited")
            .add_attribute("deposited_by", info.sender.to_string())
            .add_attribute("denom", info.funds[0].denom.clone())
            .add_attribute("amount", info.funds[0].amount.to_string()),
    );
    return Ok(response);
}

pub fn get_id(chain_id: String, address: String) -> String {
    chain_id + SEPARATOR + &address
}

fn create_stream(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    whitelisted_addresses: Option<Vec<(String, String)>>,
    mut start_time: u64,
    pay_per_month: Uint128,
    recipient: String, // much be router address
    remarks: Option<String>,
) -> StdResult<Response<RouterMsg>> {
    is_owner(deps.as_ref(), &info)?;
    deps.api.addr_validate(&recipient)?;
    let pay_per_sec = pay_per_month / Uint128::from(2592000u128);
    if pay_per_sec <= Uint128::from(0u128) {
        return Err(StdError::GenericErr {
            msg: "Pay_Per_Month Should Be Greater than 2592000".to_string(),
        });
    }

    if start_time == 0 {
        start_time = env.block.time.seconds();
    }

    if start_time < env.block.time.seconds() {
        return Err(StdError::GenericErr {
            msg: "Start_Time_Should'be_In_Past".to_string(),
        });
    }

    let waddressess = whitelisted_addresses.clone().unwrap_or_default();

    let stream_id = STREAM_INDEXER.load(deps.storage).unwrap();
    STREAM_INDEXER.save(deps.storage, &(stream_id + 1u64))?;

    let mut whitelisted_addresses_map: HashMap<String, bool> = HashMap::new();
    whitelisted_addresses_map.insert(get_id(env.block.chain_id, recipient.clone()), true);
    for (chain_id, address) in waddressess.clone() {
        match REMOTE_CONTRACT_MAPPING.load(deps.storage, chain_id.clone()) {
            Ok(_) => {
                whitelisted_addresses_map.insert(get_id(chain_id, address), true);
            }
            Err(_) => {
                return Err(StdError::GenericErr {
                    msg: "Invalid_ChainId_Passed!!".to_string(),
                });
            }
        }
    }

    let router_pay_metadata = RouterPayStreamMetadata {
        recipient_owner: recipient.clone(),
        whitelisted_addresses: whitelisted_addresses_map,
        created_at: env.block.time.seconds(),
        start_time,
        pay_per_sec,
        reason: remarks.unwrap_or_default(),
        last_withdrawn_at: start_time,
        is_sending: false,
    };

    ROUTER_PAY_STREAM_METADATA_MP.save(deps.storage, stream_id.clone(), &router_pay_metadata)?;

    //add this stream to userlist
    let mut prev_user_streams = USER_STREAMS
        .load(deps.storage, recipient.clone())
        .unwrap_or_default();
    prev_user_streams.insert(stream_id, true);
    USER_STREAMS.save(deps.storage, recipient.clone(), &prev_user_streams)?;

    let mut create_event = Event::new("StreamCreated")
        .add_attribute("stream_id", stream_id.to_string())
        .add_attribute(
            "whitelisted_addresses",
            format!("{:?}", whitelisted_addresses),
        )
        .add_attribute("recipient", recipient)
        .add_attribute("created_at", router_pay_metadata.created_at.to_string())
        .add_attribute("created_by", info.sender.to_string())
        .add_attribute("start_time", router_pay_metadata.start_time.to_string())
        .add_attribute("pay_per_sec", router_pay_metadata.pay_per_sec)
        .add_attribute("pay_per_month", pay_per_month);

    if router_pay_metadata.reason != "" {
        create_event = create_event
            .clone()
            .add_attribute("reason", router_pay_metadata.reason);
    }

    Ok(Response::new().add_event(create_event))
}

fn cancel_stream(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    stream_id: u64,
    remarks: Option<String>,
) -> StdResult<Response<RouterMsg>> {
    is_owner(deps.as_ref(), &info)?;
    match ROUTER_PAY_STREAM_METADATA_MP.load(deps.storage, stream_id.clone()) {
        Ok(router_pay_metadata) => {
            let delta: u64 = env.block.time.seconds() - router_pay_metadata.last_withdrawn_at;
            let total_to_be_paid = Uint128::from(delta) * router_pay_metadata.pay_per_sec;

            let mut total_balance =
                get_route_balance(deps.as_ref(), env.contract.address.to_string()).unwrap();

            match check_valid_route_fund(info.clone()) {
                Ok(amount) => total_balance += amount,
                Err(_) => {}
            }

            if total_to_be_paid > total_balance {
                return Err(StdError::GenericErr {
                    msg: "Insufficient_Balance".to_string(),
                });
            }

            let mut response = Response::new();
            if total_to_be_paid > Uint128::from(0u128) {
                let bank_msg = BankMsg::Send {
                    to_address: router_pay_metadata.clone().recipient_owner.into(),
                    amount: vec![Coin {
                        amount: total_to_be_paid,
                        denom: "route".to_string(),
                    }],
                };
                response = response.clone().add_message(bank_msg);
            }

            ROUTER_PAY_STREAM_METADATA_MP.remove(deps.storage, stream_id);

            let mut user_info = USER_STREAMS
                .load(deps.storage, router_pay_metadata.clone().recipient_owner)
                .unwrap();
            user_info.remove(&stream_id);
            USER_STREAMS.save(
                deps.storage,
                router_pay_metadata.clone().recipient_owner,
                &user_info,
            )?;

            let cancel_events = [Event::new("StreamCancelled")
                .add_attribute("stream_id", stream_id.to_string())
                .add_attribute("cancelled_by", info.sender.to_string())
                .add_attribute("cancelled_at", env.block.time.seconds().to_string())
                .add_attribute("reason", remarks.unwrap_or_default())
                .add_attribute("paid_to", router_pay_metadata.recipient_owner)
                .add_attribute("amount_paid_to_payee", total_to_be_paid.to_string())];

            Ok(response.add_events(cancel_events))
        }
        Err(_) => Err(StdError::GenericErr {
            msg: "NOT_FOUND".to_string(),
        }),
    }
}

fn _before_withdraw(
    deps: Deps<RouterQuery>,
    env: Env,
    stream_id: u64,
    max_amount: Option<Uint128>,
    sender: String,
    chain_id: String,
) -> StdResult<WithDrawResponse> {
    match ROUTER_PAY_STREAM_METADATA_MP.load(deps.storage, stream_id.clone()) {
        Ok(router_pay_metadata) => {
            if !router_pay_metadata
                .whitelisted_addresses
                .contains_key(&get_id(chain_id, sender.clone()))
                && router_pay_metadata.recipient_owner != sender
            {
                return Err(StdError::GenericErr {
                    msg: "Not_Authorized".to_string(),
                });
            }

            if router_pay_metadata.is_sending {
                return Err(StdError::GenericErr {
                    msg: "Last_Request_Still_Pending_Wait!!".to_string(),
                });
            }
            let paid_from_sec = router_pay_metadata.last_withdrawn_at;
            let mut paid_to_sec = env.block.time.seconds();

            let delta = paid_to_sec.clone() - router_pay_metadata.last_withdrawn_at;
            let mut total_amount_to_be_paid: Uint128 =
                Uint128::from(delta) * router_pay_metadata.pay_per_sec;

            let max_amount = max_amount.unwrap_or(Uint128::from(0u32));
            if total_amount_to_be_paid > max_amount && max_amount != Uint128::from(0u32) {
                let lhs_u128: u128 = max_amount.into();
                let rhs_u128: u128 = router_pay_metadata.pay_per_sec.into();
                let result_u128 = lhs_u128.checked_div(rhs_u128).unwrap();
                match u64::try_from(result_u128) {
                    Ok(result) => {
                        paid_to_sec = result + router_pay_metadata.last_withdrawn_at;
                        total_amount_to_be_paid =
                            Uint128::from(result) * router_pay_metadata.pay_per_sec;
                    }
                    Err(_) => {
                        return Err(StdError::GenericErr {
                            msg: "Failed to convert to u64".to_string(),
                        })
                    }
                }
            }

            if total_amount_to_be_paid
                > get_route_balance(deps, env.contract.address.to_string()).unwrap()
            {
                return Err(StdError::GenericErr {
                    msg: "Contract_Don't_Have_Enough_Balance".to_string(),
                });
            }
            Ok(WithDrawResponse {
                total_amount_to_be_paid,
                paid_from_sec,
                paid_to_sec,
            })
        }
        Err(_) => Err(StdError::GenericErr {
            msg: "Stream_Not_Found".to_string(),
        }),
    }
}

fn withdraw_on_router_chain(
    deps: DepsMut<RouterQuery>,
    env: Env,
    stream_id: u64,
    max_amount: Option<Uint128>,
    recipient: String,
    sender: String,
    src_chain_id: String,
) -> StdResult<Response<RouterMsg>> {
    deps.api.addr_validate(&recipient)?;
    match _before_withdraw(
        deps.as_ref(),
        env.clone(),
        stream_id.clone(),
        max_amount,
        sender,
        src_chain_id,
    ) {
        Err(err) => Err(err),
        Ok(withdraw_response) => {
            let mut router_pay_metadata: RouterPayStreamMetadata = ROUTER_PAY_STREAM_METADATA_MP
                .load(deps.storage, stream_id.clone())
                .unwrap();
            router_pay_metadata.last_withdrawn_at = withdraw_response.paid_to_sec;

            ROUTER_PAY_STREAM_METADATA_MP.save(
                deps.storage,
                stream_id.clone(),
                &router_pay_metadata,
            )?;

            let bank_msg = BankMsg::Send {
                to_address: recipient.clone().into(),
                amount: vec![Coin {
                    amount: withdraw_response.total_amount_to_be_paid,
                    denom: "route".to_string(),
                }],
            };

            let withdraw_events: [Event; 1] = [Event::new("WithdrawOnRouterChain")
                .add_attribute("stream_id", stream_id.to_string())
                .add_attribute(
                    "amount",
                    withdraw_response.total_amount_to_be_paid.to_string(),
                )
                .add_attribute("paid_from_sec", withdraw_response.paid_from_sec.to_string())
                .add_attribute("paid_to_sec", withdraw_response.paid_to_sec.to_string())
                .add_attribute("recipient", recipient)];

            Ok(Response::new()
                .add_message(bank_msg)
                .add_events(withdraw_events))
        }
    }
}

fn withdraw_on_other_chain(
    deps: DepsMut<RouterQuery>,
    env: Env,
    stream_id: u64,
    max_amount: Option<Uint128>,
    recipient: String,
    dst_chain_id: String,
    sender: String,
    src_chain_id: String,
) -> StdResult<Response<RouterMsg>> {
    match _before_withdraw(
        deps.as_ref(),
        env.clone(),
        stream_id.clone(),
        max_amount,
        sender,
        src_chain_id.clone(),
    ) {
        Err(err) => Err(err),
        Ok(withdraw_response) => {
            let mut router_pay_metadata = ROUTER_PAY_STREAM_METADATA_MP
                .load(deps.storage, stream_id.clone())
                .unwrap();
            router_pay_metadata.is_sending = true;
            ROUTER_PAY_STREAM_METADATA_MP.save(
                deps.storage,
                stream_id.clone(),
                &router_pay_metadata,
            )?;

            let dst_contract_add_res =
                REMOTE_CONTRACT_MAPPING.load(deps.storage, dst_chain_id.clone());
            if let Err(_) = dst_contract_add_res {
                return Err(StdError::GenericErr {
                    msg: "Dst_Chain_Not_Supported".to_string(),
                });
            }

            let dst_contract_add: String = dst_contract_add_res.unwrap();

            let encoded_payload: Vec<u8> = encode(&[
                Token::Uint(U256::from(Uint128::u128(
                    &withdraw_response.total_amount_to_be_paid,
                ))),
                Token::String(recipient.clone()),
            ]);

            let request_packet: Bytes = encode(&[
                Token::String(dst_contract_add),
                Token::Bytes(encoded_payload),
            ]);

            let dst_gas_price: u64 = get_oracle_gas_price(deps.as_ref(), dst_chain_id.clone())
                .unwrap()
                .gas_price;

            let ack_gas_price: u64 = get_oracle_gas_price(deps.as_ref(), env.block.chain_id)
                .unwrap()
                .gas_price;

            let request_metadata: RequestMetaData = RequestMetaData {
                dest_gas_limit: DST_GAS_LIMIT.load(deps.storage).unwrap(),
                dest_gas_price: dst_gas_price,
                ack_gas_limit: ACK_GAS_LIMIT.load(deps.storage).unwrap(),
                ack_gas_price,
                relayer_fee: RELAYER_FEE.load(deps.storage).unwrap(),
                ack_type: AckType::AckOnBoth,
                is_read_call: false,
                asm_address: String::from(""),
            };

            let i_send_request: RouterMsg = RouterMsg::CrosschainCall {
                version: 1,
                route_amount: withdraw_response.total_amount_to_be_paid,
                route_recipient: recipient.clone(),
                dest_chain_id: dst_chain_id.clone(),
                request_metadata: request_metadata.get_abi_encoded_bytes(),
                request_packet,
            };

            let isend_submessage: SubMsg<RouterMsg> = SubMsg {
                id: CREATE_OUTBOUND_REPLY_ID,
                msg: i_send_request.into(),
                gas_limit: None,
                reply_on: ReplyOn::Always,
            };

            let temp_outbound_info = OutboundInfo {
                stream_id: stream_id.clone(),
                total_amount_to_be_paid: withdraw_response.total_amount_to_be_paid,
                paid_to_sec: withdraw_response.paid_to_sec,
            };
            TEMP_OUTBOUND_INFO.save(deps.storage, &temp_outbound_info)?;

            let withdraw_events: [Event; 1] = [Event::new("WithdrawOnOtherChain")
                .add_attribute("stream_id", stream_id.to_string())
                .add_attribute(
                    "amount",
                    withdraw_response.total_amount_to_be_paid.to_string(),
                )
                .add_attribute("paid_from_sec", withdraw_response.paid_from_sec.to_string())
                .add_attribute("paid_to_sec", withdraw_response.paid_to_sec.to_string())
                .add_attribute("recipient", recipient)];

            Ok(Response::new()
                .add_submessage(isend_submessage)
                .add_events(withdraw_events))
        }
    }
}

pub fn withdraw_salary(
    deps: DepsMut<RouterQuery>,
    env: Env,
    stream_id: u64,
    max_amount: Option<Uint128>,
    recipient: String,
    dst_chain_id: Option<String>,
    sender: String,
    src_chain_id: String,
) -> StdResult<Response<RouterMsg>> {
    if let Some(chain_id) = dst_chain_id {
        if chain_id.len() == 0 || chain_id == env.block.chain_id {
            return withdraw_on_router_chain(
                deps,
                env,
                stream_id,
                max_amount,
                recipient,
                sender,
                src_chain_id,
            );
        }

        return withdraw_on_other_chain(
            deps,
            env.clone(),
            stream_id,
            max_amount,
            recipient,
            chain_id,
            sender,
            src_chain_id,
        );
    }

    return withdraw_on_router_chain(
        deps,
        env,
        stream_id,
        max_amount,
        recipient,
        sender,
        src_chain_id,
    );
}

pub fn update_whitelist_address(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    stream_id: u64,
    address: String,
    chain_id: String,
    to: bool,
) -> StdResult<Response<RouterMsg>> {
    match ROUTER_PAY_STREAM_METADATA_MP.load(deps.storage, stream_id) {
        Ok(mut routerpay_metadata) => {
            if routerpay_metadata.recipient_owner != info.sender.clone().to_string() {
                return Err(StdError::GenericErr {
                    msg: "Unauthorized".to_string(),
                });
            }
            let id = get_id(chain_id.clone(), address.to_lowercase());

            let mut response = Response::new();
            if to {
                if routerpay_metadata.whitelisted_addresses.contains_key(&id) {
                    return Err(StdError::GenericErr {
                        msg: "Already_WhiteListed".to_string(),
                    });
                }

                routerpay_metadata.whitelisted_addresses.insert(id, to);

                response = response.clone().add_event(
                    Event::new("WhiteListedAddress")
                        .add_attribute("address", address.to_lowercase())
                        .add_attribute("chain_id", chain_id)
                        .add_attribute("whitelisted_by", info.sender.to_string()),
                );
            } else {
                if !routerpay_metadata.whitelisted_addresses.contains_key(&id) {
                    return Err(StdError::GenericErr {
                        msg: "Not_WhiteListed".to_string(),
                    });
                }

                routerpay_metadata
                    .whitelisted_addresses
                    .remove(&get_id(chain_id.clone(), address.to_lowercase()));

                response = response.clone().add_event(
                    Event::new("BlackListedAddress")
                        .add_attribute("address", address.to_lowercase())
                        .add_attribute("chain_id", chain_id)
                        .add_attribute("blacklisted_by", info.sender.to_string()),
                );
            }
            ROUTER_PAY_STREAM_METADATA_MP.save(deps.storage, stream_id, &routerpay_metadata)?;
            Ok(response)
        }
        Err(_) => Err(StdError::GenericErr {
            msg: "Stream_Not_Found".to_string(),
        }),
    }
}

fn update_crosschain_metadata(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    dst_gas_limit: Option<u64>,
    ack_gas_limit: Option<u64>,
    relayer_fee: Option<Uint128>,
) -> StdResult<Response<RouterMsg>> {
    is_owner(deps.as_ref(), &info)?;

    let response = Response::new();
    let mut update_event = Event::new("CrossChainMetaUpdated");
    if let Some(ack_glimit) = ack_gas_limit {
        ACK_GAS_LIMIT.save(deps.storage, &ack_glimit)?;
        update_event = update_event
            .clone()
            .add_attribute("ack_gas_limit", ack_glimit.to_string());
    }

    if let Some(dst_glimit) = dst_gas_limit {
        DST_GAS_LIMIT.save(deps.storage, &dst_glimit)?;
        update_event = update_event
            .clone()
            .add_attribute("dst_gas_limit", dst_glimit.to_string());
    }

    if let Some(rfee) = relayer_fee {
        RELAYER_FEE.save(deps.storage, &rfee)?;
        update_event = update_event
            .clone()
            .add_attribute("relayer_fee", rfee.to_string());
    }
    Ok(response.add_event(update_event))
}

pub fn withdraw_funds(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    recipient: String,
    amount: Uint128,
) -> StdResult<Response<RouterMsg>> {
    is_owner(deps.as_ref(), &info)?;

    let bank_msg = BankMsg::Send {
        to_address: recipient.clone(),
        amount: vec![Coin {
            amount,
            denom: "route".to_string(),
        }],
    };
    Ok(Response::new().add_message(bank_msg).add_event(
        Event::new("WithdrawFromContract")
            .add_attribute("recipient", recipient)
            .add_attribute("amount", amount),
    ))
}
