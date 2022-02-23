# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from proto.audio.analysis import analysis_pb2 as proto_dot_audio_dot_analysis_dot_analysis__pb2
from proto.grpc import controller_pb2 as proto_dot_grpc_dot_controller__pb2
from proto.grpc import descriptors_pb2 as proto_dot_grpc_dot_descriptors__pb2
from proto.grpc import remote_pb2 as proto_dot_grpc_dot_remote__pb2
from proto.grpc import session_pb2 as proto_dot_grpc_dot_session__pb2


class RemoteControllerStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Connect = channel.unary_stream(
                '/proto.grpc.RemoteController/Connect',
                request_serializer=proto_dot_grpc_dot_controller__pb2.ControllerConnectRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_controller__pb2.ControllerUpdate.FromString,
                )
        self.Disconnect = channel.unary_unary(
                '/proto.grpc.RemoteController/Disconnect',
                request_serializer=proto_dot_grpc_dot_controller__pb2.ControllerDisconnectRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_remote__pb2.Empty.FromString,
                )
        self.AddAudioInputStream = channel.unary_unary(
                '/proto.grpc.RemoteController/AddAudioInputStream',
                request_serializer=proto_dot_grpc_dot_controller__pb2.AddAudioInputStreamRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_descriptors__pb2.AudioInputStream.FromString,
                )
        self.AddAudioAnalyzer = channel.unary_unary(
                '/proto.grpc.RemoteController/AddAudioAnalyzer',
                request_serializer=proto_dot_grpc_dot_controller__pb2.AddAudioAnalyzerRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_descriptors__pb2.AudioAnalyzer.FromString,
                )
        self.AddAudioOutputStream = channel.unary_unary(
                '/proto.grpc.RemoteController/AddAudioOutputStream',
                request_serializer=proto_dot_grpc_dot_controller__pb2.AddAudioOutputStreamRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_descriptors__pb2.AudioOutputStream.FromString,
                )
        self.SubscribeToAudioAnalyzer = channel.unary_unary(
                '/proto.grpc.RemoteController/SubscribeToAudioAnalyzer',
                request_serializer=proto_dot_grpc_dot_controller__pb2.SubscribeToAudioAnalyzerRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_session__pb2.InstanceSubscriptions.FromString,
                )
        self.ConnectLightsToAudioAnalyzer = channel.unary_unary(
                '/proto.grpc.RemoteController/ConnectLightsToAudioAnalyzer',
                request_serializer=proto_dot_grpc_dot_controller__pb2.ConnectLightsToAudioAnalyzerRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_session__pb2.InstanceSubscriptions.FromString,
                )
        self.GetSessions = channel.unary_unary(
                '/proto.grpc.RemoteController/GetSessions',
                request_serializer=proto_dot_grpc_dot_controller__pb2.GetSessionsRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_session__pb2.Sessions.FromString,
                )
        self.RequestRecordingFrame = channel.unary_unary(
                '/proto.grpc.RemoteController/RequestRecordingFrame',
                request_serializer=proto_dot_grpc_dot_controller__pb2.RecordingFrameRequest.SerializeToString,
                response_deserializer=proto_dot_audio_dot_analysis_dot_analysis__pb2.AudioAnalysisResult.FromString,
                )


