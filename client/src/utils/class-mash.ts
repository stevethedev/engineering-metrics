import {
  ComponentType,
  createElement,
  CSSProperties,
  FC,
  forwardRef,
  ForwardRefExoticComponent,
  PropsWithoutRef,
  ReactNode,
  RefAttributes,
} from "react";
import { processVars, VarProp } from "./process-vars";
import { hasOwnProperty } from "./objects";

export type ClassName =
  | undefined
  | string
  | Partial<Record<string, boolean>>
  | MashedComponent
  | ClassName[];

const extractClassName = (className: ClassName): string => {
  if (className === undefined) {
    return "";
  }

  if (typeof className === "string") {
    return className;
  }

  if (Array.isArray(className)) {
    return className.map(extractClassName).join(" ");
  }

  if (hasOwnProperty(className, MasherSymbol)) {
    return extractClassName(
      (className as InternalMashedComponent)[MasherSymbol]
    );
  }

  return extractClassName(
    Object.entries(className)
      .filter(([, value]) => value)
      .map(([key]) => key)
  );
};

export const classMash = (...classNames: ClassName[]) => {
  return classNames.map(extractClassName).filter(Boolean).join(" ");
};

export const classMashFactory = (...classNames: ClassName[]) => {
  const className = classMash(...classNames);
  return (...additionalClassNames: ClassName[]) => {
    return classMash(className, ...additionalClassNames);
  };
};

export interface MashedProps {
  className?: ClassName;
  vars?: VarProp;
  style?: CSSProperties;
}

export type MashedComponent<Props extends object = object> =
  ForwardRefExoticComponent<
    PropsWithoutRef<
      PropsWithoutRef<MashedProps & Omit<Props, keyof MashedProps>>
    > &
      RefAttributes<FC<Omit<Props, keyof MashedProps>>>
  >;

const MasherSymbol: unique symbol = Symbol("Masher");

type InternalMashedComponent<Props extends object = object> =
  MashedComponent<Props> & {
    [MasherSymbol]: ClassName;
  };

const isInternalMashedComponent = <Props extends object>(
  component: unknown
): component is InternalMashedComponent<Props> => {
  return hasOwnProperty(component, MasherSymbol);
};

export const mash = <Props extends object>(
  Component: keyof JSX.IntrinsicElements | ComponentType<Props>,
  ...classNames: ClassName[]
): MashedComponent<Props> => {
  const defaultClassName = isInternalMashedComponent(Component)
    ? Component[MasherSymbol]
    : undefined;
  const masher = classMashFactory(defaultClassName, classNames);

  const MashedComponent = forwardRef<
    FC<Props & { children?: ReactNode | ReactNode[] }>,
    PropsWithoutRef<
      MashedProps & Props & { children?: ReactNode | ReactNode[] }
    >
  >(({ className, vars, style, children, ...props }, ref) => {
    return createElement(
      Component as ComponentType<object>,
      {
        ...props,
        ref,
        className: masher(className),
        style: {
          ...style,
          ...processVars(vars ?? {}),
        },
      },
      children
    );
  });

  Object.assign(MashedComponent, { [MasherSymbol]: classMash(classNames) });
  MashedComponent.displayName = `Mashed(${String(Component)})`;
  MashedComponent.toString = () => MashedComponent.displayName ?? "<anonymous>";

  const DEFAULT_PROPS = "defaultProps";
  if (hasOwnProperty(Component, DEFAULT_PROPS)) {
    MashedComponent.defaultProps = Component[DEFAULT_PROPS];
  }

  return MashedComponent as MashedComponent<Props>;
};
