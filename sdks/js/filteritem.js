// source: common/v1/common.proto
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

goog.provide('proto.common.FilterItem');
goog.provide('proto.common.FilterItem.FilterOperator');
goog.provide('proto.common.FilterItem.ValueCase');

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
proto.common.FilterItem = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.common.FilterItem.oneofGroups_);
};
goog.inherits(proto.common.FilterItem, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.common.FilterItem.displayName = 'proto.common.FilterItem';
}

/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.common.FilterItem.oneofGroups_ = [[3,4,5,6,7]];

/**
 * @enum {number}
 */
proto.common.FilterItem.ValueCase = {
  VALUE_NOT_SET: 0,
  STR_VALUE: 3,
  INT_VALUE: 4,
  BOOL_VALUE: 5,
  RFC3339_DATETIME_VALUE: 6,
  UUID_VALUE: 7
};

/**
 * @return {proto.common.FilterItem.ValueCase}
 */
proto.common.FilterItem.prototype.getValueCase = function() {
  return /** @type {proto.common.FilterItem.ValueCase} */(jspb.Message.computeOneofCase(this, proto.common.FilterItem.oneofGroups_[0]));
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
proto.common.FilterItem.prototype.toObject = function(opt_includeInstance) {
  return proto.common.FilterItem.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.common.FilterItem} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.FilterItem.toObject = function(includeInstance, msg) {
  var f, obj = {
    field: jspb.Message.getFieldWithDefault(msg, 1, ""),
    operator: jspb.Message.getFieldWithDefault(msg, 2, 0),
    strValue: jspb.Message.getFieldWithDefault(msg, 3, ""),
    intValue: jspb.Message.getFieldWithDefault(msg, 4, 0),
    boolValue: jspb.Message.getBooleanFieldWithDefault(msg, 5, false),
    rfc3339DatetimeValue: jspb.Message.getFieldWithDefault(msg, 6, ""),
    uuidValue: jspb.Message.getFieldWithDefault(msg, 7, "")
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
 * @return {!proto.common.FilterItem}
 */
proto.common.FilterItem.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.common.FilterItem;
  return proto.common.FilterItem.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.common.FilterItem} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.common.FilterItem}
 */
proto.common.FilterItem.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setField(value);
      break;
    case 2:
      var value = /** @type {!proto.common.FilterItem.FilterOperator} */ (reader.readEnum());
      msg.setOperator(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setStrValue(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setIntValue(value);
      break;
    case 5:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setBoolValue(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setRfc3339DatetimeValue(value);
      break;
    case 7:
      var value = /** @type {string} */ (reader.readString());
      msg.setUuidValue(value);
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
proto.common.FilterItem.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.common.FilterItem.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.common.FilterItem} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.FilterItem.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getField();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getOperator();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 3));
  if (f != null) {
    writer.writeString(
      3,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 4));
  if (f != null) {
    writer.writeInt64(
      4,
      f
    );
  }
  f = /** @type {boolean} */ (jspb.Message.getField(message, 5));
  if (f != null) {
    writer.writeBool(
      5,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 6));
  if (f != null) {
    writer.writeString(
      6,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 7));
  if (f != null) {
    writer.writeString(
      7,
      f
    );
  }
};


/**
 * @enum {number}
 */
proto.common.FilterItem.FilterOperator = {
  FILTER_OPERATOR_UNSPECIFIED: 0,
  FILTER_OPERATOR_EQ: 1,
  FILTER_OPERATOR_NE: 2,
  FILTER_OPERATOR_GT: 3,
  FILTER_OPERATOR_GTE: 4,
  FILTER_OPERATOR_LT: 5,
  FILTER_OPERATOR_LTE: 6,
  FILTER_OPERATOR_IN: 7,
  FILTER_OPERATOR_NOT_IN: 8
};

/**
 * optional string field = 1;
 * @return {string}
 */
proto.common.FilterItem.prototype.getField = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.setField = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional FilterOperator operator = 2;
 * @return {!proto.common.FilterItem.FilterOperator}
 */
proto.common.FilterItem.prototype.getOperator = function() {
  return /** @type {!proto.common.FilterItem.FilterOperator} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.common.FilterItem.FilterOperator} value
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.setOperator = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional string str_value = 3;
 * @return {string}
 */
proto.common.FilterItem.prototype.getStrValue = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.setStrValue = function(value) {
  return jspb.Message.setOneofField(this, 3, proto.common.FilterItem.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.clearStrValue = function() {
  return jspb.Message.setOneofField(this, 3, proto.common.FilterItem.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.FilterItem.prototype.hasStrValue = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional int64 int_value = 4;
 * @return {number}
 */
proto.common.FilterItem.prototype.getIntValue = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.setIntValue = function(value) {
  return jspb.Message.setOneofField(this, 4, proto.common.FilterItem.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.clearIntValue = function() {
  return jspb.Message.setOneofField(this, 4, proto.common.FilterItem.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.FilterItem.prototype.hasIntValue = function() {
  return jspb.Message.getField(this, 4) != null;
};


/**
 * optional bool bool_value = 5;
 * @return {boolean}
 */
proto.common.FilterItem.prototype.getBoolValue = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 5, false));
};


/**
 * @param {boolean} value
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.setBoolValue = function(value) {
  return jspb.Message.setOneofField(this, 5, proto.common.FilterItem.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.clearBoolValue = function() {
  return jspb.Message.setOneofField(this, 5, proto.common.FilterItem.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.FilterItem.prototype.hasBoolValue = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional string rfc3339_datetime_value = 6;
 * @return {string}
 */
proto.common.FilterItem.prototype.getRfc3339DatetimeValue = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.setRfc3339DatetimeValue = function(value) {
  return jspb.Message.setOneofField(this, 6, proto.common.FilterItem.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.clearRfc3339DatetimeValue = function() {
  return jspb.Message.setOneofField(this, 6, proto.common.FilterItem.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.FilterItem.prototype.hasRfc3339DatetimeValue = function() {
  return jspb.Message.getField(this, 6) != null;
};


/**
 * optional string uuid_value = 7;
 * @return {string}
 */
proto.common.FilterItem.prototype.getUuidValue = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 7, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.setUuidValue = function(value) {
  return jspb.Message.setOneofField(this, 7, proto.common.FilterItem.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.common.FilterItem} returns this
 */
proto.common.FilterItem.prototype.clearUuidValue = function() {
  return jspb.Message.setOneofField(this, 7, proto.common.FilterItem.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.FilterItem.prototype.hasUuidValue = function() {
  return jspb.Message.getField(this, 7) != null;
};


