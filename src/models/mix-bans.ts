import mongoose from 'mongoose';
import { IMixBans } from './types/IMixBans';

type IMixBansModel = IMixBans & Document;

const MixBansSchema = new mongoose.Schema(
  {
    discordId: { type: String, unique: true, required: true }
  },
  { timestamps: true }
);

MixBansSchema.index({ discordId: 1 }, { unique: true });

const MixBans = mongoose.model<IMixBansModel>('mix_bans', MixBansSchema);

export { MixBans };

// model mix_bans {
//   id         String   @id @default(uuid())
//   discordId String
//   createdAt  DateTime @default(now())
//   updatedAt  DateTime @updatedAt
// }
