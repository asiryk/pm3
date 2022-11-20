use std::collections::HashMap;

struct Client {
    pub last_idx: usize,
}

/// Unique identifier to distinguish clients.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ClientId(pub u128);

/// Structure that allows getting logs for multiple clients with a single buffer.
pub struct LogBuffer {
    buffer: Vec<String>,
    clients: HashMap<ClientId, Client>,
}

// TODO: remove cliennts
// TODO: implement cleaninng function (lookup through clients, checks most early idx
// and removes everytihing before this idx)
impl LogBuffer {
    pub fn new() -> Self {
        return LogBuffer {
            buffer: vec![],
            clients: HashMap::new(),
        };
    }

    /// Get unread lines for the client and move its last index
    /// to the current buffer position to prevent logging the same place twice.
    /// If the client id was not present, creates new client.
    pub fn consume_unread(&mut self, id: &ClientId) -> Vec<String> {
        let buf_len = self.buffer.len();
        let mut client = self
            .clients
            .entry(*id)
            .or_insert_with(|| Client { last_idx: buf_len });

        let slice = &self.buffer[client.last_idx..buf_len];
        client.last_idx = buf_len;

        return slice.into();
    }

    pub fn write(&mut self, line: String) {
        self.buffer.push(line);
    }

    pub fn len_clients(&self) -> usize {
        return self.clients.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_with_no_clients() {
        let buffer = LogBuffer::new();

        assert_eq!(0, buffer.len_clients());
    }

    #[test]
    fn client_should_read_no_lines_if_empty() {
        // 1. Create buffer with empty Vec
        let mut buffer = LogBuffer::new();

        // 2. Create client
        let client_id = ClientId(0);
        let lines = buffer.consume_unread(&client_id);

        // 3. Assert empty Vec
        assert_eq!(Vec::<String>::new(), lines);
    }

    #[test]
    fn client_should_read_no_lines() {
        // 1. Create buffer with two lines
        let mut buffer = LogBuffer::new();
        buffer.write(String::from("line1"));
        buffer.write(String::from("line2"));

        // 2. Create client
        let client_id = ClientId(0);
        let lines = buffer.consume_unread(&client_id);

        // 3. Assert empty Vec
        assert_eq!(Vec::<String>::new(), lines);
    }

    #[test]
    fn client_should_read_multiple_lines() {
        // 1. Create buffer with empty vec
        let mut buffer = LogBuffer::new();

        // 2. Create client (read no lines)
        let client_id = ClientId(0);
        buffer.consume_unread(&client_id);

        // 3. Insert new lines into buffer
        let new_lines = vec![
            String::from("line1"),
            String::from("line2"),
            String::from("line3"),
        ];
        buffer.write(String::from("line1"));
        buffer.write(String::from("line2"));
        buffer.write(String::from("line3"));

        // 4. Assert client lines
        let client_lines = buffer.consume_unread(&client_id);

        assert_eq!(new_lines, client_lines);

        let new_lines = vec![String::from("line4"), String::from("line5")];

        buffer.write(String::from("line4"));
        buffer.write(String::from("line5"));

        let client_lines = buffer.consume_unread(&client_id);

        assert_eq!(new_lines, client_lines);
    }
}
