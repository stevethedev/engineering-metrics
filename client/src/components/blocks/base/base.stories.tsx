import { Base, BaseProps } from "./base";

export default {
  title: "Blocks/Base",
  component: Base,
  args: {
    children: "This is an example component.",
  },
};

export const Default = (args: BaseProps) => <Base {...args} />;
Default.storyName = "Base";
