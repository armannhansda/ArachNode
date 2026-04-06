const { MongoClient } = require("mongodb");

const MONGODB_URI = process.env.MONGODB_URI || "mongodb://127.0.0.1:27017";
const DB_NAME = process.env.MONGODB_DB_NAME || "search_engine";
const COLLECTION_NAME = "pages";

let client;
let collection;

async function getPagesCollection() {
  if (collection) {
    return collection;
  }

  client = new MongoClient(MONGODB_URI);
  await client.connect();

  const db = client.db(DB_NAME);
  collection = db.collection(COLLECTION_NAME);

  return collection;
}

module.exports = {
  getPagesCollection,
};
