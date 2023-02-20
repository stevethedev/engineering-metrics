const path = require("path");
const fs = require("fs/promises");
const { existsSync, writeFileSync, createWriteStream } = require("fs");
const { spawnSync } = require("child_process");
const os = require("os");
const axios = require("axios");
const decompress = require("decompress");

const INPUT_JTD_DIR = process.env.INPUT_JTD_DIR || path.join(__dirname, "src");
const OUTPUT_RS_DIR =
  process.env.OUTPUT_RS_DIR ||
  path.join(
    __dirname,
    "..",
    "server",
    "workspace",
    "lib-json-schema",
    "src",
    "generated"
  );
const OUTPUT_TS_DIR =
  process.env.OUTPUT_TS_DIR ||
  path.join(__dirname, "..", "client", "src", "generated");
const JTD_EXTENSION = process.env.JTD_EXTENSION || ".jtd.json";
const JTD_EXECUTOR_PATH =
  process.env.JTD_EXECUTOR_PATH || path.join(__dirname, ".bin");

const getJtdExecutor = () => {
  const ver = spawnSync("jtd-codegen", ["--version"], { shell: false })
  if (!ver.error) {
    return "jtd-codegen";
  }
  return `${path.join(JTD_EXECUTOR_PATH, "jtd-codegen")}${os.platform() === "win32" ? ".exe" : ""}`;
}

const JTD_EXECUTOR = getJtdExecutor();

const setup = async () => {
  await Promise.all([setupPaths(), setupJtd()]);
};

const getJtdCodegenDownloadPath = () => {
  const platform = os.platform();

  const version = "0.4.1";
  const getUrl = (target) => `https://github.com/jsontypedef/json-typedef-codegen/releases/download/v${version}/${target}.zip`;

  if (platform === "win32") {
    return getUrl("x86_64-pc-windows-gnu");
  }

  if (platform === "linux") {
    // check if GNU or musl or other
    const lddResult = spawnSync("ldd", ["/usr/bin/env"], { shell: false });
    const isMusl = !lddResult.error && lddResult.stdout.toString().includes("musl");
    return getUrl(isMusl ? "x86_64-unknown-linux-musl" : "x86_64-unknown-linux-gnu");
  }

  if (platform === "darwin") {
    return getUrl("x86_64-apple-darwin");
  }

  throw new Error(`Unsupported platform: ${platform}`);
}

const downloadJtdCodegen = async (downloadDir) => {
  const downloadPath = getJtdCodegenDownloadPath();

  // Download the `downloadPath` to `jtd-codegen.zip`
  const response = await axios.get(downloadPath, { responseType: "stream" });
  const writer = createWriteStream(path.join(downloadDir, "jtd-codegen.zip"));
  response.data.pipe(writer);

  await new Promise((resolve, reject) => {
    writer.on("finish", resolve);
    writer.on("error", reject);
  });
}

const unzipJtdCodegen = async (downloadDir, unzipDir) => {
  await decompress(path.join(downloadDir, "jtd-codegen.zip"), unzipDir);
}

const setupJtd = async () => {
  if (!spawnSync("jtd-codegen", ["--version"], { shell: false }).error) {
    process.stdout.write("jtd-codegen already installed.\n");
    return;
  }

  if (!existsSync(JTD_EXECUTOR_PATH)) {
    await fs.mkdir(JTD_EXECUTOR_PATH, { recursive: true });
  }

  if (!spawnSync(JTD_EXECUTOR, ["--version"], { shell: false }).error) {
    process.stdout.write("jtd-codegen already installed.\n");
    return;
  }

  process.stdout.write("jtd-codegen not found. Installing...\n");
  const downloadDir = await fs.mkdtemp(os.tmpdir());
  await downloadJtdCodegen(downloadDir);
  await unzipJtdCodegen(downloadDir, JTD_EXECUTOR_PATH);
  console.log(spawnSync(JTD_EXECUTOR, ["--version"], { shell: false }))
  const version = spawnSync(JTD_EXECUTOR, ["--version"], { shell: false }).stdout.toString().trim();
  process.stdout.write(`jtd-codegen installed. Version: ${version}\n`);
};

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
};

