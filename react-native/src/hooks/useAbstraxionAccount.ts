import { useMobContext } from '../MobProvider';

/**
 * Hook providing account state and auth actions.
 * API-compatible with @burnt-labs/abstraxion-react-native.
 */
export function useAbstraxionAccount() {
  const {
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
  } = useMobContext();

  return {
    data: { bech32Address: bech32Address ?? '' },
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
  };
}
