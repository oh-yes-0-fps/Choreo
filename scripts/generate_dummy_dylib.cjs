const fs = require("fs");

const bu = require("./build_utils.cjs");

fs.writeFileSync(
  bu.getDummyResourcePath(),
  'dummy file (see "scripts/build_utils.cjs")'
);
