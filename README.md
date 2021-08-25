# sse-msg-codec

A Rust lib to encode and decode the SSE (Server-Sent Events) protocol message.

## Protocol Referer

See: [https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format)

## Usage

Encode:

```Rust
let encoded = sse_msg_codec::encode(
    Some("test-id"), 
    Some("test-event"), 
    Some("test-data line1\nline2"),
);
assert_eq!(
    encoded, 
    "id: test-id\nevent: test-event\ndata: test-data line1\ndata: line2\n\n".to_owned(),
);
```

Decode: 

```Rust
let msg = "id: test-id\nevent: test-event\ndata: test-data line1\ndata: line2\n\n";
let decoded = sse_msg_codec::decode(msg);
assert_eq!(decoded, SseMsg {
    id: Some("test-id".to_owned()),
    event: Some("test-event".to_owned()),
    data: Some("test-data line1\nline2".to_owned()),
});
```
