// source: user_api/v1/user.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

goog.provide('proto.user_api.v1.AuthnWithProviderRequest');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.google.protobuf.Struct');

goog.forwardDeclare('proto.user_api.v1.Provider');
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.user_api.v1.AuthnWithProviderRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.user_api.v1.AuthnWithProviderRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.user_api.v1.AuthnWithProviderRequest.displayName = 'proto.user_api.v1.AuthnWithProviderRequest';
}



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.user_api.v1.AuthnWithProviderRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.user_api.v1.AuthnWithProviderRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.AuthnWithProviderRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    provider: jspb.Message.getFieldWithDefault(msg, 1, 0),
    instanceName: jspb.Message.getFieldWithDefault(msg, 2, ""),
    requestOrigin: jspb.Message.getFieldWithDefault(msg, 3, ""),
    clientState: (f = msg.getClientState()) && proto.google.protobuf.Struct.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.user_api.v1.AuthnWithProviderRequest}
 */
proto.user_api.v1.AuthnWithProviderRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.user_api.v1.AuthnWithProviderRequest;
  return proto.user_api.v1.AuthnWithProviderRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.user_api.v1.AuthnWithProviderRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.user_api.v1.AuthnWithProviderRequest}
 */
proto.user_api.v1.AuthnWithProviderRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.user_api.v1.Provider} */ (reader.readEnum());
      msg.setProvider(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setInstanceName(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setRequestOrigin(value);
      break;
    case 4:
      var value = new proto.google.protobuf.Struct;
      reader.readMessage(value,proto.google.protobuf.Struct.deserializeBinaryFromReader);
      msg.setClientState(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.user_api.v1.AuthnWithProviderRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.user_api.v1.AuthnWithProviderRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.AuthnWithProviderRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getProvider();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getInstanceName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getRequestOrigin();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getClientState();
  if (f != null) {
    writer.writeMessage(
      4,
      f,
      proto.google.protobuf.Struct.serializeBinaryToWriter
    );
  }
};


/**
 * optional Provider provider = 1;
 * @return {!proto.user_api.v1.Provider}
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.getProvider = function() {
  return /** @type {!proto.user_api.v1.Provider} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.user_api.v1.Provider} value
 * @return {!proto.user_api.v1.AuthnWithProviderRequest} returns this
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.setProvider = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional string instance_name = 2;
 * @return {string}
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.getInstanceName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.user_api.v1.AuthnWithProviderRequest} returns this
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.setInstanceName = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string request_origin = 3;
 * @return {string}
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.getRequestOrigin = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.user_api.v1.AuthnWithProviderRequest} returns this
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.setRequestOrigin = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional google.protobuf.Struct client_state = 4;
 * @return {?proto.google.protobuf.Struct}
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.getClientState = function() {
  return /** @type{?proto.google.protobuf.Struct} */ (
    jspb.Message.getWrapperField(this, proto.google.protobuf.Struct, 4));
};


/**
 * @param {?proto.google.protobuf.Struct|undefined} value
 * @return {!proto.user_api.v1.AuthnWithProviderRequest} returns this
*/
proto.user_api.v1.AuthnWithProviderRequest.prototype.setClientState = function(value) {
  return jspb.Message.setWrapperField(this, 4, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.user_api.v1.AuthnWithProviderRequest} returns this
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.clearClientState = function() {
  return this.setClientState(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.user_api.v1.AuthnWithProviderRequest.prototype.hasClientState = function() {
  return jspb.Message.getField(this, 4) != null;
};

