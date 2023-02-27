const path = require("path");
const { SEA_ORM_CLI, run } = require("./lib/exec");

const main = async () => {
  const isHelp = process.argv.includes("--help");
  const name = process.argv[2];

  if (isHelp || !name) {
    process.stdout.write("Usage: migration_add <name>");
    return;
  }

  const migrationDir = path.resolve(
    __dirname,
    "..",
    "server",
    "workspace",
    "lib-database-migration"
  );

  const isAdded = await run(SEA_ORM_CLI, [
    "migrate",
    "generate",
    "--migration-dir",
    migrationDir,
    name,
  ]);
  if (!isAdded) {
    process.stdout.write("Migration generation failed\n");
    process.exit(1);
  }

  process.stdout.write("Migration generation complete\n");
};

module.exports = main;

if (require.main === module) {
  main().catch((error) => {
    process.stdout.write(error.message);
    process.exit(1);
  });
}
