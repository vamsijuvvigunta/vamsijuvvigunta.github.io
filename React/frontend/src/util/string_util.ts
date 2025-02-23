// Trims the supplied char from the front of the string if it exists.
export function trimCharFromFront(str: string, char: string): string {
    return str.replace(new RegExp(`^${char}+`), '');
  }