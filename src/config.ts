import dotenv from "dotenv";

dotenv.config();

const { DISCORD_TOKEN, DISCORD_CLIENT_ID, MONGODB_URI } = process.env;

if (!DISCORD_TOKEN || !DISCORD_CLIENT_ID || !MONGODB_URI) {
  console.log(process.env);
  throw new Error("Missing environment variables");
}

export const config = {
  DISCORD_TOKEN,
  DISCORD_CLIENT_ID,
  MONGODB_URI,
};
