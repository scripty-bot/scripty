more-info-on-command = \n你需要帮忙给一个具体的指令的话，发送 `{ $contextPrefix }help <name>`\n```
# Language configuration strings
user-language-set-success = 用户的语言记为 `{ $language }` 了。
user-language-set-success-description = 要返回语言到英文的话，请发送 `{ $contextPrefix }language user_language en`.
command-not-found-suggestions = 你本来的意思是 `{ $suggestion }` 吗?
no-help-found = 我找不到帮助给叫 `{ $commandName }`的指令。
default-category-name = 指令
# Context menu command translation strings
context-menu-command-title = \n菜单的指令\n
context-menu-command-user = { $commandName } (在用户)\n
# Help menu translation strings
command-not-found = 我找不到叫 `{ $commandName }` 的指令。
context-menu-command-message = { $commandName } (在信息)\n
language-set-failure-description-unsupported =
    如果你想要帮我们翻译这一个语言的话，
    请加入我们的 Support Server：{ $supportServerInvite }.
language-set-failure-description-db =
    在该语言的时候，数据库返回了一个问题。
    我刚才报告了这个问题。我们有时间的时候，我们就会检查只一个问题。请不要垃圾这一个指令。（如果你好奇，发现的错误是：{ $error }）
guild-language-set-success = 伺服器的语言改变到 `{ $language }`了。
guild-language-set-success-description = 要返回语言到英文的话，请发送 `{ $contextPrefix }language guild_language en`.
language-set-failure-title-unsupported = 机器人不支持您指定的语言.
language-set-failure-title-invalid = 没有一个语言叫 `{ $language }`.
language-set-failure-description-invalid = 您指定的语言是一个无效的语言标识符。原因：{ $error }
language-set-failure-title-db = 数据库返回了一个错误。

# This message is shown as the embed title when a user tries to invoke the root command of a group.


# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.


# This message is shown on successfuly joining a voice channel.
# {$targetMention} is the mention of the channel the bot joined.


# This message is shown as the embed description when a user tries to invoke the root command of a group.


# This message is shown when the user is not in a voice channel, nor was a voice channel specified.


# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.


# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.


# This is shown as the number of transcriptions the algorithm has discovered.


# This is shown as the title of the transcript


# This is shown as the percent accuracy of the transcription (roughly)


# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.


# This is shown when the algorithm encounters an error

join-success-premium = 您可以使用 `/premium info`指令查看该服务器的高级订阅状态哦。
join-create-thread-in-thread = 我不能在子频道中创建子频道。请在普通频道中运行此命令，可能是 { $parentChannelMention }。
join-create-thread-in-unsupported = Discord 不支持在 { $targetMention } 中创建子频道。请使用其他频道，或者不要创建子频道。
join-success-footer-free-trial-upsell = 这个服务器可免费试用高级版。请DM机器人来提出申请试用。
join-target-not-text-based = 你让我发送转录的频道,也就是： ({ $targetMention }) 不是文字频道。请尝试使用文字频道，或者在 `目标频道` 参数中选择其他类别的频道。
join-success-help-title = 需要帮助吗？
join-success-help-description = 你可以通过{ $supportServerInvite }加入官方服务器，你也可以私信这个bot。
join-forum-requires-tags = 你尝试让我使用的论坛频道需要标签。我不知道应该使用什么标签，所以无法加入该频道。请尝试其他频道，或者请管理员移除标签要求吧.
leave-success = 成功的离开了那个语音频道.
cmds_help = 帮助
    .description = 显示此帮助菜单
    .command = 命令
    .command-description = 要显示帮助的特定命令
join-ephemeral-not-thread = 要使用 ephemeral 参数，您必须选择一个子频道作为目标，您可以通过将 create_thread 设置为 true 或者使用 target_channel 指定一个子频道来实现。
cmds_leave = leave
    .description = 离开当前的语音通话。
premium-info-embed-description-has-subscription = 您可以在 <https://dash.scripty.org/premium>管理您的高级订阅，感谢您对Scripty的支持！
premium-info-embed-max-duration = 会话时长上限（秒）
premium-info-embed-max-file-length = 最大文件时长（秒）
cmds_ping = ping
    .description = 获取机器人与服务器连接延迟。
join-success-description = 成功加入了 { $voiceTargetMention }频道，bot会将转录内容发送到 { $outputChannelMention }。
premium-info-embed-max-users = 最大同时在线用户数
premium-info-embed-trial-available-title = 想要免费试用Premium吗？
premium-info-embed-trial-available-description = 请私信机器人以开始设置为期3天的高级试用。
join-failed-dropped = Discord似乎出现了一些问题，我们无法解决，请您稍后重试.
no-channel-specified = 您不在任何语音聊天中，也没有告诉我要加入哪个频道。可以尝试使用 { $contextPrefix }join <channel> 来指定一个语音频道，或者自己加入一个语音频道后再重新运行此命令。
language-set-partially-translated-help = 想要帮助我们将Scripty翻译成您的语言吗？ 请登录「https://hosted.weblate.org/engage/scripty-bot/.」查看机器人的翻译项目。
join-no-one-in-channel = { $targetMention }里面一个人都没有，如果没人，我是不会加入的，那是浪费计算资源.
premium-not-premium = 您不是高级用户，请您前往「 https://scripty.org/premium」订阅高级套餐；如果您是已订阅了高级套餐，请您私信bot，这样我们就可以恢复您的订阅状态。
config-verbose-enabled = Scripty 现在在转录过程中会提供详细信息。
config-transcribe-voice-messages-disabled = Scripty 现在将停止转录语音消息。
config-transcribe-voice-messages-enabled = Scripty 现在将转录语音消息。
config-verbose-disabled = Scripty 在转录过程中不再提供详细信息。
premium-info-embed-description-no-subscription = 您可以在<https://dash.scripty.org/premium>订阅高级版。除了你得到的好处，您还帮助我们更进一步让Scripty成为最好的语音转文本机器人：)
premium-info-embed-manage-subscription-user-has-unclaimed-title = 看起来你购买了高级版！
join-thread-title = 转录自{ $timestamp }
premium-info-embed-title = 订阅状态
config-transcribe-video-requires-premium =
    转录视频文件是tier2订阅者的功能，因为这样做所需要的算力非常高
    如果您想要订阅Premium tier2，您可以访问「https://dash.scripty.org/premium.」
    如果这个特性以前是启用的，现在是禁用的。
config-auto-detect-lang-disabled = Scripty现在不会自动检测现在使用的语言。
config-auto-detect-lang-enabled = Scripty现在会自动检测正在使用的语言.
premium-removed = 如果您在之前购买了Premium，那么你现在已经成功的移除了Premium订阅，如果您想重新购买或升级，您可以访问<https://dash.scripty.org/premium>来重新购买。
premium-info-embed-manage-subscription-user-has-unclaimed-description = 如果你要在这个服务器上声明，你需要运行{ $claimCommand }命令
config-translate-enabled = Scripty现在将翻译成英文。
config-translate-disabled = Scripty现在正在尝试将成员正在说的短语转录为英文，但是不会进行翻译。
config-auto-detect-lang-requires-premium =
    自动检测语言功能是一项Premium订阅功能，因为这样做所需要的算力较高.
    如果您想订阅高级功能，你可以在「https://dash.scripty.org/premium」订阅高级功能，亦或者私信bot来获得高级功能免费试用权限
    如果这个功能以前是启用的，那么现在是禁用的。
config-transcribe-only-role-enabled = Scripty现在只会转录来自{ $roleId }的消息.
root-command-invoked-title = 这是一条根命令！
config-transcribe-video-enabled = Scripty现在会转录视频文件.
config-transcribe-video-disabled = Scripty现在不会转录视频文件.
config-translate-not-english = 您必须将您的语言设置为英语来启用翻译，您可以使用命令{ $contextPrefix }config language en`.进行设置.
config-transcribe-only-role-disabled = Scripty现在将会转录所有成员的消息.
cmds_debug = debug
    .description = 输出Scripty脚本内的调试信息.
