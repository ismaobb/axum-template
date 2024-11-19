use std::{convert::Infallible, time::Duration};

use axum::{
	extract::Query,
	response::{
		sse::{Event, KeepAlive},
		Sse,
	},
	Json,
};
use axum_extra::extract::WithRejection;
use futures::Stream;
use reqwest::header;
use tokio::time::sleep;
use tokio_stream::StreamExt;

use crate::core::ApiErr;

use super::dto::ConversationRequest;

// 定义一个异步函数，用于处理对话请求
pub async fn conversation_handler(
	// WithRejection(Json(conversation_request), _): WithRejection<Json<ConversationRequest>, ApiErr>,
	Query(conversation_request): Query<ConversationRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiErr> {
	// 调用fetch_ims_oa函数，获取消息流
	let stream = fetch_ims_oa(conversation_request)
		.await?
		.map(|message| Event::default().data(message))
		.map(Ok);

	// 返回Sse类型的消息流，并设置KeepAlive
	Ok(Sse::new(stream).keep_alive(KeepAlive::new()))
}

// 定义一个异步函数，用于获取消息流
async fn fetch_ims_oa(conversation_request: ConversationRequest) -> Result<impl Stream<Item = String>, ApiErr> {
	// 创建一个reqwest客户端
	let client = reqwest::Client::builder().timeout(Duration::from_secs(3)).build()?;
	// 发送GET请求，获取响应
	let response = client
		.get("http://192.168.20.111:3000")
		// .header(header::CONTENT_TYPE, "application/json")
		.send()
		.await?;
	// 获取响应的字节流
	let stream = response.bytes_stream().map(move |chunk| {
		// 将字节流转换为字符串
		let chunk = chunk.unwrap();
		let chunk_text = String::from_utf8_lossy(&chunk);
		// 打印日志
		tracing::info!("chunk_text: {}", chunk_text);
		// 返回字符串

		format!("prompt: {} chunk_text {}", conversation_request.prompt, chunk_text)
	});
	// 返回字节流
	Ok(stream)
}
