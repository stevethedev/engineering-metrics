import { ReactNode } from "react";

import styles from "./default.module.scss";
import { mash } from "../../../utils/class-mash";
import { BoxProps } from "../../blocks/box";
import { getSections } from "./section/get-sections";
import { Grid } from "../../blocks/grid";

export { Header as DefaultHeader } from "./section/header";
export { Footer as DefaultFooter } from "./section/footer";
export { Main as DefaultMain } from "./section/main";
export { Nav as DefaultNav } from "./section/nav";
export { Aside as DefaultAside } from "./section/aside";

export interface Props extends BoxProps {
  children: ReactNode | ReactNode[];
}

export const DefaultLayout = mash<Props>(({ children, ...props }) => {
  const { header, footer, main, nav, aside } = getSections(
    Array.isArray(children) ? children : [children]
  );

  return (
    <Grid rows="auto 1fr auto" {...props}>
      {header}
      <Grid cols="auto 1fr auto">
        {nav}
        {main}
        {aside}
      </Grid>
      {footer}
    </Grid>
  );
}, styles.layout);
