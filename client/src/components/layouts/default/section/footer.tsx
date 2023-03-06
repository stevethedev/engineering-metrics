import { mash } from "../../../../utils/class-mash";
import { Vstack, VstackProps } from "../../../blocks/vstack";

export type FooterProps = VstackProps;

export const Footer = mash<FooterProps>((props) => {
  return <Vstack flex={0} {...props} />;
});
