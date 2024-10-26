# Router Pay Streaming Contract

The Router PayStream Contract is a Smart Contract that facilitates Salary Streaming between a Payer and a Payee. It enables the Creation, Management, and Withdrawal of Salary Streams across Different Chains.

This Documentation Provides a Guide for Developers to Understand and Interact with the Router PayStream Contract. It Covers the Contract's Message Signatures, Deployment Process, and Usage Instructions.

## Execute Messages

The Router PayStream Contract Supports various Messages for Different Operations. Each Message has a Specific Structure that Developers need to follow when Interacting with the Contract.

### CreateStream

The `CreateStream` function allows the Payer to Create a Salary Stream for a Payee with or without Specifying a reason. The Parameters for this function are -

- **whitelisted_addresses**: `Option<Vec<(String,String)>>` Addresses of Chains where the Payee can Withdraw funds.
- **start_time**: `u64` The Start time of the Stream.
- **pay_per_month**: `Uint128` The Payment amount per month, internally we Convert it in `pay_per_sec`.
- **recipient**: `String` Owner of Stream who can do Whitelist Address or Blacklist.
- **remarks**: `Option<String>` Creator can set remarks if any for Stream, e.g for what reason Stream is created.

_CreateStream Message Structure_:

```json
{
  "create_stream": {
    "whitelisted_addresses": [
      {
        "chain_id": "cosmos",
        "address": "cosmos1..."
      },
      {
        "chain_id": "ethereum",
        "address": "0x..."
      }
    ],
    "start_time": 1623391200,
    "pay_per_month": "1000000",
    "recipient": "router14rvuwugcmd94uf6ajkslwh5kc8kl5kxgdmkpze",
    "remarks": "Optional remarks"
  }
}
```

### CancelStream

The `CancelStream` function allows the Payer to Cancel a Salary Stream at anytime. When a Stream is Canceled, any remaining ROUTE Tokens are transferred to the Payee's Owner Address, and the Stream is Closed. The Parameters for this function are -

- **stream_id**: `u64` The ID of the Stream to be Canceled.
- **remarks**: `Option<String>` Payer can Pass reason for Cancelling.

_CancelStream Message Structure_:

```json
{
  "cancel_stream": {
    "stream_id": 1234,
    "remarks": "Optional remarks"
  }
}
```

### DepositRoute

The `DepositRoute function` enables any User to Deposit ROUTE Tokens into the Smart Contract. This function is used to add funds to the Contract for Salary Payments.

_DepositRoute Message Structure_:

```json
{
  "deposit_route": {}
}
```

### WithdrawSalary

