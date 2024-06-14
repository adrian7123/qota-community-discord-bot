import moment from 'moment';
import { IMix } from '../models/types/IMix';
import { IPlayer } from '../models/types/IPlayer';

export class MixHelper {
  createMix(date: Date) {}

  makeMixListMessage(mix: IMix, players: IPlayer[]): string {
    const day = moment(mix.date).format('DD/MM/YY');
    const hour = moment(mix.date).format('HH/mm');

    const message = [`Mix Que Ota Community **${day}** **${hour}**`, ''];

    let pos = 0;

    for (let player of players) {
      pos++;
      message.push(`${pos} - <@${player.discordId}>`);
    }

    message.push('\n');

    return message.join('\n');
  }
}
