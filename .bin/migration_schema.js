const path = require("path");
const { SEA_ORM_CLI, run } = require("./lib/exec");

const main = async () => {
  const isHelp = process.argv.includes("--help");
  const databaseUrl =
    process.argv[2] ?? "postgres://postgres:postgres@database/postgres";

  if (isHelp || !databaseUrl) {
    process.stdout.write(
      "Usage: migration_schema [database_url = postgres://postgres:postgres@localhost:5432/postgres]"
    );
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
    "generate",
    "entity",
    "--database-url",
    databaseUrl,
    "--migration-dir",
    migrationDir,
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
