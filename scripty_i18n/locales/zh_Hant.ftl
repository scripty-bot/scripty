# Help menu translation strings
command-not-found = 沒有名為 `{ $commandName }` 的指令。
command-not-found-suggestions = 你是指 `{ $suggestion }` 嗎？
no-help-found = 沒有給指令`{ $commandName }`的幫助。
language-set-failure-description-unsupported = 如果你想幫助我們添加對該語言的支持，請加入我們的支援伺服器 { $supportServerInvite }。
default-category-name = 指令
# Context menu command translation strings
context-menu-command-title =
    { "" }
    菜單指令：
    { "" }
context-menu-command-user =
    { " " }
    { $commandName }（在用戶上）
    { "" }
context-menu-command-message =
    { " " }
    { $commandName } (發送信息時)
    { "" }
more-info-on-command =
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
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = 數據庫在嘗試設置你的語言時遇到錯誤。已報告此錯誤，我們將對其進行調查。請不要重複使用該指令。 (假如你好奇的話, 以下是該錯誤。{ $error })
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = 你所指定的語言是無效的語言標識。原因: { $error }
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = 數據庫錯誤。
# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    WebSocket 延遲: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    網際網路 (HTTP) 延遲 { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    數據庫延遲: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)

    注意：如果任何延遲等於 0ms，則表示當前無法計算該延遲。
    請稍後再試。
data-storage-embed-title = 數據存儲
data-storage-toggle-msgs-btn = 開啟或關閉信息存儲

# This is shown as the number of transcriptions the algorithm has discovered.


# This is shown as the title of the transcript


# This is shown as the percent accuracy of the transcription (roughly)


# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.


# This is shown when the algorithm encounters an error

data-storage-opted-out-audio = 你選擇不存儲音頻以進行模型訓練。
data-storage-opted-in-audio = 你選擇存儲音頻以進行模型訓練。
data-storage-opted-in-msgs = 你選擇存儲你的信息以進行記分器培訓。（記分器用於預測特定單詞在音頻中出現的可能性）。
blocked-entity-guild = 此伺服器被禁止使用 Scripty。 { $reason } 你可以嘗試在支援伺服器中對此禁止令提出上訴：{ $supportServerInvite }。
voice-connection-error-ws-closed-unknown-opcode = 由於未知的操作碼，Discord 關閉了連接
data-storage-opted-out-msgs = 你選擇不存儲你的信息以進行記分器培訓。（記分器用於預測特定單詞在音頻中出現的可能性）。
data-storage-command-timed-out = 超時。 如果你仍想改變設置，請重新運行此指令。
blocked-entity-no-reason-given = 沒有給出禁止的理由。
blocked-entity-reason-given = 禁止的原因：{ $reason }。
blocked-entity-user = 你被禁止使用 Scripty。 { $reason } 你可以嘗試在支援伺服器中對此禁止令提出上訴：{ $supportServerInvite }。
voice-connection-error-internal-lib-error = 內部錯誤
voice-connection-error-host-io-error = 主機 IO 錯誤
data-storage-toggle-audio-btn = 開啟或關閉音頻存儲
voice-connection-error-proto-violation = Scripty 和 Discord 在協議上不同意
voice-connection-error-timed-out = 等待連接超時
voice-connection-error-ws-closed-no-reason = Discord 無故關閉連接
voice-connection-error-ws-closed-already-authenticated = 由於已通過身份驗證，Discord 關閉了連接
voice-connection-error-ws-closed-session-timeout = 會話超時
voice-connection-error-ws-closed-server-not-found = 找不到語音伺服器
voice-connection-error-ws-closed-unknown-protocol = Discord 無法識別協議
voice-connection-error-ws-closed-unknown-encryption-mode = Discord 無法識別加密方案
voice-connection-error-msg-reconnect = 我遇到了一個問題（{ $reason }）並與語音聊天斷開了連接。 我會在 30 秒後嘗試重新連接。
voice-connection-error-ws-closed-invalid-payload = 由於無效的負載，Discord 關閉了連接
voice-connection-error-ws-closed-not-authenticated = 由於未通過身份驗證，Discord 關閉了連接
voice-connection-error-ws-closed-authentication-failed = 由於身份驗證失敗，Discord 關閉了連接
voice-connection-error-ws-closed-session-invalid = Discord 廢止了此會話
voice-connection-error-ws-closed-server-crashed = Discord 語音服務器崩潰
voice-connection-error-unknown = 因不明原因斷開連接
voice-connection-error-msg-no-reconnect = 我遇到了一個問題（{ $reason }）並與語音頻道斷開了連接。
# This is shown as the number of transcriptions the algorithm has discovered.
transcription-info-transcript-count = { $count }	個轉錄中的 1 個。
# This is shown as the title of the transcript
transcription-info-transcription-title = 轉錄
# This is shown as the percent accuracy of the transcription (roughly)
transcription-info-transcription-confidence = 信心
# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.
transcription-info-transcription-ssrc = 同步源 (SSRC) { $ssrc }
# This is shown when the algorithm encounters an error
transcription-info-transcription-error =
    內部錯誤：運行語音到文本算法失敗，錯誤：{ $error }
    SSRC: { $ssrc }
    這已被記錄，並將盡快修復。
    如果可能，請聯繫支援伺服器中的核心開發人員：{ $supportServerInvite }。
    非常感謝！
data-storage-embed-description =
    { "**" }注意**：以下所有內容**完全可選**，並且選擇退出**無論如何都不會影響你使用 Scripty 的體驗。
    內容如下。

    Scripty 需要大量音頻和文本數據來訓練正確的語音到文本模型。 並非每個人都能夠捐贈或購買高級等級來幫助我們，因此你可以提供幫助的一個重要方式是允許我們存儲你的數據，例如用於訓練模型的音頻和信息。
    我們了解這些數據可能非常個人，因此這完全是選擇性的，不會以任何方式影響你的體驗。

    下面是我們將如何使用它：
    { "*" } 通過存儲的信息，我們會將它們提供給針對你的語言的記分器。 該記分器將允許算法為一組音頻選擇最可能的單詞。 儘管非常有用，但這並不像音頻那麼重要。
    { "*" } 使用存儲的音頻，我們會將其及其轉錄內容輸入到模型中，以提高語音到文本模型的準確性。 這非常有用，即使你的麥克風很差並且背景噪音很大：事實上，噪音越多越好，只要人類仍然可以聽清你在說什麼。

    如果你選擇加入，並且稍後決定退出，你的數據仍會被存儲，但你可以通過聯繫支持服務器中的核心開發人員請求刪除：`{ $contextPrefix }delete_all_data`。 我們將永久擦除你的所有數據。
    你的數據存儲在核心開發人員擁有的硬件上，並被鎖定。 對於任何試圖成功訪問的人來說，這將是極其困難的。

    你可以使用以下按鈕切換你的選擇。
