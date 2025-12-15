# 网络命令体系改动核心思路

1. **统一通道数据结构**  
   - 原来 `mpsc::Sender` 携带的是包含 `CommandHandler` trait 对象与 `SharedStatHandle` 的二元组，无法同时支持多种 `Result` 类型。  
   - 现在定义 `NetCommand` trait，并用 `Box<dyn NetCommand + Send>` 作为通道消息，实现“命令 + 状态句柄”的封装，从根本上解决类型不匹配问题。

2. **在 `CommandFuture` 内部封装任务**  
   - 新增 `CommandTask<T>`，把泛型 `CommandHandler` 与其 `SharedStatHandle` 打包成实现了 `NetCommand` 的对象。  
   - `CommandFuture` 投递 `CommandTask`，而不是直接投递 `CommandHandler`，并在 `poll` 中等待 `SharedStatHandle` 被唤醒，让调用端能够得到命令执行结果。

3. **事件循环只处理 `NetCommand`**  
   - `EventLoop` 的 `active` 队列和 `command_receiver` 现在只存放 `NetCommand`。  
   - 事件循环调用 `NetCommand::run`、`on_swarm_event`，不再关心内部的 `SharedStatHandle` 细节，使得网络层职责更聚焦。

4. **客户端直接等待 `CommandFuture`**  
   - `NetClient::dial` 通过 `CommandFuture::new(...).await?` 直接提交并等待任务，确保 API 层拿到命令执行结果或错误。  
   - 上层逻辑不需要知道命令是怎样被执行的，只需处理 `Result`。

这一整套改动的核心思想是：借由 `NetCommand` 这个统一的 trait-object 封装，把 `CommandHandler` 的类型信息局限在 `CommandFuture` 内部，让网络事件循环和客户端都面对单一、简单的接口，从而消除发送端和接收端的类型错配，并为后续添加更多命令类型打好基础。
