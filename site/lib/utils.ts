type ClassValue = string | false | null | undefined;

export function cn(...inputs: ClassValue[]) {
  return inputs.filter(Boolean).join(" ");
}
