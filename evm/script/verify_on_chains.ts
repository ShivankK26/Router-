import { task } from 'hardhat/config';
import { TaskArguments } from 'hardhat/types';
import { providers } from 'ethers';
import { ChainType } from '../utils/types';
import {
  getContractAddress,
  getSignerFromPrivateKeyOrMnemonic,
} from '../utils/utils';
import { getChainId } from '../utils/chain';
import { verifyOnEachChains } from '../utils/OnEachChain';

async function verifyOnSingleChain(
  provider: providers.JsonRpcProvider,
  chain: ChainType,
) {
  const { run } = require('hardhat');

  const routerPayAddress = (await getContractAddress('routerPay', chain.key))
    ?.proxy;

  console.log('Verifying POAP NFT Contract...');
  await run(`verify:verify`, {
    address: routerPayAddress,
    constructorArguments: [],
    provider,
  });
  console.log('Router Pay Contract Verified!!');
}

task('VERIFY_ONEACH', 'verify contracts on provided chains')
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

    signer = (await hre.ethers.getSigners())[0];

    const chainList: string[] = Array.from(
      new Set(chainlist.trim().split(' ')),
    );

    chainList.map((chain: string) => {
      if (!getChainId(chain)) throw new Error('invalid chain provided');
    });

    await verifyOnEachChains(verifyOnSingleChain, chainList, signer);
  });
