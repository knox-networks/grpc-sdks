// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: google/protobuf/timestamp.proto
// Protobuf Java Version: 4.27.1

package com.google.protobuf;

/**
 * <pre>
 * A Timestamp represents a point in time independent of any time zone or local
 * calendar, encoded as a count of seconds and fractions of seconds at
 * nanosecond resolution. The count is relative to an epoch at UTC midnight on
 * January 1, 1970, in the proleptic Gregorian calendar which extends the
 * Gregorian calendar backwards to year one.
 *
 * All minutes are 60 seconds long. Leap seconds are "smeared" so that no leap
 * second table is needed for interpretation, using a [24-hour linear
 * smear](https://developers.google.com/time/smear).
 *
 * The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By
 * restricting to that range, we ensure that we can convert to and from [RFC
 * 3339](https://www.ietf.org/rfc/rfc3339.txt) date strings.
 *
 * # Examples
 *
 * Example 1: Compute Timestamp from POSIX `time()`.
 *
 * Timestamp timestamp;
 * timestamp.set_seconds(time(NULL));
 * timestamp.set_nanos(0);
 *
 * Example 2: Compute Timestamp from POSIX `gettimeofday()`.
 *
 * struct timeval tv;
 * gettimeofday(&amp;tv, NULL);
 *
 * Timestamp timestamp;
 * timestamp.set_seconds(tv.tv_sec);
 * timestamp.set_nanos(tv.tv_usec * 1000);
 *
 * Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.
 *
 * FILETIME ft;
 * GetSystemTimeAsFileTime(&amp;ft);
 * UINT64 ticks = (((UINT64)ft.dwHighDateTime) &lt;&lt; 32) | ft.dwLowDateTime;
 *
 * // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
 * // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
 * Timestamp timestamp;
 * timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
 * timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
 *
 * Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.
 *
 * long millis = System.currentTimeMillis();
 *
 * Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
 * .setNanos((int) ((millis % 1000) * 1000000)).build();
 *
 * Example 5: Compute Timestamp from Java `Instant.now()`.
 *
 * Instant now = Instant.now();
 *
 * Timestamp timestamp =
 * Timestamp.newBuilder().setSeconds(now.getEpochSecond())
 * .setNanos(now.getNano()).build();
 *
 * Example 6: Compute Timestamp from current time in Python.
 *
 * timestamp = Timestamp()
 * timestamp.GetCurrentTime()
 *
 * # JSON Mapping
 *
 * In JSON format, the Timestamp type is encoded as a string in the
 * [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the
 * format is "{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z"
 * where {year} is always expressed using four digits while {month}, {day},
 * {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
 * seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
 * are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
 * is required. A proto3 JSON serializer should always use UTC (as indicated by
 * "Z") when printing the Timestamp type and a proto3 JSON parser should be
 * able to accept both UTC and other timezones (as indicated by an offset).
 *
 * For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
 * 01:30 UTC on January 15, 2017.
 *
 * In JavaScript, one can convert a Date object to this format using the
 * standard
 * [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
 * method. In Python, a standard `datetime.datetime` object can be converted
 * to this format using
 * [`strftime`](https://docs.python.org/2/library/time.html#time.strftime) with
 * the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one can use
 * the Joda Time's [`ISODateTimeFormat.dateTime()`](
 * http://joda-time.sourceforge.net/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime()
 * ) to obtain a formatter capable of generating timestamps in this format.
 * </pre>
 *
 * Protobuf type {@code google.protobuf.Timestamp}
 */
