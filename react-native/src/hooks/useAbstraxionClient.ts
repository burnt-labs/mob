import { useMobContext } from '../MobProvider';

/**
 * Hook providing read-only query methods.
 * API-compatible with @burnt-labs/abstraxion-react-native.
 */
export function useAbstraxionClient() {
  const {
    getAccount,
    getBalance,
    getAllBalances,
    getHeight,
    queryContractSmart,
  } = useMobContext();

  return {
    getAccount,
    getBalance,
    getAllBalances,
    getHeight,
    queryContractSmart,
  };
}
