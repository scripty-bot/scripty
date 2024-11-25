## join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = join
    .description = Join a voice chat. Transcripts will be logged to the channel you run this command in.
    .voice_channel = voice_channel
    .voice_channel-description = Voice chat to bind to.
    .record_transcriptions = record_transcriptions
    .record_transcriptions-description = Log all transcripts? Users will be DMed when Scripty leaves the channel. Defaults to false.
    .target_channel = target_channel
    .target_channel-description = Send transcripts here, instead of the current channel. Target a forum to create a new post.
    .create_thread = create_thread
    .create_thread-description = Create a new thread for this transcription? Defaults to false.

# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = You're not in a voice chat, nor did you tell me a channel to join. Try `{ $contextPrefix }join <channel>` to specify a voice chat, or join a voice chat yourself and re-run this command.
# This message is shown on successfuly joining a voice channel.
# { $targetMention } is the mention of the channel the bot joined.
join-success-description = Successfully joined { $voiceTargetMention }, and sending transcription output to { $outputChannelMention }.
join-success-premium = You can check out this server's Premium status with `/premium info`.
join-success-help-title = Need help?
join-success-help-description = You can either join the support server at { $supportServerInvite }, or DM the bot.
join-success-footer-free-trial-upsell = This server is eligible for a free trial of Premium. DM the bot to request one.
# This message is shown when the user attempts to make Scripty join a voice channel, but there is no one in the channel.
join-no-one-in-channel = There's no one in { $targetMention }. I'm not joining if there's no one there, as that's a waste of limited resources.
# This message is shown when Discord tosses a Dropped or TimedOut error when trying to join a voice channel.
join-failed-dropped = Discord appears to be having issues, we cannot do anything about this. Please try again later.
# This message is shown when the bot does not have permissions for the voice channel it is trying to join.
join-no-permission = I don't have permission to join { $targetMention }. Please give me the View Channel and Join permissions, or join a different voice chat where I do have permissions.
# This message is shown when the user has told the bot to create a thread while in a thread.
join-create-thread-in-thread = I can't create a thread while in a thread. Please run this command in a normal channel, likely { $parentChannelMention }.
# If the user specifies they would like to create a thread, this is set as the thread name. { $timestamp } is the current timestamp, in ISO format.
join-thread-title = Transcription from { $timestamp }
# If the user specifies they would like to create a forum post, this is the contents of the initial message. { $timestamp } is the current timestamp, in ISO format, and { $authorMention } is the mention of the user who ran the command.
join-forum-thread-content = { $authorMention } started a transcription at { $timestamp }.
# Message shown when a user tries to use an ephemeral thread but cannot do so as a thread was not selected
join-ephemeral-not-thread = To use the ephemeral parameter, you must select a thread as the target, by either setting `create_thread` to true or by targeting a thread with `target_channel`.
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = The forum channel you tried to make me use requires tags. I have no way of knowing what tags to use, so I cannot join that channel. Please use a different channel, or ask an admin to remove the tag requirement.
# This message is shown when the user has told the bot to send transcripts to a non-text-based channel (ie category). `target_channel` should be translated, as slash command arguments are localized.
join-target-not-text-based = The channel you told me to send transcripts to ({ $targetMention }) is not a text-based channel. Please use a text-based channel, or pick a different channel in the `target_channel` argument.
# This message is shown when the user requests the bot create a new thread in a channel, but the channel doesn't support threads being created (usually voice channels)
join-create-thread-in-unsupported = Discord does not support threads in { $targetMention }. Please use a different channel, or do not create a thread.

## Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = leave
    .description = Leave any current voice call.
# This is shown when the bot successfully leaves a voice call
leave-success = Left VC successfully.

## Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = help
    .description = Show this help menu
    .command = command
    .command-description = Specific command to show help about

## transcribe_message command
# This and all attributes show up exclusively in the slash command picker when `transcribe_message` is selected.
cmds_transcribe_message = transcribe_message
    .description = Transcribe a message. Reply to a message to transcribe it.

## premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = premium
    .description = Premium commands
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = claim
    .description = Claim your premium within the server where this command is executed.
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = remove
    .description = Remove your premium from the server where this command is executed.