public final class Timestamp extends
    com.google.protobuf.GeneratedMessage implements
    // @@protoc_insertion_point(message_implements:google.protobuf.Timestamp)
    TimestampOrBuilder {
private static final long serialVersionUID = 0L;
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 27,
      /* patch= */ 1,
      /* suffix= */ "",
      Timestamp.class.getName());
  }
  // Use Timestamp.newBuilder() to construct.
  private Timestamp(com.google.protobuf.GeneratedMessage.Builder<?> builder) {
    super(builder);
  }
  private Timestamp() {
  }

  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return com.google.protobuf.TimestampProto.internal_static_google_protobuf_Timestamp_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return com.google.protobuf.TimestampProto.internal_static_google_protobuf_Timestamp_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            com.google.protobuf.Timestamp.class, com.google.protobuf.Timestamp.Builder.class);
  }

  public static final int SECONDS_FIELD_NUMBER = 1;
  private long seconds_ = 0L;
  /**
   * <pre>
   * Represents seconds of UTC time since Unix epoch
   * 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
   * 9999-12-31T23:59:59Z inclusive.
   * </pre>
   *
   * <code>int64 seconds = 1;</code>
   * @return The seconds.
   */
  @java.lang.Override
  public long getSeconds() {
    return seconds_;
  }

  public static final int NANOS_FIELD_NUMBER = 2;
  private int nanos_ = 0;
  /**
   * <pre>
   * Non-negative fractions of a second at nanosecond resolution. Negative
   * second values with fractions must still have non-negative nanos values
   * that count forward in time. Must be from 0 to 999,999,999
   * inclusive.
   * </pre>
   *
   * <code>int32 nanos = 2;</code>
   * @return The nanos.
   */
  @java.lang.Override
  public int getNanos() {
    return nanos_;
  }

  private byte memoizedIsInitialized = -1;
  @java.lang.Override
  public final boolean isInitialized() {
    byte isInitialized = memoizedIsInitialized;
    if (isInitialized == 1) return true;
    if (isInitialized == 0) return false;

    memoizedIsInitialized = 1;
    return true;
  }

  @java.lang.Override
  public void writeTo(com.google.protobuf.CodedOutputStream output)
                      throws java.io.IOException {
    if (seconds_ != 0L) {
      output.writeInt64(1, seconds_);
    }
    if (nanos_ != 0) {
      output.writeInt32(2, nanos_);
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (seconds_ != 0L) {
      size += com.google.protobuf.CodedOutputStream
        .computeInt64Size(1, seconds_);
    }
    if (nanos_ != 0) {
      size += com.google.protobuf.CodedOutputStream
        .computeInt32Size(2, nanos_);
    }
    size += getUnknownFields().getSerializedSize();
    memoizedSize = size;
    return size;
  }

  @java.lang.Override
  public boolean equals(final java.lang.Object obj) {
    if (obj == this) {
     return true;
    }
    if (!(obj instanceof com.google.protobuf.Timestamp)) {
      return super.equals(obj);
    }
    com.google.protobuf.Timestamp other = (com.google.protobuf.Timestamp) obj;

    if (getSeconds()
        != other.getSeconds()) return false;
    if (getNanos()
        != other.getNanos()) return false;
    if (!getUnknownFields().equals(other.getUnknownFields())) return false;
    return true;
  }

  @java.lang.Override
  public int hashCode() {
    if (memoizedHashCode != 0) {
      return memoizedHashCode;
    }
    int hash = 41;
    hash = (19 * hash) + getDescriptor().hashCode();
    hash = (37 * hash) + SECONDS_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashLong(
        getSeconds());
    hash = (37 * hash) + NANOS_FIELD_NUMBER;
    hash = (53 * hash) + getNanos();
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static com.google.protobuf.Timestamp parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static com.google.protobuf.Timestamp parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static com.google.protobuf.Timestamp parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static com.google.protobuf.Timestamp parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static com.google.protobuf.Timestamp parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static com.google.protobuf.Timestamp parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static com.google.protobuf.Timestamp parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static com.google.protobuf.Timestamp parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public static com.google.protobuf.Timestamp parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input);
  }

  public static com.google.protobuf.Timestamp parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static com.google.protobuf.Timestamp parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static com.google.protobuf.Timestamp parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  @java.lang.Override
  public Builder newBuilderForType() { return newBuilder(); }
  public static Builder newBuilder() {
    return DEFAULT_INSTANCE.toBuilder();
  }
  public static Builder newBuilder(com.google.protobuf.Timestamp prototype) {
    return DEFAULT_INSTANCE.toBuilder().mergeFrom(prototype);
  }
  @java.lang.Override
  public Builder toBuilder() {
    return this == DEFAULT_INSTANCE
        ? new Builder() : new Builder().mergeFrom(this);
  }

  @java.lang.Override
  protected Builder newBuilderForType(
      com.google.protobuf.GeneratedMessage.BuilderParent parent) {
    Builder builder = new Builder(parent);
    return builder;
  }
  /**
   * <pre>
   * A Timestamp represents a point in time independent of any time zone or local
   * calendar, encoded as a count of seconds and fractions of seconds at
   * nanosecond resolution. The count is relative to an epoch at UTC midnight on
   * January 1, 1970, in the proleptic Gregorian calendar which extends the
   * Gregorian calendar backwards to year one.
   *
   * All minutes are 60 seconds long. Leap seconds are "smeared" so that no leap
   * second table is needed for interpretation, using a [24-hour linear
   * smear](https://developers.google.com/time/smear).
   *
   * The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By
   * restricting to that range, we ensure that we can convert to and from [RFC
   * 3339](https://www.ietf.org/rfc/rfc3339.txt) date strings.
   *
   * # Examples
   *
   * Example 1: Compute Timestamp from POSIX `time()`.
   *
   * Timestamp timestamp;
   * timestamp.set_seconds(time(NULL));
   * timestamp.set_nanos(0);
   *
   * Example 2: Compute Timestamp from POSIX `gettimeofday()`.
   *
   * struct timeval tv;
   * gettimeofday(&amp;tv, NULL);
   *
   * Timestamp timestamp;
   * timestamp.set_seconds(tv.tv_sec);
   * timestamp.set_nanos(tv.tv_usec * 1000);
   *
   * Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.
   *
   * FILETIME ft;
   * GetSystemTimeAsFileTime(&amp;ft);
   * UINT64 ticks = (((UINT64)ft.dwHighDateTime) &lt;&lt; 32) | ft.dwLowDateTime;
   *
   * // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
   * // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
   * Timestamp timestamp;
   * timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
   * timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
   *
   * Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.
   *
   * long millis = System.currentTimeMillis();
   *
   * Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
   * .setNanos((int) ((millis % 1000) * 1000000)).build();
   *
   * Example 5: Compute Timestamp from Java `Instant.now()`.
   *
   * Instant now = Instant.now();
   *
   * Timestamp timestamp =
   * Timestamp.newBuilder().setSeconds(now.getEpochSecond())
   * .setNanos(now.getNano()).build();
   *
   * Example 6: Compute Timestamp from current time in Python.
   *
   * timestamp = Timestamp()
   * timestamp.GetCurrentTime()
   *
   * # JSON Mapping
   *
   * In JSON format, the Timestamp type is encoded as a string in the
   * [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the
   * format is "{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z"
   * where {year} is always expressed using four digits while {month}, {day},
   * {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
   * seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
   * are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
   * is required. A proto3 JSON serializer should always use UTC (as indicated by
   * "Z") when printing the Timestamp type and a proto3 JSON parser should be
   * able to accept both UTC and other timezones (as indicated by an offset).
   *
   * For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
   * 01:30 UTC on January 15, 2017.
   *
   * In JavaScript, one can convert a Date object to this format using the
   * standard
   * [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
   * method. In Python, a standard `datetime.datetime` object can be converted
   * to this format using
   * [`strftime`](https://docs.python.org/2/library/time.html#time.strftime) with
   * the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one can use
   * the Joda Time's [`ISODateTimeFormat.dateTime()`](
   * http://joda-time.sourceforge.net/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime()
   * ) to obtain a formatter capable of generating timestamps in this format.
   * </pre>
   *
   * Protobuf type {@code google.protobuf.Timestamp}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessage.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:google.protobuf.Timestamp)
      com.google.protobuf.TimestampOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return com.google.protobuf.TimestampProto.internal_static_google_protobuf_Timestamp_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return com.google.protobuf.TimestampProto.internal_static_google_protobuf_Timestamp_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              com.google.protobuf.Timestamp.class, com.google.protobuf.Timestamp.Builder.class);
    }

    // Construct using com.google.protobuf.Timestamp.newBuilder()
    private Builder() {

    }

    private Builder(
        com.google.protobuf.GeneratedMessage.BuilderParent parent) {
      super(parent);

    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      bitField0_ = 0;
      seconds_ = 0L;
      nanos_ = 0;
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return com.google.protobuf.TimestampProto.internal_static_google_protobuf_Timestamp_descriptor;
    }

    @java.lang.Override
    public com.google.protobuf.Timestamp getDefaultInstanceForType() {
      return com.google.protobuf.Timestamp.getDefaultInstance();
    }

    @java.lang.Override
    public com.google.protobuf.Timestamp build() {
      com.google.protobuf.Timestamp result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public com.google.protobuf.Timestamp buildPartial() {
      com.google.protobuf.Timestamp result = new com.google.protobuf.Timestamp(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(com.google.protobuf.Timestamp result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.seconds_ = seconds_;
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.nanos_ = nanos_;
      }
    }

    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof com.google.protobuf.Timestamp) {
        return mergeFrom((com.google.protobuf.Timestamp)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(com.google.protobuf.Timestamp other) {
      if (other == com.google.protobuf.Timestamp.getDefaultInstance()) return this;
      if (other.getSeconds() != 0L) {
        setSeconds(other.getSeconds());
      }
      if (other.getNanos() != 0) {
        setNanos(other.getNanos());
      }
      this.mergeUnknownFields(other.getUnknownFields());
      onChanged();
      return this;
    }

    @java.lang.Override
    public final boolean isInitialized() {
      return true;
    }

    @java.lang.Override
    public Builder mergeFrom(
        com.google.protobuf.CodedInputStream input,
        com.google.protobuf.ExtensionRegistryLite extensionRegistry)
        throws java.io.IOException {
      if (extensionRegistry == null) {
        throw new java.lang.NullPointerException();
      }
      try {
        boolean done = false;
        while (!done) {
          int tag = input.readTag();
          switch (tag) {
            case 0:
              done = true;
              break;
            case 8: {
              seconds_ = input.readInt64();
              bitField0_ |= 0x00000001;
              break;
            } // case 8
            case 16: {
              nanos_ = input.readInt32();
              bitField0_ |= 0x00000002;
              break;
            } // case 16
            default: {
              if (!super.parseUnknownField(input, extensionRegistry, tag)) {
                done = true; // was an endgroup tag
              }
              break;
            } // default:
          } // switch (tag)
        } // while (!done)
      } catch (com.google.protobuf.InvalidProtocolBufferException e) {
        throw e.unwrapIOException();
      } finally {
        onChanged();
      } // finally
      return this;
    }
    private int bitField0_;

    private long seconds_ ;
    /**
     * <pre>
     * Represents seconds of UTC time since Unix epoch
     * 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
     * 9999-12-31T23:59:59Z inclusive.
     * </pre>
     *
     * <code>int64 seconds = 1;</code>
     * @return The seconds.
     */
    @java.lang.Override
    public long getSeconds() {
      return seconds_;
    }
    /**
     * <pre>
     * Represents seconds of UTC time since Unix epoch
     * 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
     * 9999-12-31T23:59:59Z inclusive.
     * </pre>
     *
     * <code>int64 seconds = 1;</code>
     * @param value The seconds to set.
     * @return This builder for chaining.
     */
    public Builder setSeconds(long value) {

      seconds_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Represents seconds of UTC time since Unix epoch
     * 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
     * 9999-12-31T23:59:59Z inclusive.
     * </pre>
     *
     * <code>int64 seconds = 1;</code>
     * @return This builder for chaining.
     */
    public Builder clearSeconds() {
      bitField0_ = (bitField0_ & ~0x00000001);
      seconds_ = 0L;
      onChanged();
      return this;
    }

    private int nanos_ ;
    /**
     * <pre>
     * Non-negative fractions of a second at nanosecond resolution. Negative
     * second values with fractions must still have non-negative nanos values
     * that count forward in time. Must be from 0 to 999,999,999
     * inclusive.
     * </pre>
     *
     * <code>int32 nanos = 2;</code>
     * @return The nanos.
     */
    @java.lang.Override
    public int getNanos() {
      return nanos_;
    }
    /**
     * <pre>
     * Non-negative fractions of a second at nanosecond resolution. Negative
     * second values with fractions must still have non-negative nanos values
     * that count forward in time. Must be from 0 to 999,999,999
     * inclusive.
     * </pre>
     *
     * <code>int32 nanos = 2;</code>
     * @param value The nanos to set.
     * @return This builder for chaining.
     */
    public Builder setNanos(int value) {

      nanos_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Non-negative fractions of a second at nanosecond resolution. Negative
     * second values with fractions must still have non-negative nanos values
     * that count forward in time. Must be from 0 to 999,999,999
     * inclusive.
     * </pre>
     *
     * <code>int32 nanos = 2;</code>
     * @return This builder for chaining.
     */
    public Builder clearNanos() {
      bitField0_ = (bitField0_ & ~0x00000002);
      nanos_ = 0;
      onChanged();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:google.protobuf.Timestamp)
  }

  // @@protoc_insertion_point(class_scope:google.protobuf.Timestamp)
  private static final com.google.protobuf.Timestamp DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new com.google.protobuf.Timestamp();
  }

  public static com.google.protobuf.Timestamp getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<Timestamp>
      PARSER = new com.google.protobuf.AbstractParser<Timestamp>() {
    @java.lang.Override
    public Timestamp parsePartialFrom(
        com.google.protobuf.CodedInputStream input,
        com.google.protobuf.ExtensionRegistryLite extensionRegistry)
        throws com.google.protobuf.InvalidProtocolBufferException {
      Builder builder = newBuilder();
      try {
        builder.mergeFrom(input, extensionRegistry);
      } catch (com.google.protobuf.InvalidProtocolBufferException e) {
        throw e.setUnfinishedMessage(builder.buildPartial());
      } catch (com.google.protobuf.UninitializedMessageException e) {
        throw e.asInvalidProtocolBufferException().setUnfinishedMessage(builder.buildPartial());
      } catch (java.io.IOException e) {
        throw new com.google.protobuf.InvalidProtocolBufferException(e)
            .setUnfinishedMessage(builder.buildPartial());
      }
      return builder.buildPartial();
    }
  };

  public static com.google.protobuf.Parser<Timestamp> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<Timestamp> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Timestamp getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

