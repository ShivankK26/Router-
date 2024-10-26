// require("hardhat-contract-sizer");
require('@nomiclabs/hardhat-waffle');
require(`@nomiclabs/hardhat-etherscan`);
require('solidity-coverage');
// require("hardhat-gas-reporter");
require('hardhat-deploy');
require('hardhat-deploy-ethers');
require('@openzeppelin/hardhat-upgrades');

import type { NetworkUserConfig } from 'hardhat/types';
require('dotenv').config();

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
const mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  throw new Error('Please set your MNEMONIC in a .env file');
}

{/* prettier-ignore */}
const chainIds: { [key: string]: number } = {
  'arbitrum-mainnet': 42161,
  'fantom-testnet': 4002,
  'bsc-testnet': 97,
  'arbitrum-goerli': 421613,
  'optimism-goerli': 420,
  'optimism-mainnet': 10,
  'polygon-mainnet': 137,
  'polygon-mumbai': 80001,
  avalanche: 43114,
  bsc: 56,
  hardhat: 31337,
  mainnet: 1,
  sepolia: 11155111,
  goerli: 5,
  fuji: 43113,
  fantom: 250,
  routertestnet: 9000,
};

const infuraApiKey: string | undefined = process.env.INFURA_API_KEY;
if (!infuraApiKey) {
  throw new Error('Please set your INFURA_API_KEY in a .env file');
}

function getChainConfig(chain: keyof typeof chainIds): NetworkUserConfig {
  let jsonRpcUrl: string;
  switch (chain) {
    case 'bsc-testnet':
      jsonRpcUrl = 'https://data-seed-prebsc-1-s1.binance.org:8545/';
      break;
    case 'optimism-goerli':
      jsonRpcUrl = 'https://goerli.optimism.io/';
      break;
    case 'arbitrum-goerli':
      jsonRpcUrl = 'https://goerli-rollup.arbitrum.io/rpc/';
      break;
    case 'fantom-testnet':
      jsonRpcUrl = 'https://rpc.testnet.fantom.network/';
      break;
    case 'fantom':
      jsonRpcUrl = 'https://rpcapi.fantom.network';
      break;
    case 'fuji':
      jsonRpcUrl = 'https://api.avax-test.network/ext/bc/C/rpc';
      break;
    case 'avalanche':
      jsonRpcUrl = 'https://api.avax.network/ext/bc/C/rpc';
      break;
    case 'bsc':
      jsonRpcUrl = 'https://bsc-dataseed1.binance.org';
      break;
    case 'polygon-mumbai':
      jsonRpcUrl =
        'https://polygon-mumbai.g.alchemy.com/v2/8NUj-xxS1p3mJMFBo0h_szwvmUhMjkH4';
      break;
    default:
      jsonRpcUrl = 'https://' + chain + '.infura.io/v3/' + infuraApiKey;
  }

  return {
    url: jsonRpcUrl,
    chainId: chainIds[chain],
    accounts: {
      count: 10,
      mnemonic,
      path: "m/44'/60'/0'/0",
    },
    timeout: 100000000,
  };
}

// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
module.exports = {
  mocha: {
    timeout: 100000000,
  },
  solidity: {
    compilers: [
      {
        version: '0.8.18',
        settings: {
          viaIR: true,
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
    ],
  },

  // solidity: "0.8.4",
  contractSizer: {
    alphaSort: false,
    runOnCompile: true,
    disambiguatePaths: false,
  },

  namedAccounts: {
    deployer: {
      default: 0, // wallet address 0, of the mnemonic in .env
    },
    proxyOwner: {
      default: 1,
    },
  },

  networks: {
    hardhat: {},
    localhost: {
      url: 'http://127.0.0.1:8545',
      chainId: 31337,
    },
    arbitrum: getChainConfig('arbitrum-mainnet'),
    avalanche: getChainConfig('avalanche'),
    bsc: getChainConfig('bsc'),
    mainnet: getChainConfig('mainnet'), // ethereum
    optimism: getChainConfig('optimism-mainnet'),
    polygonMainnet: getChainConfig('polygon-mainnet'),
    polygonMumbai: getChainConfig('polygon-mumbai'),
    sepolia: getChainConfig('sepolia'),
    avalancheFuji: getChainConfig('fuji'),
    goerli: getChainConfig('goerli'),
    fantom: getChainConfig('fantom'),
    bscTestnet: getChainConfig('bsc-testnet'),
    arbitrumGoerli: getChainConfig('arbitrum-goerli'),
    optimismGoerli: getChainConfig('optimismgoerli'),
    fantomTestnet: getChainConfig('fantom-testnet'),
    routerTestnet: getChainConfig('routertestnet'),
    defaultNetwork: getChainConfig('polygon-mumbai'),
  },
  etherscan: {
    apiKey: {
      polygonMumbai: process.env.POLYGON_API_KEY,
      bscTestnet: process.env.BSC_TESTNET_API_KEY,
      goerli: process.env.ETHERSCAN_API,
      avalancheFujiTestnet: process.env.FUJI_API,
    },
  },
};
