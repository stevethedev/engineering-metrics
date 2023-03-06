import { addDecorator } from "@storybook/react";

export const parameters = {
  actions: { argTypesRegex: "^on[A-Z].*" },
  layout: "fullscreen",
  controls: {
    matchers: {
      color: /(background|color)$/i,
      date: /Date$/,
      boolean: /^(is|has|should)/i,
    },
  },
};

addDecorator((Story) => {
  return (
    <div
      style={{
        boxSizing: "border-box",
        minHeight: "100vh",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <Story />
    </div>
  );
});
