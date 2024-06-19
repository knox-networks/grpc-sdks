// source: google/api/http.proto
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

goog.provide('proto.google.api.Http');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.google.api.HttpRule');

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
proto.google.api.Http = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.google.api.Http.repeatedFields_, null);
};
goog.inherits(proto.google.api.Http, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.google.api.Http.displayName = 'proto.google.api.Http';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.google.api.Http.repeatedFields_ = [1];



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
proto.google.api.Http.prototype.toObject = function(opt_includeInstance) {
  return proto.google.api.Http.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.google.api.Http} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.google.api.Http.toObject = function(includeInstance, msg) {
  var f, obj = {
    rulesList: jspb.Message.toObjectList(msg.getRulesList(),
    proto.google.api.HttpRule.toObject, includeInstance),
    fullyDecodeReservedExpansion: jspb.Message.getBooleanFieldWithDefault(msg, 2, false)
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
 * @return {!proto.google.api.Http}
 */
proto.google.api.Http.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.google.api.Http;
  return proto.google.api.Http.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.google.api.Http} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.google.api.Http}
 */
proto.google.api.Http.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.google.api.HttpRule;
      reader.readMessage(value,proto.google.api.HttpRule.deserializeBinaryFromReader);
      msg.addRules(value);
      break;
    case 2:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setFullyDecodeReservedExpansion(value);
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
proto.google.api.Http.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.google.api.Http.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.google.api.Http} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.google.api.Http.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getRulesList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.google.api.HttpRule.serializeBinaryToWriter
    );
  }
  f = message.getFullyDecodeReservedExpansion();
  if (f) {
    writer.writeBool(
      2,
      f
    );
  }
};


/**
 * repeated HttpRule rules = 1;
 * @return {!Array<!proto.google.api.HttpRule>}
 */
proto.google.api.Http.prototype.getRulesList = function() {
  return /** @type{!Array<!proto.google.api.HttpRule>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.google.api.HttpRule, 1));
};


/**
 * @param {!Array<!proto.google.api.HttpRule>} value
 * @return {!proto.google.api.Http} returns this
*/
proto.google.api.Http.prototype.setRulesList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.google.api.HttpRule=} opt_value
 * @param {number=} opt_index
 * @return {!proto.google.api.HttpRule}
 */
proto.google.api.Http.prototype.addRules = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.google.api.HttpRule, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.google.api.Http} returns this
 */
proto.google.api.Http.prototype.clearRulesList = function() {
  return this.setRulesList([]);
};


/**
 * optional bool fully_decode_reserved_expansion = 2;
 * @return {boolean}
 */
proto.google.api.Http.prototype.getFullyDecodeReservedExpansion = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 2, false));
};


/**
 * @param {boolean} value
 * @return {!proto.google.api.Http} returns this
 */
proto.google.api.Http.prototype.setFullyDecodeReservedExpansion = function(value) {
  return jspb.Message.setProto3BooleanField(this, 2, value);
};

