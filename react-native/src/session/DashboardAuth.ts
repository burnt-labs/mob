import * as WebBrowser from 'expo-web-browser';
import * as Linking from 'expo-linking';
import type { GrantConfig, SessionMetadata } from '../types';

// Dashboard URLs per chain ID, sourced from xion.js/packages/constants
const DASHBOARD_URLS: Record<string, string> = {
  'xion-mainnet-1': 'https://settings.mainnet.burnt.com',
  'xion-testnet-1': 'https://settings.testnet.burnt.com',
  'xion-testnet-2': 'https://auth.testnet.burnt.com',
};

interface DashboardAuthOptions {
  /** Chain ID used to resolve the dashboard URL */
  chainId?: string;
  /** Explicit dashboard URL (overrides chain ID lookup) */
  dashboardUrl?: string;
  callbackUrl?: string;
  granteeAddress: string;
  grants?: GrantConfig;
  /** Treasury contract address */
  treasury?: string;
}

interface DashboardAuthResult {
  granterAddress: string;
  metadata: SessionMetadata;
}

function getDashboardUrl(options: DashboardAuthOptions): string {
  if (options.dashboardUrl) return options.dashboardUrl;
  if (options.chainId && DASHBOARD_URLS[options.chainId]) {
    return DASHBOARD_URLS[options.chainId];
  }
  // Default to testnet-2
  return DASHBOARD_URLS['xion-testnet-2'];
}

/**
 * Opens the XION dashboard for session authorization and returns
 * the granter address from the callback deep link.
 *
 * URL construction matches xion.js/packages/abstraxion-core/AbstraxionAuth.ts
 */
export async function openDashboardAuth(
  options: DashboardAuthOptions
): Promise<DashboardAuthResult> {
  const dashboardUrl = getDashboardUrl(options);
  const callbackUrl = options.callbackUrl ?? Linking.createURL('mob-auth');

  // Build dashboard URL with grant parameters (matching xion.js convention)
  const params = new URLSearchParams();
  params.set('grantee', options.granteeAddress);
  params.set('redirect_uri', callbackUrl);

  if (options.treasury) {
    params.set('treasury', options.treasury);
  }

  if (options.grants?.bank) {
    params.set('bank', JSON.stringify(options.grants.bank.spendLimit));
  }

  if (options.grants?.stake) {
    params.set('stake', 'true');
  }

  if (options.grants?.contracts) {
    params.set('contracts', JSON.stringify(options.grants.contracts));
  }

  const authUrl = `${dashboardUrl}?${params.toString()}`;

  console.log('[MobAuth] Dashboard URL:', authUrl);
  console.log('[MobAuth] Callback URL:', callbackUrl);

  // Open the browser and wait for the callback
  const result = await WebBrowser.openAuthSessionAsync(authUrl, callbackUrl);

  console.log('[MobAuth] Browser result:', result.type, 'url' in result ? result.url : '');

  if (result.type !== 'success' || !result.url) {
    throw new DashboardAuthError('Dashboard authorization was cancelled or failed');
  }

  // Parse the callback URL
  const parsed = Linking.parse(result.url);
  const granterAddress = parsed.queryParams?.granter as string | undefined;

  if (!granterAddress) {
    throw new DashboardAuthError('No granter address in callback URL');
  }

  const expiresAtStr = parsed.queryParams?.expires_at as string | undefined;
  const sessionDuration = options.grants?.sessionDuration ?? 86400; // default 24h
  const now = Math.floor(Date.now() / 1000);

  const metadata: SessionMetadata = {
    granter: granterAddress,
    grantee: options.granteeAddress,
    createdAt: now,
    expiresAt: expiresAtStr ? parseInt(expiresAtStr, 10) : now + sessionDuration,
  };

  return { granterAddress, metadata };
}

export class DashboardAuthError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'DashboardAuthError';
  }
}
