import {
  DefaultAside,
  DefaultFooter,
  DefaultHeader,
  DefaultLayout,
  DefaultMain,
  DefaultNav,
  Props,
} from "./default";
import React from "react";

export default {
  title: "Layouts/Default",
  component: DefaultLayout,
};

export const Default = (args: Props) => (
  <DefaultLayout {...args}>
    <DefaultHeader style={{ outline: "1px solid" }}>Header</DefaultHeader>
    <DefaultNav style={{ outline: "1px solid" }}>Nav</DefaultNav>
    <DefaultMain style={{ outline: "1px solid" }}>Body</DefaultMain>
    <DefaultAside style={{ outline: "1px solid" }}>Aside</DefaultAside>
    <DefaultFooter as={"b"} style={{ outline: "1px solid" }}>
      Footer
    </DefaultFooter>
  </DefaultLayout>
);
