const path = require("path");
const fs = require("fs/promises");
const { existsSync, writeFileSync } = require("fs");
const { spawnSync } = require("child_process");

const INPUT_JTD_DIR = process.env.INPUT_JTD_DIR || path.join(__dirname, "src");
const OUTPUT_RS_DIR = process.env.OUTPUT_RS_DIR || path.join(__dirname, "..", "server", "workspace", "lib-json-schema", "src", "generated");
const OUTPUT_TS_DIR = process.env.OUTPUT_TS_DIR || path.join(__dirname, "..", "client", "src", "generated");
const JTD_EXTENSION = process.env.JTD_EXTENSION || ".jtd.json";
const JTD_EXECUTOR_PATH = process.env.JTD_EXECUTOR_PATH || path.join(__dirname, ".bin");
const JTD_EXECUTOR = path.join(JTD_EXECUTOR_PATH, "bin", "jtd-codegen");

const setup = async () => {
  await Promise.all([
    setupPaths(),
    setupJtd(),
  ])
}

const setupJtd = async () => {
  if (!spawnSync("jtd-codegen", ["--version"], { shell: false }).error) {
    process.stdout.write("jtd-codegen already installed.\n");
    return;
  }

  if (!existsSync(JTD_EXECUTOR_PATH)) {
    await fs.mkdir(JTD_EXECUTOR_PATH, { recursive: true });
  }

  if (spawnSync("cargo", ["--version"], { shell: false }).error) {
    process.stdout.write("Cargo not found. Please install it.\n");
    return;
  }

  if (!spawnSync(JTD_EXECUTOR, ["--version"], { shell: false }).error) {
    process.stdout.write("jtd-codegen already installed.\n");
    return;
  }

  process.stdout.write("jtd-codegen not found. Installing...\n");
  spawnSync("cargo", [
    "install",
    "--root", JTD_EXECUTOR_PATH,
    "jtd-codegen"
  ], { shell: false });
}

const setupPaths = async () => {
  process.stdout.write("Setting up paths...\n");

  await Promise.all([
    fs.rm(OUTPUT_RS_DIR, { recursive: true, force: true }),
    fs.rm(OUTPUT_TS_DIR, { recursive: true, force: true }),
  ]);

  await Promise.all([
    fs.mkdir(OUTPUT_RS_DIR, { recursive: true }),
    fs.mkdir(OUTPUT_TS_DIR, { recursive: true }),
  ]);

  process.stdout.write("Paths set up.\n");
}

const listFiles = async (dir) => {
  // Recursively list all files in INPUT_JTD_DIR
  const files = await fs.readdir(dir, { withFileTypes: true });
  const files_list = await Promise.all(files.map(async (file) => {
    const res = path.join(dir, file.name);
    return file.isDirectory() ? listFiles(res) : res;
  }));

  // Filter out files that don't end with JTD_EXTENSION
  return files_list.flat()
    .filter((file) => file.endsWith(JTD_EXTENSION));
}

const updateRsMods = (dir) => {
  while (dir !== OUTPUT_RS_DIR) {
    const modName = path.basename(dir);
    dir = path.dirname(dir);
    if (!existsSync(path.join(dir, "mod.rs"))) {
      writeFileSync(path.join(dir, "mod.rs"), "// Generated code. Do not edit.\n");
    }
    writeFileSync(path.join(dir, "mod.rs"), `pub mod ${modName};\n`, { flag: "a" });
  }
}

const toCamelCase = (str) => {
  return str.replace(/-([a-z])/g, (g) => g[1].toUpperCase());
}

const tsModCache = {};

const updateTsMods = (dir) => {
  let first = true;
  while (dir !== OUTPUT_TS_DIR) {
    const modName = path.basename(dir);
    dir = path.dirname(dir);
    const targetFile = path.join(dir, "index.ts")

    if (!existsSync(targetFile)) {
      writeFileSync(targetFile, "// Generated code. Do not edit.\n");
    }
    const exportedAs = first ? '' : `as ${toCamelCase(modName)}`;
    const exportLine = `export * ${exportedAs} from "./${modName}";`;

    tsModCache[targetFile] = tsModCache[targetFile] || [];
    if (tsModCache[targetFile].includes(exportLine)) {
      return;
    }

    tsModCache[targetFile].push(exportLine);
    writeFileSync(targetFile, `${exportLine}\n`, { flag: "a" });
    first = false;
  }
}

const processJtd = async (file) => {
  process.stdout.write(`Processing ${file}...\n`);

  const parentName = path.dirname(path.relative(INPUT_JTD_DIR, file));
  const moduleName = path.basename(file, JTD_EXTENSION);

  const rsDir = path.join(OUTPUT_RS_DIR, parentName.replaceAll('-', '_'), moduleName.replaceAll('-', '_'));
  const tsDir = path.join(OUTPUT_TS_DIR, parentName, moduleName);

  await Promise.all([
    fs.mkdir(rsDir, { recursive: true }),
    fs.mkdir(tsDir, { recursive: true }),
  ]);

  spawnSync(JTD_EXECUTOR, [
    "--rust-out", rsDir,
    "--typescript-out", tsDir,
    "--", file,
  ], { shell: false });

  updateRsMods(rsDir);
  updateTsMods(tsDir);
}

const main = async () => {
  await setup();
  const files = await listFiles(INPUT_JTD_DIR);
  await Promise.all(files.map(processJtd));
}

main().then(() => {
  process.stdout.write("Done.\n");
});
