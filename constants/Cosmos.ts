export const COSMOS_MNEMONIC_KEY = "cosmos_mnemonic_key";
export const COSMOS_GRANTING_ADDRESS_KEY = "cosmos_granting_address_key";
export const TENDERMINT_RPC = "https://rpc.carbon-1.burnt.com:443/219fdee19b488dbce1e656b9bbc5c1cb6178438e98a1aef3ff1b2be36443a24f";


import { Bech32Address } from "@keplr-wallet/cosmos";
import type { AppCurrency, ChainInfo } from "@keplr-wallet/types";

const TURNT: AppCurrency = {
    coinDenom: "turnt",
    coinMinimalDenom: "uturnt",
    coinDecimals: 6,
    // coinGeckoId: "osmosis",
    // coinImageUrl: "https://dhj8dql1kzq2v.cloudfront.net/white/osmo.png",
};

const currencies: AppCurrency[] = [TURNT];

export const burntTestnet: ChainInfo = {
    rpc: 'https://rpc.carbon-1.burnt.com:443/219fdee19b488dbce1e656b9bbc5c1cb6178438e98a1aef3ff1b2be36443a24f',
    rest: 'https://api.carbon-1.burnt.com:443/5a0a32d7253d08ef842b1dc707db9b71dd7209bb38d4dc60f8d5fd4806bd1391',
    chainId: "carbon-1",
    chainName: "Burnt Testnet",
    stakeCurrency: TURNT,
    bip44: {
        coinType: 118,
    },
    bech32Config: Bech32Address.defaultBech32Config("burnt"),
    currencies,
    feeCurrencies: [TURNT],
    coinType: 118,
};