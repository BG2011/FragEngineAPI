# 🚀 FragEngine API Deployment Guide

This guide provides step-by-step instructions for hosting your **FragEngine API** for free. Since the project is now a standalone lightweight service, it builds perfectly on Render's free tier.

---

## 🌐 Option 1: Render (Recommended)

[Render](https://render.com/) is the easiest way to host this API.

### 1. Connect your GitHub repository
1. Go to [Render Dashboard](https://dashboard.render.com/).
2. Select **New** -> **Web Service**.
3. Connect your GitHub Repo: `https://github.com/BG2011/FragEngineAPI`.
4. Render will detect the `Dockerfile` automatically.

### 2. Configure Environment Variables
In the Render dashboard, go to **Environment** and add:
- `DATABASE_URL`: Your Supabase connection string.
- `PORT`: `3000`
- `RUST_LOG`: `info`

### 3. Deploy!
The build will take a few minutes (it uses 1 core to save memory). Once finished, your API will be live at `https://your-service-name.onrender.com`.

---

## 🔑 Managing API Keys & Tiers

The API enforces both **Feature Tiers** and **Numerical Limits**. Manage them via the Supabase SQL Editor:

### Create a New Key
```sql
-- Create a PRO key with 50,000 requests/month
INSERT INTO api_keys (key, tier, request_limit) 
VALUES ('secret_pro_key_123', 'PRO', 50000);

-- Create an ULTRA key with 500,000 requests/month
INSERT INTO api_keys (key, tier, request_limit) 
VALUES ('secret_ultra_key_999', 'ULTRA', 500000);
```

### Reset Monthly Usage
Run this on the 1st of every month to reset all counts:
```sql
UPDATE api_keys SET request_count = 0;
```

### Monitor Usage
```sql
SELECT key, tier, request_count, request_limit, last_used 
FROM api_keys 
ORDER BY request_count DESC;
```

---

## 🛡️ RapidAPI Security (The Master Key Strategy)

To avoid manually adding every RapidAPI user to your database, you should use the **"Master Key"** secondary authentication method.

### 1. Create a Master Key in Supabase
Run this in the Supabase SQL Editor:
```sql
INSERT INTO api_keys (key, tier, request_limit) 
VALUES ('RAPIDAPI_INTERNAL_MASTER_KEY_999', 'ULTRA', NULL);
```

### 2. Configure RapidAPI "Add Header" Transformation
Follow these steps in the [RapidAPI Provider Dashboard](https://rapidapi.com/studio):

1.  Go to the **"Definition"** or **"Endpoints"** tab.
2.  Navigate to the **"Transformations"** sub-tab.
3.  In the **"Request Transformations"** section, click **"+ Add Header"**.
4.  **Header Name**: `x-api-key`
5.  **Header Value**: `RAPIDAPI_INTERNAL_MASTER_KEY_999`
6.  Check the box: **"Hide from Consumer"**.

### ✅ Success!
Now, when a user pays on RapidAPI:
- RapidAPI validates their billing and limits.
- RapidAPI automatically attaches your **Master Key** to the request before sending it to Render.
- Your Render server sees the Master Key and grants full access.

---

## 🛠️ Local Development

To run the API locally:
1. Ensure you have a `.env` file with `DATABASE_URL`.
2. Run:
```bash
cargo run
```
