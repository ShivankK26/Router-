<h1>EVM Contracts  </h1>

**<h2>Router Pay Streaming Contract</h2>**

The Router Pay Streaming contract is a smart contract that facilitates salary streaming between a payer and a payee. It enables the creation, management, and withdrawal of salary streams across different chains.

**<h2>Functionality of Router Pay Streaming</h2>**

**<h3>WithdrawFund</h3>**

The **\`WithdrawFund\`** function is called on the EVM chain to initiate a withdrawal request. It relays the request to the router chain's smart contract for processing. The parameters for this function are:

- **dstChainId**: `string` The ID of the destination chain where the payee wants to receive the route tokens. If you pass an empty string, the withdrawal will happen on the router chain.
- **recipient**: `string` The address of the recipient who will receive the route tokens.
- **maxAmount**: `uint64` The maximum amount to receive. If you pass zero, all accumulated tokens will be transferred to the recipient.
- **streamId**: `uint64` The ID of the stream from which to withdraw.
- **requestMetadata**: `bytes` request metadata required for the withdrawal.

1. **Handling Withdrawal on Router Chain**

   When a withdrawal request is received on the router chain, the smart contract handles it based on whether the destination chain is the router chain or dst_chain is an empty string. In such cases, the withdrawal will be processed on the router chain itself.

2. **Handling Withdrawal on Other Chain**
   After a withdrawal request is processed, if the destination chain is some other chain, the IReceive sudo function on the router will send an ISend request to the destination chain. Upon a successful transaction on the destination chain, an acknowledgment will be received on the router chain, and the router pay metadata will be updated accordingly.

**<h3>Note:</h3>**

You can create the requestMetadata parameter in `TypeScript` or `JavaScript` using the following function:

```ts
function getRequestMetadata(
  destGasLimit: number,
  destGasPrice: number,
  ackGasLimit: number,
  ackGasPrice: number,
  relayerFees: string,
  ackType: number,
  isReadCall: boolean,
  asmAddress: string,
): string {
  return ethers.utils.solidityPack(
    [
      'uint64',
      'uint64',
      'uint64',
      'uint64',
      'uint128',
      'uint8',
      'bool',
      'string',
    ],
    [
      destGasLimit,
      destGasPrice,
      ackGasLimit,
      ackGasPrice,
      relayerFees,
      ackType,
      isReadCall,
      asmAddress,
    ],
  );
}
```
