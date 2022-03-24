donation-title = Donations
donation-description = Scripty is not cheap to run. It is currently running on a $1,500 USD server with an AMD Ryzen 9 3900 CPU and 128GB RAM. Even with that hardware, we estimate it can only handle 100 concurrent transcriptions. Donations would allow us to scale our hardware capacity and handle many more concurrent transcriptions, perhaps tens of thousands someday with enough donations.

 Training a model is not easy either, as that needs relatively recent (CUDA 10.1 support) Nvidia GPUs. We hate asking for donations, but we absolutely can't support the bot out of our own pockets, since it's just too expensive. So we're asking for help, and giving rewards in the form of premium subscriptions.

 You can view more info at https://scripty.org/premium, but the gist of it is that there are 6 tiers ranging in price from $5 USD to $100 USD per month. The $100 tier comes with its own managed instance of Scripty for your own server, with a custom name, and profile picture.

 We also support one-time donations directly through GitHub Sponsors:
 {"[https://github.com/sponsors/tazz4843](https://github.com/sponsors/tazz4843?frequency=one-time&sponsor=tazz4843)"}

 You can view these tiers at https://scripty.org/premium.

 <3
 ~ 0/0 and valkyrie_pilot

## Help menu translation strings
command-not-found = No command with name `{ $commandName }` found.
command-not-found-suggestions = Did you mean `{ $suggestion }`?
no-help-found = No help found for command `{ $commandName }`.
default-category-name = Commands
## Context menu command translation strings
context-menu-command-title = {""}
 Context menu commands:
 {""}
context-menu-command-user = {"  "}
 {$commandName} (on user)
 {""}
context-menu-command-message = {"  "}
 {$commandName} (on message)
 {""}
more-info-on-command = {""}
 For more information on a specific command, type `{$contextPrefix}help <name>`
 ```

## Language configuration strings
# This message is shown as the embed title when the user sets their language successfully.
user-language-set-success = User language set to `{$language}`.
# This message is shown as the embed description when the user sets their language successfully.
user-language-set-success-description = To return to English, type `{$contextPrefix}language user_language en`.

# This message is shown as the embed title when the guild sets their language successfully.
guild-language-set-success = Guild language set to `{$language}`.
# This message is shown as the embed description when the guild sets their language successfully.
guild-language-set-success-description = To return to English, type `{$contextPrefix}language guild_language en`.

# This message is shown as the embed title when an entity tries to set their language to an unsupported language.
language-set-failure-title-unsupported = The language you specified is not supported by the bot.
# This message is shown as the embed description when an entity tries to set their language to an unsupported language.
language-set-failure-description-unsupported = If you'd like to help with adding support for this language, please join the support server at {$supportServerInvite}.

# This message is shown as the embed title when an entity tries to set their language to an invalid language.
language-set-failure-title-invalid = Language `{$language}` not found.
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = The language you specified is an invalid language identifier. Reason: {$error}

# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = Database error.
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = The database encountered an error while attempting to set your language. This error has been reported, and we'll look into it. Please do not spam this command. (If you're curious, here's the error: {$error})

## Command invocation contexts
# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = This is a root command!
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = Please invoke only this command's subcommands to use it. See `{$contextPrefix}help {$commandName}` for more info.


## join command
# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = You're not in a voice chat, nor did you tell me a channel to join. Try `{$contextPrefix}join <channel>` to specify a voice chat, or join a voice chat yourself and re-run this command.

# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.
bot-not-set-up = Looks like you haven't set up the bot yet. Do that first with `{$contextPrefix}setup`.

# This message is shown on successfuly joining a voice channel.
# {$targetMention} is the mention of the channel the bot joined.
join-success = Successfully joined {$targetMention}.

# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.
webhook-deleted = Looks like you deleted the webhook I use! *bonk* Re-run `{$contextPrefix}setup` to fix this.


## ping command
# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
 WebSocket latency: {$wsLatencyMs}ms ({$wsLatencyNs}ns)
 HTTP latency: {$httpLatencyMs}ms ({$httpLatencyNs}ns)
 Database latency: {$pgLatencyMs}ms ({$pgLatencyNs}ns)

 Note: if any latency is equal to 0ms, it means that specific latency could not be calculated right now.\n
 Try again later.
