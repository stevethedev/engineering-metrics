import { PageHeader, PageHeaderProps } from "./page-header";

export default {
  title: "Containers/PageHeader",
  component: PageHeader,
};

export const Default = (args: PageHeaderProps) => <PageHeader {...args} />;
Default.storyName = "PageHeader";
