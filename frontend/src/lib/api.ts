const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001';

export interface AppInfo {
  app_id: string;
  dau_today: number;
  total_installs: number;
}

export interface DauData {
  date: string;
  dau: number;
}

export interface InstallData {
  date: string;
  installs: number;
}

export interface RetentionData {
  cohort_date: string;
  day0: number;
  day1: number | null;
  day7: number | null;
  day30: number | null;
}

export interface FeedbackData {
  id: number;
  content: string;
  user_id: string | null;
  contact: string | null;
  created_at: string;
}

class ApiClient {
  private readKey: string = '';

  setReadKey(key: string) {
    this.readKey = key;
    if (typeof window !== 'undefined') {
      localStorage.setItem('appscope_read_key', key);
    }
  }

  getReadKey(): string {
    if (this.readKey) return this.readKey;
    if (typeof window !== 'undefined') {
      this.readKey = localStorage.getItem('appscope_read_key') || '';
    }
    return this.readKey;
  }

  clearReadKey() {
    this.readKey = '';
    if (typeof window !== 'undefined') {
      localStorage.removeItem('appscope_read_key');
    }
  }

  private async fetch<T>(endpoint: string): Promise<T> {
    const res = await fetch(`${API_BASE}${endpoint}`, {
      headers: {
        'X-Read-Key': this.getReadKey(),
      },
    });

    if (!res.ok) {
      if (res.status === 401) {
        throw new Error('Unauthorized');
      }
      throw new Error(`API Error: ${res.status}`);
    }

    return res.json();
  }

  async getApps(): Promise<AppInfo[]> {
    const data = await this.fetch<{ apps: AppInfo[] }>('/api/apps');
    return data.apps;
  }

  async getDau(appId: string, days: number = 30): Promise<DauData[]> {
    const data = await this.fetch<{ data: DauData[] }>(
      `/api/stats/dau?app_id=${encodeURIComponent(appId)}&days=${days}`
    );
    return data.data;
  }

  async getInstalls(appId: string, days: number = 30): Promise<{ total: number; data: InstallData[] }> {
    return this.fetch<{ total: number; data: InstallData[] }>(
      `/api/stats/installs?app_id=${encodeURIComponent(appId)}&days=${days}`
    );
  }

  async getRetention(appId: string): Promise<RetentionData[]> {
    const data = await this.fetch<{ data: RetentionData[] }>(
      `/api/stats/retention?app_id=${encodeURIComponent(appId)}`
    );
    return data.data;
  }

  async getFeedbacks(appId: string, limit: number = 50): Promise<FeedbackData[]> {
    const data = await this.fetch<{ data: FeedbackData[] }>(
      `/api/feedbacks?app_id=${encodeURIComponent(appId)}&limit=${limit}`
    );
    return data.data;
  }
}

export const api = new ApiClient();
