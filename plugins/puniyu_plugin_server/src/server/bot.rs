use actix_ws::{Message, Session};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures_util::StreamExt;

//todo: 完善注册的连接
static CONNECTIONS: LazyLock<Arc<RwLock<HashMap<String, Session>>>> = LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));
pub async fn ws_handler(
	req: HttpRequest,
	body: web::Payload,
	path: web::Path<String>,
) -> Result<HttpResponse, Error> {
	
	let bot_id = path.into_inner();
	println!("Bot {} 正在连接...", bot_id);
	
	let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;
	
	{
		let mut conns = CONNECTIONS.write().unwrap();
		conns.insert(bot_id.clone(), session);
	}
	
	println!("Bot {} 已连接", bot_id);
	let bot_id_clone = bot_id.clone();
	actix_web::rt::spawn(async move {
		while let Some(Ok(msg)) = msg_stream.next().await {
			match msg {
				Message::Text(text) => {
					println!("[Bot {}] 收到: {}", bot_id_clone, text);
				}
				Message::Close(reason) => {
					println!("[Bot {}] 断开连接: {:?}", bot_id_clone, reason);
					let mut conns = CONNECTIONS.write().unwrap();
					conns.remove(&bot_id_clone);
					break;
				}
				_ => {}
			}
		}
		let mut conns = CONNECTIONS.write().unwrap();
		conns.remove(&bot_id_clone);
	});
	
	Ok(response)
}