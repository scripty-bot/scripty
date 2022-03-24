donation-title = 捐款
donation-description =
    Scripty 運行起來並不便宜。它目前在配備 AMD Ryzen 9 3900 CPU 和 128GB RAM 的 1,500 美元服務器上運行。即使有那種硬件，我們估計它也只能同時處理100個轉錄。捐贈會讓我們能夠擴展我們的硬件容量和同時處理更多轉錄。如果我們有足夠的捐款的話，我們將能夠擴展我們的硬件容量或許我們還能夠同時處理數以千計的轉錄。
    
    訓練模型同樣不容易, 因為這需要相對較新的 (支持 CUDA 10.1) 英偉達顯卡。我們不喜歡問別人捐款，但我們沒有能力自掏腰包運行 Scripty 因為它運行起來太貴了。所以我們正在尋求幫助，並以訂閱的形式給予獎勵
    
    您可以在網站 https://scripty.org/premium 查看更多信息，但主要的要點是，價格從每月 5 美元到 100 美元不等.100 美元的套餐包括單獨為你的伺服器服務的 Scripty，自定義名稱和頭像。
    
    你還能夠直接通過 GitHub 贊助一次性捐贈:
    { "[https://github.com/sponsors/tazz4843](https://github.com/sponsors/tazz4843?frequency=one-time&sponsor=tazz4843)" }
    
    您可以在 https://scripty.org/premium 查看這些等級。
    
    <3
    ~ 0/0 和 valkyrie_pilot
# Help menu translation strings
command-not-found = 沒有名為 `{ $commandName }` 的指令。
command-not-found-suggestions = 你是指 `{ $suggestion }` 嗎？
no-help-found = 沒有給指令`{ $commandName }`的幫助。
language-set-failure-description-unsupported = 如果你想幫助我們添加對該語言的支持，請加入我們的支援伺服器 { $supportServerInvite }。
default-category-name = 指令
# Context menu command translation strings
context-menu-command-title =
    { "" }
    菜單指令:
    { "" }
context-menu-command-user =
    { "  " }
    { $commandName } (在用戶上)
    { "" }
context-menu-command-message =
    { "  " }
    { $commandName } (發送信息時)
    { "" }
more-info-on-command =
     { "" }
    有關特定指令的更多信息，發送 `{ $contextPrefix }help <name>`
     ```
# Language configuration strings
user-language-set-success = 成功將用戶語言設置為 `{ $language }`。
user-language-set-success-description = 要將用戶語言重置為英語，發送 `{ $contextPrefix }language user_language en`。
guild-language-set-success = 成功將伺服器語言設置為 `{ $language }`。
guild-language-set-success-description = 要將伺服器語言重置為英語，發送 `{ $contextPrefix }language guild_language en`。
language-set-failure-title-unsupported = Scripty 暫時還不支持你指定的語言。
language-set-failure-title-invalid = 語言 `{ $language }` 並不存在。
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = 請僅使用此指令的子指令。請參閱`{ $contextPrefix }help { $commandName }` 以獲取更多信息。
# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = 這是一個頂層指令！
# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = 您不在語音頻道中，也沒有指定要加入的頻道。用`{ $contextPrefix }join <channel>` 指定語音頻道，或自己進入語音頻道後重新使用此指令。
# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.
bot-not-set-up = 看來你還沒有設置 Scripty。 發送 `{ $contextPrefix }setup` 然後發送此指令。
# This message is shown on successfuly joining a voice channel.
# {$targetMention} is the mention of the channel the bot joined.
join-success = 成功加入 { $targetMention }。
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = 數據庫在嘗試設置你的語言時遇到錯誤。已報告此錯誤，我們將對其進行調查。請不要重複使用該指令。 (假如你好奇的話, 以下是該錯誤。{ $error })
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = 你所指定的語言是無效的語言標識。原因: { $error }
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = 數據庫錯誤。
# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.
webhook-deleted = 看起來你刪除了我使用的 webhook！ *打* 重新運行 `{ $contextPrefix }setup` 以解決這個問題。
# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    WebSocket 延遲: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    網際網路 (HTTP) 延遲 { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    數據庫延遲: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)
    
    注意：如果任何延遲等於 0ms，則表示當前無法計算該延遲。
    請稍後再試。
