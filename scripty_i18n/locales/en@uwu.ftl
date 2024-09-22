# join command
# This message is shown when Discord tosses a Dropped or TimedOut error when trying to join a voice channel.
join-failed-dropped = discowd appeaws to be having issues, 🥺 w-we cannot do a-anything about t-this. òωó pwease twy a-again watew. o.O
# join command
# If the user specifies they would like to create a thread, this is set as the thread name. { $timestamp } is the current timestamp, in ISO format.
join-thread-title = twanscwiption fwom { $timestamp }
# join command
# If the user specifies they would like to create a forum post, this is the contents of the initial message. { $timestamp } is the current timestamp, in ISO format, and { $authorMention } is the mention of the user who ran the command.
join-forum-thread-content = { $authorMention } stawted a twanscwiption at { $timestamp }. >_<
# Help menu translation strings
default-category-name = commands
# premium command
# This is shown when the user successfully claims one of their premium subscriptions.
premium-claimed = you have successfuwwy cwaimed pwemium o-on this sewvew. rawr i-if you wouwd w-wike to upgwade, σωσ o-ow puwchase mowe s-swots, σωσ head t-to <https://dash.scripty.org/premium>. >_< i-if you wouwd w-wike to wemove youw pwemium fwom this guiwd, :3 wun `{ $commandPrefix }pwemium wemove`. (U ﹏ U)
# Context menu command translation strings
context-menu-command-title =
    { "" }
    context menu commands:
    { "" }
# premium command
# This is shown to the user when they have too many used servers to add more.
premium-too-many-guilds = you have cwaimed { $totalServers } pwemium keys. y-you cannot add any m-mowe, ( ͡o ω ͡o ) unwess y-you upgwade youw p-pwemium subscwiption a-at <https://dash.scripty.org/premium>, UwU o-ow w-wemove some with t-the `{ $commandPrefix }pwemium wemove` command. rawr x3
# premium command
# This is shown when the user successfully removes their premium from this guild.
premium-removed = if you awe the usew who had cwaimed p-pwemium, ( ͡o ω ͡o ) you h-have nyow successfuwwy w-wemoved youw p-pwemium fwom t-this sewvew. UwU if y-you wouwd wike t-to upgwade, rawr x3 ow puwchase m-mowe swots, rawr head to <https://dash.scripty.org/premium>.
# Help menu translation strings
command-not-found = no command with name `{ $commandName }` found. >_<
# Context menu command translation strings
context-menu-command-user =
    { "" }
    { $commandName } (on usew)
    { "" }
# Help menu translation strings
command-not-found-suggestions = did you mean `{ $suggestion }`?
# Context menu command translation strings
context-menu-command-message =
    { "" }
    { $commandName } (on message)
    { "" }
# Help menu translation strings
no-help-found = no hewp found fow command `{ $commandName }`. >_<
# join command
# This message is shown when the bot does not have permissions for the voice channel it is trying to join.
join-no-permission = i don't have pewmission to join { $targetMention }. (U ᵕ U❁) p-pwease give me t-the view channew a-and join pewmissions, (⑅˘꒳˘) o-ow join a-a diffewent voice c-chat whewe i d-do have pewmissions. ( ͡o ω ͡o )
# join command
# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = you'we nyot in a voice chat, ( ͡o ω ͡o ) nyow d-did you teww me a-a channew to join. UwU t-twy `{ $contextPrefix }join <channew>` t-to specify a-a voice chat, rawr x3 o-ow join a voice c-chat youwsewf a-and we-wun this command. rawr
# join command
# This message is shown when the user attempts to make Scripty join a voice channel, but there is no one in the channel.
join-no-one-in-channel = thewe's nyo one in { $targetMention }. 🥺 i'm nyot joining i-if thewe's n-nyo one thewe, òωó a-as that's a waste o-of wimited wesouwces. o.O
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = you awe nyot a pwemium subscwibew. (U ᵕ U❁) s-subscwibe at https://scripty.org/premium. (⑅˘꒳˘) i-if you k-know you awe o-one, ( ͡o ω ͡o ) pwease dm the b-bot that way w-we can weinstate y-youw pwemium. UwU
# Context menu command translation strings
more-info-on-command =
    fow mowe infowmation on a specific c-command, (ꈍᴗꈍ) type `{ $contextPrefix }help <name>`
    ```
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = Weft vc successfuwwy.
# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = The fowum channew uwu twied tuwu make me use wequiwes tags. I have no way of knowing whawt tags tuwu use, so i cannot join thawt channew. Pwease use a diffewent channew, ow awsk an admin tuwu wemove the tag wequiwement.
# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = join
    .description = join a voice chat. 🥺 twanscwipts wiww b-be wogged to t-the channew you w-wun this command i-in. òωó
    .voice_channel = voice_channew
    .voice_channel-description = voice chat to bind to
    .record_transcriptions = wecowd_twanscwiptions
    .record_transcriptions-description = wog aww twanscwipts? usews wiww be d-dmed when scwipty w-weaves the channew. 🥺 d-defauwts t-to fawse. òωó
    .target_channel = tawget_channew
    .target_channel-description = send twanscwipts hewe, 🥺 instead of t-the cuwwent channew. òωó t-tawget a fowum t-to cweate a n-nyew post. o.O
    .create_thread = cweate_thwead
    .create_thread-description = cweate a nyew thwead fow this twanscwiption? d-defauwts t-to fawse. ^•ﻌ•^
# Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = weave
    .description = weave any cuwwent voice caww. >_<
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = hewp
    .description = show this hewp menu
    .command = command
    .command-description = specific command to show hewp about
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = pwemium
    .description = pwemium commands
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = cwaim
    .description = cwaim youw pwemium within the sewvew w-whewe this command i-is exekawaii~d. ^•ﻌ•^
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = wemove
    .description = wemove youw pwemium fwom the sewvew w-whewe this command i-is exekawaii~d. ^•ﻌ•^
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language user_language` is selected.
cmds_user_language = usew
    .description = set youw usew wanguage to one of t-the avaiwabwe wanguages. (ꈍᴗꈍ)
    .language = language
    .language-description = the wanguage you want to set youw u-usew wanguage to. (ꈍᴗꈍ)
