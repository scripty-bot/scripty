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
# premium command
# This is shown when the guild the user is running this command in has not yet agreed to the ToS.
premium-server-not-set-up = this sewvew has not yet agweed to s-scwipty's tos and p-pwivacy powicy. 🥺 d-do that fiwst w-with the `{ $commandPrefix }tewms_of_sewvice` command.
# join command
# This message is shown on successfuly joining a voice channel.
# { $targetMention } is the mention of the channel the bot joined.
join-success =
    successfuwwy joined { $voiceTargetMention }, nyaa~~ and s-sending twanscwiption o-output to { $outputChannelMention }. /(^•ω•^)
    { "" }
    n-nyote: youw cuwwent p-pwemium tiew i-is { $tier }. rawr t-this awwows fow { $maxUsers } usews t-to be twanscwipted a-at once. OwO awong with this, (U ﹏ U) the bot wiww automaticawwy weave aftew { $leaveDuration } s-seconds, >_< wegawdwess of how many usews a-awe in the channew. rawr x3 this is to p-pwevent abuse of ouw systems. mya
    if you wouwd wike mowe usews, nyaa~~ a wongew d-duwation of usage, (⑅˘꒳˘) and wouwd w-wike to awso s-suppowt the bot, rawr x3 considew subscwibing to ouw pwemium: <https://dash.scripty.org/premium>
    if you know you awe a pwemium s-subscwibew awweady, (✿oωo) pwease dm the bot that way we can weinstate youw pwemium. (ˆ ﻌ ˆ)♡
    { $freeTrialUpsell }
# join command
# This message is shown when the bot does not have permissions for the voice channel it is trying to join.
join-no-permission = i don't have pewmission to join { $targetMention }. (U ᵕ U❁) p-pwease give me t-the view channew a-and join pewmissions, (⑅˘꒳˘) o-ow join a-a diffewent voice c-chat whewe i d-do have pewmissions. ( ͡o ω ͡o )
# ToS command
# This is sent when the user has not yet agreed to the ToS and must do so.
agreeing-to-tos = you can view scwipty's tewms of sewvice a-and pwivacy p-powicy at https://scripty.org/terms a-and https://scripty.org/privacy w-wespectivewy. UwU y-you may cwick t-the button bewow t-to agwee to b-both of these documents, rawr x3 and use scwipty. rawr
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
# ToS command
# This and all attributes show up exclusively in the slash command picker when `terms_of_service` is selected.
cmds_terms_of_service = tewms_of_sewvice
    .description = view and agwee to scwipty's tewms o-of sewvice and p-pwivacy powicy. ^•ﻌ•^
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to select a button in time.
tos-agree-timed-out = timed out. ^•ﻌ•^ wewun this command if y-you stiww want to a-agwee to the tos. OwO
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = Weft vc successfuwwy.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user agrees to the ToS.
tos-agree-success = you have successfuwwy agweed to scwipty's t-tewms of s-sewvice and pwivacy p-powicy. 🥺 you m-may nyow use scwipty.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to agree to the ToS, usually by explicitly clicking the "No" button.
disagreed-to-tos = Uwu have disagweed tuwu scwipty's tewms of sewvice awnd pwivacy powicy. If uwu wouwd wike tuwu use scwipty, uwu must agwee tuwu these documents. Uwu may duwu so by wunning thiws command again.
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
