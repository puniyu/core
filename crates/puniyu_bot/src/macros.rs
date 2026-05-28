/// 创建并注册机器人。
///
#[macro_export]
macro_rules! register_bot {
    (bot: $bot:expr $(,)?) => {{
        let bot: ::std::sync::Arc<$crate::Bot> = $bot;
        $crate::BotRegistry::register(bot)
    }};
    (runtime: $runtime:expr $(,)?) => {{
        $crate::BotRegistry::register(::std::sync::Arc::new($crate::Bot::new($runtime)))
    }};
}

/// 按索引或 UIN 注销机器人。
#[macro_export]
macro_rules! unregister_bot {
    ($index:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_index($index) }};
    (index: $index:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_index($index) }};
    (bot_id: $bot_id:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_bot_id($bot_id) }};
}
