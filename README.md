# AppScope

A lightweight, self-hosted analytics system for tracking app metrics.

## Features

- Daily Active Users (DAU) tracking
- Install tracking
- Retention rate analysis (Day 1/7/30)
- User feedback collection
- Simple dashboard with charts

## Architecture

- **Backend**: Rust (Axum) with SQLite
- **Frontend**: Next.js with Tailwind CSS
- **SDK**: TypeScript client library

## Quick Start

### Using Docker Compose

1. Clone the repository
2. Copy `.env.example` to `.env` and configure your keys
3. Run:

```bash
docker-compose up -d
```

The dashboard will be available at `http://localhost:3000`

### Manual Setup

#### Backend

```bash
cd backend
cp .env.example .env
# Edit .env with your keys
cargo run --release
```

#### Frontend

```bash
cd frontend
cp .env.example .env.local
# Edit .env.local with your API URL
npm install
npm run build
npm start
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `WRITE_KEY` | Key for client SDK to send events | `wk_default_key` |
| `READ_KEY` | Key for dashboard access | `rk_default_key` |
| `DATABASE_URL` | SQLite database path | `sqlite:appscope.db` |
| `PORT` | Backend server port | `3001` |
| `NEXT_PUBLIC_API_URL` | Backend API URL for frontend | `http://localhost:3001` |

## SDK Usage

### Installation

```bash
npm install @appscope/analytics
```

### Usage

```typescript
import { Analytics } from '@appscope/analytics';

const analytics = new Analytics({
  writeKey: 'wk_your_write_key',
  appId: 'your-app-name',
  apiEndpoint: 'https://your-server.com',
});

// Auto-tracks $open and $install events

// Track custom events
analytics.track('button_click', { button: 'signup' });

// Submit feedback
analytics.feedback('Great app!', 'user@email.com');
```

## API Endpoints

### Write APIs (requires `X-Write-Key` header)

- `POST /api/track` - Track events
- `POST /api/feedback` - Submit feedback

### Read APIs (requires `X-Read-Key` header)

- `GET /api/apps` - List all apps
- `GET /api/stats/dau?app_id=xxx&days=30` - Get DAU data
- `GET /api/stats/installs?app_id=xxx&days=30` - Get install data
- `GET /api/stats/retention?app_id=xxx` - Get retention data
- `GET /api/feedbacks?app_id=xxx&limit=50` - Get feedback list

## License

MIT
