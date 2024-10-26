import { task } from 'hardhat/config';
import { TaskArguments } from 'hardhat/types';
import { providers } from 'ethers';
import { ChainType } from '../utils/types';
import {
  getContractAddress,
  getSignerFromPrivateKeyOrMnemonic,
} from '../utils/utils';
import { getChainId, getChainInfo } from '../utils/chain';
import { enrollForEachChains } from '../utils/OnEachChain';
import { routerPayContractAddressOnRouterChain } from '../config';

const gasLimit = 1000000;
export async function enrollOnSingleChain(
  signer,
  provider: providers.JsonRpcProvider,
  chain: ChainType,
  chainlist: string[], // chainlist entered by user
  addOn: string[],
) {
  const { ethers } = require('hardhat');
  const network = await provider.getNetwork();

  const RouterPay_Factory = (
    await ethers.getContractFactory('RouterPay')
  ).connect(signer);

  let poapNFTAddress;
  let routerPay;
  let tx;

  routerPay = await RouterPay_Factory.attach(poapNFTAddress);

  const prevAdd = await routerPay.routerPayOnRouterChainAddress();
  if (
    prevAdd.toLowerCase() ==
    routerPayContractAddressOnRouterChain?.toLowerCase()
  ) {
    console.log('Already Updated for chain -> ', chain);
    return;
  }
  tx = await routerPay.updateRouterPayContract(
    routerPayContractAddressOnRouterChain,
    {
      gasLimit,
    },
  );
  console.log(
    `UpdateRouterPayContract[-> ${chain}]: tx send with hash `,
    tx.hash,
  );
  await tx.wait();
  console.log(`UpdateRouterPayContract[-> ${chain}]: tx went successfully`);
  console.log();
}
task('ENROLL_ONEACH', 'enroll contracts on provided chains')
  //   .addParam('contractlist', 'Description of contract list parameter')
  .addParam('chainlist', 'Description of chainlist parameter')
  .addOptionalParam(
    'pkm',
    'Description of private key or mnemonic as parameter',
  )
  .setAction(async (taskArgs: TaskArguments, hre: any) => {
    const { chainlist, pkm } = taskArgs;
    let signer;
    if (pkm) signer = getSignerFromPrivateKeyOrMnemonic(pkm);
    else {
      // load from env
      const morp = process.env.MNEMONIC || process.env.PRIVATE_KEY;
      if (!morp) throw new Error('Provide mnemonic or private key');
      signer = getSignerFromPrivateKeyOrMnemonic(morp);
    }

    const chainList: string[] = Array.from(
      new Set(chainlist.trim().split(' ')),
    );

    chainList.map((chain: string) => {
      if (!getChainId(chain)) throw new Error('invalid chain provided');
    });

    await enrollForEachChains(enrollOnSingleChain, chainList, signer, []);
  });
