const { execSync } = require("child_process");
const fs = require("fs");
const glob = require("glob");
const path = require("path");

const bu = require("./build_utils.cjs");

function deleteTargetDir() {
  const targetDir = bu.getSrcTauriPath() + "/target";
  if (fs.existsSync(targetDir)) {
    fs.rmSync(targetDir, {
      recursive: true,
      force: true
    });
  }
}

function copyDylibs() {
  let dylibDirPrefix = "lib";

  if (process.platform === "win32") {
    dylibDirPrefix = "bin";
  }

  const dylibs = glob.sync(
    bu.getSrcTauriPath() +
      "/target/**/release/build/trajoptlib-*/out/" +
      dylibDirPrefix +
      "/" +
      bu.getDylibPattern()
  );

  dylibs.forEach((dylib) => {
    fs.copyFileSync(dylib, bu.getSrcTauriPath() + "/" + path.basename(dylib));
  });
}

deleteTargetDir();

execSync("cd src-tauri && cargo build --release");

copyDylibs();

fs.rmSync(bu.getDummyResourcePath());

if (process.platform === "win32") {
  fs.copyFileSync(
    bu.getSrcTauriPath() + "/target/release/WebView2Loader.dll",
    bu.getSrcTauriPath() + "/WebView2Loader.dll"
  );
}

deleteTargetDir();
