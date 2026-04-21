# ArachNode

## Local Services

The crawler and search API both expect MongoDB to be running before searches can read from the `pages` collection.

Default MongoDB settings:

```powershell
$env:MONGODB_URI="mongodb://127.0.0.1:27017"
$env:MONGODB_DB_NAME="search_engine"
```

On Windows, start the local MongoDB service from an Administrator PowerShell:

```powershell
Start-Service MongoDB
```

Then run the API:

```powershell
cd search-api
npm run dev
```
