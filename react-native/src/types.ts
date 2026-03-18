/** Chain configuration matching mob's ChainConfig */
export interface ChainConfig {
  chainId: string;
  rpcEndpoint: string;
  grpcEndpoint?: string;
  addressPrefix: string;
  coinType?: number;
  gasPrice?: string;
  /** Fee granter address (e.g. treasury contract) for gas-free transactions */
  feeGranter?: string;
}

/** Coin amount with denomination */
export interface Coin {
  denom: string;
  amount: string;
}

/** Transaction fee */
export interface Fee {
  amount: Coin[];
  gasLimit: number;
  payer?: string;
  granter?: string;
}

/** Account information */
export interface AccountInfo {
  address: string;
  accountNumber: number;
  sequence: number;
  pubKey?: string;
}

/** Transaction response */
export interface TxResponse {
  txhash: string;
  code: number;
  rawLog: string;
  gasWanted: number;
  gasUsed: number;
  height: number;
}

/** Protobuf-encoded message (type_url + value) */
export interface Message {
  typeUrl: string;
  value: number[]; // byte array
}

/** Signer info returned from native module */
export interface SignerInfo {
  address: string;
  publicKeyHex: string;
}

/** Session signer info returned from native module */
export interface SessionSignerInfo {
  granterAddress: string;
  granteeAddress: string;
  publicKeyHex: string;
}

/** Session metadata */
export interface SessionMetadata {
  granter: string;
  grantee: string;
  createdAt: number;
  expiresAt: number;
  description?: string;
}

// --- xion.js-compatible config types ---

/** Contract grant permission */
export interface ContractGrant {
  address: string;
  amounts: Coin[];
}

/** Stake grant permission */
export interface StakeGrant {
  validators: string[];
  maxAmount?: Coin;
}

/** Bank send grant permission */
export interface BankGrant {
  spendLimit: Coin[];
}

/** Combined grant configuration for dashboard auth */
export interface GrantConfig {
  contracts?: ContractGrant[];
  stake?: StakeGrant;
  bank?: BankGrant;
  sessionDuration?: number; // seconds
}

/** Top-level configuration for MobProvider */
export interface MobConfig {
  chain: ChainConfig;
  dashboardUrl?: string;
  callbackUrl?: string;
  grants?: GrantConfig;
  /** Treasury contract address used as fee granter for gas-free transactions */
  treasury?: string;
  /** Indexer URL for querying indexed chain data */
  indexerUrl?: string;
}
