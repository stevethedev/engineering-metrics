import { Vstack, VstackProps } from "./vstack";

export default {
  title: "Blocks/Vstack",
  component: Vstack,
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

export const Default = (args: VstackProps) => (
  <Vstack {...args}>
    <div style={{ outline: "1px solid" }}>1</div>
    <div style={{ outline: "1px solid" }}>2</div>
    <div style={{ outline: "1px solid" }}>3</div>
  </Vstack>
);
Default.storyName = "Vstack";
Default.args = {
  flex: 1,
  gap: "var(--sp--1)",
};
