import * as path from "jsr:@std/path";
import { diffStr } from "jsr:@std/internal";
import * as color from "jsr:@std/fmt/colors";
import { walk } from "jsr:@std/fs/walk";

const absolutePathToDir = path.join("types");

if (import.meta.main) {
  console.log("Running proto-codemod.ts");

  for await (const filename of walk(absolutePathToDir, { exts: ["ts"] })) {
    const extname = path.extname(filename.name);
    if (extname !== ".ts") {
      continue;
    }
    const decoder = new TextDecoder("utf-8");
    const file_path = filename.path;
    const file = decoder.decode(Deno.readFileSync(file_path));

    const modifiedFile = file.replaceAll(
      /( from\s+["'](\.{1,2}\/[^"']*?))(?<!\.ts)(["'])/g,
      "$1.ts$3"
    );

    console.log("Working on file", file_path);

    const changes = diffStr(file, modifiedFile);
    // console.log("modify file", changes);

    changes.forEach((part) => {
      if (part.type === "added") {
        console.log(
          color.brightGreen(`+ ${part.value.replaceAll(/\n$/g, "")}`)
        ); // Green for added lines
      } else if (part.type === "removed") {
        console.log(color.brightRed(`- ${part.value.replaceAll(/\n$/g, "")}`)); // Red for removed lines
      } else {
        // console.log(color.gray(`  ${part.value.replaceAll(/\n$/g, "")}`)); // Unchanged lines in gray
      }
    });

    Deno.writeTextFileSync(file_path, modifiedFile);
  }
}
