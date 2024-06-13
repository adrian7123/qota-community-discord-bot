import mongoose, { Schema } from 'mongoose';
import { IMix } from './types/IMix';

type IMixModel = IMix & Document;

const MixSchema = new mongoose.Schema(
  {
    date: {
      type: Date,
      default: Date.now()
    },
    players: [
      {
        type: Schema.ObjectId,
        ref: 'players'
      }
    ],
    expired: { type: Boolean, default: false }
  },
  { timestamps: true }
);

MixSchema.index({ date: 1 }, { unique: true });

const Mix = mongoose.model<IMixModel>('mix', MixSchema);

export { Mix };

// model mix {
//   id        String         @id @default(uuid())
//   date      DateTime       @default(now())
//   players   mix_player[]
//   createdAt DateTime       @default(now())
//   updatedAt DateTime       @updatedAt
//   expired   Boolean        @default(false)
//   cron      mix_schedule[]
// }
