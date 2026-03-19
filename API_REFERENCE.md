# 🔫 FragEngine API v1.0 - Reference Manual

Welcome to the **FragEngine API**. This professional-grade Rust API provides real-time access to the most granular CS2 performance data, team statistics, and player metrics.

---

## 🚀 Getting Started

### Authentication
All requests require an API key sent in either the `x-api-key` or `X-RapidAPI-Key` header.

**RapidAPI Example:**
```bash
curl -H "X-RapidAPI-Key: your_rapidapi_key" \
     -H "X-RapidAPI-Host: fragengine.p.rapidapi.com" \
     https://cs2-datapipeline.onrender.com/teams
```

---

## 💎 Subscription Tiers
| Feature | BASIC | STARTER | PRO | ULTRA |
| --- | :---: | :---: | :---: | :---: |
| Price | **Free** | **$14.99/mo** | **$39.99/mo** | **$99.99/mo** |
| Requests / mo | 500 | 5,000 | 50,000 | 500,000 |
| Team Lists (`/teams`) | ✅ | ✅ | ✅ | ✅ |
| Team Details (`/teams/:id`) | ✅ | ✅ | ✅ | ✅ |
| Roster Data (`/players`) | ❌ | ✅ | ✅ | ✅ |
| Map Statistics (`/map-stats`) | ❌ | ❌ | ✅ | ✅ |
| **Historical H2H Records** | ❌ | ❌ | ✅ | ✅ |
| **Granular Player Metrics** | ❌ | ❌ | ❌ | ✅ |

---

## 📋 Endpoints & JSON Examples

### 1. Teams [BASIC]

#### `GET /teams`
Returns a list of all professional teams.

**Response Example:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Natus Vincere",
    "world_ranking": "#1",
    "avg_player_age": "22.4"
  }
]
```

#### `GET /teams/:id`
Fetch a specific team by **UUID** or **Name**.
- **Path Param**: `id` (e.g., `natus-vincere` or `uuid`)

---

### 2. Rosters & Performance [STARTER+]

#### `GET /teams/:id/players`
Get the current roster for a team. [Starts at STARTER Tier]

**Response Example:**
```json
[
  {
    "name": "jL",
    "hltv_id": 19206,
    "rating": "1.18",
    "maps_played": 142
  }
]
```

#### `GET /teams/:id/map-stats`
Aggregate performance metrics across all maps. [Starts at PRO Tier]

**Response Example:**
```json
[
  {
    "map_name": "Mirage",
    "ct_rounds_won": "54.2%",
    "t_rounds_won": "48.9%",
    "round_win_after_first_kill": "72.4%"
  }
]
```

---

### 3. Professional Analytics [ULTRA]

#### `GET /players/:id/stats`
Granular player metrics broken down by map.

**Response Example:**
```json
[
  {
    "map_name": "Ancient",
    "rating_3_0": "1.24",
    "adr": "84.2",
    "kast": "76.4%",
    "multi_kill": "18.2%"
  }
]
```

#### `GET /h2h/:team1/:team2`
Historical records between two teams.

---

## 🛠️ Error Codes

| Code | Status | Meaning |
| --- | --- | --- |
| `200` | OK | Request successful. |
| `401` | Unauthorized | Missing or invalid API Key. |
| `403` | Forbidden | Tier Upgrade Required for this endpoint. |
| `404` | Not Found | Team or Player ID does not exist. |
| `429` | Too Many Requests | Monthly request limit exceeded. |
| `500` | Server Error | Internal database or server issue. |

---

## 🛠️ Integration Examples

### Python (requests)
```python
import requests

HEADERS = {"X-RapidAPI-Key": "YOUR_KEY"}
BASE_URL = "https://cs2-datapipeline.onrender.com"

# Fetch Team Stats
res = requests.get(f"{BASE_URL}/teams/1/map-stats", headers=HEADERS)
print(res.json())
```
