import AsyncStorage from '@react-native-async-storage/async-storage';
import { MobModule } from '../MobModule';
import type { ChainConfig, SessionMetadata, SignerInfo } from '../types';

const STORAGE_KEY = '@mob/session';

/**
 * Manages session key lifecycle using the native MobSessionManager (Rust via UniFFI).
 *
 * Key generation, signer creation, and client wiring are handled in Rust.
 * This class is responsible only for persisting the opaque export blob
 * via AsyncStorage and calling the native manager's methods.
 */
export class SessionManager {
  private _sessionId: string | null = null;
  private _granterAddress: string | null = null;
  private _granteeAddress: string | null = null;

  /** Whether a session is currently active and not expired */
  get isActive(): boolean {
    return this._sessionId !== null;
  }

  /** The granter (main account) address, if a session is active */
  get granterAddress(): string | null {
    return this._granterAddress;
  }

  /** The grantee (session key) address */
  get granteeAddress(): string | null {
    return this._granteeAddress;
  }

  /** The native session ID (opaque handle for native calls) */
  get sessionId(): string | null {
    return this._sessionId;
  }

  /**
   * Generate a new session key.
   *
   * Creates a native MobSessionManager with a random key pair.
   * Call this before opening dashboard auth — pass the returned address
   * as the grantee.
   */
  async generateSessionKey(prefix: string): Promise<SignerInfo> {
    // Create a native session manager
    this._sessionId = await MobModule.createSessionManager(prefix);
    const info = await MobModule.sessionGenerateKey(this._sessionId);
    this._granteeAddress = info.address;
    return info;
  }

  /**
   * Activate and persist the session after dashboard authorization.
   *
   * Passes the metadata to the native session manager which creates
   * the signing client internally. The session state is then exported
   * as an opaque blob and stored in AsyncStorage.
   */
  async saveSession(
    metadata: SessionMetadata,
    config: ChainConfig
  ): Promise<void> {
    if (!this._sessionId || !this._granteeAddress) {
      throw new Error('No session key generated — call generateSessionKey() first');
    }

    await MobModule.sessionActivate(
      this._sessionId,
      metadata.granter,
      metadata.grantee,
      metadata.createdAt,
      metadata.expiresAt,
      metadata.description,
      config
    );

    this._granterAddress = metadata.granter;

    // Export the session state and persist
    const exported = await MobModule.sessionExport(this._sessionId);
    await AsyncStorage.setItem(STORAGE_KEY, JSON.stringify(exported));
  }

  /**
   * Attempt to restore a previous session from storage.
   *
   * Reads the opaque blob from AsyncStorage, passes it to the native
   * MobSessionManager.restore() which recreates the signer and client.
   * Returns true if a valid (non-expired) session was restored.
   */
  async restoreSession(config: ChainConfig): Promise<boolean> {
    const raw = await AsyncStorage.getItem(STORAGE_KEY);
    if (!raw) return false;

    let exportedBytes: number[];
    try {
      exportedBytes = JSON.parse(raw);
    } catch {
      await this.clearSession();
      return false;
    }

    try {
      this._sessionId = await MobModule.sessionRestore(exportedBytes, config);
    } catch {
      // Session expired or data corrupted
      await this.clearSession();
      return false;
    }

    const [granter, grantee, active] = await Promise.all([
      MobModule.sessionGranterAddress(this._sessionId),
      MobModule.sessionGranteeAddress(this._sessionId),
      MobModule.sessionIsActive(this._sessionId),
    ]);

    if (!active) {
      await this.clearSession();
      return false;
    }

    this._granterAddress = granter;
    this._granteeAddress = grantee;
    return true;
  }

  /**
   * Clear the current session and remove stored data.
   */
  async clearSession(): Promise<void> {
    if (this._sessionId) {
      try {
        await MobModule.sessionDeactivate(this._sessionId);
      } catch {
        // Session may already be deactivated
      }
    }

    this._sessionId = null;
    this._granterAddress = null;
    this._granteeAddress = null;

    await AsyncStorage.removeItem(STORAGE_KEY);
  }
}
