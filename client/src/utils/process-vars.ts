import { isValidString, startsWith, toKebabCase } from "./strings";
import { Entries, fromEntries, toEntries } from "./objects";
import { isNullish } from "./nullish";
import { isValidNumber } from "./numbers";

export type ProcessedVars<
  P extends Record<string, string | number | undefined | null>
> = {
  [K in keyof P as K extends `--${string}` ? K : `--${K & string}`]?: Exclude<
    P[K],
    undefined | null
  >;
};

export interface VarObject<T extends string | number = string | number> {
  default?: T;

  tablet?: T;

  desktop?: T;
}

export type Var<
  P extends string | number | undefined | null =
    | string
    | number
    | undefined
    | null
> = P | VarObject<Exclude<P, undefined | null>>;

export type VarProp<
  P extends Record<string, string | number | undefined | null> = Record<
    string,
    string | number | undefined | null
  >
> = {
  [K in keyof P]: P[K] | Var<P[K]>;
};

/**
 * Process the vars object to be used in the css variables
 * @param vars The vars object.
 * @returns The processed vars object.
 */
export const processVars = <
  P extends Record<string, string | number | undefined | null>
>(
  vars?: VarProp<P>
): ProcessedVars<P> => {
  if (!vars) {
    return {};
  }

  const entries = toEntries(vars);
  const kebabEntries = entries.map(([k, value]) => {
    const key = String(k);
    const dashedKey = startsWith(key, "--") ? key : `--${key}`;
    const kebabKey = toKebabCase(dashedKey) as `--${string}`;
    return [kebabKey, value] as const;
  });
  const flattenedEntries = kebabEntries.flatMap(
    ([key, value]): [`--${string}`, P[keyof P]][] => {
      if (isNullish(value)) {
        return [];
      }

      if (typeof value !== "object") {
        return [[key, value as P[keyof P]]];
      }

      return toEntries<VarObject>(value).map(([k, v]) => [
        k === "default" ? key : `${key}--${k}`,
        v as P[keyof P],
      ]);
    }
  );

  const filteredEntries = flattenedEntries.filter(([, value]) => {
    return !isNullish(value) && (isValidString(value) || isValidNumber(value));
  }) as Entries<ProcessedVars<P>>;

  return fromEntries(filteredEntries);
};
