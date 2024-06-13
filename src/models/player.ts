import mongoose, { Schema } from 'mongoose';
import { IPlayer } from './types/IPlayer';

type IPlayerModel = IPlayer & Document;

const PlayerSchema = new mongoose.Schema(
  {
    name: { type: String },
    discordId: { type: String }
  },
  { timestamps: true }
);

PlayerSchema.index({ schedule: 1 }, { unique: true });

const Player = mongoose.model<IPlayerModel>('players', PlayerSchema);

export { Player };

// model mix_player {
//   id         String   @id @default(uuid())
//   name       String
//   discordId String
//   mix        mix?     @relation(fields: [mixId], references: [id], onDelete: Cascade)
//   mixId      String?
//   createdAt  DateTime @default(now())
//   updatedAt  DateTime @updatedAt
// }
