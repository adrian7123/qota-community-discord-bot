import {
  CommandInteraction,
  PermissionFlagsBits,
  SlashCommandBuilder
} from 'discord.js';

import { MixHelper } from '../helpers/mix-helper';

export const lista = {
  data: new SlashCommandBuilder()
    .setName('lista')
    .setDescription('Lista de participantes do mix'),

  execute(interaction: CommandInteraction) {
    return interaction.reply(
      new MixHelper().makeMixListMessage(
        {
          date: new Date(),
          players: [],
          expired: false,
          createdAt: new Date(),
          updatedAt: new Date()
        },
        [
          {
            name: '',
            discordId: '496355712352518155',
            createdAt: new Date(),
            updatedAt: new Date()
          },
          {
            name: '',
            discordId: '496355712352518155',
            createdAt: new Date(),
            updatedAt: new Date()
          },
          {
            name: '',
            discordId: '496355712352518155',
            createdAt: new Date(),
            updatedAt: new Date()
          },
          {
            name: '',
            discordId: '496355712352518155',
            createdAt: new Date(),
            updatedAt: new Date()
          },
          {
            name: '',
            discordId: '496355712352518155',
            createdAt: new Date(),
            updatedAt: new Date()
          }
        ]
      )
    );
  }
};
