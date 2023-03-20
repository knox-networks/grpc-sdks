/* eslint-disable */
/*Generated by GenDocu.com*/
// package: authority_api.v1
// file: authority_api/v1/authority.proto

var authority_api_v1_authority_pb = require("../../authority_api/v1/authority_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var Authority = (function () {
  function Authority() {}
  Authority.serviceName = "authority_api.v1.Authority";
  return Authority;
}());

Authority.SetIssuerLimit = {
  methodName: "SetIssuerLimit",
  service: Authority,
  requestStream: false,
  responseStream: false,
  requestType: authority_api_v1_authority_pb.SetIssuerLimitRequest,
  responseType: authority_api_v1_authority_pb.SetIssuerLimitResponse
};

Authority.GetIssuerLimits = {
  methodName: "GetIssuerLimits",
  service: Authority,
  requestStream: false,
  responseStream: false,
  requestType: authority_api_v1_authority_pb.GetIssuerLimitsRequest,
  responseType: authority_api_v1_authority_pb.GetIssuerLimitsResponse
};

Authority.Authorize = {
  methodName: "Authorize",
  service: Authority,
  requestStream: true,
  responseStream: true,
  requestType: authority_api_v1_authority_pb.AuthorizeRequest,
  responseType: authority_api_v1_authority_pb.AuthorizeResponse
};

Authority.Redeem = {
  methodName: "Redeem",
  service: Authority,
  requestStream: false,
  responseStream: false,
  requestType: authority_api_v1_authority_pb.RedeemRequest,
  responseType: authority_api_v1_authority_pb.RedeemResponse
};

Authority.GetEmissary = {
  methodName: "GetEmissary",
  service: Authority,
  requestStream: false,
  responseStream: false,
  requestType: authority_api_v1_authority_pb.GetEmissaryRequest,
  responseType: authority_api_v1_authority_pb.GetEmissaryResponse
};

Authority.GetNotary = {
  methodName: "GetNotary",
  service: Authority,
  requestStream: false,
  responseStream: false,
  requestType: authority_api_v1_authority_pb.GetNotaryRequest,
  responseType: authority_api_v1_authority_pb.GetNotaryResponse
};

exports.Authority = Authority;

function AuthorityClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

AuthorityClient.prototype.setIssuerLimit = function setIssuerLimit(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Authority.SetIssuerLimit, {
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

AuthorityClient.prototype.getIssuerLimits = function getIssuerLimits(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Authority.GetIssuerLimits, {
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

AuthorityClient.prototype.authorize = function authorize(metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.client(Authority.Authorize, {
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

AuthorityClient.prototype.redeem = function redeem(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Authority.Redeem, {
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

AuthorityClient.prototype.getEmissary = function getEmissary(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Authority.GetEmissary, {
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

AuthorityClient.prototype.getNotary = function getNotary(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Authority.GetNotary, {
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

exports.AuthorityClient = AuthorityClient;

