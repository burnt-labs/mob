import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from 'react';
import { MobModule } from './MobModule';
import { SessionManager } from './session/SessionManager';
import { openDashboardAuth } from './session/DashboardAuth';
import type {
  MobConfig,
  ChainConfig,
  Coin,
  AccountInfo,
  TxResponse,
  Message,
} from './types';

// --- Context state ---

interface MobContextState {
  // Account
  bech32Address: string | null;
  isConnected: boolean;
  isConnecting: boolean;
  isInitializing: boolean;
  isLoading: boolean;
  isReturningFromAuth: boolean;
  isLoggingIn: boolean;
  isError: boolean;
  error: string;
  // Auth
  login: () => Promise<void>;
  logout: () => Promise<void>;
  // Read queries
  getAccount: (address: string) => Promise<AccountInfo>;
  getBalance: (address: string, denom: string) => Promise<Coin>;
  getAllBalances: (address: string) => Promise<Coin[]>;
  getHeight: () => Promise<number>;
  queryContractSmart: (contract: string, queryMsg: object) => Promise<unknown>;
  // Signing (requires session)
  send: (to: string, amount: Coin[], memo?: string) => Promise<TxResponse>;
  executeContract: (
    contract: string,
    msg: object,
    funds?: Coin[],
    memo?: string,
    gasLimit?: number
  ) => Promise<TxResponse>;
  signAndBroadcast: (
    messages: Message[],
    memo?: string,
    gasLimit?: number
  ) => Promise<TxResponse>;
  signArb: (data: string) => Promise<string>;
}

const MobContext = createContext<MobContextState | null>(null);

// --- Network detection ---
// Dashboard URLs per chain ID, sourced from xion.js/packages/constants
const DASHBOARD_URLS: Record<string, string> = {
  'xion-mainnet-1': 'https://settings.mainnet.burnt.com',
  'xion-testnet-1': 'https://settings.testnet.burnt.com',
  'xion-testnet-2': 'https://auth.testnet.burnt.com',
};

async function detectDashboardUrl(rpcEndpoint: string): Promise<string | null> {
  try {
    const resp = await fetch(`${rpcEndpoint}/status`);
    const json = await resp.json();
    const networkId: string = json?.result?.node_info?.network ?? '';
    return DASHBOARD_URLS[networkId] ?? null;
  } catch {
    return null;
  }
}

// --- Provider ---

interface MobProviderProps {
  config: MobConfig;
  children: React.ReactNode;
}

/** Merge treasury address into ChainConfig as feeGranter */
function resolveChainConfig(config: MobConfig): ChainConfig {
  if (config.treasury && !config.chain.feeGranter) {
    return { ...config.chain, feeGranter: config.treasury };
  }
  return config.chain;
}

