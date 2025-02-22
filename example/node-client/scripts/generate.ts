import * as posix from "jsr:@std/path/posix";
import * as win32 from "jsr:@std/path/windows";

const out_dir = posix.join("types");
const npm_bin_dir = posix.join("node_modules", ".bin");
const proto_dir = win32.join("proto", "fms", "fms.proto");
const proto_loader_gen_path = posix.join(
  npm_bin_dir,
  "proto-loader-gen-types.cmd"
);

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  try {
    await Deno.remove(out_dir, { recursive: true });
  } catch (err) {
    console.error(err);
  }
  await Deno.mkdir(out_dir);

  const command = new Deno.Command(proto_loader_gen_path, {
    args: [
      "--longs=String",
      "--enums=String",
      "--defaults",
      "--oneofs",
      "--grpcLib=@grpc/grpc-js",
      `--outDir=${out_dir}`,
      proto_dir,
    ],
  });

  const { code, stdout, stderr } = await command.output();

  console.log("code", code);
  console.log("stdout", new TextDecoder().decode(stdout));
  console.log("stderr", new TextDecoder().decode(stderr));
}
