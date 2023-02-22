const { NPM_EXECUTABLE, CARGO_EXECUTABLE, run } = require("./lib/exec");
const { SERVER_DIR, CLIENT_DIR } = require("./lib/dir");

const testClient = async () => run(NPM_EXECUTABLE, ["run", "test"], CLIENT_DIR);

const testServer = async () => run(CARGO_EXECUTABLE, ["test"], SERVER_DIR);

const test = async ({ isClient = true, isServer = true } = {}) => {
  const isClientPassed = !isClient || (await testClient());
  const isServerPassed = !isServer || (await testServer());
  return isClientPassed && isServerPassed;
};

const main = async () => {
  const isHelp = process.argv.includes("--help");

  if (isHelp) {
    process.stdout.write("Usage:\n");
    process.stdout.write("  test [--all]\n");
    process.stdout.write("  test [--client] [--server]\n");
    return;
  }

  const isAll = process.argv.includes("--all");
  const isClient = isAll || process.argv.includes("--client");
  const isServer = isAll || process.argv.includes("--server");

  const passed = await test({ isClient, isServer });
  if (!passed) {
    process.stderr.write("Tests failed\n");
    process.exit(1);
  }

  process.stdout.write("Tests passed\n");
};

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
