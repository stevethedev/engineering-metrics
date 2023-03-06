import { Hstack, HstackProps } from "./hstack";

export default {
  title: "Blocks/Hstack",
  component: Hstack,
  argTypes: {
    valign: {
      control: {
        type: "select",
        options: ["start", "center", "end"],
      },
    },
    halign: {
      control: {
        type: "select",
        options: ["start", "center", "end"],
      },
    },
  },
};

export const Default = (args: HstackProps) => (
  <Hstack {...args}>
    <div style={{ outline: "1px solid" }}>1</div>
    <div style={{ outline: "1px solid" }}>2</div>
    <div style={{ outline: "1px solid" }}>3</div>
  </Hstack>
);
Default.storyName = "Hstack";
Default.args = {
  flex: 1,
  gap: "var(--sp--1)",
};
