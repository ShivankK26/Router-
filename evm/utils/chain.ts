// chainType -> chainId -> Data
export const chainIdsMap = {
  '80001': {
    chainId: '80001',
    key: 'polygonMumbai',
    rpc: 'https://polygon-mumbai.g.alchemy.com/v2/8NUj-xxS1p3mJMFBo0h_szwvmUhMjkH4',
    chainName: 'Polygon Mumbai',
    gateway: '0xcAa6223D0d41FB27d6FC81428779751317FC24cB',
  },
  '5': {
    chainId: '5',
    key: 'goerli',
    rpc: 'https://goerli.infura.io/v3/91531d5460e34331a77e37156c61e223',
    chainName: 'Goerli',
    gateway: '0x',
  },
  '43113': {
    chainId: '43113',
    key: 'avalancheFuji',
    rpc: 'https://avalanche-fuji.infura.io/v3/91531d5460e34331a77e37156c61e223',
    chainName: 'Avalanche Fuji',
    gateway: '0xcAa6223D0d41FB27d6FC81428779751317FC24cB',
  },
  'router_9601-1': {
    chainId: 'router_9601-1',
    key: 'routerTestnet',
    rpc: 'https://devnet-alpha.evm.rpc.routerprotocol.com/',
    chainName: 'Router Testnet',
  },
};

export const chainKeysMap = {
  goerli: '5',
  avalancheFuji: '43113',
  polygonMumbai: '80001',
  routerTestnet: 'router_9601-1',
};

export function getChainInfo(chain: string) {
  if (chainKeysMap[chain]) return chainIdsMap[chainKeysMap[chain]];
  return chainIdsMap[chain] || null;
}

export function getChainId(chain: string): string | null {
  return getChainInfo(chain)?.chainId;
}

export function getChainKey(chain: string): string | null {
  return getChainInfo(chain)?.key;
}

export function getUChainList(chains: string[]): string[] {
  const array: string[] = [];
  chains.map((chain) => {
    const chainId = getChainId(chain);
    if (chainId) array.push(chainId);
  });
  return Array.from(new Set(array));
}
