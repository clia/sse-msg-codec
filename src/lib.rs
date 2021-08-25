//! Encode and Decode SSE Messages

const ID_HEAD: &str = "id: ";
const EVENT_HEAD: &str = "event: ";
const DATA_HEAD: &str = "data: ";

/// Parsed SSE Message struct
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SseMsg {
    pub id: Option<String>,
    pub event: Option<String>,
    pub data: Option<String>,
}

/// Encode a SSE message
pub fn encode(id: Option<&str>, event: Option<&str>, data: Option<&str>) -> String {
    let mut result = String::new();

    if let Some(id) = id {
        result.push_str("id: ");
        result.push_str(id);
        result.push_str("\n");
    }
    if let Some(event) = event {
        result.push_str("event: ");
        result.push_str(event);
        result.push_str("\n");
    }
    if let Some(data) = data {
        let processed = data.replace("\n", "\ndata: ");
        result.push_str("data: ");
        result.push_str(&processed);
        result.push_str("\n");
    }
    result.push_str("\n");
    result
}

/// Decode a SSE message
pub fn decode(msg: &str) -> SseMsg {
    let mut sse_msg = SseMsg {
        id: None,
        event: None,
        data: None,
    };
    
    let v: Vec<&str> = msg.split('\n').collect();

    let mut data = String::new();
    
    for line in v {
        if line.starts_with(ID_HEAD) {
            sse_msg.id = Some(line[ID_HEAD.len()..].to_string());
        } else if line.starts_with("event: ") {
            sse_msg.event = Some(line[EVENT_HEAD.len()..].to_string());
        } else if line.starts_with("data: ") {
            if data.len() > 0 {
                data.push_str("\n");
            }
            data.push_str(&line[DATA_HEAD.len()..]);
        }
    }

    if data.len() > 0 {
        sse_msg.data = Some(data);
    }

    sse_msg
}

#[cfg(test)]
mod tests {
    use super::SseMsg;

    #[test]
    fn test_encode() {
        let encoded = super::encode(Some("test-id"), Some("test-event"), Some("test-data line1\nline2"));
        assert_eq!(encoded, "id: test-id\nevent: test-event\ndata: test-data line1\ndata: line2\n\n".to_owned());
    }

    #[test]
    fn test_decode() {
        let msg = "id: test-id\nevent: test-event\ndata: test-data line1\ndata: line2\n\n";
        let decoded = super::decode(msg);
        assert_eq!(decoded, SseMsg {
            id: Some("test-id".to_owned()),
            event: Some("test-event".to_owned()),
            data: Some("test-data line1\nline2".to_owned()),
        });
    }
}
