import { ContractFactory, Contract, providers, Wallet, Signer } from 'ethers';
const { upgrades } = require('hardhat');

interface DeployOptions {
  unsafeAllow?: string[];
  provider?: providers.Provider;
  signer?: Signer;
}

export class Deploy {
  async deployProxy(
    factoryContract: ContractFactory,
    constructorArguments: any[],
    opt: DeployOptions = {},
  ): Promise<Contract> {
    // Deployment scenario
    const provider = opt.provider || new providers.JsonRpcProvider();
    const unsafeAllow = opt.unsafeAllow || [];

    const contract = await upgrades.deployProxy(
      factoryContract,
      constructorArguments,
      {
        unsafeAllow,
        provider,
      },
    );
    await contract.deployed();

    return contract;
  }
}
