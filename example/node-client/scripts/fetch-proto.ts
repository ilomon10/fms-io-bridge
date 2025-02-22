import {loadPackageDefinition} from "@grpc/grpc-js";
import proto_loader from "@grpc/proto-loader";
import protobuf from "protobufjs";

if(import.meta.main) {
  proto_loader.loadSync("grpc.reflection.v1alpha.ServerReflection");
  const grpc = loadPackageDefinition();
  
}