The **\`WithdrawSalary\`** function allows the payee to initiate a withdrawal from the salary stream on the router chain or on other chain. The parameters for this function are:

- **stream_id**: `u64` The ID of the stream from which to withdraw.
- **recipient**: `String` The address of the recipient who will receive the withdrawn route tokens.
- **dst_chain_id**: `Option<String>` The chain ID to which the payee wants to withdraw the route, if 'None' or 'Router Chain ID' is passed, the route will be transferred to the router chain; otherwise, it will be transferred to the destination chain if it is enrolled, or the call will be reverted.
- **max_amount**: `Option<Uint128>` Max amount to withdraw from salary, if passed zero or None then it will withdraw all accumulated amount

_WithdrawSalary Message Structure_ :

```json
{
  "withdraw_salary": {
    "stream_id": 1234,
    "recipient": "router14rvuwugcmd94uf6ajkslwh5kc8kl5kxgdmkpze",
    "dst_chain_id": "cosmos",
    "max_amount": "1000000"
  }
}
```

### EnrollRemoteContract

The **\`EnrollRemoteContract\`** function allows the payee to initiate a withdrawal from the salary stream on the router chain or on other chain. The parameters for this function are:

- **chain_id**: `String` dst chain id
- **remote_contract**: `String` contract address on dst chain

_EnrollRemoteContract Message Structure_ :

```json
{
  "enroll_remote_contract": {
    "chain_id": "43113",
    "remote_contract": "0x2C8e4027D332ac6f2210A6517c25CcE8a2c83e0e"
  }
}
```

### MapChainType

The **\`MapChainType\`** function allows the contract owner to map chain type. The parameters for this function are:

- **chain_id**: `String` dst chain id
- **chain_type**: u64 chain type of dst chain

_MapChainType Message Structure_ :

```json
{
  "map_chain_type": {
    "chain_id": "43113",
    "chain_type": 1
  }
}
```

### WithdrawFunds

The **\`WithdrawFunds\`** function allows the owner to withdraw funds from the contract. The parameters for this function are:

- **recipient**: `String` router address of recipient to which amount will be transferred
- **amount**: `Uint128` amount to be withdrawn

_WithdrawFunds Message Structure_ :

```json
{
  "withdraw_funds": {
    "recipient": "router14rvuwugcmd94uf6ajkslwh5kc8kl5kxgdmkpze",
    "amount": "1000000"
  }
}
```

### UpdateWhiteListAddress

The **\`UpdateWhiteListAddress\`** function allows the stream owner to whitelist or blacklist a address for their stream. The parameters for this function are:

- **stream_id**: `u64` stream id for which this operation to be applied
- **address**: `String` address to be whitelisted
- **chain_id**: `String` chain id of provided address
- **to**: `bool` true means whitelist and false means blacklist

_UpdateWhiteListAddress Message Structure_ :

```json
{
  "update_white_list_address": {
    "stream_id": 1234,
    "address": "router1...",
    "chain_id": "router_9601-1",
    "to": true
  }
}
```

### UpdateCrossChainMetadata

The **\`UpdateCrossChainMetadata\`** function allows the ownwer of contract to update metadata such as ack_gas_limit or dst_gas_limit or relayer_fee. The parameters for this function are:

- **dst_gas_limit**: `Option<u64>` dst gas limit for IReceive, it's optional if passed Some(\_) then only it will update the dst_gas_limit
- **ack_gas_limit**: `Option<u64>` ack gas limit for sudo msg IAck on rotuer chain
- **relayer_fee**: `Option<Uint128>` realyer fee for relaying the ISend message to dst chain

_UpdateCrossChainMetadata Message Structure_ :

```json
{
  "update_cross_chain_metadata": {
    "dst_gas_limit": 1000000,
    "ack_gas_limit": 1000000,
    "relayer_fee": "10"
  }
}
```

## Query Messages

### GetContractVersion

The **\`GetContractVersion\`** function Fetches the contract version.

_GetContractVersion Message Structure_ :

```json
{
  "get_contract_version": {}
}
```

### GetRemoteContract

The **\`GetRemoteContract\`** function Fetches the contract address of provided chainid. The parameters for this function is:

- **chain_id**: `String` dst chain id for which the query is being made.

_GetRemoteContract Message Structure_ :

```json
{
  "get_remote_contract": {
    "chain_id": "43113"
  }
}
```

### GetRouterPayMetadata

The **\`GetRouterPayMetadata\`** function Fetches the metadat mapped to provided stream id. The parameters for this function is:

- **stream_id**: `u64` stream id for which the query is being made.

_GetRouterPayMetadata Message Structure_ :

```json
{
  "get_router_pay_metadata": {
    "stream_id": 1234
  }
}
```

### GetOwner

The **\`GetOwner\`** function Fetches the contract owner.

_GetOwner Message Structure_ :

```json
{
  "get_owner": {}
}
```

### GetCrossChainMetadata

The **\`GetCrossChainMetadata\`** function fetches the crosschain metadata contains dst_gas_limit,ack_gas_limit and relayer_fee.

_GetCrossChainMetadata Message Structure_ :

```json
{
  "get_cross_chain_metadata": {}
}
```

### GetStreams

The **\`GetStreams\`** function fetches stream metadata for the range provided. The parameters for this function are:

- **from**: `u64` stream id from where to fetch
- **to**: `Option<u64>` end stream id to where to fetch if not provided then it will fetches next 10 streams

_GetStreams Message Structure_ :

```json
{
  "get_streams": {
    "from": 0,
    "to": 100
  }
}
```

### GetStreamWhiteListAddress

The **\`GetStreamWhiteListAddress\`** function fetches all whitelist address provided stream id, returns `Vec<(String,String)>`. The parameters for this function is:

- **stream_id**: `u64` stream id for which the query is being made.

_GetStreamWhiteListAddress Message Structure_ :

```json
{
  "get_stream_white_list_address": {
    "stream_id": 1234
  }
}
```

### GetUserStreamIds

The **\`GetUserStreamIds\`** function fetches all stream ids associated with a specific router address. The parameters for this function is:

- **address**: `String` user router address to get all stream ids

_GetUserStreamIds Message Structure_ :

```json
{
  "get_user_stream_ids": {
    "address": "router..."
  }
}
```

### GetUserStreamsInfo

The **\`GetUserStreamsInfo\`** function fetches detailed information about the streams associated with a specific address. The parameters for this function is:

- **address**: `String` user router address to get all stream ids

_GetUserStreamsInfo Message Structure_ :

```json
{
  "get_user_streams_info": {
    "address": "router14rvuwugcmd94uf6ajkslwh5kc8kl5kxgdmkpze"
  }
}
```

### IsWhiteListed

The **\`IsWhiteListed\`** function checks if a specific address is whitelisted for a particular stream on a specific chain. The parameters for this function are:

- **stream_id**: `u64` stream if for which this checks applies
- **chain_id**: `String` chain id where address belongs to
- **address**: `String` address for provided chain id

_IsWhiteListed Message Structure_ :

```json
{
  "is_white_listed": {
    "stream_id": 1234,
    "chain_id": "43113",
    "address": "0x5561b5eaa45573011343545e3756a6735899dff3"
  }
}
```

### GetAccumulatedAmount

The **\`GetAccumulatedAmount\`** function fetches the accumulated amount for a specific stream. The parameters for this function is:

- **stream_id**: `u64` stream if for which this checks applies

_GetAccumulatedAmount Message Structure_ :

```json
{
  "get_accumulated_amount": {
    "stream_id": 1234
  }
}
```

These functions and their functionalities form the core of Router Pay Streaming, allowing for the creation, management, and withdrawal of salary streams on different chains.

**Note:** Please note that these examples are for illustration purposes only. Replace the placeholder values with the actual addresses, IDs, and other relevant information specific to your use case.
