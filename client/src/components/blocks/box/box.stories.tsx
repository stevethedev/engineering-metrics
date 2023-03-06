import { Box, BoxProps } from "./box";

export default {
  title: "Blocks/Box",
  component: Box,
  argTypes: {
    as: {
      control: {
        type: "text",
      },
    },
    children: {
      control: {
        type: "text",
      },
    },
    isLifted: {
      control: {
        type: "boolean",
      },
    },
    isInline: {
      control: {
        type: "boolean",
      },
    },
  },
  args: {
    as: "div",
    children: "This is an example component.",
  },
};

export const Default = (args: BoxProps) => <Box {...args} />;
Default.storyName = "Box";
