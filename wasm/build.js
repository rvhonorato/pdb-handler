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
  type: "module",
  files: getOutputFiles(),
  sideEffects: false,
};

fs.writeFileSync(
  path.join(PKG_DIR, "package.json"),
  JSON.stringify(packageJson, null, 2),
);

// Add TypeScript declarations
addTypeDeclarations();

console.log("✅ Build completed successfully!");
console.log("To publish to NPM:");
console.log(`   cd ${PKG_DIR}`);
console.log("   npm publish");

// --- Helper Functions ---

// Make sure the NPM package version matches the `wasm` version
function getPackageVersion() {
  try {
    const cargoToml = fs.readFileSync(
      path.join(__dirname, "Cargo.toml"),
      "utf8",
    );
    const versionMatch = cargoToml.match(/version\s*=\s*"([\d.]+)"/);
    return versionMatch ? versionMatch[1] : "0.1.0";
  } catch (e) {
    return "0.1.0";
  }
}

function getOutputFiles() {
  return fs
    .readdirSync(PKG_DIR)
    .filter(
      (file) =>
        file.endsWith(".wasm") ||
        file.endsWith(".js") ||
        file.endsWith(".d.ts"),
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

// TODO: Find a smarter way to add the type declarations
function addTypeDeclarations() {
  const dtsContent = `declare module "${PACKAGE_NAME}" {
  export function init(): Promise<void>;
  export class PdbHandlerApi {
    list_chains(data: Uint8Array): string[];
    list_unknown_residues(data: Uint8Array): Map<string, string[]>;
    guess_moltype(data: Uint8Array): Map<string, string[]>;
    list_residues(data: Uint8Array): Map<string, string[]>;
    chains_in_contact(data: Uint8Array): string[][];
  }
}`;

  const dtsPath = path.join(PKG_DIR, `${PACKAGE_NAME.replace(/-/g, "_")}.d.ts`);
  if (!fs.existsSync(dtsPath)) {
    fs.writeFileSync(dtsPath, dtsContent);
  }
}
