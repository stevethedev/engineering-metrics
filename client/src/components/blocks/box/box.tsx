import styles from "./box.module.scss";

import { CSSProperties, HTMLAttributes, ReactNode } from "react";
import { mash } from "../../../utils/class-mash";
import { VarProp } from "../../../utils/process-vars";
import { Base, BaseProps } from "../base";

export interface BoxProps
  extends HTMLAttributes<HTMLDivElement | HTMLSpanElement>,
    VarProp<{
      flex?: CSSProperties["flex"];
    }> {
  as?: BaseProps["as"];

  children?: ReactNode[] | ReactNode;

  isInline?: boolean;

  isLifted?: boolean;
}

export const Box = mash<BoxProps>(
  ({ as, children, isInline, className, flex, isLifted, ...props }) => {
    const display = isInline ? "inline" : null;

    return (
      <Base
        as={as ?? (isInline ? "span" : "div")}
        vars={{ flex, display }}
        className={[className ?? "", { [styles["box--lifted"]]: isLifted }]}
        {...props}
      >
        {children}
      </Base>
    );
  },
  styles.box
);
