export interface AnalyticsConfig {
  writeKey: string;
  appId: string;
  apiEndpoint: string;
  autoTrack?: boolean;
}

export interface TrackProperties {
  [key: string]: string | number | boolean | null | undefined;
}

export class Analytics {
  private writeKey: string;
  private appId: string;
  private endpoint: string;
  private userId: string;

  constructor(config: AnalyticsConfig) {
    this.writeKey = config.writeKey;
    this.appId = config.appId;
    this.endpoint = config.apiEndpoint.replace(/\/$/, '');
    this.userId = this.getOrCreateUserId();

    if (config.autoTrack !== false) {
      this.trackOpen();
      this.trackInstallIfFirst();
    }
  }

  private getOrCreateUserId(): string {
    if (typeof window === 'undefined' || typeof localStorage === 'undefined') {
      return 'u_' + this.generateUUID();
    }

    const storageKey = `appscope_${this.appId}_user_id`;
    let id = localStorage.getItem(storageKey);
    if (!id) {
      id = 'u_' + this.generateUUID();
      localStorage.setItem(storageKey, id);
    }
    return id;
  }

  private generateUUID(): string {
    if (typeof crypto !== 'undefined' && crypto.randomUUID) {
      return crypto.randomUUID();
    }
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
      const r = (Math.random() * 16) | 0;
      const v = c === 'x' ? r : (r & 0x3) | 0x8;
      return v.toString(16);
    });
  }

  private trackInstallIfFirst(): void {
    if (typeof window === 'undefined' || typeof localStorage === 'undefined') {
      return;
    }

    const installedKey = `appscope_${this.appId}_installed`;
    if (!localStorage.getItem(installedKey)) {
      this.track('$install');
      localStorage.setItem(installedKey, 'true');
    }
  }

  private trackOpen(): void {
    this.track('$open');
  }

  private getPlatform(): string {
    if (typeof window === 'undefined') return 'server';
    const ua = navigator.userAgent;
    if (/Mac/.test(ua)) return 'macos';
    if (/Win/.test(ua)) return 'windows';
    if (/Linux/.test(ua)) return 'linux';
    if (/iPhone|iPad/.test(ua)) return 'ios';
    if (/Android/.test(ua)) return 'android';
    return 'unknown';
  }

  track(event: string, properties?: TrackProperties): void {
    const payload = {
      app_id: this.appId,
      event,
      user_id: this.userId,
      properties: {
        ...properties,
        platform: this.getPlatform(),
        timestamp: new Date().toISOString(),
      },
    };

    fetch(`${this.endpoint}/api/track`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Write-Key': this.writeKey,
      },
      body: JSON.stringify(payload),
    }).catch(() => {
      // Silent fail - don't affect main app
    });
  }

  feedback(content: string, contact?: string): void {
    const payload = {
      app_id: this.appId,
      content,
      user_id: this.userId,
      contact,
    };

    fetch(`${this.endpoint}/api/feedback`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Write-Key': this.writeKey,
      },
      body: JSON.stringify(payload),
    }).catch(() => {
      // Silent fail
    });
  }

  getUserId(): string {
    return this.userId;
  }
}

export default Analytics;
