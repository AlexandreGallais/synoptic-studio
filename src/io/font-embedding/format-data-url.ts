import { bytesToBase64 } from "./bytes-to-base64";

/**
 * A file written as a `data:` URL in base 64 (RFC 2397, §3): `data:`, its media type,
 * `;base64,` and its bytes in base 64 (`bytesToBase64`), `21 + 4 · ⌈n / 3⌉` characters for a font
 * of `n` bytes (`DERIV-font-embedding`, step 6).
 *
 * @kind format
 * @param mediaType - its media type, such as `font/ttf`
 * @param bytes - the content of the file
 * @returns the `data:` URL, holding the whole file
 * @see REF-RFC-2397
 */
export function formatDataUrl(mediaType: string, bytes: Readonly<Uint8Array>): string {
  return `data:${mediaType};base64,${bytesToBase64(bytes)}`;
}
