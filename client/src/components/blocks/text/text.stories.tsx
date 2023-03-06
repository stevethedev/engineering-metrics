import { Text, TextProps } from "./text";

export default {
  title: "Blocks/Text",
  component: Text,
};

export const Default = (args: TextProps) => <Text {...args} />;
Default.storyName = "Text";
Default.args = {
  children: "This is an example component.",
};
