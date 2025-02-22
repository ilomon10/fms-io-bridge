// Original file: proto/fms/gps/v1/gps.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { LatLngRequest as _fms_gps_v1_LatLngRequest, LatLngRequest__Output as _fms_gps_v1_LatLngRequest__Output } from '../../../fms/gps/v1/LatLngRequest.ts';
import type { LatLngResponse as _fms_gps_v1_LatLngResponse, LatLngResponse__Output as _fms_gps_v1_LatLngResponse__Output } from '../../../fms/gps/v1/LatLngResponse.ts';

export interface GPSClient extends grpc.Client {
  GetLatLng(argument: _fms_gps_v1_LatLngRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_fms_gps_v1_LatLngResponse__Output>): grpc.ClientUnaryCall;
  GetLatLng(argument: _fms_gps_v1_LatLngRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_fms_gps_v1_LatLngResponse__Output>): grpc.ClientUnaryCall;
  GetLatLng(argument: _fms_gps_v1_LatLngRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_fms_gps_v1_LatLngResponse__Output>): grpc.ClientUnaryCall;
  GetLatLng(argument: _fms_gps_v1_LatLngRequest, callback: grpc.requestCallback<_fms_gps_v1_LatLngResponse__Output>): grpc.ClientUnaryCall;
  getLatLng(argument: _fms_gps_v1_LatLngRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_fms_gps_v1_LatLngResponse__Output>): grpc.ClientUnaryCall;
  getLatLng(argument: _fms_gps_v1_LatLngRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_fms_gps_v1_LatLngResponse__Output>): grpc.ClientUnaryCall;
  getLatLng(argument: _fms_gps_v1_LatLngRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_fms_gps_v1_LatLngResponse__Output>): grpc.ClientUnaryCall;
  getLatLng(argument: _fms_gps_v1_LatLngRequest, callback: grpc.requestCallback<_fms_gps_v1_LatLngResponse__Output>): grpc.ClientUnaryCall;
  
}

export interface GPSHandlers extends grpc.UntypedServiceImplementation {
  GetLatLng: grpc.handleUnaryCall<_fms_gps_v1_LatLngRequest__Output, _fms_gps_v1_LatLngResponse>;
  
}

export interface GPSDefinition extends grpc.ServiceDefinition {
  GetLatLng: MethodDefinition<_fms_gps_v1_LatLngRequest, _fms_gps_v1_LatLngResponse, _fms_gps_v1_LatLngRequest__Output, _fms_gps_v1_LatLngResponse__Output>
}
