import { Vstack, VstackProps } from "../../../blocks/vstack";
import { mash } from "../../../../utils/class-mash";

export type AsideProps = VstackProps;

export const Aside = mash<AsideProps>(({ ...props }) => {
  return <Vstack flex={0} {...props} />;
});
