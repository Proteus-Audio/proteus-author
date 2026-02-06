import fs from "node:fs";
import path from "node:path";

type BumpKind = "major" | "breaking" | "minor";

const ROOT = process.cwd();
const ARG = process.argv[2] as BumpKind | undefined;

if (ARG !== "major" && ARG !== "breaking" && ARG !== "minor") {
  console.error("Usage: bump-version <major|breaking|minor>");
  process.exit(1);
}

const bumpKind: BumpKind = ARG;

const bumpVersion = (current: string, kind: BumpKind): string => {
  const match = current.trim().match(/^(\d+)\.(\d+)\.(\d+)$/);
  if (!match) {
    throw new Error(`Invalid semver version: ${current}`);
  }
  const major = Number(match[1]);
  const minor = Number(match[2]);
  const patch = Number(match[3]);

  if (kind === "minor") {
    return `${major}.${minor + 1}.0`;
  }
  const newMajor = major + 1;
  return `${newMajor}.0.0`;
};

const updatePackageJson = (newVersion: string) => {
  const filePath = path.join(ROOT, "package.json");
  const content = fs.readFileSync(filePath, "utf8");
  const json = JSON.parse(content);
  json.version = newVersion;
  fs.writeFileSync(filePath, JSON.stringify(json, null, 2) + "\n");
};

const updateTauriConf = (newVersion: string) => {
  const filePath = path.join(ROOT, "src-tauri", "tauri.conf.json");
  const content = fs.readFileSync(filePath, "utf8");
  const json = JSON.parse(content);
  json.version = newVersion;
  fs.writeFileSync(filePath, JSON.stringify(json, null, 2) + "\n");
};

const updateCargoToml = (newVersion: string) => {
  const filePath = path.join(ROOT, "src-tauri", "Cargo.toml");
  const content = fs.readFileSync(filePath, "utf8");
  const lines = content.split(/\r?\n/);

  let inPackage = false;
  let updated = false;

  for (let i = 0; i < lines.length; i += 1) {
    const line = lines[i];
    const trimmed = line.trim();

    if (trimmed.startsWith("[") && trimmed.endsWith("]")) {
      inPackage = trimmed === "[package]";
    }

    if (inPackage && trimmed.startsWith("version")) {
      const match = line.match(/^(\s*version\s*=\s*)".*?"\s*$/);
      if (match) {
        lines[i] = `${match[1]}"${newVersion}"`;
        updated = true;
        break;
      }
    }
  }

  if (!updated) {
    throw new Error("Failed to find [package] version in Cargo.toml");
  }

  fs.writeFileSync(filePath, lines.join("\n"));
};

const packageJsonPath = path.join(ROOT, "package.json");
const pkg = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
const currentVersion = pkg.version as string;
const newVersion = bumpVersion(currentVersion, bumpKind);

updatePackageJson(newVersion);
updateTauriConf(newVersion);
updateCargoToml(newVersion);

console.log(`${currentVersion} -> ${newVersion}`);
