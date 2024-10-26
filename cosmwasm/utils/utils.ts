import {
  ChainRestAuthApi,
  Network,
  PrivateKey,
  getEndpointsForNetwork,
} from "@routerprotocol/router-chain-sdk-ts";
const dotenv = require("dotenv");
dotenv.config();

export function getPrivateKey(): PrivateKey {
  let mnemonic: string | undefined = process.env.MNEMONIC;
  let accPrivateKey: string | undefined = process.env.PRIVATE_KEY;

  if (process.env.IS_MNEOMIC) {
    if (mnemonic) return PrivateKey.fromMnemonic(mnemonic);
  }
  if (accPrivateKey) return PrivateKey.fromPrivateKey(accPrivateKey);
  throw new Error("Please set your PRIVATE_KEY or MNEOMIC in the .env file");
}

export async function getAccount(network: Network) {
  const endpoint = getEndpointsForNetwork(network);

  let privateKey: PrivateKey = getPrivateKey();
  const alice = privateKey.toBech32();
  const publicKey = privateKey.toPublicKey().toBase64();

  /** Get Faucet Accounts details */
  const aliceAccount = await new ChainRestAuthApi(
    endpoint.lcdEndpoint
  ).fetchAccount(alice);
  return aliceAccount;
}

export function getNetworkFromEnv(): Network {
  let network = Network.AlphaDevnet;
  if (process.env.NETWORK == "devnet") {
    network = Network.Devnet;
  } else if (process.env.NETWORK == "testnet") {
    network = Network.Testnet;
  } else if (process.env.NETWORK == "mainnet") {
    network = Network.Mainnet;
  } else if (process.env.NETWORK && process.env.NETWORK != "alpha-devnet") {
    throw new Error("Please set your NETWORK in the .env file");
  }
  return network;
}
