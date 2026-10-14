use chrono::{FixedOffset, Utc};
use serde_json::{Value, json};

use crate::tools::{base::ToolError, function::FunctionTool};

pub fn build_time_tool() -> FunctionTool {
    let schema = json!({
        "type": "object",
        "properties": {
            "timezone_offset_hours": {
                "type": "integer",
                "description": "相对于UTC的时区偏移小时数，例如北京时间填8，不填默认返回UTC时间"
            }
        },
        "required": []
    });

    FunctionTool::new(
        "get_current_time",
        "获取当前时间，可以指定时区偏移小时数",
        schema,
        |args: Value| async move {
            let offset_hours = args
                .get("timezone_offset_hours")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);

            if !(-12..=14).contains(&offset_hours) {
                return Err(ToolError::InvalidArguments(
                    "时区偏移小时数必须在 -12 到 14 之间".to_owned(),
                ));
            }

            let utc_now = Utc::now();
            let offset_seconds = (offset_hours * 3600) as i32;
            let offset = FixedOffset::east_opt(offset_seconds)
                .ok_or_else(|| ToolError::ExecutionFailed("时区偏移计算失败".to_string()))?;
            let local_time = utc_now.with_timezone(&offset);

            Ok(local_time.format("%Y-%m-%d %H:%M:%S").to_string())
        },
    )
}
