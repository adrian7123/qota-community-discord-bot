import {
  CommandInteraction,
  PermissionFlagsBits,
  SlashCommandBuilder,
} from "discord.js";
import moment from "moment";

export const criarlista = {
  data: new SlashCommandBuilder()
    .setName("criarlista")
    .setDescription("Criar lista para os participantes do mix")
    .setDefaultMemberPermissions(PermissionFlagsBits.BanMembers),

  execute(interaction: CommandInteraction) {
    return interaction.reply(
      `Lista de espera para o dia **${moment(Date.now()).format(
        "DD/MM/YY",
      )}** e hora **${moment(Date.now()).format(
        "HH:mm",
      )}** criada com êxito 🗓️.\n\n`,
    );
  },
};
