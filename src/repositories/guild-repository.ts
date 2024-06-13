import { Guild } from "../models/guild";
import { IGuild } from "../models/types/IGuild";

export const checkIfServerExistsAndPut = async (id: string) => {
  const guild = await Guild.findOne({ id });

  if (!guild) {
    await Guild.create({ id });
  }
};

export const getAllGuilds = async (): Promise<IGuild[]> => {
  const guilds = await Guild.find();

  return guilds;
};
