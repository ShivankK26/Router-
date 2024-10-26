import { task } from 'hardhat/config';
import { TaskArguments } from 'hardhat/types';
import { providers } from 'ethers';
import { ChainType, ReturnType } from '../utils/types';
import {
  getPrevDeployment,
  getSignerFromPrivateKeyOrMnemonic,
} from '../utils/utils';
import { getChainId } from '../utils/chain';
import { deployOnEachChains } from '../utils/OnEachChain';
import { routerPayContractAddressOnRouterChain } from '../config';

async function deployOnSingleChain(
  signer,
  provider: providers.JsonRpcProvider,
  chainInfo: ChainType,
): Promise<ReturnType> {
  const hre = require('hardhat');
  hre.provider = provider;
  hre.network.provider = provider;

  const { ethers, upgrades } = hre;
  const network = await provider.getNetwork();

  const opts = {
    // unsafeAllow: ['external-library-linking'], // any unsafe operations
    // timeout: 100000000,
    pollingInterval: 5000,
    signer,
    provider,
  };

  const RouterPay_Factory = await (
    await ethers.getContractFactory('RouterPay')
  ).connect(signer);
  const gatewayContractAddress = chainInfo.gateway;

  const _chainId = getChainId(network.name);

  const routerPay = await upgrades.deployProxy(
    RouterPay_Factory,
    [routerPayContractAddressOnRouterChain, gatewayContractAddress, _chainId],
    opts,
  );

  const rTxReceipt = await routerPay.connect(signer).deployed();
  const rBlockNumber = (
    await provider.getTransaction(rTxReceipt.deployTransaction.hash)
  ).blockNumber;

  console.log('Router Pay contract deployed to ', routerPay.address);

  return {
    routerPay: {
      blockNumber: rBlockNumber,
      address: routerPay.address,
      isProxy: true, // by default false
      isUpps: true, // by default true
    },
  };
}

task('DEPLOY_ONEACH', 'deploy contracts on provided chains')
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

    await deployOnEachChains(deployOnSingleChain, chainList, signer);

    let args: {
      chainlist: string;
      pkm?: string;
      enrollwith?: string;
    } = {
      chainlist,
    };
    if (pkm)
      args = {
        ...args,
        pkm,
      };

    console.log();
    const prevDeployment = await getPrevDeployment();
    if (Object.keys(prevDeployment).length) {
      let enrollwith = '';
      Object.keys(prevDeployment).map((key) => (enrollwith += `${key} `));
      args = {
        ...args,
        enrollwith,
      };
      await hre.run('ENROLLADDED_ONEACH', args);
    } else {
      await hre.run('ENROLL_ONEACH', args);
    }
  });
