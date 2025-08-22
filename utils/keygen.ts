const KEY_LENGTH: number = 70;
const CHARS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';

/**
 * Generates a secure random string of a given length.
 * @param length The desired length of the string.
 * @returns A random string.
 */
function generateApiKey(length: number): string {
  const randomBytes = new Uint8Array(length);
  crypto.getRandomValues(randomBytes);
  let result = '';
  for (let i = 0; i < length; i++) {
    result += CHARS[randomBytes[i] % CHARS.length];
  }
  return result;
}

const apiKey = generateApiKey(KEY_LENGTH);

console.log("Your new API key is:");
console.log(apiKey);
