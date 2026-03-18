import { useMobContext } from '../MobProvider';

/**
 * Hook providing signing methods.
 * API-compatible with @burnt-labs/abstraxion-react-native.
 */
export function useAbstraxionSigningClient() {
  const {
    isConnected,
    send,
    executeContract,
    signAndBroadcast,
    signArb,
  } = useMobContext();

  return {
    isConnected,
    send,
    executeContract,
    signAndBroadcast,
    signArb,
  };
}
