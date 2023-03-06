import { mash } from "../../../utils/class-mash";
import { type BoxProps } from "../../blocks/box";
import {
  DefaultFooter,
  DefaultHeader,
  DefaultLayout,
  DefaultMain,
} from "../default";
import { PageHeader } from "../../containers/page-header";

export type SharedProps = BoxProps;

export const SharedLayout = mash<SharedProps>(({ children }) => {
  return (
    <DefaultLayout>
      <DefaultHeader>
        <PageHeader />
      </DefaultHeader>
      <DefaultMain>{children}</DefaultMain>
      <DefaultFooter>Test</DefaultFooter>
    </DefaultLayout>
  );
});
