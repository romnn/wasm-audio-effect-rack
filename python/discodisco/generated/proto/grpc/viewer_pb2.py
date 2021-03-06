# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: proto/grpc/viewer.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from proto.audio.analysis import analysis_pb2 as proto_dot_audio_dot_analysis_dot_analysis__pb2
from proto.grpc import connection_pb2 as proto_dot_grpc_dot_connection__pb2
from proto.grpc import session_pb2 as proto_dot_grpc_dot_session__pb2
from proto.grpc import remote_pb2 as proto_dot_grpc_dot_remote__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='proto/grpc/viewer.proto',
  package='proto.grpc',
  syntax='proto3',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\x17proto/grpc/viewer.proto\x12\nproto.grpc\x1a#proto/audio/analysis/analysis.proto\x1a\x1bproto/grpc/connection.proto\x1a\x18proto/grpc/session.proto\x1a\x17proto/grpc/remote.proto\"\xbe\x01\n\x0cViewerUpdate\x12*\n\theartbeat\x18\x01 \x01(\x0b\x32\x15.proto.grpc.HeartbeatH\x00\x12,\n\nassignment\x18\x02 \x01(\x0b\x32\x16.proto.grpc.AssignmentH\x00\x12J\n\x15\x61udio_analysis_result\x18\x64 \x01(\x0b\x32).proto.audio.analysis.AudioAnalysisResultH\x00\x42\x08\n\x06update\"\x1b\n\x19UpdateSubscriptionRequest\"@\n\x14ViewerConnectRequest\x12(\n\x08instance\x18\x01 \x01(\x0b\x32\x16.proto.grpc.InstanceId\"\x19\n\x17ViewerDisconnectRequest2\xf3\x01\n\x0cRemoteViewer\x12I\n\x07\x43onnect\x12 .proto.grpc.ViewerConnectRequest\x1a\x18.proto.grpc.ViewerUpdate\"\x00\x30\x01\x12\x46\n\nDisconnect\x12#.proto.grpc.ViewerDisconnectRequest\x1a\x11.proto.grpc.Empty\"\x00\x12P\n\x12UpdateSubscription\x12%.proto.grpc.UpdateSubscriptionRequest\x1a\x11.proto.grpc.Empty\"\x00\x62\x06proto3'
  ,
  dependencies=[proto_dot_audio_dot_analysis_dot_analysis__pb2.DESCRIPTOR,proto_dot_grpc_dot_connection__pb2.DESCRIPTOR,proto_dot_grpc_dot_session__pb2.DESCRIPTOR,proto_dot_grpc_dot_remote__pb2.DESCRIPTOR,])




_VIEWERUPDATE = _descriptor.Descriptor(
  name='ViewerUpdate',
  full_name='proto.grpc.ViewerUpdate',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='heartbeat', full_name='proto.grpc.ViewerUpdate.heartbeat', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='assignment', full_name='proto.grpc.ViewerUpdate.assignment', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='audio_analysis_result', full_name='proto.grpc.ViewerUpdate.audio_analysis_result', index=2,
      number=100, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
    _descriptor.OneofDescriptor(
      name='update', full_name='proto.grpc.ViewerUpdate.update',
      index=0, containing_type=None,
      create_key=_descriptor._internal_create_key,
    fields=[]),
  ],
  serialized_start=157,
  serialized_end=347,
)


_UPDATESUBSCRIPTIONREQUEST = _descriptor.Descriptor(
  name='UpdateSubscriptionRequest',
  full_name='proto.grpc.UpdateSubscriptionRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=349,
  serialized_end=376,
)


_VIEWERCONNECTREQUEST = _descriptor.Descriptor(
  name='ViewerConnectRequest',
  full_name='proto.grpc.ViewerConnectRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='instance', full_name='proto.grpc.ViewerConnectRequest.instance', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=378,
  serialized_end=442,
)


_VIEWERDISCONNECTREQUEST = _descriptor.Descriptor(
  name='ViewerDisconnectRequest',
  full_name='proto.grpc.ViewerDisconnectRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=444,
  serialized_end=469,
)

_VIEWERUPDATE.fields_by_name['heartbeat'].message_type = proto_dot_grpc_dot_connection__pb2._HEARTBEAT
_VIEWERUPDATE.fields_by_name['assignment'].message_type = proto_dot_grpc_dot_session__pb2._ASSIGNMENT
_VIEWERUPDATE.fields_by_name['audio_analysis_result'].message_type = proto_dot_audio_dot_analysis_dot_analysis__pb2._AUDIOANALYSISRESULT
_VIEWERUPDATE.oneofs_by_name['update'].fields.append(
  _VIEWERUPDATE.fields_by_name['heartbeat'])
