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
language-set-failure-description-invalid = 你给我的语言不是一个真正的一个语言标识符。
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
