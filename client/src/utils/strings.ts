export const isValidString = (value: unknown): value is string => {
  return typeof value === "string";
};

export const startsWith = <T extends string>(
  str: string,
  search: T
): str is `${T}${string}` => str.startsWith(search);

export const toKebabCase = (str: string): string => {
  return str
    .replace(/([a-z])([A-Z])/g, "$1-$2")
    .replace(/\s/g, "_")
    .toLowerCase();
};
