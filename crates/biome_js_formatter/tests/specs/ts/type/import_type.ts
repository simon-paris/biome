type ImportType1 = typeof import('source');

type ImportType2 = import('source');

type QualifiedImportType = typeof import('source').Qualified<TypeParams>;

type WithTypeAssertion = import("a.json", { assert: { type: "json" } });

const ShouldNotHaveTrailingCommas = require("long/long/long/long/long/long/long/long/long/long/path.json") as typeof import("/long/long/long/long/long/long/long/long/long/long/path.json");
