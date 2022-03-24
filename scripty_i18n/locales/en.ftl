donation-title = Donations
donation-description = Scripty is not cheap to run. It is currently running on a $1,500 USD server with an
 AMD Ryzen 9 3900 CPU and 128GB RAM. Even with that hardware, we estimate it can only handle
 100 concurrent transcriptions. Donations would allow us to scale our hardware capacity and
 handle many more concurrent transcriptions, perhaps tens of thousands someday with enough donations.\n\n
 Training a model is not easy either, as that needs relatively recent (CUDA 10.1 support)
 Nvidia GPUs. We hate asking for donations, but we absolutely can't support the bot out of our
 own pockets, since it's just too expensive. So we're asking for help, and giving rewards in the
 form of premium subscriptions.\n\n
 You can view more info at https://scripty.org/premium, but the gist of it is that there are 6
 tiers ranging in price from $5 USD to $100 USD per month. The $100 tier comes with its own
 managed instance of Scripty for your own server, with a custom name, and profile picture.\n\n
 We also support one-time donations directly through GitHub Sponsors:
 {"[https://github.com/sponsors/tazz4843](https://github.com/sponsors/tazz4843?frequency=one-time&sponsor=tazz4843)"}\n\n\n
 https://scripty.org/premium

# Help menu translation strings
command-not-found = No command with name `{ $commandName }` found.
command-not-found-suggestions = Did you mean `{ $suggestion }`?
no-help-found = No help found for command `{ $commandName }`.
default-category-name = Commands
# Context menu command translation strings
context-menu-command-title = \nContext menu commands:\n
context-menu-command-user =   {$commandName} (on user)\n
context-menu-command-message =   {$commandName} (on message)\n
more-info-on-command = \nFor more information on a specific command, type `{$contextPrefix}help <name>`\n```

# Language configuration strings
user-language-set-success = User language set to `{$language}`.
user-language-set-success-description = To return to English, type `{$contextPrefix}language user_language en`.
guild-language-set-success = Guild language set to `{$language}`.
guild-language-set-success-description = To return to English, type `{$contextPrefix}language guild_language en`.

language-set-failure-title-unsupported = The language you specified is not supported by the bot.
language-set-failure-description-unsupported = If you'd like to help with adding support for this
 language, please join the support server at {$supportServerInvite}.
language-set-failure-title-invalid = Language `{$language}` not found.
language-set-failure-description-invalid = The language you specified is an invalid language identifier.
language-set-failure-title-db = Database error.
language-set-failure-description-db = The database encountered an error while attempting to set your language.
 This error has been reported, and we'll look into it. Please do not spam this command. (If you're curious, here's the error: {$error})

# Command invocation contexts
root-command-invoked-title = This is a root command!
root-command-invoked-description = Please invoke only this command's subcommands to use it.\nSee
 `{$contextPrefix}help {$commandName}` for more info.


