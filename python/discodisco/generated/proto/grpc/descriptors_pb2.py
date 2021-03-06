# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: proto/grpc/descriptors.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='proto/grpc/descriptors.proto',
  package='proto.grpc',
  syntax='proto3',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\x1cproto/grpc/descriptors.proto\x12\nproto.grpc\"S\n\x14\x41udioInputDescriptor\x12\x0f\n\x07\x62\x61\x63kend\x18\x01 \x01(\t\x12\x0e\n\x06\x64\x65vice\x18\x02 \x01(\t\x12\x0c\n\x04host\x18\x03 \x01(\t\x12\x0c\n\x04\x66ile\x18\x04 \x01(\t\"X\n\x17\x41udioAnalyzerDescriptor\x12\x0c\n\x04name\x18\x01 \x01(\t\x12/\n\x05input\x18\n \x01(\x0b\x32 .proto.grpc.AudioInputDescriptor\"w\n\x15\x41udioOutputDescriptor\x12\x0f\n\x07\x62\x61\x63kend\x18\x01 \x01(\t\x12\x0e\n\x06\x64\x65vice\x18\x02 \x01(\t\x12\x0c\n\x04host\x18\x03 \x01(\t\x12/\n\x05input\x18\n \x01(\x0b\x32 .proto.grpc.AudioInputDescriptor\"H\n\x10\x41udioInputStream\x12\x34\n\ndescriptor\x18\x01 \x01(\x0b\x32 .proto.grpc.AudioInputDescriptor\"H\n\rAudioAnalyzer\x12\x37\n\ndescriptor\x18\x01 \x01(\x0b\x32#.proto.grpc.AudioAnalyzerDescriptor\"J\n\x11\x41udioOutputStream\x12\x35\n\ndescriptor\x18\x01 \x01(\x0b\x32!.proto.grpc.AudioOutputDescriptorb\x06proto3'
)




_AUDIOINPUTDESCRIPTOR = _descriptor.Descriptor(
  name='AudioInputDescriptor',
  full_name='proto.grpc.AudioInputDescriptor',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='backend', full_name='proto.grpc.AudioInputDescriptor.backend', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='device', full_name='proto.grpc.AudioInputDescriptor.device', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='host', full_name='proto.grpc.AudioInputDescriptor.host', index=2,
      number=3, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='file', full_name='proto.grpc.AudioInputDescriptor.file', index=3,
      number=4, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
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
  serialized_start=44,
  serialized_end=127,
)


_AUDIOANALYZERDESCRIPTOR = _descriptor.Descriptor(
  name='AudioAnalyzerDescriptor',
  full_name='proto.grpc.AudioAnalyzerDescriptor',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='proto.grpc.AudioAnalyzerDescriptor.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='input', full_name='proto.grpc.AudioAnalyzerDescriptor.input', index=1,
      number=10, type=11, cpp_type=10, label=1,
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
  serialized_start=129,
  serialized_end=217,
)


_AUDIOOUTPUTDESCRIPTOR = _descriptor.Descriptor(
  name='AudioOutputDescriptor',
  full_name='proto.grpc.AudioOutputDescriptor',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='backend', full_name='proto.grpc.AudioOutputDescriptor.backend', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='device', full_name='proto.grpc.AudioOutputDescriptor.device', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='host', full_name='proto.grpc.AudioOutputDescriptor.host', index=2,
      number=3, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='input', full_name='proto.grpc.AudioOutputDescriptor.input', index=3,
      number=10, type=11, cpp_type=10, label=1,
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
  serialized_start=219,
  serialized_end=338,
)


_AUDIOINPUTSTREAM = _descriptor.Descriptor(
  name='AudioInputStream',
  full_name='proto.grpc.AudioInputStream',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='descriptor', full_name='proto.grpc.AudioInputStream.descriptor', index=0,
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
  serialized_start=340,
  serialized_end=412,
)


_AUDIOANALYZER = _descriptor.Descriptor(
  name='AudioAnalyzer',
  full_name='proto.grpc.AudioAnalyzer',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='descriptor', full_name='proto.grpc.AudioAnalyzer.descriptor', index=0,
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
  serialized_start=414,
  serialized_end=486,
)


