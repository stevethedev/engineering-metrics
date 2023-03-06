import { Vstack, VstackProps } from "../../../blocks/vstack";
import { mash } from "../../../../utils/class-mash";

export type HeaderProps = VstackProps;

export const Header = mash<HeaderProps>(({ ...props }) => {
  return <Vstack flex={0} {...props} />;
});
