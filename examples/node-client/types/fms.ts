import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';

import type { GPSClient as _fms_gps_v1_GPSClient, GPSDefinition as _fms_gps_v1_GPSDefinition } from './fms/gps/v1/GPS.ts';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  fms: {
    gps: {
      v1: {
        GPS: SubtypeConstructor<typeof grpc.Client, _fms_gps_v1_GPSClient> & { service: _fms_gps_v1_GPSDefinition }
        LatLngRequest: MessageTypeDefinition
        LatLngResponse: MessageTypeDefinition
      }
    }
  }
}

