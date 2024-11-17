import { randomBytes } from 'crypto';

export function generateRandomHex(size: number): string {
  const bytes = randomBytes(size);
  return '0x' + bytes.toString('hex');
}