debug-info-message = 将此消息发送至Scripty支持服务器中向你请求此消息的帮助人员.
root-command-invoked-description = 请只使用该命令的子命令来使用这个功能，更多请参考{ $contextPrefix }help{ $commandName }。
data-storage-embed-description =
    { "**" }NOTE**: everything that follows is **entirely optional**, and opting out **will not**, in any way, affect your experience with Scripty.
    That said, here goes.

    Scripty requires a lot of audio and text data to train a proper speech-to-text model. Not everyone is able to donate or buy premium to help us out, so a big way you can help out is by allowing us to store your data like audio and messages for training a model.
    We understand this data can be extremely personal, so this is entirely opt-in and will not affect your experience in any way.

    Here's what we'd do with it:
    { "*" } With stored messages, we would feed them into a scorer targeted to your language. This scorer would allow the algorithm to select the most likely words for a given set of sounds. Although immensely helpful, this isn't as important as audio. Note that this message data is encrypted with AES 256-bit encryption.
    { "*" } With stored audio, we would feed it and the transcript of it into a model to increase the accuracy of the speech-to-text model. This is insanely helpful, even if you have a poor microphone and lots of background noise: in fact, the more noise, the better, as long as a human can still make out what you are saying.

    If you are opted in, and you decide later to opt out, your data is still stored, but you can request deletion of your voice data by running `{ $contextPrefix }delete_all_data`. However, it is impossible to delete your message data. This is because we do not store a link of what user sent what message.
    Your data is stored on servers that are locked down tightly. It would be extremely difficult for anyone attempting to gain access to successfully do so.

    You can toggle your choices using the below buttons.
