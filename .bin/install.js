const { run, CARGO_EXECUTABLE, NPM_EXECUTABLE } = require("./lib/exec");
const {
  JSON_SCHEMA_DIR,
  SERVER_DIR,
  CLIENT_DIR,
  TEST_DIR,
} = require("./lib/dir");

const installJsonSchema = async () =>
  run(NPM_EXECUTABLE, ["ci"], JSON_SCHEMA_DIR);

const runJsonSchema = async () =>
  run(NPM_EXECUTABLE, ["run", "start"], JSON_SCHEMA_DIR);

const installClient = async () => run(NPM_EXECUTABLE, ["ci"], CLIENT_DIR);

const installServer = async () => run(CARGO_EXECUTABLE, ["build"], SERVER_DIR);

const installTests = async () => run(NPM_EXECUTABLE, ["ci"], TEST_DIR);

const install = async ({
  isServer = true,
  isClient = true,
  isJsonSchema = true,
  isTests = true,
}) => {
  const isJsonSchemaPassed =
    !isJsonSchema || ((await installJsonSchema()) && (await runJsonSchema()));
  const isServerPassed = !isServer || (await installServer());
  const isClientPassed = !isClient || (await installClient());
  const isTestsPassed = !isTests || (await installTests());

  return (
    isServerPassed && isClientPassed && isJsonSchemaPassed && isTestsPassed
  );
};

const main = async () => {
  const isHelp = process.argv.includes("--help");

  if (isHelp) {
    process.stdout.write("Usage:\n");
    process.stdout.write("  install [--all]\n");
    process.stdout.write(
      "  install [--server] [--client] [--json-schema] [--tests]\n"
    );
    return;
  }

  const isAll = process.argv.includes("--all");
  const isServer = isAll || process.argv.includes("--server");
  const isClient = isAll || process.argv.includes("--client");
  const isJsonSchema = isAll || process.argv.includes("--json-schema");
  const isTests = isAll || process.argv.includes("--tests");

  const passed = await install({ isServer, isClient, isJsonSchema, isTests });
  if (!passed) {
    process.stderr.write("Install failed\n");
    process.exit(1);
  }

  process.stdout.write("Install complete\n");
};

module.exports = main;

if (require.main === module) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}
