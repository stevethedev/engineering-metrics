import styles from "./hstack.module.scss";
import { Box, BoxProps } from "../box";
import { mash } from "../../../utils/class-mash";
import { VarProp } from "../../../utils/process-vars";
import { CSSProperties } from "react";

export interface HstackProps
  extends BoxProps,
    VarProp<{
      gap?: CSSProperties["gap"];
      valign?: CSSProperties["alignItems"];
      halign?: CSSProperties["justifyContent"];
    }> {}

export const Hstack = mash<HstackProps>(
  ({
    children,
    halign: justifyContent,
    valign: alignItems,
    gap,

    ...props
  }) => {
    return (
      <Box
        vars={{
          gap,
          alignItems,
          justifyContent,
        }}
        {...props}
      >
        {children}
      </Box>
    );
  },
  styles.hstack
);
