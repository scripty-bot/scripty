donation-title = 捐款
donation-description =
    Scripty 運行起來並不便宜。它目前在配備 AMD Ryzen 9 3900 CPU 和 128GB RAM 的 1,500 美元服務器上運行。即使有那種硬件，我們估計它也只能同時處理100個轉錄。捐贈會讓我們能夠擴展我們的硬件容量和同時處理更多轉錄。如果我們有足夠的捐款的話，我們將能夠擴展我們的硬件容量或許我們還能夠同時處理數以千計的轉錄。
    
    訓練模型同樣不容易, 因為這需要相對較新的 (支持 CUDA 10.1) 英偉達顯卡。我們不喜歡問別人捐款，但我們沒有能力自掏腰包運行 Scripty 因為它運行起來太貴了。所以我們正在尋求幫助，並以高級等級的形式給予獎勵
    
    你可以在網站 https://scripty.org/premium 查看更多信息，但主要的要點是，價格從每月 5 美元到 100 美元不等.100 美元的套餐包括單獨為你的伺服器服務的 Scripty，自定義名稱和頭像。
    
    你還能夠直接通過 GitHub 贊助一次性捐贈:
    { "[https://github.com/sponsors/tazz4843](https://github.com/sponsors/tazz4843?frequency=one-time&sponsor=tazz4843)" }
    
    你可以在 https://scripty.org/premium 查看這些等級。
    
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
no-channel-specified = 你不在語音頻道中，也沒有指定要加入的頻道。用`{ $contextPrefix }join <channel>` 指定語音頻道，或自己進入語音頻道後重新使用此指令。
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
setup-arg2-description = 用於轉錄的語言。
setup-tos-agree =
    通過設置 Scripty，你同意其隱私政策和服務條款。
    隱私政策：https://scripty.org/privacy
    服務條款：https://scripty.org/terms
setup-tos-agree-failure = 你必須同意服務條款和隱私政策才能使用 Scripty。 正在取消設置。
setup-success-title = 設置成功！
setup-arg3-description = Scripty 是否應該發送詳細信息。
setup-arg1-description = 將所有轉錄發送到的頻道。
setup-description =
    設置 Scripty。
    
    這將初始化 Scripty 的功能，
    允許你使用 `~join` 將機器人綁定到語音頻道。
    
    參數 1 是將所有轉錄發送到的頻道。
    
    參數 2 是可選的，是用於轉錄的語言。
    
    參數 3 是可選的，它決定 Scripty 是否應該發送詳細信息。
setup-success-description =
    幾點注意事項：
    
    
    1）不要刪除在目標頻道中創建的 webhook。
    
    2）Scripty 運行起來非常昂貴，並且需要大量的處理能力，所以如果你能提供捐款，我們將不勝感激。 我們提供其他高級等級，以提高轉錄用戶數量的限制，默認為 5。不過，核心功能將永遠免費。 如果你有興趣，請查看 `{ $contextPrefix }donate` 指令。
    
    3）如果你選擇了英語以外的語言（默認），請注意該語言的轉錄質量會低得多。 很快我們將添加一項功能，讓你可以用自己的聲音幫助轉錄準確性（見註 5）。
    
    4）如果你不是美國中年男性，預計轉錄準確度會降低。 這是由於模型固有的偏差，我們唯一能做的就是訓練更準確的模型（再次參見註釋 5）。
    
    5）為了幫助我們訓練更準確的模型，請考慮允許我們存儲你的音頻和轉錄以進行訓練。 請參見 `{ $contextPrefix }train storage` 指令。
    
    6）我也不好意思再煩你，但請考慮捐贈。 訓練新模型需要***難以想像***的處理能力（我們說的是多個 英偉達 RTX 3090 顯卡），一點錢都有很大幫助。 同樣，如果你有興趣，請查看 `{ $contextPrefix }donate` 指令。
    
    
    感謝你對 Scripty 的興趣！<3
    ~ 0/0 + valkyrie_pilot
data-storage-embed-title = 數據存儲
data-storage-toggle-msgs-btn = 開啟或關閉信息存儲
None = data-storage-toggle-audio-btn =
None = data-storage-opted-in-audio =
None = data-storage-opted-out-audio =
None = data-storage-opted-in-msgs =
None = data-storage-opted-out-msgs =
None = data-storage-command-timed-out =
None = blocked-entity-no-reason-given =
None = blocked-entity-reason-given =
None = blocked-entity-guild =
None = blocked-entity-user =
None = voice-connection-error-internal-lib-error =
None = voice-connection-error-host-io-error =
None = voice-connection-error-proto-violation =
None = voice-connection-error-timed-out =
None = voice-connection-error-ws-closed-no-reason =
None = voice-connection-error-ws-closed-unknown-opcode =
None = voice-connection-error-ws-closed-invalid-payload =
None = voice-connection-error-ws-closed-not-authenticated =
None = voice-connection-error-ws-closed-authentication-failed =
None = voice-connection-error-ws-closed-already-authenticated =
None = voice-connection-error-ws-closed-session-invalid =
None = voice-connection-error-ws-closed-session-timeout =
None = voice-connection-error-ws-closed-server-not-found =
None = voice-connection-error-ws-closed-unknown-protocol =
None = voice-connection-error-ws-closed-server-crashed =
None = voice-connection-error-ws-closed-unknown-encryption-mode =
None = voice-connection-error-unknown =
None = voice-connection-error-msg-no-reconnect =
None = voice-connection-error-msg-reconnect =

# This is shown as the number of transcriptions the algorithm has discovered.

None = transcription-info-transcript-count =

# This is shown as the title of the transcript

None = transcription-info-transcription-title =

# This is shown as the percent accuracy of the transcription (roughly)

None = transcription-info-transcription-confidence =

# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.

None = transcription-info-transcription-ssrc =

# This is shown when the algorithm encounters an error

None = transcription-info-transcription-error =
data-storage-opted-out-audio = 您選擇不存儲音頻以進行模型訓練。
data-storage-opted-in-audio = 您選擇存儲音頻以進行模型訓練。