class RemoteControllerServicer(object):
    """Missing associated documentation comment in .proto file."""

    def Connect(self, request, context):
        """connect and disconnect
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Disconnect(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def AddAudioInputStream(self, request, context):
        """start and stop analyzing audio
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def AddAudioAnalyzer(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def AddAudioOutputStream(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def SubscribeToAudioAnalyzer(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ConnectLightsToAudioAnalyzer(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetSessions(self, request, context):
        """query sessions
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def RequestRecordingFrame(self, request, context):
        """recording
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_RemoteControllerServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'Connect': grpc.unary_stream_rpc_method_handler(
                    servicer.Connect,
                    request_deserializer=proto_dot_grpc_dot_controller__pb2.ControllerConnectRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_controller__pb2.ControllerUpdate.SerializeToString,
            ),
            'Disconnect': grpc.unary_unary_rpc_method_handler(
                    servicer.Disconnect,
                    request_deserializer=proto_dot_grpc_dot_controller__pb2.ControllerDisconnectRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_remote__pb2.Empty.SerializeToString,
            ),
            'AddAudioInputStream': grpc.unary_unary_rpc_method_handler(
                    servicer.AddAudioInputStream,
                    request_deserializer=proto_dot_grpc_dot_controller__pb2.AddAudioInputStreamRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_descriptors__pb2.AudioInputStream.SerializeToString,
            ),
            'AddAudioAnalyzer': grpc.unary_unary_rpc_method_handler(
                    servicer.AddAudioAnalyzer,
                    request_deserializer=proto_dot_grpc_dot_controller__pb2.AddAudioAnalyzerRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_descriptors__pb2.AudioAnalyzer.SerializeToString,
            ),
            'AddAudioOutputStream': grpc.unary_unary_rpc_method_handler(
                    servicer.AddAudioOutputStream,
                    request_deserializer=proto_dot_grpc_dot_controller__pb2.AddAudioOutputStreamRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_descriptors__pb2.AudioOutputStream.SerializeToString,
            ),
            'SubscribeToAudioAnalyzer': grpc.unary_unary_rpc_method_handler(
                    servicer.SubscribeToAudioAnalyzer,
                    request_deserializer=proto_dot_grpc_dot_controller__pb2.SubscribeToAudioAnalyzerRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_session__pb2.InstanceSubscriptions.SerializeToString,
            ),
            'ConnectLightsToAudioAnalyzer': grpc.unary_unary_rpc_method_handler(
                    servicer.ConnectLightsToAudioAnalyzer,
                    request_deserializer=proto_dot_grpc_dot_controller__pb2.ConnectLightsToAudioAnalyzerRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_session__pb2.InstanceSubscriptions.SerializeToString,
            ),
            'GetSessions': grpc.unary_unary_rpc_method_handler(
                    servicer.GetSessions,
                    request_deserializer=proto_dot_grpc_dot_controller__pb2.GetSessionsRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_session__pb2.Sessions.SerializeToString,
            ),
            'RequestRecordingFrame': grpc.unary_unary_rpc_method_handler(
                    servicer.RequestRecordingFrame,
                    request_deserializer=proto_dot_grpc_dot_controller__pb2.RecordingFrameRequest.FromString,
                    response_serializer=proto_dot_audio_dot_analysis_dot_analysis__pb2.AudioAnalysisResult.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'proto.grpc.RemoteController', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class RemoteController(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def Connect(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_stream(request, target, '/proto.grpc.RemoteController/Connect',
            proto_dot_grpc_dot_controller__pb2.ControllerConnectRequest.SerializeToString,
            proto_dot_grpc_dot_controller__pb2.ControllerUpdate.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Disconnect(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteController/Disconnect',
            proto_dot_grpc_dot_controller__pb2.ControllerDisconnectRequest.SerializeToString,
            proto_dot_grpc_dot_remote__pb2.Empty.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def AddAudioInputStream(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteController/AddAudioInputStream',
            proto_dot_grpc_dot_controller__pb2.AddAudioInputStreamRequest.SerializeToString,
            proto_dot_grpc_dot_descriptors__pb2.AudioInputStream.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def AddAudioAnalyzer(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteController/AddAudioAnalyzer',
            proto_dot_grpc_dot_controller__pb2.AddAudioAnalyzerRequest.SerializeToString,
            proto_dot_grpc_dot_descriptors__pb2.AudioAnalyzer.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def AddAudioOutputStream(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteController/AddAudioOutputStream',
            proto_dot_grpc_dot_controller__pb2.AddAudioOutputStreamRequest.SerializeToString,
            proto_dot_grpc_dot_descriptors__pb2.AudioOutputStream.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def SubscribeToAudioAnalyzer(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteController/SubscribeToAudioAnalyzer',
            proto_dot_grpc_dot_controller__pb2.SubscribeToAudioAnalyzerRequest.SerializeToString,
            proto_dot_grpc_dot_session__pb2.InstanceSubscriptions.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ConnectLightsToAudioAnalyzer(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteController/ConnectLightsToAudioAnalyzer',
            proto_dot_grpc_dot_controller__pb2.ConnectLightsToAudioAnalyzerRequest.SerializeToString,
            proto_dot_grpc_dot_session__pb2.InstanceSubscriptions.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetSessions(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteController/GetSessions',
            proto_dot_grpc_dot_controller__pb2.GetSessionsRequest.SerializeToString,
            proto_dot_grpc_dot_session__pb2.Sessions.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def RequestRecordingFrame(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteController/RequestRecordingFrame',
            proto_dot_grpc_dot_controller__pb2.RecordingFrameRequest.SerializeToString,
            proto_dot_audio_dot_analysis_dot_analysis__pb2.AudioAnalysisResult.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
