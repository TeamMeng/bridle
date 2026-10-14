use serde_json::{Value, json};

use crate::tools::{base::ToolError, function::FunctionTool};

/// 词法单元：把表达式字符串拆分成的最小单位
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, ToolError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\t' => {
                i += 1;
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LeftParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                i += 1;
            }
            c if c.is_ascii_digit() || c == '.' => {
                // 连续读取数字字符，直到遇到非数字非小数点字符为止
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let number_str: String = chars[start..i].iter().collect();
                let number = number_str.parse::<f64>().map_err(|_| {
                    ToolError::InvalidArguments(format!("无法解析数字: {}", number_str))
                })?;
                tokens.push(Token::Number(number));
            }
            other => {
                return Err(ToolError::InvalidArguments(format!(
                    "表达式中存在无法识别的字符: {}",
                    other
                )));
            }
        }
    }
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }

    /// 处理最高层级：加法和减法
    fn parse_expr(&mut self) -> Result<f64, ToolError> {
        let mut result = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.advance();
                    result += self.parse_term()?;
                }
                Some(Token::Minus) => {
                    self.advance();
                    result -= self.parse_term()?;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    /// 处理中间层级：乘法和除法，优先级高于加减
    fn parse_term(&mut self) -> Result<f64, ToolError> {
        let mut result = self.parse_factor()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.advance();
                    result *= self.parse_factor()?;
                }
                Some(Token::Slash) => {
                    self.advance();
                    let divisor = self.parse_factor()?;
                    if divisor == 0.0 {
                        return Err(ToolError::ExecutionFailed("除数不能为零".to_string()));
                    }
                    result /= divisor;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    /// 处理最底层：单个数字或者括号包裹的子表达式
    fn parse_factor(&mut self) -> Result<f64, ToolError> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(n),
            Some(Token::LeftParen) => {
                let result = self.parse_expr()?;
                match self.advance() {
                    Some(Token::RightParen) => Ok(result),
                    _ => Err(ToolError::InvalidArguments("缺少右括号".to_string())),
                }
            }
            Some(Token::Minus) => {
                // 处理负号，比如 -5 这种写法
                let value = self.parse_factor()?;
                Ok(-value)
            }
            other => Err(ToolError::InvalidArguments(format!(
                "表达式格式错误，位置: {:?}",
                other
            ))),
        }
    }
}

/// 对外暴露的求值入口函数
fn evaluate_expression(expression: &str) -> Result<f64, ToolError> {
    let tokens = tokenize(expression)?;
    let mut parser = Parser::new(tokens);
    parser.parse_expr()
}

/// 构造计算器工具实例
pub fn build_calculator_tool() -> FunctionTool {
    let schema = json!({
        "type": "object",
        "properties": {
            "expression": {
                "type": "string",
                "description": "需要计算的算术表达式，支持加减乘除和括号，例如 (3 + 4) * 2"
            }
        },
        "required": ["expression"]
    });

    FunctionTool::new(
        "calculator",
        "计算一个算术表达式的结果，支持加减乘除和括号",
        schema,
        |args: Value| async move {
            let expression = args["expression"]
                .as_str()
                .ok_or_else(|| ToolError::InvalidArguments("缺少参数 expression".to_string()))?;
            let result = evaluate_expression(expression)?;
            Ok(result.to_string())
        },
    )
}