_AUDIOOUTPUTSTREAM = _descriptor.Descriptor(
  name='AudioOutputStream',
  full_name='proto.grpc.AudioOutputStream',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='descriptor', full_name='proto.grpc.AudioOutputStream.descriptor', index=0,
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
  serialized_start=488,
  serialized_end=562,
)

_AUDIOANALYZERDESCRIPTOR.fields_by_name['input'].message_type = _AUDIOINPUTDESCRIPTOR
_AUDIOOUTPUTDESCRIPTOR.fields_by_name['input'].message_type = _AUDIOINPUTDESCRIPTOR
_AUDIOINPUTSTREAM.fields_by_name['descriptor'].message_type = _AUDIOINPUTDESCRIPTOR
_AUDIOANALYZER.fields_by_name['descriptor'].message_type = _AUDIOANALYZERDESCRIPTOR
_AUDIOOUTPUTSTREAM.fields_by_name['descriptor'].message_type = _AUDIOOUTPUTDESCRIPTOR
DESCRIPTOR.message_types_by_name['AudioInputDescriptor'] = _AUDIOINPUTDESCRIPTOR
DESCRIPTOR.message_types_by_name['AudioAnalyzerDescriptor'] = _AUDIOANALYZERDESCRIPTOR
DESCRIPTOR.message_types_by_name['AudioOutputDescriptor'] = _AUDIOOUTPUTDESCRIPTOR
DESCRIPTOR.message_types_by_name['AudioInputStream'] = _AUDIOINPUTSTREAM
DESCRIPTOR.message_types_by_name['AudioAnalyzer'] = _AUDIOANALYZER
DESCRIPTOR.message_types_by_name['AudioOutputStream'] = _AUDIOOUTPUTSTREAM
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

AudioInputDescriptor = _reflection.GeneratedProtocolMessageType('AudioInputDescriptor', (_message.Message,), {
  'DESCRIPTOR' : _AUDIOINPUTDESCRIPTOR,
  '__module__' : 'proto.grpc.descriptors_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.AudioInputDescriptor)
  })
_sym_db.RegisterMessage(AudioInputDescriptor)

AudioAnalyzerDescriptor = _reflection.GeneratedProtocolMessageType('AudioAnalyzerDescriptor', (_message.Message,), {
  'DESCRIPTOR' : _AUDIOANALYZERDESCRIPTOR,
  '__module__' : 'proto.grpc.descriptors_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.AudioAnalyzerDescriptor)
  })
_sym_db.RegisterMessage(AudioAnalyzerDescriptor)

AudioOutputDescriptor = _reflection.GeneratedProtocolMessageType('AudioOutputDescriptor', (_message.Message,), {
  'DESCRIPTOR' : _AUDIOOUTPUTDESCRIPTOR,
  '__module__' : 'proto.grpc.descriptors_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.AudioOutputDescriptor)
  })
_sym_db.RegisterMessage(AudioOutputDescriptor)

AudioInputStream = _reflection.GeneratedProtocolMessageType('AudioInputStream', (_message.Message,), {
  'DESCRIPTOR' : _AUDIOINPUTSTREAM,
  '__module__' : 'proto.grpc.descriptors_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.AudioInputStream)
  })
_sym_db.RegisterMessage(AudioInputStream)

AudioAnalyzer = _reflection.GeneratedProtocolMessageType('AudioAnalyzer', (_message.Message,), {
  'DESCRIPTOR' : _AUDIOANALYZER,
  '__module__' : 'proto.grpc.descriptors_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.AudioAnalyzer)
  })
_sym_db.RegisterMessage(AudioAnalyzer)

AudioOutputStream = _reflection.GeneratedProtocolMessageType('AudioOutputStream', (_message.Message,), {
  'DESCRIPTOR' : _AUDIOOUTPUTSTREAM,
  '__module__' : 'proto.grpc.descriptors_pb2'
  # @@protoc_insertion_point(class_scope:proto.grpc.AudioOutputStream)
  })
_sym_db.RegisterMessage(AudioOutputStream)


# @@protoc_insertion_point(module_scope)
