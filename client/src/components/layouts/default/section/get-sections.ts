import { isValidElement, ReactNode } from "react";
import { Header } from "./header";
import { Footer } from "./footer";
import { Main } from "./main";
import { Nav } from "./nav";
import { Aside } from "./aside";

interface Sections {
  header?: ReactNode;
  nav?: ReactNode;
  main?: ReactNode;
  aside?: ReactNode;
  footer?: ReactNode;
}

export const getSections = (children: readonly ReactNode[]): Sections => {
  return children.reduce((acc: Sections, child) => {
    if (isValidElement(child)) {
      const { type } = child;
      if (type === Header) {
        acc.header = child;
      } else if (type === Footer) {
        acc.footer = child;
      } else if (type === Main) {
        acc.main = child;
      } else if (type === Nav) {
        acc.nav = child;
      } else if (type === Aside) {
        acc.aside = child;
      }
    }
    return acc;
  }, {});
};