_VIEWERUPDATE.fields_by_name['heartbeat'].containing_oneof = _VIEWERUPDATE.oneofs_by_name['update']
_VIEWERUPDATE.oneofs_by_name['update'].fields.append(
  _VIEWERUPDATE.fields_by_name['assignment'])
_VIEWERUPDATE.fields_by_name['assignment'].containing_oneof = _VIEWERUPDATE.oneofs_by_name['update']
_VIEWERUPDATE.oneofs_by_name['update'].fields.append(
  _VIEWERUPDATE.fields_by_name['audio_analysis_result'])
_VIEWERUPDATE.fields_by_name['audio_analysis_result'].containing_oneof = _VIEWERUPDATE.oneofs_by_name['update']
_VIEWERCONNECTREQUEST.fields_by_name['instance'].message_type = proto_dot_grpc_dot_session__pb2._INSTANCEID
DESCRIPTOR.message_types_by_name['ViewerUpdate'] = _VIEWERUPDATE
DESCRIPTOR.message_types_by_name['UpdateSubscriptionRequest'] = _UPDATESUBSCRIPTIONREQUEST
DESCRIPTOR.message_types_by_name['ViewerConnectRequest'] = _VIEWERCONNECTREQUEST
DESCRIPTOR.message_types_by_name['ViewerDisconnectRequest'] = _VIEWERDISCONNECTREQUEST
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

ViewerUpdate = _reflection.GeneratedProtocolMessageType('ViewerUpdate', (_message.Message,), {
  'DESCRIPTOR' : _VIEWERUPDATE,
  '__module__' : 'proto.grpc.viewer_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.ViewerUpdate)
  })
_sym_db.RegisterMessage(ViewerUpdate)

UpdateSubscriptionRequest = _reflection.GeneratedProtocolMessageType('UpdateSubscriptionRequest', (_message.Message,), {
  'DESCRIPTOR' : _UPDATESUBSCRIPTIONREQUEST,
  '__module__' : 'proto.grpc.viewer_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.UpdateSubscriptionRequest)
  })
_sym_db.RegisterMessage(UpdateSubscriptionRequest)

ViewerConnectRequest = _reflection.GeneratedProtocolMessageType('ViewerConnectRequest', (_message.Message,), {
  'DESCRIPTOR' : _VIEWERCONNECTREQUEST,
  '__module__' : 'proto.grpc.viewer_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.ViewerConnectRequest)
  })
_sym_db.RegisterMessage(ViewerConnectRequest)

ViewerDisconnectRequest = _reflection.GeneratedProtocolMessageType('ViewerDisconnectRequest', (_message.Message,), {
  'DESCRIPTOR' : _VIEWERDISCONNECTREQUEST,
  '__module__' : 'proto.grpc.viewer_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.ViewerDisconnectRequest)
  })
_sym_db.RegisterMessage(ViewerDisconnectRequest)



_REMOTEVIEWER = _descriptor.ServiceDescriptor(
  name='RemoteViewer',
  full_name='proto.grpc.RemoteViewer',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_start=472,
  serialized_end=715,
  methods=[
  _descriptor.MethodDescriptor(
    name='Connect',
    full_name='proto.grpc.RemoteViewer.Connect',
    index=0,
    containing_service=None,
    input_type=_VIEWERCONNECTREQUEST,
    output_type=_VIEWERUPDATE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
  _descriptor.MethodDescriptor(
    name='Disconnect',
    full_name='proto.grpc.RemoteViewer.Disconnect',
    index=1,
    containing_service=None,
    input_type=_VIEWERDISCONNECTREQUEST,
    output_type=proto_dot_grpc_dot_remote__pb2._EMPTY,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
  _descriptor.MethodDescriptor(
    name='UpdateSubscription',
    full_name='proto.grpc.RemoteViewer.UpdateSubscription',
    index=2,
    containing_service=None,
    input_type=_UPDATESUBSCRIPTIONREQUEST,
    output_type=proto_dot_grpc_dot_remote__pb2._EMPTY,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
])
_sym_db.RegisterServiceDescriptor(_REMOTEVIEWER)

DESCRIPTOR.services_by_name['RemoteViewer'] = _REMOTEVIEWER

# @@protoc_insertion_point(module_scope)
