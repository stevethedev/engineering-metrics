import { Vstack, VstackProps } from "../../../blocks/vstack";
import { mash } from "../../../../utils/class-mash";

export type MainProps = VstackProps;

export const Main = mash<MainProps>((props) => {
  return <Vstack flex={1} {...props} />;
});
