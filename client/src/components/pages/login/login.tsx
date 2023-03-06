import { ReactNode } from "react";
import { mash } from "../../../utils/class-mash";
import { type BoxProps } from "../../blocks/box";

export interface LoginProps extends BoxProps {
  children: ReactNode[] | ReactNode;
}

export const Login = mash<LoginProps>(({ children }) => {
  return <div>{children}</div>;
});