cmds_premium_info = info
    .description = Get information on this server's Scripty Premium status.
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = You are not a premium subscriber. Subscribe at https://scripty.org/premium. If you know you are one, please DM the bot that way we can reinstate your premium.
# This is shown to the user when they have too many used servers to add more.
premium-too-many-guilds = You have claimed { $totalServers } premium keys. You cannot add any more, unless you upgrade your premium subscription at <https://dash.scripty.org/premium>, or remove some with the `{ $commandPrefix }premium remove` command.
# This is shown when the user successfully claims one of their premium subscriptions.
premium-claimed = You have successfully claimed premium on this server. If you would like to upgrade, or purchase more slots, head to <https://dash.scripty.org/premium>. If you would like to remove your premium from this guild, run `{ $commandPrefix }premium remove`.
# This is shown when the user successfully removes their premium from this guild.
premium-removed = If you are the user who had claimed Premium, you have now successfully removed your premium from this server. If you would like to upgrade, or purchase more slots, head to <https://dash.scripty.org/premium>.

premium-info-embed-title = Premium Status
premium-info-embed-description-no-subscription = You can subscribe to Premium at <https://dash.scripty.org/premium>. On top of the perks you get, you also help us in our goal to make Scripty the best bot out there for speech-to-text :)
premium-info-embed-description-has-subscription = You can manage your subscription at <https://dash.scripty.org/premium>. Thanks for supporting Scripty!
premium-info-embed-current-tier = Current tier
premium-info-embed-max-users = Maximum concurrent users
premium-info-embed-max-duration = Maximum session duration (seconds)
premium-info-embed-max-file-length = Maximum file length (seconds)
premium-info-embed-trial-available-title = Want a free trial of Premium?
premium-info-embed-trial-available-description = DM the bot to get started on setting up a 3 day trial of Premium.
premium-info-embed-manage-subscription-user-has-unclaimed-title = Looks like you purchased Premium!
premium-info-embed-manage-subscription-user-has-unclaimed-description = To claim it in this server, run { $claimCommand }.

## config - verbose command
cmds_config_verbose = verbose
    .description = Toggle whether Scripty is verbose during transcriptions.
    .verbose = verbose
    .verbose-description = Defaults to false
config-verbose-enabled = Scripty will now be verbose during transcriptions.
config-verbose-disabled = Scripty will no longer be verbose during transcriptions.

## config - transcribe voice messages command
config_transcribe_voice_messages = transcribe_voice_messages
    .description = Toggle whether Scripty transcribes voice messages.
    .transcribe_voice_messages = transcribe_voice_messages
    .transcribe_voice_messages-description = Defaults to true
config-transcribe-voice-messages-enabled = Scripty will now transcribe voice messages.
config-transcribe-voice-messages-disabled = Scripty will no longer transcribe voice messages.

## config - transcribe audio command
config_transcribe_audio = transcribe_audio
    .description = Toggle whether Scripty transcribes arbitrary audio files. Requires premium.
    .transcribe_audio = transcribe_audio
    .transcribe_audio-description = Defaults to false
config-transcribe-audio-enabled = Scripty will now transcribe audio files.
config-transcribe-audio-disabled = Scripty will no longer transcribe audio files.
config-transcribe-audio-requires-premium = Transcribing audio files is a Premium feature, as it is computationally expensive to transcode audio files.
  If you would like to upgrade to Premium, head to https://dash.scripty.org/premium. You can also request a free trial of Premium by DMing the bot.
  If this feature was enabled before, it is now disabled.

## config - transcribe video command
config_transcribe_video = transcribe_video
    .description = Toggle whether Scripty transcribes arbitrary video files. Requires T2 premium.
    .transcribe_video = transcribe_video
    .transcribe_video-description = Defaults to false
config-transcribe-video-enabled = Scripty will now transcribe video files.
config-transcribe-video-disabled = Scripty will no longer transcribe video files.
config-transcribe-video-requires-premium = Transcribing video files is a Premium Tier 2 feature, as it is very computationally expensive to transcode video files.
  If you would like to upgrade to Premium Tier 2, head to https://dash.scripty.org/premium.
  If this feature was enabled before, it is now disabled.

## config - auto detect language command
config_auto_detect_lang = auto_detect_lang
    .description = Try to automatically detect the language being spoken? Very inaccurate vs setting a language.
    .auto_detect_lang = auto_detect_lang
    .auto_detect_lang-description = Defaults to false

config-auto-detect-lang-enabled = Scripty will now automatically detect the language being spoken.
config-auto-detect-lang-disabled = Scripty will no longer automatically detect the language being spoken.
config-auto-detect-lang-requires-premium = Automatically detecting the language is a Premium feature, as it is extremely computationally expensive to re-run the model twice to figure out the language.
  If you would like to upgrade to Premium, head to https://dash.scripty.org/premium. You can also request a free trial of Premium by DMing the bot.
  If this feature was enabled before, it is now disabled.

