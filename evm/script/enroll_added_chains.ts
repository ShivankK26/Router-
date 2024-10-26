import { task } from 'hardhat/config';
import { TaskArguments } from 'hardhat/types';
import { providers } from 'ethers';
import { ChainType } from '../utils/types';
import { getSignerFromPrivateKeyOrMnemonic } from '../utils/utils';
import { getChainId, getUChainList } from '../utils/chain';
import { enrollForEachChains } from '../utils/OnEachChain';
import { enrollOnSingleChain as enrollAOnSingleChain } from './enroll_on_chains';

async function enrollOnSingleChain(
  signer,
  provider: providers.JsonRpcProvider,
  chain: ChainType,
  chainlist: string[], // chainlist entered by user
  enrollWith: string[],
) {
  const uChainList = Array.from(new Set([...chainlist, ...enrollWith]));
  await enrollAOnSingleChain(signer, provider, chain, uChainList, []);
}

async function enrollOtherWithAddOnChain(
  signer,
  provider: providers.JsonRpcProvider,
  chain: ChainType,
  chainlist: string[], // chainlist entered by user
  enrollWith: string[],
) {
  await enrollAOnSingleChain(signer, provider, chain, enrollWith, []);
}

task('ENROLLADDED_ONEACH', 'enroll contracts on provided chains')
  .addParam('chainlist', 'Description of chainlist parameter')
  .addParam('enrollwith', 'Description of added chainlist parameter')
  .addOptionalParam(
    'pkm',
    'Description of private key or mnemonic as parameter',
  )
  .setAction(async (taskArgs: TaskArguments, hre: any) => {
    const { chainlist, pkm, enrollwith } = taskArgs;
    let signer;
    if (pkm) signer = getSignerFromPrivateKeyOrMnemonic(pkm);
    else {
      // load from env
      const morp = process.env.MNEMONIC || process.env.PRIVATE_KEY;
      if (!morp) throw new Error('Provide mnemonic or private key');
      signer = getSignerFromPrivateKeyOrMnemonic(morp);
    }

    // signer = (await hre.ethers.getSigners())[0];
    const chainList: string[] = Array.from(
      new Set(chainlist.trim().split(' ')),
    );

    const enrollWith: string[] = Array.from(
      new Set(enrollwith.trim().split(' ')),
    );

    chainList.map((chain: string) => {
      if (!getChainId(chain))
        throw new Error(`invalid chain provided ${chain}`);
    });
    enrollWith.map((chain: string) => {
      if (!getChainId(chain))
        throw new Error(`invalid chain provided ${chain}`);
    });

    await enrollForEachChains(
      enrollOnSingleChain,
      getUChainList(chainList),
      signer,
      getUChainList(enrollWith),
    );
    console.log('Enrolling enrollwith chains to add on');
    await enrollForEachChains(
      enrollOtherWithAddOnChain,
      getUChainList(enrollWith),
      signer,
      getUChainList(chainList),
    );
  });
