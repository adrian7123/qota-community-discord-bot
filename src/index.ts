import { Client } from 'discord.js';
import { config } from './config';
import { commands } from './commands';
import { deployCommands } from './deploy-commands';
import {
  checkIfServerExistsAndPut,
  getAllGuilds
} from './repositories/guild-repository';
import mongoose from 'mongoose';

mongoose.connect(config.MONGODB_URI!, {
  dbName: config.MONGODB_DB_NAME
});

const client = new Client({
  intents: ['Guilds', 'GuildMessages', 'DirectMessages', 'MessageContent']
});

client.once('ready', async () => {
  console.log('Discord bot is ready! 🤖');

  const guilds = await getAllGuilds();

  const promises = [];

  for (let guild of guilds) {
    promises.push(deployCommands({ guildId: guild.id }));
  }

  await Promise.all(promises);
});

client.on('guildCreate', async (guild) => {
  await deployCommands({ guildId: guild.id });
});

client.on('messageCreate', async (message) => {
  console.log(message);
  await deployCommands({ guildId: message!.guild!.id });
  await checkIfServerExistsAndPut(message!.guild!.id);
});

client.on('interactionCreate', async (interaction) => {
  if (!interaction.isCommand()) {
    return;
  }

  const { commandName } = interaction;

  if (commands[commandName as keyof typeof commands]) {
    commands[commandName as keyof typeof commands].execute(interaction);
  }
});

client.login(config.DISCORD_TOKEN);
