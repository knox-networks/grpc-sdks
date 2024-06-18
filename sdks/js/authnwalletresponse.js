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

goog.provide('proto.user_api.v1.AuthnWalletResponse');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.user_api.v1.AuthToken');
goog.require('proto.user_api.v1.User');

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
proto.user_api.v1.AuthnWalletResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.user_api.v1.AuthnWalletResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.user_api.v1.AuthnWalletResponse.displayName = 'proto.user_api.v1.AuthnWalletResponse';
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
proto.user_api.v1.AuthnWalletResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.user_api.v1.AuthnWalletResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.user_api.v1.AuthnWalletResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.AuthnWalletResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    user: (f = msg.getUser()) && proto.user_api.v1.User.toObject(includeInstance, f),
    authToken: (f = msg.getAuthToken()) && proto.user_api.v1.AuthToken.toObject(includeInstance, f)
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
 * @return {!proto.user_api.v1.AuthnWalletResponse}
 */
proto.user_api.v1.AuthnWalletResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.user_api.v1.AuthnWalletResponse;
  return proto.user_api.v1.AuthnWalletResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.user_api.v1.AuthnWalletResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.user_api.v1.AuthnWalletResponse}
 */
proto.user_api.v1.AuthnWalletResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.user_api.v1.User;
      reader.readMessage(value,proto.user_api.v1.User.deserializeBinaryFromReader);
      msg.setUser(value);
      break;
    case 2:
      var value = new proto.user_api.v1.AuthToken;
      reader.readMessage(value,proto.user_api.v1.AuthToken.deserializeBinaryFromReader);
      msg.setAuthToken(value);
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
proto.user_api.v1.AuthnWalletResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.user_api.v1.AuthnWalletResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.user_api.v1.AuthnWalletResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.AuthnWalletResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUser();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.user_api.v1.User.serializeBinaryToWriter
    );
  }
  f = message.getAuthToken();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.user_api.v1.AuthToken.serializeBinaryToWriter
    );
  }
};


/**
 * optional User user = 1;
 * @return {?proto.user_api.v1.User}
 */
proto.user_api.v1.AuthnWalletResponse.prototype.getUser = function() {
  return /** @type{?proto.user_api.v1.User} */ (
    jspb.Message.getWrapperField(this, proto.user_api.v1.User, 1));
};


/**
 * @param {?proto.user_api.v1.User|undefined} value
 * @return {!proto.user_api.v1.AuthnWalletResponse} returns this
*/
proto.user_api.v1.AuthnWalletResponse.prototype.setUser = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.user_api.v1.AuthnWalletResponse} returns this
 */
proto.user_api.v1.AuthnWalletResponse.prototype.clearUser = function() {
  return this.setUser(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.user_api.v1.AuthnWalletResponse.prototype.hasUser = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional AuthToken auth_token = 2;
 * @return {?proto.user_api.v1.AuthToken}
 */
proto.user_api.v1.AuthnWalletResponse.prototype.getAuthToken = function() {
  return /** @type{?proto.user_api.v1.AuthToken} */ (
    jspb.Message.getWrapperField(this, proto.user_api.v1.AuthToken, 2));
};


/**
 * @param {?proto.user_api.v1.AuthToken|undefined} value
 * @return {!proto.user_api.v1.AuthnWalletResponse} returns this
*/
proto.user_api.v1.AuthnWalletResponse.prototype.setAuthToken = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.user_api.v1.AuthnWalletResponse} returns this
 */
proto.user_api.v1.AuthnWalletResponse.prototype.clearAuthToken = function() {
  return this.setAuthToken(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.user_api.v1.AuthnWalletResponse.prototype.hasAuthToken = function() {
  return jspb.Message.getField(this, 2) != null;
};


