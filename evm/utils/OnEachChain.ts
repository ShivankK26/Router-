import { getChainInfo } from './chain';
import { updateChainDeploymentInfo } from './utils';
const { ethers } = require('ethers');

export async function deployOnEachChains(
  onSingleChain: Function,
  chainList: string[],
  signer,
) {
  const hre = require('hardhat');

  for (const chain of chainList) {
    const { rpc, chainName, key, chainId } = getChainInfo(chain);
    const id = chainId != 'router_9000-1' ? chainId : '9000';
    console.log(`Deploying on ${chainName}...`);
    const provider = new ethers.providers.JsonRpcProvider(rpc, {
      name: key,
      chainId: parseInt(id),
    });

    // const provider = signer.provider; //TODO:remove this

    const deploymentData = await onSingleChain(
      signer.connect(provider),
      provider,
      getChainInfo(chain),
    );

    const chainInfo = {};
    await Promise.all(
      Object.keys(deploymentData).map(async (key) => {
        const { address, isProxy, isUpps, blockNumber } = deploymentData[key];

        const proxy = isProxy ? address : '';
        const impl = isProxy
          ? isUpps
            ? await hre.upgrades.erc1967.getImplementationAddress(proxy) //0x01 is the slot where the implementation address is stored
            : await provider.getStorageAt(address, '0x38') //0x38 is the slot where the implementation address is stored
          : address;

        chainInfo[key] = {};
        chainInfo[key]['proxy'] = proxy;
        chainInfo[key]['isProxy'] = isProxy;
        chainInfo[key]['isUpps'] = isUpps;
        chainInfo[key]['impl'] = impl;
        chainInfo[key]['blockNumber'] = blockNumber;
      }),
    );
    await updateChainDeploymentInfo(chain, chainInfo);
    console.log(`Deployment on ${chainName} completed!`);
  }
  console.log('Deployment Completed!!');
}

export async function verifyOnEachChains(
  onSingleChain: Function,
  chainList: string[],
  signer,
) {
  for (const chain of chainList) {
    const { rpc, chainName, key, chainId } = getChainInfo(chain);
    console.log(`Verifying on ${chainName}...`);
    const provider = new ethers.providers.JsonRpcProvider(rpc, {
      chainId: parseInt(chainId),
      name: key,
    });
    await onSingleChain(provider, getChainInfo(chain));
  }
}

export async function enrollForEachChains(
  onSingleChain: Function,
  chainList: string[],
  signer,
  addOn: string[],
) {
  for (const chain of chainList) {
    const { rpc, chainName, key, chainId } = getChainInfo(chain);
    const id = chainId == 'router_9000-1' ? '9000' : chainId;
    console.log(`Enrolling for ${chainName}...`);
    const provider = new ethers.providers.JsonRpcProvider(rpc, {
      name: key,
      chainId: parseInt(id),
    });

    // const provider = signer.provider; //TODO:remove this
    await onSingleChain(
      signer.connect(provider),
      provider,
      getChainInfo(chain),
      chainList,
      addOn,
    );

    console.log(`Enrolling For ${chainName} Completed!!`);
    console.log();
  }
}
