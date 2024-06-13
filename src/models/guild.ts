import mongoose from "mongoose";
import { IGuild } from "./types/IGuild";

type IGuildModel = IGuild & Document;

const GuildSchema = new mongoose.Schema({
  id: {
    type: String,
    unique: true,
  },
});

GuildSchema.index({ id: 1 }, { unique: true });

const Guild = mongoose.model<IGuildModel>("guilds", GuildSchema);

export { Guild };
