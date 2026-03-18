import { requireNativeModule } from 'expo-modules-core';
import type {
  ChainConfig,
  Coin,
  AccountInfo,
  TxResponse,
  Message,
  SignerInfo,
} from './types';

interface MobNativeModule {
  // Signer management
  createSignerFromMnemonic(
    mnemonic: string,
    prefix: string,
    derivationPath?: string
  ): Promise<SignerInfo>;

  signBytes(signerAddress: string, message: number[]): Promise<number[]>;

  destroySigner(address: string): Promise<void>;

  // Keychain signer (iOS secure key storage)
  createKeychainSigner(
    mnemonic: string,
    prefix: string,
    derivationPath: string | undefined,
    identifier: string,
    requireBiometrics?: boolean
  ): Promise<SignerInfo>;

  loadKeychainSigner(
    identifier: string,
    prefix: string,
    derivationPath?: string
  ): Promise<SignerInfo>;

  deleteKeychainSigner(identifier: string): Promise<void>;

  listKeychainSigners(): Promise<string[]>;

  keychainSignerExists(identifier: string): Promise<boolean>;

  keychainSignBytes(signerAddress: string, message: number[]): Promise<number[]>;

  createClientWithKeychainSigner(
    config: ChainConfig,
    signerAddress: string
  ): Promise<string>;

  // Client management
  createClient(config: ChainConfig): Promise<string>;

  createClientWithSigner(
    config: ChainConfig,
    signerAddress: string
  ): Promise<string>;

  destroyClient(clientId: string): Promise<void>;

  // Read queries
  getAccount(clientId: string, address: string): Promise<AccountInfo>;

  getBalance(clientId: string, address: string, denom: string): Promise<Coin>;

  getAllBalances(clientId: string, address: string): Promise<Coin[]>;

  getHeight(clientId: string): Promise<number>;

  getTx(clientId: string, hash: string): Promise<TxResponse>;

  hasGrants(
    clientId: string,
    granter: string,
    grantee: string
  ): Promise<boolean>;

  queryContractSmart(
    clientId: string,
    contractAddress: string,
    queryMsg: number[]
  ): Promise<number[]>;

  // Transactions
  send(
    clientId: string,
    toAddress: string,
    amount: Coin[],
    memo?: string
  ): Promise<TxResponse>;

  executeContract(
    clientId: string,
    contractAddress: string,
    msg: number[],
    funds: Coin[],
    memo?: string,
    gasLimit?: number
  ): Promise<TxResponse>;

  signAndBroadcastMulti(
    clientId: string,
    messages: Message[],
    memo?: string,
    gasLimit?: number
  ): Promise<TxResponse>;

  // Session manager — backed by Rust MobSessionManager via UniFFI
  createSessionManager(addressPrefix: string): Promise<string>;

  sessionGenerateKey(sessionId: string): Promise<SignerInfo>;

  sessionActivate(
    sessionId: string,
    granter: string,
    grantee: string,
    createdAt: number,
    expiresAt: number,
    description: string | undefined,
    config: ChainConfig
  ): Promise<void>;

  sessionExport(sessionId: string): Promise<number[]>;

  sessionRestore(data: number[], config: ChainConfig): Promise<string>;

  sessionDeactivate(sessionId: string): Promise<void>;

  sessionIsActive(sessionId: string): Promise<boolean>;

  sessionGranterAddress(sessionId: string): Promise<string | null>;

  sessionGranteeAddress(sessionId: string): Promise<string | null>;

  sessionSignBytes(sessionId: string, message: number[]): Promise<number[]>;

  // Session-scoped transactions (use the session manager's internal client)
  sessionSend(
    sessionId: string,
    toAddress: string,
    amount: Coin[],
    memo?: string
  ): Promise<TxResponse>;

  sessionExecuteContract(
    sessionId: string,
    contractAddress: string,
    msg: number[],
    funds: Coin[],
    memo?: string,
    gasLimit?: number
  ): Promise<TxResponse>;

  sessionSignAndBroadcastMulti(
    sessionId: string,
    messages: Message[],
    memo?: string,
    gasLimit?: number
  ): Promise<TxResponse>;

  sessionQueryContractSmart(
    sessionId: string,
    contractAddress: string,
    queryMsg: number[]
  ): Promise<number[]>;
}

export const MobModule = requireNativeModule<MobNativeModule>('Mob');
