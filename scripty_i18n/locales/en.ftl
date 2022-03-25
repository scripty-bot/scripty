donation-title = Donations
donation-description =
    Scripty is not cheap to run. It is currently running on a $1,500 USD server with an AMD Ryzen 9 3900 CPU and 128GB RAM. Even with that hardware, we estimate it can only handle 100 concurrent transcriptions. Donations would allow us to scale our hardware capacity and handle many more concurrent transcriptions, perhaps tens of thousands someday with enough donations.
    
    Training a model is not easy either, as that needs relatively recent (CUDA 10.1 support) Nvidia GPUs. We hate asking for donations, but we absolutely can't support the bot out of our own pockets, since it's just too expensive. So we're asking for help, and giving rewards in the form of premium subscriptions.
    
    You can view more info at https://scripty.org/premium, but the gist of it is that there are 6 tiers ranging in price from $5 USD to $100 USD per month. The $100 tier comes with its own managed instance of Scripty for your own server, with a custom name, and profile picture.
    
    We also support one-time donations directly through GitHub Sponsors:
    { "[https://github.com/sponsors/tazz4843](https://github.com/sponsors/tazz4843?frequency=one-time&sponsor=tazz4843)" }
    
    You can view these tiers at https://scripty.org/premium.
    
    <3
    ~ 0/0 and valkyrie_pilot

## Help menu translation strings

command-not-found = No command with name `{ $commandName }` found.
command-not-found-suggestions = Did you mean `{ $suggestion }`?
no-help-found = No help found for command `{ $commandName }`.
default-category-name = Commands

## Context menu command translation strings

context-menu-command-title =
    { "" }
    Context menu commands:
    { "" }
context-menu-command-user =
    { "  " }
    { $commandName } (on user)
    { "" }
context-menu-command-message =
    { "  " }
    { $commandName } (on message)
    { "" }
more-info-on-command =
    { "" }
    For more information on a specific command, type `{ $contextPrefix }help <name>`
    ```

## Language configuration strings

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

# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = You're not in a voice chat, nor did you tell me a channel to join. Try `{ $contextPrefix }join <channel>` to specify a voice chat, or join a voice chat yourself and re-run this command.
# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.
bot-not-set-up = Looks like you haven't set up the bot yet. Do that first with `{ $contextPrefix }setup`.
# This message is shown on successfuly joining a voice channel.
# {$targetMention} is the mention of the channel the bot joined.
join-success = Successfully joined { $targetMention }.
# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.
webhook-deleted = Looks like you deleted the webhook I use! *bonk* Re-run `{ $contextPrefix }setup` to fix this.

## ping command

# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    WebSocket latency: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    HTTP latency: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Database latency: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)
    
    Note: if any latency is equal to 0ms, it means that specific latency could not be calculated right now.
    Try again later.

## setup command

setup-description =
    Set the bot up.
    
    This will initialize the bare framework of the bot,
    allowing you to use `~join` to bind the bot to a voice chat.
    
    Argument 1 is the channel to send all transcriptions to.
    
    Argument 2 is optional, and is the language to use for transcription.
    
    Argument 3 is optional, and defines whether or not the bot should be verbose.
setup-arg1-description = The channel to send all transcriptions to.
setup-arg2-description = The language to use for transcription.
setup-arg3-description = Whether or not the bot should be verbose.
setup-tos-agree =
    By setting up Scripty, you agree to both its Privacy Policy and Terms of Service.
    Privacy Policy: https://scripty.org/privacy
    Terms of Service: https://scripty.org/terms
setup-tos-agree-failure = You must agree to both the Terms of Service and the Privacy Policy to use Scripty. Cancelling setup.
setup-success-title = Set up successfully!
setup-success-description =
    A couple notes:
    
    
    1) Do not delete the webhook that was created in the target channel.
    
    2) The bot is extremely expensive to run, and requires a serious amount of processing power, so it'd be amazing if you could donate a bit. We offer premium tiers that boost the limit on the number of users transcripted, which defaults to 5. The core features will stay free forever, though. If you're interested, check out the `{ $contextPrefix }donate` command.
    
    3) If you chose a language other than English (the default) note that transcriptions for it will be much, much lower quality. Soon we will be adding a feature that allows you to help transcription accuracy with your own voice (see note 5).
    
    4) If you are not a middle-aged American male, expect lower transcription accuracy. This is due to inherent bias within the model, and the only thing we can do about it is train more accurate models (again, see note 5).
    
    5) To help us train more accurate models, consider allowing us to store your audio and transcriptions for training. See the `{ $contextPrefix }train_storage` command.
    
    6) I don't exactly want to ask again, but please consider donating. It takes an ***insane*** amount of processing power to train new models (we're talking multiple Nvidia RTX 3090 GPUs), and every little bit of money helps a lot. Again, if you're interested, check out the `{ $contextPrefix }donate` command.
    
    
    Thanks for checking out Scripty! <3
    ~ 0/0 + valkyrie_pilot

## train_storage command

data-storage-embed-title = Data Storage
data-storage-embed-description = **NOTE**: everything that follows is **entirely optional**, and opting out **will not**, in any way, affect your experience with Scripty.
    That said, here goes.
    
    Scripty requires a lot of audio and text data to train a proper speech-to-text model. Not everyone is able to donate or buy premium to help us out, so a big way you can help out is by allowing us to store your data like audio and messages for training a model.
    We understand this data can be extremely personal, so this is entirely opt-in and will not affect your experience in any way.
    
    Here's what we'd do with it:
None = * With stored messages, we would feed them into a scorer targeted to your language. This scorer would allow the algorithm to select the most likely words for a given set of sounds. Although immensely helpful, this isn't as important as audio.
data-storage-toggle-audio-btn = Toggle Audio Storage
data-storage-toggle-msgs-btn = Toggle Message Storage
data-storage-opted-in-audio = You are now opted into storing your audio for model training.
data-storage-opted-out-audio = You are now opted out of storing your audio for model training.
data-storage-opted-in-msgs = You are now opted into storing your messages for scorer training.
data-storage-opted-out-msgs = You are now opted out of storing your messages for scorer training.
data-storage-command-timed-out = Timed out. Rerun this command if you still want to manage settings.

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
test = test
