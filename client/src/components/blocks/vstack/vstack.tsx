import styles from "./vstack.module.scss";
import { Box, BoxProps } from "../box";
import { mash } from "../../../utils/class-mash";
import { VarProp } from "../../../utils/process-vars";
import { CSSProperties } from "react";

export interface VstackProps
  extends BoxProps,
    VarProp<{
      gap?: CSSProperties["gap"];
      valign?: CSSProperties["alignItems"];
      halign?: CSSProperties["justifyContent"];
    }> {}

export const Vstack = mash<VstackProps>(
  ({
    children,
    halign: alignItems,
    valign: justifyContent,
    gap,

    ...props
  }) => {
    return (
      <Box vars={{ gap, alignItems, justifyContent }} {...props}>
        {children}
      </Box>
    );
  },
  styles.vstack
);
