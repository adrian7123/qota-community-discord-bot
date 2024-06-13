import dotenv from 'dotenv';

dotenv.config();

const { DISCORD_TOKEN, DISCORD_CLIENT_ID, MONGODB_URI, MONGODB_DB_NAME } =
  process.env;

if (!DISCORD_TOKEN || !DISCORD_CLIENT_ID || !MONGODB_URI || !MONGODB_DB_NAME) {
  console.log(process.env);
  throw new Error('Missing environment variables');
}

export const config = {
  DISCORD_TOKEN,
  DISCORD_CLIENT_ID,
  MONGODB_URI,
  MONGODB_DB_NAME
};
