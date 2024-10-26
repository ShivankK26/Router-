<h1>Cosmwasm Contracts</h1>

**<h2>Router Pay Streaming Contract</h2>**

The Router Pay Streaming contract is a smart contract that facilitates salary streaming between a payer and a payee. It enables the creation, management, and withdrawal of salary streams across different chains.

This documentation provides a guide for developers to understand and interact with the Router Pay Streaming contract. It covers the contract's message signatures, deployment process, and usage instructions.

**<h2>Deployment Process</h2>**

Regardless of the deployment option you choose, you can now proceed to interact with the Router Pay Streaming contract by executing messages or querying its state.

**<h3>Option 1: Deploying Locally</h3>**

To deploy the Router Pay Streaming contract locally, follow these steps:

- Run **`yarn deploy:routerchain`** to deploy the contract on the router chain. This command will deploy the contract and save the deployment information in the **`deployment/deployment.json`** file. The contract will be initialized with default values, where the owner will be set to the deployer's address.

* Import the contract on the router station using the contract address from the **`deployment/deployment.json`** file. This will allow you to execute and query the contract directly on the router station.

**<h3>Option 2: Manual Deployment</h3>**
If you prefer to manually deploy the contract, follow these steps:

- Build the contract by running sh shell/build.sh. This will generate a .wasm file in the artifacts directory.
- Deploy the generated .wasm file on the router station using the provided code. Take note of the code_id assigned to the deployed contract.
- Initialize the contract by passing the initial message as follows:

  ```json
  {
    "owner": "router1...",
    "relayer_fee": "1000000000",
    "dst_gas_limit": 500000,
    "ack_gas_limit": 500000
  }
  ```

* The contract is now deployed and initialized on the router station, ready for execution and querying.

**<h2>Execute Messages</h2>**
The Router Pay Streaming contract supports various messages for different operations. Each message has a specific structure that developers need to follow when interacting with the contract.

**<h3>CreateStream</h3>**

The **\`CreateStream\`** function allows the payer to create a salary stream for a payee with or without specifying a reason. The parameters for this function are:

- **whitelisted_addresses**: `Option<Vec<(String,String)>>` Addresses of chains where the payee can withdraw funds.
- **start_time**: `u64` The start time of the stream.
- **pay_per_month**: `Uint128` The payment amount per month, internally we convert it in pay_per_sec
- **recipient**: `String` Owner of stream who can do whitelist address or blacklist
- **remarks**: `Option<String>` Creator can set remarks if any for stream, e.g for what reason stream is created

_CreateStream Message Structure_ :

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

**<h3>CancelStream</h3>**

The **\`CancelStream\`** function allows the payer to cancel a salary stream at any time. When a stream is canceled, any remaining route tokens are transferred to the payee's owner address, and the stream is closed. The parameters for this function are:

- stream_id: `u64` The ID of the stream to be canceled.
- remarks: `Option<String>` Payer can pass reason for cancelling

_CancelStream Message Structure_ :

```json
{
  "cancel_stream": {
    "stream_id": 1234,
    "remarks": "Optional remarks"
  }
}
```

**<h3>DepositRoute</h3>**

The **\`DepositRoute function\`** enables any user to deposit route tokens into the smart contract. This function is used to add funds to the contract for salary payments.

_DepositRoute Message Structure_ :

```json
{
  "deposit_route": {}
}
```

**<h3>WithdrawSalary</h3>**

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

**<h3>EnrollRemoteContract</h3>**

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

**<h3>MapChainType</h3>**

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

**<h3>WithdrawFunds</h3>**

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

**<h3>UpdateWhiteListAddress</h3>**

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

**<h3>UpdateCrossChainMetadata</h3>**

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

**<h2>Query Messages</h2>**

**<h3>GetContractVersion</h3>**

The **\`GetContractVersion\`** function Fetches the contract version.

_GetContractVersion Message Structure_ :

```json
{
  "get_contract_version": {}
}
```

**<h3>GetRemoteContract</h3>**

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

**<h3>GetRouterPayMetadata</h3>**

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

**<h3>GetOwner</h3>**

The **\`GetOwner\`** function Fetches the contract owner.

_GetOwner Message Structure_ :

```json
{
  "get_owner": {}
}
```

**<h3>GetCrossChainMetadata</h3>**

The **\`GetCrossChainMetadata\`** function fetches the crosschain metadata contains dst_gas_limit,ack_gas_limit and relayer_fee.

_GetCrossChainMetadata Message Structure_ :

```json
{
  "get_cross_chain_metadata": {}
}
```

**<h3>GetStreams</h3>**

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

**<h3>GetStreamWhiteListAddress</h3>**

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

**<h3>GetUserStreamIds</h3>**

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

**<h3>GetUserStreamsInfo</h3>**

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

**<h3>IsWhiteListed</h3>**

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

**<h3>GetAccumulatedAmount</h3>**

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