export function MobProvider({ config, children }: MobProviderProps) {
  const [bech32Address, setBech32Address] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const [isInitializing, setIsInitializing] = useState(true);
  const [isReturningFromAuth, setIsReturningFromAuth] = useState(false);
  const [isLoggingIn, setIsLoggingIn] = useState(false);
  const [isError, setIsError] = useState(false);
  const [error, setError] = useState('');
  const [readClientId, setReadClientId] = useState<string | null>(null);
  const [dashboardUrl, setDashboardUrl] = useState<string | undefined>(
    config.dashboardUrl
  );

  const sessionManager = useRef(new SessionManager()).current;
  const chainConfig = useMemo(() => resolveChainConfig(config), [config]);

  const isConnected = bech32Address !== null;
  const isLoading = isInitializing || isConnecting;

  // Initialize read-only client on mount, detect network, restore session
  useEffect(() => {
    let clientId: string | null = null;
    let cancelled = false;

    (async () => {
      try {
        clientId = await MobModule.createClient(chainConfig);
        if (cancelled) return;
        setReadClientId(clientId);

        // Auto-detect dashboard URL if not provided
        if (!config.dashboardUrl) {
          const detected = await detectDashboardUrl(chainConfig.rpcEndpoint);
          if (!cancelled && detected) {
            setDashboardUrl(detected);
          }
        }

        // Attempt session restoration
        const restored = await sessionManager.restoreSession(chainConfig);
        if (!cancelled && restored) {
          setBech32Address(sessionManager.granterAddress);
        }
      } catch (e) {
        if (!cancelled) {
          setIsError(true);
          setError(e instanceof Error ? e.message : String(e));
        }
      } finally {
        if (!cancelled) {
          setIsInitializing(false);
        }
      }
    })();

    return () => {
      cancelled = true;
      if (clientId) {
        MobModule.destroyClient(clientId).catch(() => {});
      }
    };
  }, [chainConfig.chainId, chainConfig.rpcEndpoint]);

  const login = useCallback(async () => {
    setIsConnecting(true);
    setIsLoggingIn(true);
    setIsError(false);
    setError('');
    try {
      // Generate a session key
      const signerInfo = await sessionManager.generateSessionKey(
        chainConfig.addressPrefix
      );

      // Open dashboard auth
      setIsReturningFromAuth(true);
      const { metadata } = await openDashboardAuth({
        chainId: chainConfig.chainId,
        dashboardUrl,
        callbackUrl: config.callbackUrl,
        granteeAddress: signerInfo.address,
        grants: config.grants,
        treasury: config.treasury,
      });
      setIsReturningFromAuth(false);

      // Verify grants exist on-chain before saving session
      if (readClientId) {
        const granted = await MobModule.hasGrants(
          readClientId,
          metadata.granter,
          metadata.grantee
        );
        if (!granted) {
          throw new Error('Grants not found on-chain after authorization');
        }
      }

      // Save session
      await sessionManager.saveSession(metadata, chainConfig);
      setBech32Address(metadata.granter);
    } catch (e) {
      setIsReturningFromAuth(false);
      setIsError(true);
      setError(e instanceof Error ? e.message : String(e));
      await sessionManager.clearSession();
      throw e;
    } finally {
      setIsConnecting(false);
      setIsLoggingIn(false);
    }
  }, [chainConfig, config, dashboardUrl, readClientId, sessionManager]);

  const logout = useCallback(async () => {
    await sessionManager.clearSession();
    setBech32Address(null);
    setIsError(false);
    setError('');
  }, [sessionManager]);

  // Read query helpers — use the read-only client
  const requireReadClient = useCallback((): string => {
    if (!readClientId) throw new Error('Client not initialized');
    return readClientId;
  }, [readClientId]);

  const getAccount = useCallback(
    (address: string) => MobModule.getAccount(requireReadClient(), address),
    [requireReadClient]
  );

  const getBalance = useCallback(
    (address: string, denom: string) =>
      MobModule.getBalance(requireReadClient(), address, denom),
    [requireReadClient]
  );

  const getAllBalances = useCallback(
    (address: string) =>
      MobModule.getAllBalances(requireReadClient(), address),
    [requireReadClient]
  );

  const getHeight = useCallback(
    () => MobModule.getHeight(requireReadClient()),
    [requireReadClient]
  );

  const queryContractSmart = useCallback(
    async (contract: string, queryMsg: object): Promise<unknown> => {
      const msgBytes = Array.from(
        new TextEncoder().encode(JSON.stringify(queryMsg))
      );
      const resultBytes = await MobModule.queryContractSmart(
        requireReadClient(),
        contract,
        msgBytes
      );
      const resultStr = new TextDecoder().decode(new Uint8Array(resultBytes));
      return JSON.parse(resultStr);
    },
    [requireReadClient]
  );

  // Signing helpers — use session-scoped native functions
  const requireSession = useCallback((): string => {
    const id = sessionManager.sessionId;
    if (!id) throw new Error('Not connected — call login() first');
    return id;
  }, [sessionManager]);

  const send = useCallback(
    (to: string, amount: Coin[], memo?: string) =>
      MobModule.sessionSend(requireSession(), to, amount, memo),
    [requireSession]
  );

  const executeContract = useCallback(
    (
      contract: string,
      msg: object,
      funds: Coin[] = [],
      memo?: string,
      gasLimit?: number
    ) => {
      const msgBytes = Array.from(
        new TextEncoder().encode(JSON.stringify(msg))
      );
      return MobModule.sessionExecuteContract(
        requireSession(),
        contract,
        msgBytes,
        funds,
        memo,
        gasLimit
      );
    },
    [requireSession]
  );

  const signAndBroadcast = useCallback(
    (messages: Message[], memo?: string, gasLimit?: number) =>
      MobModule.sessionSignAndBroadcastMulti(
        requireSession(),
        messages,
        memo,
        gasLimit
      ),
    [requireSession]
  );

  const signArb = useCallback(
    async (data: string): Promise<string> => {
      const sessionId = requireSession();
      const grantee = sessionManager.granteeAddress;
      if (!grantee) throw new Error('Not connected');

      // ADR-036: sign arbitrary data
      const signDoc = {
        chain_id: '',
        account_number: '0',
        sequence: '0',
        fee: { gas: '0', amount: [] },
        msgs: [
          {
            type: 'sign/MsgSignData',
            value: {
              signer: grantee,
              data: btoa(data),
            },
          },
        ],
        memo: '',
      };

      const docBytes = Array.from(
        new TextEncoder().encode(JSON.stringify(signDoc))
      );
      const sigBytes = await MobModule.sessionSignBytes(sessionId, docBytes);
      // Return base64-encoded signature
      return btoa(String.fromCharCode(...sigBytes));
    },
    [requireSession, sessionManager]
  );

  const value = useMemo<MobContextState>(
    () => ({
      bech32Address,
      isConnected,
      isConnecting,
      isInitializing,
      isLoading,
      isReturningFromAuth,
      isLoggingIn,
      isError,
      error,
      login,
      logout,
      getAccount,
      getBalance,
      getAllBalances,
      getHeight,
      queryContractSmart,
      send,
      executeContract,
      signAndBroadcast,
      signArb,
    }),
    [
      bech32Address,
      isConnected,
      isConnecting,
      isInitializing,
      isLoading,
      isReturningFromAuth,
      isLoggingIn,
      isError,
      error,
      login,
      logout,
      getAccount,
      getBalance,
      getAllBalances,
      getHeight,
      queryContractSmart,
      send,
      executeContract,
      signAndBroadcast,
      signArb,
    ]
  );

  return <MobContext.Provider value={value}>{children}</MobContext.Provider>;
}

/** Access the MobContext. Must be used within a MobProvider. */
export function useMobContext(): MobContextState {
  const ctx = useContext(MobContext);
  if (!ctx) {
    throw new Error('useMobContext must be used within a MobProvider');
  }
  return ctx;
}
