const { NPM_EXECUTABLE, run, CARGO_EXECUTABLE } = require("./lib/exec");
const { CLIENT_DIR, SERVER_DIR } = require("./lib/dir");

const lintClient = async () => {
  const isLinted = await run(NPM_EXECUTABLE, ["run", "lint"], CLIENT_DIR);
  const isTypeChecked = await run(
    NPM_EXECUTABLE,
    ["run", "type-check"],
    CLIENT_DIR
  );

  return isLinted && isTypeChecked;
};

const lintServer = async () => {
  const isCheck = await run(CARGO_EXECUTABLE, ["check"], SERVER_DIR);
  const isClippy = await run(CARGO_EXECUTABLE, ["clippy"], SERVER_DIR);

  return isCheck && isClippy;
};

const lint = async ({ isServer, isClient } = {}) => {
  const isServerLinted = isServer ? await lintServer() : true;
  const isClientLinted = isClient ? await lintClient() : true;

  return isServerLinted && isClientLinted;
};

const main = async () => {
  const isHelp = process.argv.includes("--help");

  if (isHelp) {
    process.stdout.write("Usage: check [--server] [--client]\n");
    return;
  }

  const isAll = process.argv.includes("--all");
  const isServer = isAll || process.argv.includes("--server");
  const isClient = isAll || process.argv.includes("--client");

  const isLinted = await lint({ isServer, isClient });

  if (!isLinted) {
    process.stdout.write("Linting failed\n");
    process.exit(1);
  }

  process.stdout.write("Linting passed\n");
};

module.exports = lint;

if (require.main === module) {
  main().catch((error) => {
    process.stdout.write(error.message);
    process.exit(1);
  });
}
