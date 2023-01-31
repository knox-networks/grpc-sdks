/* eslint-disable */
/*Generated by GenDocu.com*/
// package: gateway
// file: gateway/v1/gateway.proto

var gateway_v1_gateway_pb = require("../../gateway/v1/gateway_pb");
var common_v1_common_pb = require("../../common/v1/common_pb");
var common_v1_packet_pb = require("../../common/v1/packet_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var Gateway = (function () {
  function Gateway() {}
  Gateway.serviceName = "gateway.Gateway";
  return Gateway;
}());

Gateway.Authenticate = {
  methodName: "Authenticate",
  service: Gateway,
  requestStream: false,
  responseStream: false,
  requestType: common_v1_common_pb.AuthenticationRequest,
  responseType: common_v1_common_pb.AuthenticationResponse
};

Gateway.Data = {
  methodName: "Data",
  service: Gateway,
  requestStream: true,
  responseStream: true,
  requestType: common_v1_packet_pb.Packet,
  responseType: common_v1_packet_pb.Packet
};

Gateway.GetConnections = {
  methodName: "GetConnections",
  service: Gateway,
  requestStream: false,
  responseStream: false,
  requestType: gateway_v1_gateway_pb.ConnectionsRequest,
  responseType: gateway_v1_gateway_pb.ConnectionsResponse
};

exports.Gateway = Gateway;

function GatewayClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

GatewayClient.prototype.authenticate = function authenticate(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Gateway.Authenticate, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

GatewayClient.prototype.data = function data(metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.client(Gateway.Data, {
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport
  });
  client.onEnd(function (status, statusMessage, trailers) {
    listeners.status.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners.end.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners = null;
  });
  client.onMessage(function (message) {
    listeners.data.forEach(function (handler) {
      handler(message);
    })
  });
  client.start(metadata);
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    write: function (requestMessage) {
      client.send(requestMessage);
      return this;
    },
    end: function () {
      client.finishSend();
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

GatewayClient.prototype.getConnections = function getConnections(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Gateway.GetConnections, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

exports.GatewayClient = GatewayClient;

