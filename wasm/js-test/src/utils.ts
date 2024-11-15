import { randomBytes } from 'crypto';

export function generateRandom32Bytes(): string {
  const bytes = randomBytes(32);
  return '0x' + bytes.toString('hex');
}