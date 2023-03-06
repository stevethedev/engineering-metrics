import { Fragment, ReactNode } from "react";
import { mash } from "../../../utils/class-mash";
import { Box, type BoxProps } from "../box";

import styles from "./text.module.scss";

type HeadingLevel = 1 | 2 | 3 | 4 | 5 | 6;

export interface TextProps extends BoxProps {
  heading?: HeadingLevel;

  children: ReactNode[] | ReactNode;

  isMono?: boolean;
}

export const Text = mash<TextProps>(
  ({ children, heading, isMono, className, ...props }) => {
    const as = getAs(heading) ?? Fragment;

    return (
      <Box
        as={as}
        className={[className, getClassName(heading, isMono)]}
        {...props}
        isInline
      >
        {children}
      </Box>
    );
  },
  styles.text
);

const getClassName = (heading?: HeadingLevel, isMono?: boolean) => {
  const classNames = [];

  if (isMono) {
    classNames.push(styles["text--mono"]);
  }

  switch (heading) {
    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
      classNames.push(styles[`text--h${heading}`]);
  }

  return classNames;
};

const getAs = (heading?: HeadingLevel) => {
  switch (heading) {
    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
      return `h${heading}` as const;
  }

  return undefined;
};
