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

goog.provide('proto.wallet_api.v1.BalanceDetail');
goog.provide('proto.wallet_api.v1.BalanceDetail.Kind');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.wallet_api.v1.WalletBalance');

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
proto.wallet_api.v1.BalanceDetail = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.wallet_api.v1.BalanceDetail, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.wallet_api.v1.BalanceDetail.displayName = 'proto.wallet_api.v1.BalanceDetail';
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
proto.wallet_api.v1.BalanceDetail.prototype.toObject = function(opt_includeInstance) {
  return proto.wallet_api.v1.BalanceDetail.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.wallet_api.v1.BalanceDetail} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.BalanceDetail.toObject = function(includeInstance, msg) {
  var f, obj = {
    balance: (f = msg.getBalance()) && proto.wallet_api.v1.WalletBalance.toObject(includeInstance, f),
    kind: jspb.Message.getFieldWithDefault(msg, 2, 0)
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
 * @return {!proto.wallet_api.v1.BalanceDetail}
 */
proto.wallet_api.v1.BalanceDetail.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.wallet_api.v1.BalanceDetail;
  return proto.wallet_api.v1.BalanceDetail.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.wallet_api.v1.BalanceDetail} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.wallet_api.v1.BalanceDetail}
 */
proto.wallet_api.v1.BalanceDetail.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.wallet_api.v1.WalletBalance;
      reader.readMessage(value,proto.wallet_api.v1.WalletBalance.deserializeBinaryFromReader);
      msg.setBalance(value);
      break;
    case 2:
      var value = /** @type {!proto.wallet_api.v1.BalanceDetail.Kind} */ (reader.readEnum());
      msg.setKind(value);
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
proto.wallet_api.v1.BalanceDetail.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.wallet_api.v1.BalanceDetail.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.wallet_api.v1.BalanceDetail} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.BalanceDetail.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getBalance();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.wallet_api.v1.WalletBalance.serializeBinaryToWriter
    );
  }
  f = message.getKind();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
};


/**
 * @enum {number}
 */
proto.wallet_api.v1.BalanceDetail.Kind = {
  KIND_AVAILABLE: 0,
  KIND_RESERVED: 1
};

/**
 * optional WalletBalance balance = 1;
 * @return {?proto.wallet_api.v1.WalletBalance}
 */
proto.wallet_api.v1.BalanceDetail.prototype.getBalance = function() {
  return /** @type{?proto.wallet_api.v1.WalletBalance} */ (
    jspb.Message.getWrapperField(this, proto.wallet_api.v1.WalletBalance, 1));
};


/**
 * @param {?proto.wallet_api.v1.WalletBalance|undefined} value
 * @return {!proto.wallet_api.v1.BalanceDetail} returns this
*/
proto.wallet_api.v1.BalanceDetail.prototype.setBalance = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.wallet_api.v1.BalanceDetail} returns this
 */
proto.wallet_api.v1.BalanceDetail.prototype.clearBalance = function() {
  return this.setBalance(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.wallet_api.v1.BalanceDetail.prototype.hasBalance = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional Kind kind = 2;
 * @return {!proto.wallet_api.v1.BalanceDetail.Kind}
 */
proto.wallet_api.v1.BalanceDetail.prototype.getKind = function() {
  return /** @type {!proto.wallet_api.v1.BalanceDetail.Kind} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.wallet_api.v1.BalanceDetail.Kind} value
 * @return {!proto.wallet_api.v1.BalanceDetail} returns this
 */
proto.wallet_api.v1.BalanceDetail.prototype.setKind = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};

