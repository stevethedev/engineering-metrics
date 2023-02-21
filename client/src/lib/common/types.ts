import { DeepReadonly } from "ts-essentials";

/**
 * Provides a compile-time check that the given parameter meets the provided type.
 * @param value
 */
export const ensure = <T>(value: T): T => value;

/**
 * Converts the given parameter into its equivalent readonly type.
 * @param value
 */
export const readonly = <T>(value: T): DeepReadonly<T> =>
  value as DeepReadonly<T>;
