const { MongoClient } = require("mongodb");

const MONGODB_URI = process.env.MONGODB_URI || "mongodb://127.0.0.1:27017";
const DB_NAME = process.env.MONGODB_DB_NAME || "search_engine";
const COLLECTION_NAME = "pages";
const CONNECTION_TIMEOUT_MS = 3000;

let client;
let collection;

function isConnectionRefused(error) {
  return (
    error?.cause?.code === "ECONNREFUSED" ||
    error?.code === "ECONNREFUSED" ||
    error?.message?.includes("ECONNREFUSED")
  );
}

async function getPagesCollection() {
  if (collection) {
    return collection;
  }

  client = new MongoClient(MONGODB_URI, {
    serverSelectionTimeoutMS: CONNECTION_TIMEOUT_MS,
  });

  try {
    await client.connect();
  } catch (error) {
    client = undefined;

    if (isConnectionRefused(error)) {
      throw new Error(
        `MongoDB is not reachable at ${MONGODB_URI}. Start MongoDB or set MONGODB_URI to your running MongoDB connection string.`
      );
    }

    throw error;
  }

  const db = client.db(DB_NAME);
  collection = db.collection(COLLECTION_NAME);

  return collection;
}

module.exports = {
  DB_NAME,
  getPagesCollection,
  MONGODB_URI,
};
