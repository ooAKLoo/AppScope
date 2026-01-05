'use client';

import { useState, useEffect, use } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { Line, Bar } from 'react-chartjs-2';
import { api, DauData, RetentionData, FeedbackData, InstallData } from '@/lib/api';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
);

interface PageProps {
  params: Promise<{ appId: string }>;
}

export default function AppDetailPage({ params }: PageProps) {
  const resolvedParams = use(params);
  const appId = resolvedParams.appId;
  const router = useRouter();

  const [dauData, setDauData] = useState<DauData[]>([]);
  const [installData, setInstallData] = useState<{ total: number; data: InstallData[] }>({ total: 0, data: [] });
  const [retentionData, setRetentionData] = useState<RetentionData[]>([]);
  const [feedbacks, setFeedbacks] = useState<FeedbackData[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [dau, installs, retention, fb] = await Promise.all([
          api.getDau(appId),
          api.getInstalls(appId),
          api.getRetention(appId),
          api.getFeedbacks(appId),
        ]);
        setDauData(dau);
        setInstallData(installs);
        setRetentionData(retention);
        setFeedbacks(fb);
      } catch {
        api.clearReadKey();
        router.push('/');
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [appId, router]);

  if (loading) {
    return (
      <div className="min-h-screen bg-slate-100 flex items-center justify-center">
        <div className="text-slate-500">Loading...</div>
      </div>
    );
  }

  const dauChartData = {
    labels: dauData.map((d) => d.date),
    datasets: [
      {
        label: 'DAU',
        data: dauData.map((d) => d.dau),
        borderColor: 'rgb(59, 130, 246)',
        backgroundColor: 'rgba(59, 130, 246, 0.1)',
        fill: true,
        tension: 0.3,
      },
    ],
  };

  const installChartData = {
    labels: installData.data.map((d) => d.date),
    datasets: [
      {
        label: 'Installs',
        data: installData.data.map((d) => d.installs),
        backgroundColor: 'rgba(34, 197, 94, 0.8)',
      },
    ],
  };

  const chartOptions = {
    responsive: true,
    plugins: {
      legend: {
        display: false,
      },
    },
    scales: {
      y: {
        beginAtZero: true,
      },
    },
  };

  const todayDau = dauData.length > 0 ? dauData[dauData.length - 1]?.dau || 0 : 0;

  return (
    <div className="min-h-screen bg-slate-100">
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex justify-between items-center">
          <div className="flex items-center gap-4">
            <Link href="/dashboard" className="text-slate-500 hover:text-slate-700">
              &larr; Back
            </Link>
            <h1 className="text-2xl font-bold text-slate-800">{appId}</h1>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8">
        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-white rounded-xl p-6 shadow-sm">
            <p className="text-sm text-slate-500">Today Active Users</p>
            <p className="text-3xl font-bold text-blue-600">{todayDau.toLocaleString()}</p>
          </div>
          <div className="bg-white rounded-xl p-6 shadow-sm">
            <p className="text-sm text-slate-500">Total Installs</p>
            <p className="text-3xl font-bold text-green-600">{installData.total.toLocaleString()}</p>
          </div>
        </div>

        {/* DAU Chart */}
        <div className="bg-white rounded-xl p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-700 mb-4">Daily Active Users (30 days)</h2>
          {dauData.length > 0 ? (
            <Line data={dauChartData} options={chartOptions} />
          ) : (
            <p className="text-slate-500 text-center py-8">No data yet</p>
          )}
        </div>

        {/* Installs Chart */}
        <div className="bg-white rounded-xl p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-700 mb-4">Daily Installs (30 days)</h2>
          {installData.data.length > 0 ? (
            <Bar data={installChartData} options={chartOptions} />
          ) : (
            <p className="text-slate-500 text-center py-8">No data yet</p>
          )}
        </div>

        {/* Retention Table */}
        <div className="bg-white rounded-xl p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-700 mb-4">Retention Rate</h2>
          {retentionData.length > 0 ? (
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-slate-200">
                    <th className="text-left py-3 px-4 text-slate-600">Cohort Date</th>
                    <th className="text-right py-3 px-4 text-slate-600">Users</th>
                    <th className="text-right py-3 px-4 text-slate-600">Day 1</th>
                    <th className="text-right py-3 px-4 text-slate-600">Day 7</th>
                    <th className="text-right py-3 px-4 text-slate-600">Day 30</th>
                  </tr>
                </thead>
                <tbody>
                  {retentionData.slice(0, 14).map((row) => (
                    <tr key={row.cohort_date} className="border-b border-slate-100 hover:bg-slate-50">
                      <td className="py-3 px-4 text-slate-800">{row.cohort_date}</td>
                      <td className="py-3 px-4 text-right text-slate-800">{row.day0}</td>
                      <td className="py-3 px-4 text-right">
                        {row.day1 !== null ? (
                          <span className={`font-medium ${row.day1 >= 40 ? 'text-green-600' : row.day1 >= 20 ? 'text-yellow-600' : 'text-red-500'}`}>
                            {row.day1.toFixed(1)}%
                          </span>
                        ) : (
                          <span className="text-slate-400">-</span>
                        )}
                      </td>
                      <td className="py-3 px-4 text-right">
                        {row.day7 !== null ? (
                          <span className={`font-medium ${row.day7 >= 20 ? 'text-green-600' : row.day7 >= 10 ? 'text-yellow-600' : 'text-red-500'}`}>
                            {row.day7.toFixed(1)}%
                          </span>
                        ) : (
                          <span className="text-slate-400">-</span>
                        )}
                      </td>
                      <td className="py-3 px-4 text-right">
                        {row.day30 !== null ? (
                          <span className={`font-medium ${row.day30 >= 10 ? 'text-green-600' : row.day30 >= 5 ? 'text-yellow-600' : 'text-red-500'}`}>
                            {row.day30.toFixed(1)}%
                          </span>
                        ) : (
                          <span className="text-slate-400">-</span>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          ) : (
            <p className="text-slate-500 text-center py-8">No retention data yet</p>
          )}
        </div>

        {/* Feedbacks */}
        <div className="bg-white rounded-xl p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-700 mb-4">User Feedback</h2>
          {feedbacks.length > 0 ? (
            <div className="space-y-4">
              {feedbacks.map((fb) => (
                <div key={fb.id} className="border-b border-slate-100 pb-4 last:border-0">
                  <p className="text-slate-800">{fb.content}</p>
                  <div className="mt-2 flex gap-4 text-sm text-slate-500">
                    <span>{new Date(fb.created_at).toLocaleString()}</span>
                    {fb.contact && <span>Contact: {fb.contact}</span>}
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <p className="text-slate-500 text-center py-8">No feedback yet</p>
          )}
        </div>
      </main>
    </div>
  );
}
