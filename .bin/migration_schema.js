const path = require("path");
const fs = require("fs");
const { SEA_ORM_CLI, run } = require("./lib/exec");

const DEFAULT_DATABASE_URL =
  process.env.DATABASE_URL ??
  "postgres://postgres:postgres@localhost:5432/postgres";

const main = async () => {
  const isHelp = process.argv.includes("--help");
  const databaseUrl = process.argv[2] ?? DEFAULT_DATABASE_URL;

  if (isHelp || !databaseUrl) {
    process.stdout.write(
      `Usage: migration_schema [database_url = ${DEFAULT_DATABASE_URL}]`
    );
    return;
  }

  const migrationDir = path.resolve(
    __dirname,
    "..",
    "server",
    "workspace",
    "lib-database",
    "src",
    "entities"
  );

  if (!fs.existsSync(migrationDir)) {
    fs.mkdirSync(migrationDir, { recursive: true });
  }

  if (!fs.existsSync(path.resolve(migrationDir, "mod.rs"))) {
    fs.writeFileSync(path.resolve(migrationDir, "mod.rs"), "mod mod;");
  }

  const isAdded = await run(SEA_ORM_CLI, [
    "generate",
    "entity",
    "--database-url",
    databaseUrl,
    "-o",
    migrationDir,
  ]);
  if (!isAdded) {
    process.stdout.write("Migration generation failed\n");
    process.exit(1);
  }

  const modData = fs.readFileSync(path.resolve(migrationDir, "mod.rs"), "utf8");
  fs.writeFileSync(
    path.resolve(migrationDir, "mod.rs"),
    modData.replace(/^mod mod;$/gm, "")
  );

  process.stdout.write("Migration generation complete\n");
};

module.exports = main;

if (require.main === module) {
  main().catch((error) => {
    process.stdout.write(error.message);
    process.exit(1);
  });
}
