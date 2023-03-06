import { Grid, GridProps } from "./grid";

export default {
  title: "Blocks/Grid",
  component: Grid,
  argTypes: {
    rows: {
      control: {
        type: "text",
      },
    },
    // cols: {
    //   control: {
    //     type: "text",
    //   },
    // },
    gap: {
      control: {
        type: "text",
      },
    },
  },
};

export const Default = (args: GridProps) => {
  return (
    <Grid {...args}>
      {Array.from({ length: 12 }, (_, i) => (
        <div key={i} style={{ border: "1px solid black" }}>
          {i}
        </div>
      ))}
    </Grid>
  );
};
Default.storyName = "Grid";
Default.args = {
  rows: "auto",
  cols: {
    default: "1fr",
    tablet: "1fr 1fr",
    desktop: "1fr 1fr 1fr",
  },
  gap: "auto",
};
