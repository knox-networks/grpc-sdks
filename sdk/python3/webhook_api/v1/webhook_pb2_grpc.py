"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...webhook_api.v1 import webhook_pb2 as webhook__api_dot_v1_dot_webhook__pb2

class WebhookManagerServiceStub(object):
    """The Service Manages the Lifecycle of Webhooks.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.CreateWebhook = channel.unary_unary('/webhook_api.v1.WebhookManagerService/CreateWebhook', request_serializer=webhook__api_dot_v1_dot_webhook__pb2.CreateWebhookRequest.SerializeToString, response_deserializer=webhook__api_dot_v1_dot_webhook__pb2.CreateWebhookResponse.FromString)
        self.ListWebhooks = channel.unary_unary('/webhook_api.v1.WebhookManagerService/ListWebhooks', request_serializer=webhook__api_dot_v1_dot_webhook__pb2.ListWebhooksRequest.SerializeToString, response_deserializer=webhook__api_dot_v1_dot_webhook__pb2.ListWebhooksResponse.FromString)
        self.DeleteWebhook = channel.unary_unary('/webhook_api.v1.WebhookManagerService/DeleteWebhook', request_serializer=webhook__api_dot_v1_dot_webhook__pb2.DeleteWebhookRequest.SerializeToString, response_deserializer=webhook__api_dot_v1_dot_webhook__pb2.DeleteWebhookResponse.FromString)
        self.ListEvents = channel.unary_unary('/webhook_api.v1.WebhookManagerService/ListEvents', request_serializer=webhook__api_dot_v1_dot_webhook__pb2.ListEventsRequest.SerializeToString, response_deserializer=webhook__api_dot_v1_dot_webhook__pb2.ListEventsResponse.FromString)
        self.RetryEvent = channel.unary_unary('/webhook_api.v1.WebhookManagerService/RetryEvent', request_serializer=webhook__api_dot_v1_dot_webhook__pb2.RetryEventRequest.SerializeToString, response_deserializer=webhook__api_dot_v1_dot_webhook__pb2.RetryEventResponse.FromString)
        self.Ping = channel.unary_unary('/webhook_api.v1.WebhookManagerService/Ping', request_serializer=webhook__api_dot_v1_dot_webhook__pb2.PingRequest.SerializeToString, response_deserializer=webhook__api_dot_v1_dot_webhook__pb2.PingResponse.FromString)
        self.ListDeliveryHistory = channel.unary_unary('/webhook_api.v1.WebhookManagerService/ListDeliveryHistory', request_serializer=webhook__api_dot_v1_dot_webhook__pb2.ListDeliveryHistoryRequest.SerializeToString, response_deserializer=webhook__api_dot_v1_dot_webhook__pb2.ListDeliveryHistoryResponse.FromString)

class WebhookManagerServiceServicer(object):
    """The Service Manages the Lifecycle of Webhooks.
    """

    def CreateWebhook(self, request, context):
        """Creates a Webhook.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListWebhooks(self, request, context):
        """Lists Webhooks.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def DeleteWebhook(self, request, context):
        """Deletes a Webhook.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListEvents(self, request, context):
        """Lists Webhook Events.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def RetryEvent(self, request, context):
        """Retries a Webhook Event.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Ping(self, request, context):
        """Ping a Webhook.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListDeliveryHistory(self, request, context):
        """Lists Delivery History.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_WebhookManagerServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'CreateWebhook': grpc.unary_unary_rpc_method_handler(servicer.CreateWebhook, request_deserializer=webhook__api_dot_v1_dot_webhook__pb2.CreateWebhookRequest.FromString, response_serializer=webhook__api_dot_v1_dot_webhook__pb2.CreateWebhookResponse.SerializeToString), 'ListWebhooks': grpc.unary_unary_rpc_method_handler(servicer.ListWebhooks, request_deserializer=webhook__api_dot_v1_dot_webhook__pb2.ListWebhooksRequest.FromString, response_serializer=webhook__api_dot_v1_dot_webhook__pb2.ListWebhooksResponse.SerializeToString), 'DeleteWebhook': grpc.unary_unary_rpc_method_handler(servicer.DeleteWebhook, request_deserializer=webhook__api_dot_v1_dot_webhook__pb2.DeleteWebhookRequest.FromString, response_serializer=webhook__api_dot_v1_dot_webhook__pb2.DeleteWebhookResponse.SerializeToString), 'ListEvents': grpc.unary_unary_rpc_method_handler(servicer.ListEvents, request_deserializer=webhook__api_dot_v1_dot_webhook__pb2.ListEventsRequest.FromString, response_serializer=webhook__api_dot_v1_dot_webhook__pb2.ListEventsResponse.SerializeToString), 'RetryEvent': grpc.unary_unary_rpc_method_handler(servicer.RetryEvent, request_deserializer=webhook__api_dot_v1_dot_webhook__pb2.RetryEventRequest.FromString, response_serializer=webhook__api_dot_v1_dot_webhook__pb2.RetryEventResponse.SerializeToString), 'Ping': grpc.unary_unary_rpc_method_handler(servicer.Ping, request_deserializer=webhook__api_dot_v1_dot_webhook__pb2.PingRequest.FromString, response_serializer=webhook__api_dot_v1_dot_webhook__pb2.PingResponse.SerializeToString), 'ListDeliveryHistory': grpc.unary_unary_rpc_method_handler(servicer.ListDeliveryHistory, request_deserializer=webhook__api_dot_v1_dot_webhook__pb2.ListDeliveryHistoryRequest.FromString, response_serializer=webhook__api_dot_v1_dot_webhook__pb2.ListDeliveryHistoryResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('webhook_api.v1.WebhookManagerService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class WebhookManagerService(object):
    """The Service Manages the Lifecycle of Webhooks.
    """

    @staticmethod
    def CreateWebhook(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/webhook_api.v1.WebhookManagerService/CreateWebhook', webhook__api_dot_v1_dot_webhook__pb2.CreateWebhookRequest.SerializeToString, webhook__api_dot_v1_dot_webhook__pb2.CreateWebhookResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListWebhooks(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/webhook_api.v1.WebhookManagerService/ListWebhooks', webhook__api_dot_v1_dot_webhook__pb2.ListWebhooksRequest.SerializeToString, webhook__api_dot_v1_dot_webhook__pb2.ListWebhooksResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def DeleteWebhook(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/webhook_api.v1.WebhookManagerService/DeleteWebhook', webhook__api_dot_v1_dot_webhook__pb2.DeleteWebhookRequest.SerializeToString, webhook__api_dot_v1_dot_webhook__pb2.DeleteWebhookResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListEvents(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/webhook_api.v1.WebhookManagerService/ListEvents', webhook__api_dot_v1_dot_webhook__pb2.ListEventsRequest.SerializeToString, webhook__api_dot_v1_dot_webhook__pb2.ListEventsResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def RetryEvent(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/webhook_api.v1.WebhookManagerService/RetryEvent', webhook__api_dot_v1_dot_webhook__pb2.RetryEventRequest.SerializeToString, webhook__api_dot_v1_dot_webhook__pb2.RetryEventResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Ping(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/webhook_api.v1.WebhookManagerService/Ping', webhook__api_dot_v1_dot_webhook__pb2.PingRequest.SerializeToString, webhook__api_dot_v1_dot_webhook__pb2.PingResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListDeliveryHistory(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/webhook_api.v1.WebhookManagerService/ListDeliveryHistory', webhook__api_dot_v1_dot_webhook__pb2.ListDeliveryHistoryRequest.SerializeToString, webhook__api_dot_v1_dot_webhook__pb2.ListDeliveryHistoryResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

class EventConsumerServiceStub(object):
    """This service is implemented by the consumer service receiving the webhook event
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Event = channel.unary_unary('/webhook_api.v1.EventConsumerService/Event', request_serializer=webhook__api_dot_v1_dot_webhook__pb2.EventRequest.SerializeToString, response_deserializer=webhook__api_dot_v1_dot_webhook__pb2.EventResponse.FromString)

class EventConsumerServiceServicer(object):
    """This service is implemented by the consumer service receiving the webhook event
    """

    def Event(self, request, context):
        """Webhook Event.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_EventConsumerServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'Event': grpc.unary_unary_rpc_method_handler(servicer.Event, request_deserializer=webhook__api_dot_v1_dot_webhook__pb2.EventRequest.FromString, response_serializer=webhook__api_dot_v1_dot_webhook__pb2.EventResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('webhook_api.v1.EventConsumerService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class EventConsumerService(object):
    """This service is implemented by the consumer service receiving the webhook event
    """

    @staticmethod
    def Event(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/webhook_api.v1.EventConsumerService/Event', webhook__api_dot_v1_dot_webhook__pb2.EventRequest.SerializeToString, webhook__api_dot_v1_dot_webhook__pb2.EventResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)