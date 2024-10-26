import fs from "fs-extra";
import { init_wasm_code } from "../utils/init_wasm";
import { upload_wasm_code } from "../utils/upload_wasm";
import { getChainInfoForNetwork } from "@routerprotocol/router-chain-sdk-ts";
import path from "path";
import { getAccount, getNetworkFromEnv } from "../utils/utils";
const deploymentPath = path.join(__dirname, "../deployment");
require("dotenv").config();

async function main() {
  let network = getNetworkFromEnv();
  const chainInfo = getChainInfoForNetwork(network);
  const chainId = chainInfo.chainId;

  let wasmSuffix = ".wasm";
  if (process.env.IS_APPLE_CHIPSET == "YES") {
    wasmSuffix = "-aarch64.wasm";
  }

  console.log("Uploading RouterPayStream Contract, Please wait...");
  const routerPayCodeId = await upload_wasm_code(
    network,
    chainId,
    __dirname + "/../artifacts/router_pay" + wasmSuffix
  );
  console.log("RouterPayStream CodeId: ", routerPayCodeId);

  console.log("Initiating RouterPayStream Contract, Please wait...");
  const deployerAccount = await getAccount(network);
  const routerPayInitMsg = JSON.stringify({
    owner: deployerAccount.account.base_account.address,
    relayer_fee: "10",
    dst_gas_limit: 500000,
    ack_gas_limit: 500000,
  });
  const routerPayAddress = await init_wasm_code(
    network,
    routerPayCodeId,
    "Wrapped Route",
    routerPayInitMsg,
    chainId
  );
  console.log("RouterPayStreaming Address: ", routerPayAddress);

  await fs.ensureDir(deploymentPath);
  await fs.writeJSON(`${deploymentPath}/deployment.json`, {
    routerPay: {
      codeId: routerPayCodeId,
      address: routerPayAddress,
    },
  });
}

main();
