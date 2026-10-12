use crate::model::Role;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct ConversationHistory {
    messages: Vec<Message>,
}

impl ConversationHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn get_all(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl Message {
    pub fn system(content: &str) -> Self {
        Message {
            role: Role::System,
            content: content.to_owned(),
        }
    }

    pub fn user(content: &str) -> Self {
        Message {
            role: Role::User,
            content: content.to_owned(),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Message {
            role: Role::Assistant,
            content: content.to_owned(),
        }
    }

    pub fn tool(content: &str) -> Self {
        Message {
            role: Role::Tool,
            content: content.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_flow() {
        let mut history = ConversationHistory::new();

        history.push(Message::system(
            "你是一个城市生活助手智能体，可以调用天气查询工具。",
        ));
        assert_eq!(1, history.len());
        assert_eq!(Role::System, history.messages[0].role);
        assert_eq!(
            "你是一个城市生活助手智能体，可以调用天气查询工具。",
            history.messages[0].content
        );

        history.push(Message::user("帮我查一下北京今天的天气，要不要带伞？"));
        assert_eq!(2, history.len());
        assert_eq!(Role::User, history.messages[1].role);
        assert_eq!(
            "帮我查一下北京今天的天气，要不要带伞？",
            history.messages[1].content
        );
    }
}