config-kiai-enabled = Scripty 现在会将获得的所有语音 XP 发送给 Kiai。 禁用 Kiai 的语音 XP 升级功能，以防止用户获得双倍 XP。
config-kiai-disabled = Scripty 将不再将获得的任何语音 XP 发送到 Kiai 的 API。
config-kiai-missing-perms = Scripty 无法在此服务器上工作，因为它缺少必要的权限。请使用 「`/application authorize` 」命令授权，指定应用程序 ID 为 「`811652199100317726`」，并授予 Scripty “查看和编辑所有等级与 XP” 的权限。
debug-not-in-call = 如果Scripty不在「Visual C++」中，那么这条指令就是无用的
voice-connection-error-internal-lib-error = 数据库内部错误
automod-remove-rule-embed-success-title = 规则删除！
automod-list-rules-embed-field-value =
    类型：{ $ruleType }
    内容：{ $ruleContent }
    action：{ $ruleAction }
blocked-entity-no-reason-given = 没有给出封禁原因。
voice-connection-error-ws-closed-unknown-opcode = 由于未知操作码，Discord 关闭了连接
automod-remove-rule-embed-failure-title = 规则删除失败！
automod-list-rules-no-rules = 您没有添加任何规则！
vote-reminders-enabled = 投票提醒已启用.
voice-connection-error-proto-violation = 库与 Discord 在协议上存在分歧
voice-connection-error-host-io-error = 主机 IO 错误
automod-add-rule-embed-failure-description-free-locked-type = 免费服务器只能使用常规规则类型，如果您想使用其他类型的规则，您可以访问「https://scripty.org/premium」来查看高级订阅选项.
automod-add-rule-embed-failure-title = 添加规则失败！
automod-add-rule-embed-failure-description-premium-limit-hard-cap = 您已达到规则的绝对最大数量（{ $hardCap }）。此限制的存在是为了确保单条消息不会过多增加延迟。
data-storage-command-timed-out = 连接超时.如果您还需要进行管理，请再次尝试运行此命令.
voice-connection-error-ws-closed-no-reason = Discord 无故关闭了连接
automod-add-rule-embed-failure-description-premium-limit = 高级订阅服务器{ $tier }仅限于{ $maxRules }，如果您想要升级到{ $nextTier }，您可以添加{ $nextTierMaxRules }规则
automod-add-rule-embed-failure-description-free-limit = 普通服务器的自定义规则上限为25条，如果您想要增加规则上限，您可以访问「https://scripty.org/premium」查看高级功能.
automod-list-rules-embed-field-name = 规则{ $ruleId }
automod-list-rules-embed-description = { $rulesLeft }中剩下的{ $maxRules }规则.
automod-remove-rule-embed-failure-description-not-setup = 你在删除规则前，你必须运行{ $contextPrefix }命令.
blocked-entity-guild = 这个服务器被禁止使用Scripty{ $reason }，您可以在Scripty的官方支持服务器进行上诉{ $supportServerInvite }.
automod-add-rule-embed-failure-description-not-setup = 您在添加规则前，您必须先运行 `{ $contextPrefix }automod setup`.
automod-add-rule-embed-failure-description-invalid-type = 无效的规则类型，更多详细信息请参考{ $contextPrefix }规则疑难帮助.
automod-remove-rule-embed-failure-description-invalid-id = 无效的规则ID,更多详细信息请参阅`{ $contextPrefix }automod list`.
vote-reminders-disabled = 投票提醒已禁用.
blocked-entity-user = 您已经被禁止使用Scripty，{ $reason }您可以在Scripty官方服务器进行上诉{ $supportServerInvite }.
voice-connection-error-ws-closed-invalid-payload = Discord 因无效的负载关闭了连接
voice-connection-error-ws-closed-session-invalid = discord 无效会话
data-storage-embed-title = 数据存储
data-storage-toggle-msgs-btn = 切换到消息存储
data-storage-toggle-audio-btn = 切换到音频存储
