import { IMix } from '../models/types/IMix';
import { IPlayer } from '../models/types/IPlayer';

export class MixHelper {
  createMix(date: Date) {}

  makeMixListMessage(mix: IMix, players: IPlayer[]): string {
    const message = ['Mix Que Ota Community **%d/%m** **%H:%M**', '\n'];

    let pos = 0;

    for (let player of players) {
      pos++;
      message.push(`${pos} - <@${player.discordId}>`);
    }

    message.push('\n');

    return message.join('\n');
  }
}
