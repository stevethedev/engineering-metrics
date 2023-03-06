import { SharedLayout, SharedProps } from "./shared";

export default {
  title: "Layouts/Shared",
  component: SharedLayout,
};

export const Default = (args: SharedProps) => (
  <SharedLayout {...args}>This is an example component.</SharedLayout>
);
Default.storyName = "Shared";
