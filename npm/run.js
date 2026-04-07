#!/usr/bin/env node
// Wrapper that spawns the asc-mcp binary with passthrough args.

const { execFileSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const BINARY = "asc-mcp";
const binaryPath = path.join(__dirname, BINARY);

if (!fs.existsSync(binaryPath)) {
  console.error(
    `asc-mcp: binary not found at ${binaryPath}\n` +
      `This usually means the postinstall download failed.\n` +
      `\nAlternatives:\n` +
      `  cargo install menot-you-mcp-asc    # if you have Rust\n` +
      `  curl -fsSL https://raw.githubusercontent.com/menot-you/apple-store-connect/main/install.sh | sh\n`
  );
  process.exit(1);
}

try {
  execFileSync(binaryPath, process.argv.slice(2), {
    stdio: "inherit",
    env: process.env,
  });
} catch (err) {
  process.exit(err.status || 1);
}
