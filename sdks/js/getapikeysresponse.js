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

goog.provide('proto.user_api.v1.GetApiKeysResponse');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.user_api.v1.ApiKeyEntity');

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
proto.user_api.v1.GetApiKeysResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.user_api.v1.GetApiKeysResponse.repeatedFields_, null);
};
goog.inherits(proto.user_api.v1.GetApiKeysResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.user_api.v1.GetApiKeysResponse.displayName = 'proto.user_api.v1.GetApiKeysResponse';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.user_api.v1.GetApiKeysResponse.repeatedFields_ = [1];



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
proto.user_api.v1.GetApiKeysResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.user_api.v1.GetApiKeysResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.user_api.v1.GetApiKeysResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.GetApiKeysResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    apiKeyEntitiesList: jspb.Message.toObjectList(msg.getApiKeyEntitiesList(),
    proto.user_api.v1.ApiKeyEntity.toObject, includeInstance)
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
 * @return {!proto.user_api.v1.GetApiKeysResponse}
 */
proto.user_api.v1.GetApiKeysResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.user_api.v1.GetApiKeysResponse;
  return proto.user_api.v1.GetApiKeysResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.user_api.v1.GetApiKeysResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.user_api.v1.GetApiKeysResponse}
 */
proto.user_api.v1.GetApiKeysResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.user_api.v1.ApiKeyEntity;
      reader.readMessage(value,proto.user_api.v1.ApiKeyEntity.deserializeBinaryFromReader);
      msg.addApiKeyEntities(value);
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
proto.user_api.v1.GetApiKeysResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.user_api.v1.GetApiKeysResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.user_api.v1.GetApiKeysResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.GetApiKeysResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApiKeyEntitiesList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.user_api.v1.ApiKeyEntity.serializeBinaryToWriter
    );
  }
};


/**
 * repeated ApiKeyEntity api_key_entities = 1;
 * @return {!Array<!proto.user_api.v1.ApiKeyEntity>}
 */
proto.user_api.v1.GetApiKeysResponse.prototype.getApiKeyEntitiesList = function() {
  return /** @type{!Array<!proto.user_api.v1.ApiKeyEntity>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.user_api.v1.ApiKeyEntity, 1));
};


/**
 * @param {!Array<!proto.user_api.v1.ApiKeyEntity>} value
 * @return {!proto.user_api.v1.GetApiKeysResponse} returns this
*/
proto.user_api.v1.GetApiKeysResponse.prototype.setApiKeyEntitiesList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.user_api.v1.ApiKeyEntity=} opt_value
 * @param {number=} opt_index
 * @return {!proto.user_api.v1.ApiKeyEntity}
 */
proto.user_api.v1.GetApiKeysResponse.prototype.addApiKeyEntities = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.user_api.v1.ApiKeyEntity, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.user_api.v1.GetApiKeysResponse} returns this
 */
proto.user_api.v1.GetApiKeysResponse.prototype.clearApiKeyEntitiesList = function() {
  return this.setApiKeyEntitiesList([]);
};