## config - transcribe only role command
config_transcribe_only_role = transcribe_only_role
    .description = Limit Scripty's transcriptions to only users with this role in a voice chat.
    .transcribe_only_role = transcribe_only_role
    .transcribe_only_role-description = Role to limit to: set empty to disable.

config-transcribe-only-role-enabled = Scripty will now only transcribe messages from users in { $roleId }.
config-transcribe-only-role-disabled = Scripty will now transcribe all users, regardless of role.

## config - translate command
config_translate = translate
    .description = Automatically translate transcriptions to English?
    .translate = translate
    .translate-description = Defaults to false

config-translate-enabled = Scripty will now translate transcriptions to English.
config-translate-disabled = Scripty will now attempt to match the phrases being spoken to English words, but will not translate.
config-translate-not-english = You must set your language to English to enable translation. Do so with `{ $contextPrefix }config language en`.

## config - enable_kiai command
config_enable_kiai = enable_kiai
    .description = Enable Scripty's Kiai integration. Run this command with no arguments to get info on Kiai.
    .enable_kiai = enable_kiai
    .enable_kiai-description = Defaults to false
config-kiai-enabled = Scripty will now send any voice XP gained to Kiai. Disable Kiai's voice XP leveling to prevent users getting double XP.
config-kiai-disabled = Scripty will no longer send any voice XP gained to Kiai's API.
config-kiai-info = You can find more info about Kiai at [kiai.app](https://www.kiai.app/?utm_source=scripty_info).
    {""}
    If you use this integration, be sure to disable Kiai's voice XP module as they will conflict.
config-kiai-missing-perms = Scripty is missing permissions to work in this server. Authorize it with the `/application authorize` command, using an application ID of `811652199100317726`, and giving Scripty the "view and edit all levels and XP" permission. 

## debug command
cmds_debug = debug
    .description = Output debugging information about Scripty internal state.
debug-info-message = Forward this message to whoever in the Scripty support server is asking you for it.
debug-not-in-call = This command is useless if Scripty isn't in a VC.

## Help menu translation strings

command-not-found = No command with name `{ $commandName }` found.
command-not-found-suggestions = Did you mean `{ $suggestion }`?
no-help-found = No help found for command `{ $commandName }`.
default-category-name = Commands

## Context menu command translation strings
context-menu-command-title =
    {""}
    Context menu commands:
    {""}
context-menu-command-user =
    {""}
    { $commandName } (on user)
    {""}
context-menu-command-message =
    {""}
    { $commandName } (on message)
    {""}
context-menu-command-unknown =
    {""}
    { $commandName } (on unknown)
    {""}
more-info-on-command =
    
    For more information on a specific command, type `{ $contextPrefix }help <name>`
    ```

## Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `user_language` is selected.
cmds_user_language = user
    .description = Set your user language to one of the available languages.
    .language = language
    .language-description = The language you want to set your user language to.

# This and all attributes show up exclusively in the slash command picker when `guild_language` is selected.
cmds_config_server_language = guild
    .description = Set this server's language to one of the available languages.
    .language = language
    .language-description = The language you want to set your guild language to.

# This message is shown as the embed title when the user sets their language successfully.
user-language-set-success = User language set to `{ $language }`.
# This message is shown as the embed description when the user sets their language successfully.
user-language-set-success-description = To return to English, type `{ $contextPrefix }language user_language en`.
# This message is shown as the embed title when the guild sets their language successfully.
guild-language-set-success = Guild language set to `{ $language }`.
# This message is shown as the embed description when the guild sets their language successfully.
guild-language-set-success-description = To return to English, type `{ $contextPrefix }language guild_language en`.
language-set-partially-translated-help = Want to help translate Scripty into your language? Check out the translation project at https://hosted.weblate.org/engage/scripty-bot/.
# This message is shown as the embed title when an entity tries to set their language to an unsupported language.
language-set-failure-title-unsupported = The language you specified is not supported by the bot.
# This message is shown as the embed description when an entity tries to set their language to an unsupported language.
language-set-failure-description-unsupported = If you'd like to help with adding support for this language, please join the support server at { $supportServerInvite }.
# This message is shown as the embed title when an entity tries to set their language to an invalid language.
language-set-failure-title-invalid = Language `{ $language }` not found.
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = The language you specified is an invalid language identifier. Reason: { $error }
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = Database error.
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = The database encountered an error while attempting to set your language. This error has been reported, and we'll look into it. Please do not spam this command. (If you're curious, here's the error: { $error })
guild-language-set-failure-translate-enabled = Your server has auto-translation enabled. This is only supported when translating to English. Disable this feature if you want to set your language.

## Command invocation contexts

# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = This is a root command!
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = Please invoke only this command's subcommands to use it. See `{ $contextPrefix }help { $commandName }` for more info.

## ping command
# This and all attributes show up exclusively in the slash command picker when `ping` is selected.
cmds_ping = ping
    .description = Get the bot latency.

# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    WebSocket latency: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    HTTP latency: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Database latency: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)
    
    Note: if any latency is equal to 0ms, it means that specific latency could not be calculated right now.
    Try again later.

## data_storage command
# This and all attributes show up exclusively in the slash command picker when `data_storage` is selected.
cmds_data_storage = data_storage
    .description = Configure storage settings for your data
data-storage-embed-title = Data Storage
data-storage-embed-description =
    {"**"}NOTE**: everything that follows is **entirely optional**, and opting out **will not**, in any way, affect your experience with Scripty.
    That said, here goes.
    
    Scripty requires a lot of audio and text data to train a proper speech-to-text model. Not everyone is able to donate or buy premium to help us out, so a big way you can help out is by allowing us to store your data like audio and messages for training a model.
    We understand this data can be extremely personal, so this is entirely opt-in and will not affect your experience in any way.
    
    Here's what we'd do with it:
    {"*"} With stored messages, we would feed them into a scorer targeted to your language. This scorer would allow the algorithm to select the most likely words for a given set of sounds. Although immensely helpful, this isn't as important as audio. Note that this message data is encrypted with AES 256-bit encryption.
    {"*"} With stored audio, we would feed it and the transcript of it into a model to increase the accuracy of the speech-to-text model. This is insanely helpful, even if you have a poor microphone and lots of background noise: in fact, the more noise, the better, as long as a human can still make out what you are saying.
    
    If you are opted in, and you decide later to opt out, your data is still stored, but you can request deletion of your voice data by running `{ $contextPrefix }delete_all_data`. However, it is impossible to delete your message data. This is because we do not store a link of what user sent what message.
    Your data is stored on servers that are locked down tightly. It would be extremely difficult for anyone attempting to gain access to successfully do so.
    
    You can toggle your choices using the below buttons.
data-storage-toggle-audio-btn = Toggle Audio Storage
data-storage-toggle-msgs-btn = Toggle Message Storage
data-storage-opted-in-audio = You are now opted into storing your audio for model training.
data-storage-opted-out-audio = You are now opted out of storing your audio for model training.
data-storage-opted-in-msgs = You are now opted into storing your messages for scorer training.
data-storage-opted-out-msgs = You are now opted out of storing your messages for scorer training.
data-storage-command-timed-out = Timed out. Rerun this command if you still want to manage settings.

## automod root command
# This and all attributes show up exclusively in the slash command picker when `automod` is selected.
cmds_automod = automod
    .description = Manage Scripty's automod
automod-root-response = This is the root command, due to Discord limitations it does nothing. See `{ $contextPrefix }help automod` for more info.

## automod setup command
# This and all attributes show up exclusively in the slash command picker when `automod setup` is selected.
cmds_setup = setup
    .description = Get started with Scripty's automod.
    .target_channel = target_channel
    .target_channel-description = The channel to send automod logs to.
    .log_recording = log_recording
    .log_recording-description = Should a recording of offending speech be sent to the target channel? Defaults to false.
    .auto_join = auto_join
    .auto_join-description = Should the bot automatically join voice if a user joins? Defaults to true.
automod-setup-embed-complete-title = Automod setup complete!
automod-setup-embed-complete-description = You can now use `{ $contextPrefix }automod rule add` to add an automod rule. { $extraDetails }
automod-setup-embed-complete-free-limit = Note that free servers are limited to 25 rules. If you'd like to remove this limit, check out our Premium over at https://scripty.org/premium.
automod-setup-embed-not-setup-title = You haven't agreed to Scripty's Terms of Service and Privacy Policy yet.
automod-setup-embed-not-setup-description = Do so first by running `{ $contextPrefix } terms_of_service`.

## automod add rule command
# This and all attributes show up exclusively in the slash command picker when `automod add rule` is selected.
cmds_add_rule = add_rule
    .description = Add an automod rule.
    .rule_type = rule_type
    .rule_type-description = The type of rule to add. See `/automod rule_help` for more info.
    .rule_type-choice-Regular = Regular
    .content = content
    .content-description = The rule content to add.
    .action = action
    .action-description = The action to take when the rule is triggered.
    .action-choice-SilentDelete = Silent delete
    .action-choice-DeleteAndLog = Delete and log
    .action-choice-DeleteLogAndKick = Delete, log, and remove user from voice
    .action-choice-DeleteLogAndSilence = Delete, log, and mute user
automod-add-rule-embed-success-title = Rule { $ruleId } added!
automod-add-rule-embed-success-description = { $rulesLeft } rules left out of { $maxRules }. { $extraDetails }
automod-add-rule-embed-extra-details-free-limit = Free servers are limited to 25 regular rules. If you'd like to increase this limit, check out our Premium over at https://scripty.org/premium.
automod-add-rule-embed-failure-title = Failed to add rule!
automod-add-rule-embed-failure-description-free-limit = Free servers are limited to 25 regular rules. If you'd like to increase this limit, check out our Premium over at https://scripty.org/premium.
automod-add-rule-embed-failure-description-premium-limit = Premium tier { $tier } servers are limited to { $maxRules } rules. If you upgrade to tier { $nextTier }, you can add { $nextTierMaxRules } rules.
automod-add-rule-embed-failure-description-premium-limit-hard-cap = You've reached the absolute maximum number of rules ({ $hardCap }). This limit exists to ensure we don't add too much latency in a single message.
automod-add-rule-embed-failure-description-invalid-type = Invalid rule type. See `{ $contextPrefix }automod rule_help` for more info.
automod-add-rule-embed-failure-description-free-locked-type = Free servers can only use regular rules. If you'd like to use other rule types, check out our Premium over at https://scripty.org/premium.
automod-add-rule-embed-failure-description-not-setup = You must run `{ $contextPrefix }automod setup` before adding rules.

## automod remove rule command
# This and all attributes show up exclusively in the slash command picker when `automod remove rule` is selected.
cmds_remove_rule = remove_rule
    .description = Remove an automod rule.
    .rule_id = rule_id
    .rule_id-description = The rule ID to remove.
automod-remove-rule-embed-success-title = Rule removed!
automod-remove-rule-embed-success-description = { $rulesLeft } rules left out of { $maxRules }.
automod-remove-rule-embed-failure-title = Failed to remove rule!
automod-remove-rule-embed-failure-description-invalid-id = Invalid rule ID. See `{ $contextPrefix }automod list` for more info.
automod-remove-rule-embed-failure-description-not-setup = You must run `{ $contextPrefix }automod setup` before removing rules.

## automod list rules command
# This and all attributes show up exclusively in the slash command picker when `automod list rules` is selected.
cmds_list_rules = list_rules
    .description = List all automod rules.
    .filter_by = filter_by
    .filter_by-description = Filter rules by their content. Leave empty to show all rules.
automod-list-rules-embed-title = Automod rules
automod-list-rules-embed-description = { $rulesLeft } rules left out of { $maxRules }.
automod-list-rules-embed-field-name = Rule { $ruleId }
automod-list-rules-embed-field-value = Type: { $ruleType }
    Content: { $ruleContent }
    Action: { $ruleAction }
automod-list-rules-footer = Page { $page } of { $maxPage }
automod-list-rules-no-rules = You don't have any rules!

## vote reminder command
cmds_vote_reminder = vote_reminder
    .description = Toggle whether Scripty will remind you to vote for the bot after the time limit has passed.
    .enabled = enabled
    .enabled-description = Enable vote reminders?
vote-reminders-enabled = Vote reminders enabled.
vote-reminders-disabled = Vote reminders disabled.

## blocked entities description

blocked-entity-no-reason-given = No reason was given for the block.
blocked-entity-reason-given = Reason given for the block: { $reason }.
blocked-entity-guild = This guild is blocked from using Scripty. { $reason } You may attempt to appeal this block in the support server: { $supportServerInvite }.
blocked-entity-user = You are blocked from using Scripty. { $reason } You may attempt to appeal this block in the support server: { $supportServerInvite }.

## voice connection errors

voice-connection-error-internal-lib-error = library internal error
voice-connection-error-host-io-error = host IO error
voice-connection-error-proto-violation = library and discord disagreed on protocol
voice-connection-error-timed-out = timed out waiting for connection
voice-connection-error-ws-closed-no-reason = discord closed connection without reason
voice-connection-error-ws-closed-unknown-opcode = discord closed connection due to unknown opcode
voice-connection-error-ws-closed-invalid-payload = discord closed connection due to an invalid payload
voice-connection-error-ws-closed-not-authenticated = discord closed connection due to not being authenticated
voice-connection-error-ws-closed-authentication-failed = discord closed connection due to authentication failure
voice-connection-error-ws-closed-already-authenticated = discord closed connection due to already being authenticated
voice-connection-error-ws-closed-session-invalid = discord invalidated session
voice-connection-error-ws-closed-session-timeout = session timed out
voice-connection-error-ws-closed-server-not-found = voice server couldn't be found
voice-connection-error-ws-closed-unknown-protocol = discord didn't recognize protocol
voice-connection-error-ws-closed-server-crashed = discord voice server crashed
voice-connection-error-ws-closed-unknown-encryption-mode = discord didn't recognize encryption scheme
voice-connection-error-unknown = disconnected for unknown reason
voice-connection-error-msg-no-reconnect = I had an issue ({ $reason }) and disconnected from the voice chat.
voice-connection-error-msg-reconnect = I had an issue ({ $reason }) and disconnected from the voice chat. I'll try reconnecting in 30 seconds.

## general errors
general-error-command-process-title = An error happened while processing { $command }.
general-error-command-process-description = ```
    { $errorFmt }
    ```
    This has been automatically reported. Please do not attempt to repeatedly use this command.

general-error-invalid-args-title = Invalid arguments while parsing { $command }.
general-error-invalid-args-description = Failed to parse `{ $input }` because `{ $error }`

general-error-invalid-structure-title = Invalid structure from Discord while parsing { $command }.
general-error-invalid-structure-description = { $description }
    
    {"**"}Note**: this is a Discord error.
    The only fix for this is to wait for Discord to propagate slash commands, which can take up to one hour.
    If you do not want to wait this hour, you should use the prefix commands: run this command with `~{ $qualifiedName } { $args }`.

general-error-cooldown-hit-title = Cooldown hit on { $command }
# Note $time will be a decimal with two digits of accuracy.
general-error-cooldown-hit-description = { $time } seconds left on cooldown.

general-error-user-missing-perms-title = You are missing perms to run { $command }.
general-error-user-missing-perms-description-known = Permissions missing: { $perms }
general-error-user-missing-perms-description-unknown = I'm not sure what permissions you're missing.
general-error-user-missing-perms-description-not-owner = Not an owner of this bot.

general-error-command-check-failed-title = A precondition for { $command } failed.
general-error-command-check-failed-description-no-reason = no reason provided

## transcription info - verbose mode

# This is shown as the number of transcriptions the algorithm has discovered.
transcription-info-transcript-count = Transcript 1 of { $count }.
# This is shown as the title of the transcript
transcription-info-transcription-title = Transcript
# This is shown as the percent accuracy of the transcription (roughly)
transcription-info-transcription-confidence = Confidence
# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.
transcription-info-transcription-ssrc = SSRC { $ssrc }
# This is shown when the algorithm encounters an error
transcription-info-transcription-error =
    internal error: running stt algorithm failed with error: { $error }
    SSRC: { $ssrc }
    This has been logged and will be fixed as soon as possible.
    If possible, please contact the core devs in the support server: { $supportServerInvite }.
    Thanks!

## Data deletion command
# This and all attributes show up exclusively in the slash command picker when `delete_all_data` is selected.
cmds_delete_all_data = delete_all_data
    .description = Delete all your data.

delete-data-title = Delete data
delete-data-description =
    This will delete all of your data. This action is permanent, irreversible, and cannot be undone.
    
    When we say "all of your data" we mean *all* of it. This includes your voice data, and your user in the database.
    This however, *does not* include any messages we may have stored from you if you opted into that. We cannot delete those messages, simply because we don't know what user sent what message.
    
    If you would like to also be banned from using the bot after this action, that way you do not accidentally readd yourself, you can click the appropriate button below.
    Note that doing so will require us to store your user ID to keep a record of banned users.
    If at any point after this action you would like to be unbanned, you can contact the support server and ask for a manual unban.
    
    Are you sure you want to delete all of your data?
delete-data-confirm = Yes, delete all data
delete-data-confirm-banned = Yes, delete all data and ban myself
delete-data-cancel = No, cancel

## generic strings
# Message shown if a guild has not claimed their free trial of premium. Always appears on its own standalone line in the surrounding message.
free-trial-upsell = We offer 3-day trials of Scripty Premium if you would like to try it out and see if it is right for you. Send the bot a DM to get started with a free trial.
