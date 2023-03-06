import { mash } from "../../../utils/class-mash";
import { Hstack } from "../../blocks/hstack";
import { Box } from "../../blocks/box";
import { Text } from "../../blocks/text";

export type PageHeaderProps = Record<string, never>;

export const PageHeader = mash<PageHeaderProps>(() => {
  return (
    <Hstack>
      <Box flex={0}>
        <Text>[LOGO]</Text>
      </Box>
      <Box flex={1}>
        <Text>[LINKS]</Text>
      </Box>
      <Box flex={0}>
        <Text>[USER]</Text>
      </Box>
    </Hstack>
  );
});
