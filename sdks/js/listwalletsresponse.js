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

goog.provide('proto.wallet_api.v1.ListWalletsResponse');
goog.provide('proto.wallet_api.v1.ListWalletsResponse.Wallet');

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
proto.wallet_api.v1.ListWalletsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.wallet_api.v1.ListWalletsResponse.repeatedFields_, null);
};
goog.inherits(proto.wallet_api.v1.ListWalletsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.wallet_api.v1.ListWalletsResponse.displayName = 'proto.wallet_api.v1.ListWalletsResponse';
}
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
proto.wallet_api.v1.ListWalletsResponse.Wallet = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.wallet_api.v1.ListWalletsResponse.Wallet, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.wallet_api.v1.ListWalletsResponse.Wallet.displayName = 'proto.wallet_api.v1.ListWalletsResponse.Wallet';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.wallet_api.v1.ListWalletsResponse.repeatedFields_ = [1];



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
proto.wallet_api.v1.ListWalletsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.wallet_api.v1.ListWalletsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.wallet_api.v1.ListWalletsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.ListWalletsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    walletsList: jspb.Message.toObjectList(msg.getWalletsList(),
    proto.wallet_api.v1.ListWalletsResponse.Wallet.toObject, includeInstance),
    totalOwnedWallets: jspb.Message.getFieldWithDefault(msg, 2, 0)
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
 * @return {!proto.wallet_api.v1.ListWalletsResponse}
 */
proto.wallet_api.v1.ListWalletsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.wallet_api.v1.ListWalletsResponse;
  return proto.wallet_api.v1.ListWalletsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.wallet_api.v1.ListWalletsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.wallet_api.v1.ListWalletsResponse}
 */
proto.wallet_api.v1.ListWalletsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.wallet_api.v1.ListWalletsResponse.Wallet;
      reader.readMessage(value,proto.wallet_api.v1.ListWalletsResponse.Wallet.deserializeBinaryFromReader);
      msg.addWallets(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setTotalOwnedWallets(value);
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
proto.wallet_api.v1.ListWalletsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.wallet_api.v1.ListWalletsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.wallet_api.v1.ListWalletsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.ListWalletsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getWalletsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.wallet_api.v1.ListWalletsResponse.Wallet.serializeBinaryToWriter
    );
  }
  f = message.getTotalOwnedWallets();
  if (f !== 0) {
    writer.writeUint32(
      2,
      f
    );
  }
};





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
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.toObject = function(opt_includeInstance) {
  return proto.wallet_api.v1.ListWalletsResponse.Wallet.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.wallet_api.v1.ListWalletsResponse.Wallet} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.toObject = function(includeInstance, msg) {
  var f, obj = {
    walletName: jspb.Message.getFieldWithDefault(msg, 1, ""),
    walletId: jspb.Message.getFieldWithDefault(msg, 2, ""),
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
 * @return {!proto.wallet_api.v1.ListWalletsResponse.Wallet}
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.wallet_api.v1.ListWalletsResponse.Wallet;
  return proto.wallet_api.v1.ListWalletsResponse.Wallet.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.wallet_api.v1.ListWalletsResponse.Wallet} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.wallet_api.v1.ListWalletsResponse.Wallet}
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setWalletName(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setWalletId(value);
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
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.wallet_api.v1.ListWalletsResponse.Wallet.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.wallet_api.v1.ListWalletsResponse.Wallet} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getWalletName();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getWalletId();
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
 * optional string wallet_name = 1;
 * @return {string}
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.getWalletName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.ListWalletsResponse.Wallet} returns this
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.setWalletName = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string wallet_id = 2;
 * @return {string}
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.getWalletId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.ListWalletsResponse.Wallet} returns this
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.setWalletId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string public_key = 3;
 * @return {string}
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.getPublicKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.ListWalletsResponse.Wallet} returns this
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.setPublicKey = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string did = 4;
 * @return {string}
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.getDid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.ListWalletsResponse.Wallet} returns this
 */
proto.wallet_api.v1.ListWalletsResponse.Wallet.prototype.setDid = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * repeated Wallet wallets = 1;
 * @return {!Array<!proto.wallet_api.v1.ListWalletsResponse.Wallet>}
 */
proto.wallet_api.v1.ListWalletsResponse.prototype.getWalletsList = function() {
  return /** @type{!Array<!proto.wallet_api.v1.ListWalletsResponse.Wallet>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.wallet_api.v1.ListWalletsResponse.Wallet, 1));
};


/**
 * @param {!Array<!proto.wallet_api.v1.ListWalletsResponse.Wallet>} value
 * @return {!proto.wallet_api.v1.ListWalletsResponse} returns this
*/
proto.wallet_api.v1.ListWalletsResponse.prototype.setWalletsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.wallet_api.v1.ListWalletsResponse.Wallet=} opt_value
 * @param {number=} opt_index
 * @return {!proto.wallet_api.v1.ListWalletsResponse.Wallet}
 */
proto.wallet_api.v1.ListWalletsResponse.prototype.addWallets = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.wallet_api.v1.ListWalletsResponse.Wallet, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.wallet_api.v1.ListWalletsResponse} returns this
 */
proto.wallet_api.v1.ListWalletsResponse.prototype.clearWalletsList = function() {
  return this.setWalletsList([]);
};


/**
 * optional uint32 total_owned_wallets = 2;
 * @return {number}
 */
proto.wallet_api.v1.ListWalletsResponse.prototype.getTotalOwnedWallets = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.wallet_api.v1.ListWalletsResponse} returns this
 */
proto.wallet_api.v1.ListWalletsResponse.prototype.setTotalOwnedWallets = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


