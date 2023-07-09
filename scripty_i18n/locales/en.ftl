## generic strings
# Message shown if a guild has not claimed their free trial of premium. Always appears on its own standalone line in the surrounding message.
free-trial-upsell = We offer 3-day trials of Scripty Premium if you would like to try it out and see if it is right for you. Send the bot a DM to get started with a free trial.

## donate command
# This is shown as the title of the donate command (eg what shows up in the slash command picker)
donations-command-name = donate
# This is shown as the description of the donate command (eg what shows up in the slash command picker)
donations-command-description = Get information on donating to support Scripty.
donation-title = Donations
donation-description =
    Scripty is not cheap to run. It is currently running on a $1,500 USD server with an AMD Ryzen 9 3900 CPU and 128GB RAM. Even with that hardware, we estimate it can only handle 100 concurrent transcriptions. Donations would allow us to scale our hardware capacity and handle many more concurrent transcriptions, perhaps tens of thousands someday with enough donations.
    
    Training a model is not easy either, as that needs relatively recent (CUDA 10.1 support) Nvidia GPUs. We hate asking for donations, but we absolutely can't support the bot out of our own pockets, since it's just too expensive. So we're asking for help, and giving rewards in the form of premium subscriptions.
    
    You can view more info at https://scripty.org/premium, but the gist of it is that there are 6 tiers ranging in price from $5 USD to $100 USD per month. The $100 tier comes with its own managed instance of Scripty for your own server, with a custom name, and profile picture.
    
    We also support one-time donations directly through GitHub Sponsors:
    {"[https://github.com/sponsors/tazz4843](https://github.com/sponsors/tazz4843?frequency=one-time&sponsor=tazz4843)"}
    
    You can view these tiers at https://scripty.org/premium.
    
    <3
    ~ 0/0 and valkyrie_pilot

## premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = You are not a premium subscriber. Subscribe at https://scripty.org/premium.
# This is shown to the user when they have too many used servers to add more.
premium-too-many-guilds = You have claimed { $totalServers } premium keys. You cannot add any more, unless you upgrade your premium subscription at <https://dash.scripty.org/premium>, or remove some with the `{ $commandPrefix }premium remove` command.
# This is shown when the guild the user is running this command in has not finished setup.
premium-server-not-set-up = This server has not been set up yet. Do that first with the `{ $commandPrefix }setup` command.
# This is shown when the user successfully claims one of their premium subscriptions.
premium-claimed = You have successfully claimed premium on this server. If you would like to upgrade, or purchase more slots, head to <https://dash.scripty.org/premium>. If you would like to remove your premium from this guild, run `{ $commandPrefix }premium remove`.
# This is shown when the user successfully removes their premium from this guild.
premium-removed = If you are the user who had claimed Premium, you have now successfully removed your premium from this server. If you would like to upgrade, or purchase more slots, head to <https://dash.scripty.org/premium>.


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
more-info-on-command =
    
    For more information on a specific command, type `{ $contextPrefix }help <name>`
    ```

## Language configuration strings
# This is shown as the title of the root language command (eg what shows up in the slash command picker)
language-root-command-name = language
# This is shown as the description of the root language command (eg what shows up in the slash command picker)
language-root-command-description = Modify your language preferences.
    
    Base command of this group. See subcommands for more information.

# This is shown as the title of the user language command (eg what shows up in the slash command picker)
language-user-command-name = user
# This is shown as the description of the user language command (eg what shows up in the slash command picker)
language-user-command-description = Set your user language to one of the available languages.
    
    Note: this only modifies your user language, not your guild language. See `guild_language` for that.
# This is shown as the description of the first argument to the user language command
language-user-argument1-description = The language you want to set your user language to.

# This is shown as the title of the guild language command (eg what shows up in the slash command picker)
language-guild-command-name = guild
# This is shown as the description of the guild language command (eg what shows up in the slash command picker)
language-guild-command-description = Set your guild language to one of the available languages.
    
    Note: this only modifies your guild language, not your user language. See `user_language` for that.
# This is shown as the description of the first argument to the guild language command
language-guild-argument1-description = The language you want to set your guild language to.

# This message is shown as the embed title when the user sets their language successfully.
user-language-set-success = User language set to `{ $language }`.
# This message is shown as the embed description when the user sets their language successfully.
user-language-set-success-description = To return to English, type `{ $contextPrefix }language user_language en`.
# This message is shown as the embed title when the guild sets their language successfully.
guild-language-set-success = Guild language set to `{ $language }`.
# This message is shown as the embed description when the guild sets their language successfully.
guild-language-set-success-description = To return to English, type `{ $contextPrefix }language guild_language en`.
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

## Command invocation contexts

# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = This is a root command!
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = Please invoke only this command's subcommands to use it. See `{ $contextPrefix }help { $commandName }` for more info.

## join command
# This is shown as the title of the join command (eg what shows up in the slash command picker)
join-command-name = join
# This is shown as the description of the join command (eg what shows up in the slash command picker)
join-command-description = Join a voice chat.
    
    Argument 1 is a voice chat to join.
    If you do not specify a voice channel to join, the bot will default to the same one you are in.
# This is shown as the description of the first argument to the join command
join-command-argument1-description = Voice chat to bind to.

# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = You're not in a voice chat, nor did you tell me a channel to join. Try `{ $contextPrefix }join <channel>` to specify a voice chat, or join a voice chat yourself and re-run this command.
# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.
bot-not-set-up = Looks like you haven't set up the bot yet. Do that first with `{ $contextPrefix }setup`.
# This message is shown on successfuly joining a voice channel.
# { $targetMention } is the mention of the channel the bot joined.
join-success = Successfully joined { $targetMention }.
    
    {"**"}Keep in mind that the bot is not perfect, and works best when you speak slowly (roughly 40-60 words per minute) and clearly.**
    
    Note: your current premium tier is { $tier }. This allows for { $maxUsers } users to be transcripted at once. Along with this, the bot will automatically leave after { $leaveDuration } seconds, regardless of how many users are in the channel. This is to prevent abuse of our systems.
    If you would like more users, a longer duration of usage, and would like to also support the bot, consider subscribing to our Premium: <https://dash.scripty.org/premium>
    { $freeTrialUpsell }
# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.
webhook-deleted = Looks like you deleted the webhook I use! *bonk* Re-run `{ $contextPrefix }setup` to fix this.
# This message is shown when the user attempts to make Scripty join a voice channel, but there is no one in the channel.
join-no-one-in-channel = There's no one in { $targetMention }. I'm not joining if there's no one there, as that's a waste of limited resources.
# This message is shown when Discord tosses a Dropped error when trying to join a voice channel.
join-failed-dropped = Discord appears to be having issues, we cannot do anything about this. Please try again later.

## ping command
# This is shown as the title of the ping command (eg what shows up in the slash command picker)
ping-command-name = ping
# This is shown as the description of the ping command (eg what shows up in the slash command picker)
ping-command-description = Get the bot latency.

# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    WebSocket latency: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    HTTP latency: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Database latency: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)
    
    Note: if any latency is equal to 0ms, it means that specific latency could not be calculated right now.
    Try again later.

## setup command
# This is shown as the title of the setup command (eg what shows up in the slash command picker)
setup-command-name = setup
# This is shown as the description of the setup command (eg what shows up in the slash command picker)
setup-command-description =
    Set the bot up.
    
    This will initialize the bare framework of the bot,
    allowing you to use `~join` to bind the bot to a voice chat.
setup-command-argument1-description = Channel to send transcriptions to. (Required)
setup-command-argument2-description = Target language to run the STT algorithm in. (Optional, defaults to English)
setup-command-argument3-description = During transcriptions, be verbose? This adds no extra overhead. (Optional, defaults to false)
setup-tos-agree =
    By setting up Scripty, you agree to both its Privacy Policy and Terms of Service.
    Privacy Policy: https://scripty.org/privacy
    Terms of Service: https://scripty.org/terms
setup-tos-agree-failure = You must agree to both the Terms of Service and the Privacy Policy to use Scripty. Cancelling setup.
setup-success-title = Set up successfully!
setup-success-description =
    A couple notes:
    
    1) The bot is extremely expensive to run, and requires a serious amount of processing power, so it'd be amazing if you could donate a bit. We offer premium tiers that come with many more features than the free version. You can find a comparison and a list of premium tiers at <https://scripty.org/premium>. The core features will stay free forever, though.

    2) If you chose a language other than English (the default) note that transcriptions for it will be much, much lower quality. You can see each language's accuracy at https://scripty.org/languages.
    
    Thanks for checking out Scripty! <3
    ~ 0/0 + valkyrie_pilot

## data_storage command
# This is shown as the title of the data_storage command (eg what shows up in the slash command picker).
data-storage-command-name = data_storage
# This is shown as the description of the data_storage command. (eg what shows up in the slash command picker).
data-storage-command-description = Configure storage settings for your data
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
# This is shown as the title of the automod root command (eg what shows up in the slash command picker)
automod-root-command-name = automod
# This is shown as the description of the automod root command (eg what shows up in the slash command picker)
automod-root-command-description = Configure automod settings
automod-root-response = This is the root command, due to Discord limitations it does nothing. See `{ $contextPrefix }help automod` for more info.

## automod setup command
# This is shown as the title of the automod setup command (eg what shows up in the slash command picker)
automod-setup-command-name = setup
# This is shown as the description of the automod setup command (eg what shows up in the slash command picker)
automod-setup-command-description = Get started with Scripty's automod.
automod-setup-argument1-description = The channel to send automod logs to.
automod-setup-argument2-description = Should a recording of offending speech be sent to the target channel? Defaults to false.
automod-setup-argument3-description = Should the bot automatically join voice if a user joins? Defaults to true.
automod-setup-embed-complete-title = Automod setup complete!
automod-setup-embed-complete-description = You can now use `{ $contextPrefix }automod rule add` to add an automod rule. { $extraDetails }
automod-setup-embed-complete-free-limit = Note that free servers are limited to 25 rules. If you'd like to remove this limit, check out our Premium over at https://scripty.org/premium.
automod-setup-embed-not-setup-title = The bot has not been set up!
automod-setup-embed-not-setup-description = Set it up first by running `{ $contextPrefix } setup`.
automod-setup-auto-join-premium-only = Auto-join is a premium feature. Check out our Premium over at https://scripty.org/premium.

## automod add rule command
# This is shown as the title of the automod add rule command (eg what shows up in the slash command picker)
automod-add-rule-command-name = add_rule
# This is shown as the description of the automod add rule command (eg what shows up in the slash command picker)
automod-add-rule-command-description = Add an automod rule.
automod-add-rule-argument1-description = The type of rule to add. See `/automod rule_help` for more info.
automod-add-rule-argument1-enum-regular-type = Regular
automod-add-rule-argument2-description = The rule content to add.
automod-add-rule-argument3-description = The action to take when the rule is triggered.
automod-add-rule-argument3-enum-silent-delete = Silent delete
automod-add-rule-argument3-enum-delete-and-log = Delete and log
automod-add-rule-argument3-enum-delete-log-and-kick = Delete, log, and remove user from voice
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
# This is shown as the title of the automod remove rule command (eg what shows up in the slash command picker)
automod-remove-rule-command-name = remove_rule
# This is shown as the description of the automod remove rule command (eg what shows up in the slash command picker)
automod-remove-rule-command-description = Remove an automod rule.
automod-remove-rule-argument1-description = The rule ID to remove.
automod-remove-rule-embed-success-title = Rule removed!
automod-remove-rule-embed-success-description = { $rulesLeft } rules left out of { $maxRules }.
automod-remove-rule-embed-failure-title = Failed to remove rule!
automod-remove-rule-embed-failure-description-invalid-id = Invalid rule ID. See `{ $contextPrefix }automod list` for more info.
automod-remove-rule-embed-failure-description-not-setup = You must run `{ $contextPrefix }automod setup` before removing rules.

## automod list rules command
# This is shown as the title of the automod list rules command (eg what shows up in the slash command picker)
automod-list-rules-command-name = list_rules
# This is shown as the description of the automod list rules command (eg what shows up in the slash command picker)
automod-list-rules-command-description = List all automod rules.
automod-list-rules-argument1-description = Filter rules by their content. Leave empty to show all rules.
automod-list-rules-embed-title = Automod rules
automod-list-rules-embed-description = { $rulesLeft } rules left out of { $maxRules }.
automod-list-rules-embed-field-name = Rule { $ruleId }
automod-list-rules-embed-field-value = Type: { $ruleType }
    Content: { $ruleContent }
    Action: { $ruleAction }
automod-list-rules-footer = Page { $page } of { $maxPage }
automod-list-rules-no-rules = You don't have any rules!

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
# This is shown as the title of the delete_all_data command (eg what shows up in the slash command picker)
delete-data-command-name = delete_all_data
# This is shown as the description of the data_storage command (eg what shows up in the slash command picker)
delete-data-command-description = Delete all your data.
    
    This command will irreversibly, permanently, delete all your data. There is no undoing this action.

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

## Credits command
# This is shown as the title of the credits command (eg what shows up in the slash command picker)
credits-command-name = credits
# This is shown as the description of the credits command (eg what shows up in the slash command picker)
credits-command-description = A list of all the things that made Scripty possible.
# Embed title for the credits command
credits-title = Credits
credits-description =
    This is a list of people who have contributed to Scripty.
    A huge thanks goes out to everyone here <3
credits-field1-title = Core Developers
credits-field1-description = 0/0 and valkyrie_pilot
credits-field2-title = Translators
credits-field2-description = Many, many people have helped translate the bot. See the `{ $contextPrefix }translators` command for a list of translators.
credits-field4-title = All Our Users
credits-field4-description = Yes, this includes you! We appreciate you using Scripty, and we hope you enjoy using it.

## Leave command
# This is shown as the title of the leave command (eg what shows up in the slash command picker)
leave-command-name = leave
# This is shown as the description of the leave command (eg what shows up in the slash command picker)
leave-command-description = Leave any current voice call.
# This is shown when the bot successfully leaves a voice call
leave-success = Left VC successfully.
