import { CSSProperties, ReactNode } from "react";

import styles from "./grid.module.scss";
import { Box, BoxProps } from "../box";
import { mash } from "../../../utils/class-mash";
import { VarProp } from "../../../utils/process-vars";

export interface GridProps
  extends BoxProps,
    VarProp<{
      rows?: CSSProperties["gridTemplateRows"];
      cols?: CSSProperties["gridTemplateColumns"];
      gap?: CSSProperties["gridGap"];
    }> {
  children: ReactNode;
}

export const Grid = mash<GridProps>(
  ({ children, rows, cols, gap, ...props }) => {
    return (
      <Box
        vars={{
          gridTemplateRows: rows,
          gridTemplateColumns: cols,
          gridGap: gap,
        }}
        {...props}
      >
        {children}
      </Box>
    );
  },
  styles.grid
);
