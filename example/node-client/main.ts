import grpc, { ServiceClientConstructor } from "@grpc/grpc-js";
import proto_loader from "@grpc/proto-loader";
import { ProtoGrpcType } from "./types/fms.ts";

const SERVER_ADDRESS = "localhost:50002";

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  const client = new grpc.Client(
    SERVER_ADDRESS,
    grpc.credentials.createInsecure()
  );
  const package_definition = proto_loader.loadSync("proto/fms/fms.proto");
  const proto = grpc.loadPackageDefinition(package_definition) as ProtoGrpcType;
  console.log(proto);
  console.log(proto.fms.gps.v1.GpsService);
}
