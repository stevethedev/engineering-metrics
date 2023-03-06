import { Vstack, VstackProps } from "../../../blocks/vstack";
import { mash } from "../../../../utils/class-mash";

export type NavProps = VstackProps;

export const Nav = mash<NavProps>(({ ...props }) => {
  return <Vstack flex={0} {...props} />;
});
