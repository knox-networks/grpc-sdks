/* eslint-disable */
/*Generated by GenDocu.com*/
// package: issuer_api.v1
// file: issuer_api/v1/issuer.proto

var issuer_api_v1_issuer_pb = require("../../issuer_api/v1/issuer_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var Issuer = (function () {
  function Issuer() {}
  Issuer.serviceName = "issuer_api.v1.Issuer";
  return Issuer;
}());

Issuer.Authenticate = {
  methodName: "Authenticate",
  service: Issuer,
  requestStream: false,
  responseStream: false,
  requestType: issuer_api_v1_issuer_pb.AuthenticateRequest,
  responseType: issuer_api_v1_issuer_pb.AuthenticateResponse
};

Issuer.Authorize = {
  methodName: "Authorize",
  service: Issuer,
  requestStream: true,
  responseStream: true,
  requestType: issuer_api_v1_issuer_pb.AuthorizeRequest,
  responseType: issuer_api_v1_issuer_pb.AuthorizeResponse
};

Issuer.Issue = {
  methodName: "Issue",
  service: Issuer,
  requestStream: false,
  responseStream: true,
  requestType: issuer_api_v1_issuer_pb.IssueRequest,
  responseType: issuer_api_v1_issuer_pb.IssueResponse
};

Issuer.GetRoles = {
  methodName: "GetRoles",
  service: Issuer,
  requestStream: false,
  responseStream: false,
  requestType: issuer_api_v1_issuer_pb.GetRolesRequest,
  responseType: issuer_api_v1_issuer_pb.GetRolesResponse
};

Issuer.SetRole = {
  methodName: "SetRole",
  service: Issuer,
  requestStream: false,
  responseStream: false,
  requestType: issuer_api_v1_issuer_pb.SetRoleRequest,
  responseType: issuer_api_v1_issuer_pb.SetRoleResponse
};

exports.Issuer = Issuer;

function IssuerClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

IssuerClient.prototype.authenticate = function authenticate(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Issuer.Authenticate, {
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

IssuerClient.prototype.authorize = function authorize(metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.client(Issuer.Authorize, {
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

IssuerClient.prototype.issue = function issue(requestMessage, metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.invoke(Issuer.Issue, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onMessage: function (responseMessage) {
      listeners.data.forEach(function (handler) {
        handler(responseMessage);
      });
    },
    onEnd: function (status, statusMessage, trailers) {
      listeners.status.forEach(function (handler) {
        handler({ code: status, details: statusMessage, metadata: trailers });
      });
      listeners.end.forEach(function (handler) {
        handler({ code: status, details: statusMessage, metadata: trailers });
      });
      listeners = null;
    }
  });
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

IssuerClient.prototype.getRoles = function getRoles(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Issuer.GetRoles, {
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

IssuerClient.prototype.setRole = function setRole(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Issuer.SetRole, {
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

exports.IssuerClient = IssuerClient;

