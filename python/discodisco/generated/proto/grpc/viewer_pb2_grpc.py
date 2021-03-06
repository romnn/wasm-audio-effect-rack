# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from proto.grpc import remote_pb2 as proto_dot_grpc_dot_remote__pb2
from proto.grpc import viewer_pb2 as proto_dot_grpc_dot_viewer__pb2


class RemoteViewerStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Connect = channel.unary_stream(
                '/proto.grpc.RemoteViewer/Connect',
                request_serializer=proto_dot_grpc_dot_viewer__pb2.ViewerConnectRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_viewer__pb2.ViewerUpdate.FromString,
                )
        self.Disconnect = channel.unary_unary(
                '/proto.grpc.RemoteViewer/Disconnect',
                request_serializer=proto_dot_grpc_dot_viewer__pb2.ViewerDisconnectRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_remote__pb2.Empty.FromString,
                )
        self.UpdateSubscription = channel.unary_unary(
                '/proto.grpc.RemoteViewer/UpdateSubscription',
                request_serializer=proto_dot_grpc_dot_viewer__pb2.UpdateSubscriptionRequest.SerializeToString,
                response_deserializer=proto_dot_grpc_dot_remote__pb2.Empty.FromString,
                )


class RemoteViewerServicer(object):
    """Missing associated documentation comment in .proto file."""

    def Connect(self, request, context):
        """Connect and Disconnect to updates
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Disconnect(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UpdateSubscription(self, request, context):
        """change subscription
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_RemoteViewerServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'Connect': grpc.unary_stream_rpc_method_handler(
                    servicer.Connect,
                    request_deserializer=proto_dot_grpc_dot_viewer__pb2.ViewerConnectRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_viewer__pb2.ViewerUpdate.SerializeToString,
            ),
            'Disconnect': grpc.unary_unary_rpc_method_handler(
                    servicer.Disconnect,
                    request_deserializer=proto_dot_grpc_dot_viewer__pb2.ViewerDisconnectRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_remote__pb2.Empty.SerializeToString,
            ),
            'UpdateSubscription': grpc.unary_unary_rpc_method_handler(
                    servicer.UpdateSubscription,
                    request_deserializer=proto_dot_grpc_dot_viewer__pb2.UpdateSubscriptionRequest.FromString,
                    response_serializer=proto_dot_grpc_dot_remote__pb2.Empty.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'proto.grpc.RemoteViewer', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class RemoteViewer(object):
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
        return grpc.experimental.unary_stream(request, target, '/proto.grpc.RemoteViewer/Connect',
            proto_dot_grpc_dot_viewer__pb2.ViewerConnectRequest.SerializeToString,
            proto_dot_grpc_dot_viewer__pb2.ViewerUpdate.FromString,
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
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteViewer/Disconnect',
            proto_dot_grpc_dot_viewer__pb2.ViewerDisconnectRequest.SerializeToString,
            proto_dot_grpc_dot_remote__pb2.Empty.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def UpdateSubscription(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/proto.grpc.RemoteViewer/UpdateSubscription',
            proto_dot_grpc_dot_viewer__pb2.UpdateSubscriptionRequest.SerializeToString,
            proto_dot_grpc_dot_remote__pb2.Empty.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
