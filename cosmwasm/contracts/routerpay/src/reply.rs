use cosmwasm_std::{from_binary, Reply, StdError, SubMsgResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use router_pay_stream::routerpay::CREATE_OUTBOUND_REPLY_ID;
use router_wasm_bindings::{types::CrosschainRequestResponse, RouterMsg, RouterQuery};

use crate::state::{ROUTER_PAY_STREAM_METADATA_MP, TEMP_OUTBOUND_INFO, TEMP_OUTBOUND_INFO_MP};

fn handle_sub_message_failed(deps: DepsMut<RouterQuery>) -> StdResult<Response<RouterMsg>> {
    let temp_outbound_info = TEMP_OUTBOUND_INFO.load(deps.storage).unwrap();
    let mut router_pay_metadata = ROUTER_PAY_STREAM_METADATA_MP
        .load(deps.storage, temp_outbound_info.stream_id.clone())
        .unwrap();
    router_pay_metadata.is_sending = false;
    ROUTER_PAY_STREAM_METADATA_MP.save(
        deps.storage,
        temp_outbound_info.stream_id.clone(),
        &router_pay_metadata,
    )?;
    TEMP_OUTBOUND_INFO.remove(deps.storage);
    Ok(Response::new())
}

pub fn handle_reply(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    msg: Reply,
) -> StdResult<Response<RouterMsg>> {
    match msg.id {
        CREATE_OUTBOUND_REPLY_ID => match msg.result {
            SubMsgResult::Ok(msg_result) => match msg_result.data {
                Some(binary_data) => {
                    let cross_chain_req_res: CrosschainRequestResponse =
                        from_binary(&binary_data).unwrap();

                    let temp_outbound_info = TEMP_OUTBOUND_INFO.load(deps.storage).unwrap();
                    TEMP_OUTBOUND_INFO_MP.save(
                        deps.storage,
                        cross_chain_req_res.request_identifier,
                        &temp_outbound_info,
                    )?;

                    //TODO: take gas fee from sender itself
                    Ok(Response::<RouterMsg>::new())
                }
                None => {
                    handle_sub_message_failed(deps)?;
                    return Err(StdError::GenericErr {
                        msg: "No_Binary_Data_Found".to_string(),
                    });
                }
            },
            SubMsgResult::Err(err) => {
                handle_sub_message_failed(deps)?;
                Err(StdError::GenericErr {
                    msg: err.to_string(),
                })
            }
        },
        id => return Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}
