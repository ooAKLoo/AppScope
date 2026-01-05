'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { api, AppInfo } from '@/lib/api';

export default function DashboardPage() {
  const [apps, setApps] = useState<AppInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const router = useRouter();

  useEffect(() => {
    const fetchApps = async () => {
      try {
        const data = await api.getApps();
        setApps(data);
      } catch {
        api.clearReadKey();
        router.push('/');
      } finally {
        setLoading(false);
      }
    };

    fetchApps();
  }, [router]);

  const handleLogout = () => {
    api.clearReadKey();
    router.push('/');
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-slate-100 flex items-center justify-center">
        <div className="text-slate-500">Loading...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-slate-100">
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex justify-between items-center">
          <h1 className="text-2xl font-bold text-slate-800">AppScope</h1>
          <button
            onClick={handleLogout}
            className="text-slate-500 hover:text-slate-700 transition"
          >
            Logout
          </button>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <h2 className="text-xl font-semibold text-slate-700 mb-6">My Apps</h2>

        {apps.length === 0 ? (
          <div className="bg-white rounded-xl p-8 text-center text-slate-500">
            No apps yet. Start sending events from your apps to see them here.
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {apps.map((app) => (
              <Link
                key={app.app_id}
                href={`/dashboard/${app.app_id}`}
                className="bg-white rounded-xl p-6 shadow-sm hover:shadow-md transition"
              >
                <h3 className="text-lg font-semibold text-slate-800 mb-4">
                  {app.app_id}
                </h3>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-sm text-slate-500">Today DAU</p>
                    <p className="text-2xl font-bold text-blue-600">
                      {app.dau_today.toLocaleString()}
                    </p>
                  </div>
                  <div>
                    <p className="text-sm text-slate-500">Total Installs</p>
                    <p className="text-2xl font-bold text-green-600">
                      {app.total_installs.toLocaleString()}
                    </p>
                  </div>
                </div>
              </Link>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}
