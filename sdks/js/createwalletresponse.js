// source: wallet_api/v1/wallet.proto
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

goog.provide('proto.wallet_api.v1.CreateWalletResponse');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');

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
proto.wallet_api.v1.CreateWalletResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.wallet_api.v1.CreateWalletResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.wallet_api.v1.CreateWalletResponse.displayName = 'proto.wallet_api.v1.CreateWalletResponse';
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
proto.wallet_api.v1.CreateWalletResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.wallet_api.v1.CreateWalletResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.wallet_api.v1.CreateWalletResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.CreateWalletResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    walletId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    walletName: jspb.Message.getFieldWithDefault(msg, 2, ""),
    publicKey: jspb.Message.getFieldWithDefault(msg, 3, ""),
    did: jspb.Message.getFieldWithDefault(msg, 4, "")
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
 * @return {!proto.wallet_api.v1.CreateWalletResponse}
 */
proto.wallet_api.v1.CreateWalletResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.wallet_api.v1.CreateWalletResponse;
  return proto.wallet_api.v1.CreateWalletResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.wallet_api.v1.CreateWalletResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.wallet_api.v1.CreateWalletResponse}
 */
proto.wallet_api.v1.CreateWalletResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setWalletId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setWalletName(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setPublicKey(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setDid(value);
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
proto.wallet_api.v1.CreateWalletResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.wallet_api.v1.CreateWalletResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.wallet_api.v1.CreateWalletResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.CreateWalletResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getWalletId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getWalletName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getPublicKey();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getDid();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * optional string wallet_id = 1;
 * @return {string}
 */
proto.wallet_api.v1.CreateWalletResponse.prototype.getWalletId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.CreateWalletResponse} returns this
 */
proto.wallet_api.v1.CreateWalletResponse.prototype.setWalletId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string wallet_name = 2;
 * @return {string}
 */
proto.wallet_api.v1.CreateWalletResponse.prototype.getWalletName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.CreateWalletResponse} returns this
 */
proto.wallet_api.v1.CreateWalletResponse.prototype.setWalletName = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string public_key = 3;
 * @return {string}
 */
proto.wallet_api.v1.CreateWalletResponse.prototype.getPublicKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.CreateWalletResponse} returns this
 */
proto.wallet_api.v1.CreateWalletResponse.prototype.setPublicKey = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string did = 4;
 * @return {string}
 */
proto.wallet_api.v1.CreateWalletResponse.prototype.getDid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.CreateWalletResponse} returns this
 */
proto.wallet_api.v1.CreateWalletResponse.prototype.setDid = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


