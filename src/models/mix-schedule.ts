import mongoose, { Schema } from 'mongoose';
import { IMixSchedule } from './types/IMixSchedule';

type IMixScheduleModel = IMixSchedule & Document;

const MixScheduleSchema = new mongoose.Schema(
  {
    schedule: { type: String, default: '* * * * * *' },
    executed: { type: Boolean, default: false },
    mix: { type: Schema.ObjectId, ref: 'mix', required: true }
  },
  { timestamps: true }
);

MixScheduleSchema.index({ schedule: 1 }, { unique: true });

const MixSchedule = mongoose.model<IMixScheduleModel>(
  'mix_schedule',
  MixScheduleSchema
);

export { MixSchedule };

// model mix_schedule {
//   id        String   @id
//   schedule  String   @default("* * * * * *")
//   executed  Boolean  @default(false)
//   mix       mix      @relation(fields: [mixId], references: [id], onDelete: Cascade)
//   createdAt DateTime @default(now())
//   updatedAt DateTime @updatedAt
//   mixId     String
// }