const listFiles = async (dir) => {
  // Recursively list all files in INPUT_JTD_DIR
  const files = await fs.readdir(dir, { withFileTypes: true });
  const files_list = await Promise.all(
    files.map(async (file) => {
      const res = path.join(dir, file.name);
      return file.isDirectory() ? listFiles(res) : res;
    })
  );

  // Filter out files that don't end with JTD_EXTENSION
  return files_list.flat().filter((file) => file.endsWith(JTD_EXTENSION));
};

const rsModCache = {};

const updateRsMods = (dir) => {
  let first = true;
  while (dir !== OUTPUT_RS_DIR) {
    const modName = path.basename(dir);
    dir = path.dirname(dir);
    const targetFile = path.join(dir, "mod.rs");
    if (!existsSync(targetFile)) {
      writeFileSync(targetFile, "// Generated code. Do not edit.\n");
    }

    const exportContents = `mod ${modName};\npub use ${modName}::*;\n`;
    const exportModule = `pub mod ${modName};\n`;

    const exportLine = first ? exportContents : exportModule;

    rsModCache[targetFile] = rsModCache[targetFile] || [];
    if (rsModCache[targetFile].includes(exportLine)) {
      return;
    }

    rsModCache[targetFile].push(exportLine);
    writeFileSync(targetFile, exportLine, {
      flag: "a",
    });
    first = false;
  }
};

const toCamelCase = (str) => {
  return str.replace(/-([a-z])/g, (g) => g[1].toUpperCase());
};

const tsModCache = {};

const updateTsMods = (dir) => {
  let first = true;
  while (dir !== OUTPUT_TS_DIR) {
    const modName = path.basename(dir);
    dir = path.dirname(dir);
    const targetFile = path.join(dir, "index.ts");

    if (!existsSync(targetFile)) {
      writeFileSync(targetFile, "// Generated code. Do not edit.\n");
    }
    const exportedAs = first ? "" : `as ${toCamelCase(modName)}`;
    const exportLine = `export * ${exportedAs} from "./${modName}";`;

    tsModCache[targetFile] = tsModCache[targetFile] || [];
    if (tsModCache[targetFile].includes(exportLine)) {
      return;
    }

    tsModCache[targetFile].push(exportLine);
    writeFileSync(targetFile, `${exportLine}\n`, { flag: "a" });
    first = false;
  }
};

const processJtd = async (file) => {
  process.stdout.write(`Processing ${file}...\n`);

  const parentName = path.dirname(path.relative(INPUT_JTD_DIR, file));
  const moduleName = path.basename(file, JTD_EXTENSION);

  const rsDir = path.join(
    OUTPUT_RS_DIR,
    parentName.replaceAll("-", "_"),
    moduleName.replaceAll("-", "_")
  );
  const tsDir = path.join(OUTPUT_TS_DIR, parentName, moduleName);

  await Promise.all([
    fs.mkdir(rsDir, { recursive: true }),
    fs.mkdir(tsDir, { recursive: true }),
  ]);

  const { stdout, stderr } = spawnSync(
    JTD_EXECUTOR,
    ["--rust-out", rsDir, "--typescript-out", tsDir, file],
    { shell: false }
  );

  const isRsSuccess = existsSync(path.join(rsDir, "mod.rs"));
  const isTsSuccess = existsSync(path.join(tsDir, "index.ts"));

  if (!isRsSuccess || !isTsSuccess) {
    if (!isRsSuccess) {
      process.stdout.write(`Failed to generate Rust code for ${file}.\n`);
    }
    if (!isTsSuccess) {
      process.stdout.write(`Failed to generate TypeScript code for ${file}.\n`);
    }

    stdout && process.stdout.write(`stdout: ${stdout}\n`);
    stderr && process.stderr.write(`stderr: ${stderr}\n`);
    throw new Error(`Failed to generate code for ${file}`);
  }

  updateRsMods(rsDir);
  updateTsMods(tsDir);
};

const main = async () => {
  await setup();
  const files = await listFiles(INPUT_JTD_DIR);

  process.stdout.write(`Found ${files.length} files.\n`);
  process.stdout.write(`Using jtd-codegen version ${spawnSync(JTD_EXECUTOR, ["--version"], { shell: false }).stdout.toString().trim()}.\n`);

  await Promise.all(files.map(processJtd));
};

main().then(() => {
  process.stdout.write("Done.\n");
});
