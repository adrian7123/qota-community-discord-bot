import * as ping from './ping';
import * as admins from './admins';
import * as base from './base';

export const commands = {
  ping,
  ...admins,
  ...base
};
