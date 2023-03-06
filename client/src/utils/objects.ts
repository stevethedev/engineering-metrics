export const hasOwnProperty = <T>(obj: T, prop: PropertyKey): prop is keyof T =>
  Object.prototype.hasOwnProperty.call(obj, prop);

export const fromEntries = <T extends object>(arr: Entries<T>) =>
  Object.fromEntries(arr) as T;

export type Entries<T extends object> = [keyof T, T[keyof T]][];
export const toEntries = <T extends object>(obj: T): Entries<T> =>
  Object.entries(obj) as [keyof T, T[keyof T]][];
