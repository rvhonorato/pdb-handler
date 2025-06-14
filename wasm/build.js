const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");

// Configuration
const PKG_DIR = path.join(__dirname, "pkg");
const PACKAGE_NAME = "pdb-handler-wasm";

// Build WASM package
console.log("🚀 Building WASM package...");
execSync(`wasm-pack build --target web`, { stdio: "inherit" });

// Create package.json
console.log("📦 Creating package.json...");
const packageJson = {
  name: PACKAGE_NAME,
  version: getPackageVersion(),
  module: `${PACKAGE_NAME.replace(/-/g, "_")}.js`,
  types: `${PACKAGE_NAME.replace(/-/g, "_")}.d.ts`,
  type: "module",
  files: getOutputFiles(),
  sideEffects: false,
};

fs.writeFileSync(
  path.join(PKG_DIR, "package.json"),
  JSON.stringify(packageJson, null, 2),
);

console.log("✅ Build completed successfully!");
console.log("To publish to NPM:");
console.log(`   cd ${PKG_DIR}`);
console.log("   npm publish");

// --- Helper Functions ---

// Make sure the NPM package version matches the `wasm` version
function getPackageVersion() {
  const cargoTomlPath = path.join(__dirname, "Cargo.toml");

  if (!fs.existsSync(cargoTomlPath)) {
    throw new Error(`Cargo.toml not found at ${cargoTomlPath}`);
  }

  let cargoToml;
  try {
    cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
  } catch (error) {
    throw new Error(`Failed to read Cargo.toml: ${error.message}`);
  }

  const versionMatch = cargoToml.match(/^\s*version\s*=\s*"([^"]+)"\s*$/m);

  if (!versionMatch) {
    throw new Error("Version field not found in Cargo.toml");
  }

  const version = versionMatch[1];

  // Validate that it's a proper semantic version
  if (
    !/^\d+\.\d+\.\d+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$/.test(
      version,
    )
  ) {
    throw new Error(`Invalid version format in Cargo.toml: "${version}"`);
  }

  return version;
}

function getOutputFiles() {
  return fs
    .readdirSync(PKG_DIR)
    .filter(
      (file) =>
        file.endsWith(".wasm") ||
        file.endsWith(".js") ||
        file.endsWith(".d.ts") ||
        file == "README.md",
    )
    .map((file) => {
      // Rename files to use package name
      if (file.startsWith("pdb_handler_wasm")) {
        const newName = file.replace(
          "pdb_handler_wasm",
          PACKAGE_NAME.replace(/-/g, "_"),
        );
        fs.renameSync(path.join(PKG_DIR, file), path.join(PKG_DIR, newName));
        return newName;
      }
      return file;
    });
}
