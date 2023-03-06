import { createElement } from "react";
import { classMash, ClassName } from "../../../utils/class-mash";
import { processVars, VarProp } from "../../../utils/process-vars";

export type BaseProps<C extends keyof JSX.IntrinsicElements = "div"> =
  JSX.IntrinsicElements[C] & {
    as: C;
    className?: ClassName;
    vars?: VarProp;
  };

export const Base = <C extends keyof JSX.IntrinsicElements>({
  as,
  children,
  className: baseClassName,
  style: baseStyle,
  vars: baseVars,
  ...props
}: BaseProps<C>) => {
  const Component = Boolean(
    (as as undefined | keyof JSX.IntrinsicElements)?.length
  )
    ? as
    : "div";
  const className = classMash(baseClassName);
  const style = {
    ...baseStyle,
    ...processVars(baseVars),
  };
  return createElement(Component, { className, style, ...props }, children);
};
