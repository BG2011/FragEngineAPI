# 🔫 FragEngine API v1.0 - Reference Manual

Welcome to the **FragEngine API**. This professional-grade Rust API provides real-time access to the most granular CS2 performance data, team statistics, and player metrics.

---

## 🚀 Getting Started

### Authentication
All requests (except `/` and `/health`) require an API key sent in the `x-api-key` header.

**Example:**
```bash
curl -H "x-api-key: your_secret_key" https://cs2-datapipeline.onrender.com/teams
```

### Base URL
- **Production**: `https://cs2-datapipeline.onrender.com`

---

## 💎 Subscription Tiers
| Feature | BASIC | PRO | ULTRA |
| --- | :---: | :---: | :---: |
| Price | **Free** | **$49/mo** | **$249/mo** |
| Team Lists (`/teams`) | ✅ | ✅ | ✅ |
| Team Details (`/teams/:id`) | ✅ | ✅ | ✅ |
| Roster Data (`/players`) | ❌ | ✅ | ✅ |
| Map Statistics (`/map-stats`) | ❌ | ✅ | ✅ |
| **Granular Player Metrics** | ❌ | ❌ | ✅ |
| **Historical H2H Records** | ❌ | ❌ | ✅ |

---

## 📋 Endpoints

### 1. Teams [FREE]

#### `GET /teams`
Returns a list of all teams in the database.

#### `GET /teams/:id`
Fetch detailed information for a specific team by its **UUID** or **Name**.

---

### 2. Rosters & Performance [PRO]

#### `GET /teams/:id/players`
Get the current roster for a team.

#### `GET /teams/:id/map-stats`
Fetch aggregate team performance metrics across all active duty maps (Win rates, conversion rates, etc.).

---

### 3. Professional Analytics [ULTRA]

#### `GET /players/:id/stats`
Returns the most granular player metrics available, broken down by map.
- `rating_3_0`: Advanced performance metric.
- `adr`, `kast`, `multi_kill` frequency.

#### `GET /h2h/:team1/:team2`
Search the historical archives for matches between two specific teams.

---

## 🛠️ Integration Examples

### Python (requests)
```python
import requests

HEADERS = {"x-api-key": "your_secret_key"}
BASE_URL = "https://cs2-datapipeline.onrender.com"

def get_team_stats(team_name):
    response = requests.get(f"{BASE_URL}/teams/{team_name}/map-stats", headers=HEADERS)
    return response.json()
```

### JavaScript (Fetch)
```javascript
const getRoster = async (teamId) => {
  const res = await fetch(`https://cs2-datapipeline.onrender.com/teams/${teamId}/players`, {
    headers: { "x-api-key": "your_secret_key" }
  });
  return res.json();
};
```
