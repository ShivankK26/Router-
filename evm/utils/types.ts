export interface ContractData {
  address: string;
  isUpps: boolean;
  isProxy: boolean;
  verify?: boolean;
  factoryContract?: true extends this['verify'] ? any : undefined;
  blockNumber: number | undefined;
}

export interface ReturnType {
  [key: string]: ContractData;
}

export interface ChainType {
  rpc: string;
  chainName: string;
  chainId: string;
  key: string;
  gateway: string;
}

export interface ContractInfo {
  proxy: string;
  impl: string;
  isProxy: string;
  isUpps: string;
}

export interface JsonType {
  [chainId: string]: {
    [key: string]: ContractInfo;
  };
}
