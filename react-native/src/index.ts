// Provider
export { MobProvider, useMobContext } from './MobProvider';

// Hooks (xion.js-compatible API)
export {
  useAbstraxionAccount,
  useAbstraxionClient,
  useAbstraxionSigningClient,
} from './hooks';

// Native module (for advanced usage)
export { MobModule } from './MobModule';

// Session management
export { SessionManager } from './session/SessionManager';
export { openDashboardAuth, DashboardAuthError } from './session/DashboardAuth';

// Types
export type {
  ChainConfig,
  Coin,
  Fee,
  AccountInfo,
  TxResponse,
  Message,
  SignerInfo,
  SessionSignerInfo,
  SessionMetadata,
  ContractGrant,
  StakeGrant,
  BankGrant,
  GrantConfig,
  MobConfig,
} from './types';
