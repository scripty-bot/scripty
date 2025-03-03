join-success-footer-free-trial-upsell = இந்த சேவையகம் பிரீமியத்தின் இலவச சோதனைக்கு தகுதியானது. ஒன்று கோர போட்.
join-no-permission = I don't have permission to join { $targetMention }. Please give me the View Channel and Join permissions, or join a different voice chat where I do have permissions.
join-forum-thread-content = { $authorMention } started a transcription { $timestamp }.
join-ephemeral-not-thread = இடைக்கால அளவுருவைப் பயன்படுத்த, `create_thread` ஐ உண்மைக்கு அமைப்பதன் மூலம் அல்லது` இலக்கு_சானல்` உடன் ஒரு நூலை குறிவைப்பதன் மூலம் நீங்கள் ஒரு நூலை இலக்காக தேர்ந்தெடுக்க வேண்டும்.
config_transcribe_voice_messages = transcribe_voice_messages
    .description = Toggle whether Scripty transcribes voice messages.
    .transcribe_voice_messages = transcribe_voice_messages
    .transcribe_voice_messages-description = Defaults to true
guild-language-set-failure-translate-enabled = உங்கள் சேவையகத்தில் தானாக மொழிபெயர்ப்பு இயக்கப்பட்டிருக்கிறது. இது ஆங்கிலத்திற்கு மொழிபெயர்க்கும்போது மட்டுமே ஆதரிக்கப்படுகிறது. உங்கள் மொழியை அமைக்க விரும்பினால் இந்த அம்சத்தை முடக்கு.
language-set-failure-description-invalid = The language you specified is an invalid language identifier. Reason: { $error }
root-command-invoked-description = Please invoke only this command's subcommands to use it. See `{ $contextPrefix }help { $commandName }` for more info.
automod-add-rule-embed-failure-description-free-limit = இலவச சேவையகங்கள் 25 வழக்கமான விதிகளுக்கு மட்டுமே. இந்த வரம்பை அதிகரிக்க நீங்கள் விரும்பினால், எங்கள் பிரீமியத்தை https://scripty.org/premium இல் பாருங்கள்.
automod-add-rule-embed-failure-description-invalid-type = Invalid rule type. See `{ $contextPrefix }automod rule_help` for more info.
automod-add-rule-embed-failure-description-free-locked-type = இலவச சேவையகங்கள் வழக்கமான விதிகளை மட்டுமே பயன்படுத்த முடியும். நீங்கள் பிற விதி வகைகளைப் பயன்படுத்த விரும்பினால், எங்கள் பிரீமியத்தை https://scripty.org/premium இல் பாருங்கள்.
automod-add-rule-embed-failure-description-not-setup = You must run `{ $contextPrefix }automod setup` before adding rules.
voice-connection-error-host-io-error = புரவலன் io பிழை
voice-connection-error-timed-out = இணைப்பிற்காக காத்திருக்கும் நேரம்
voice-connection-error-ws-closed-unknown-opcode = அறியப்படாத ஆப்கோட் காரணமாக முரண்பாடு மூடிய இணைப்பு
voice-connection-error-ws-closed-not-authenticated = அங்கீகரிக்கப்படாததால் முரண்பாடு மூடிய இணைப்பு
cmds_remove_rule = remove_rule
    .description = Remove an automod rule.
    .rule_id = rule_id
    .rule_id-description = The rule ID to remove.
automod-remove-rule-embed-failure-description-invalid-id = Invalid rule ID. See `{ $contextPrefix }automod list` for more info.
cmds_list_rules = list_rules
    .description = List all automod rules.
    .filter_by = filter_by
    .filter_by-description = Filter rules by their content. Leave empty to show all rules.
automod-list-rules-embed-field-name = Rule { $ruleId }
automod-list-rules-embed-field-value =
    Type: { $ruleType }
    Content: { $ruleContent }
    Action: { $ruleAction }
vote-reminders-enabled = வாக்கு நினைவூட்டல்கள் இயக்கப்பட்டன.
voice-connection-error-internal-lib-error = நூலக உள் பிழை
voice-connection-error-ws-closed-already-authenticated = ஏற்கனவே அங்கீகரிக்கப்பட்டதால் முரண்பாடு மூடிய இணைப்பு
general-error-command-process-title = An error happened while processing { $command }.
general-error-invalid-structure-title = Invalid structure from Discord while parsing { $command }.
general-error-invalid-structure-description =
    { $description }

    { "**" }Note**: this is a Discord error.
    The only fix for this is to wait for Discord to propagate slash commands, which can take up to one hour.
    If you do not want to wait this hour, you should use the prefix commands: run this command with `~{ $qualifiedName } { $args }`.
general-error-user-missing-perms-description-not-owner = இந்த போட் உரிமையாளர் அல்ல.
general-error-command-check-failed-description-no-reason = எந்த காரணமும் வழங்கப்படவில்லை
transcription-info-transcript-count = Transcript 1 of { $count }.
voice-connection-error-ws-closed-session-invalid = முரண்பாடு செல்லாத அமர்வு
voice-connection-error-ws-closed-session-timeout = அமர்வு நேரம் முடிந்தது
voice-connection-error-ws-closed-unknown-protocol = முரண்பாடு நெறிமுறையை அங்கீகரிக்கவில்லை
voice-connection-error-ws-closed-unknown-encryption-mode = குறியாக்கத் திட்டத்தை முரண்பாடு அங்கீகரிக்கவில்லை
transcription-info-transcription-title = படியெடுத்தல்
transcription-info-transcription-ssrc = SSRC { $ssrc }
config-transcribe-audio-enabled = ச்கிரிப்டி இப்போது ஆடியோ கோப்புகளை படியெடுக்கும்.
config-transcribe-only-role-enabled = Scripty will now only transcribe messages from users in { $roleId }.
config-transcribe-only-role-disabled = ச்கிரிப்டி இப்போது அனைத்து பயனர்களையும் பாத்திரத்தைப் பொருட்படுத்தாமல் படியெடுக்கும்.
join-create-thread-in-unsupported = Discord does not support threads in { $targetMention }. Please use a different channel, or do not create a thread.
data-storage-embed-title = தரவு சேமிப்பு
delete-data-title = தரவை நீக்கு
language-set-failure-description-db = The database encountered an error while attempting to set your language. This error has been reported, and we'll look into it. Please do not spam this command. (If you're curious, here's the error: { $error })
cmds_ping = ping
    .description = Get the bot latency.
voice-connection-error-msg-no-reconnect = I had an issue ({ $reason }) and disconnected from the voice chat.
join-success-description = Successfully joined { $voiceTargetMention }, and sending transcription output to { $outputChannelMention }.
join-success-premium = இந்த சேவையகத்தின் காப்பீடு நிலையை `/பிரீமியம் தகவல்` உடன் பார்க்கலாம்.
join-success-help-title = உதவி தேவையா?
join-success-help-description = You can either join the support server at { $supportServerInvite }, or DM the bot.
cmds_transcribe_message = transcribe_message
    .description = Transcribe a message. Reply to a message to transcribe it.
cmds_premium = premium
    .description = Premium commands
cmds_premium_remove = remove
    .description = Remove your premium from the server where this command is executed.
cmds_premium_info = info
    .description = Get information on this server's Scripty Premium status.
premium-info-embed-title = காப்பீடு நிலை
premium-info-embed-description-no-subscription = <Https://dash.scripty.org/premium> இல் நீங்கள் பிரீமியத்திற்கு குழுசேரலாம். நீங்கள் பெறும் சலுகைகளுக்கு மேல், ச்கிரிப்ட்டை பேச்சு-க்கு-உரைக்கான சிறந்த போட் ஆக மாற்ற எங்கள் குறிக்கோளில் எங்களுக்கு உதவுகிறீர்கள் :)
premium-info-embed-description-has-subscription = உங்கள் சந்தாவை <https://dash.scripty.org/premium> இல் நிர்வகிக்கலாம். ச்கிரிப்ட்டை ஆதரித்ததற்கு நன்றி!
premium-info-embed-current-tier = தற்போதைய அடுக்கு
premium-info-embed-max-users = அதிகபட்ச ஒரே நேரத்தில் பயனர்கள்
premium-info-embed-manage-subscription-user-has-unclaimed-title = நீங்கள் காப்பீடு வாங்கியதாகத் தெரிகிறது!
premium-info-embed-manage-subscription-user-has-unclaimed-description = To claim it in this server, run { $claimCommand }.
cmds_config_verbose = verbose
    .description = Toggle whether Scripty is verbose during transcriptions.
    .verbose = verbose
    .verbose-description = Defaults to false
command-not-found-suggestions = Did you mean `{ $suggestion }`?
no-help-found = No help found for command `{ $commandName }`.
context-menu-command-message =
    { "" }
    { $commandName } (on message)
    { "" }
context-menu-command-unknown =
    { "" }
    { $commandName } (on unknown)
    { "" }
user-language-set-success-description = To return to English, type `{ $contextPrefix }language user_language en`.
language-set-failure-title-unsupported = நீங்கள் குறிப்பிட்ட மொழி போட் ஆதரிக்கவில்லை.
language-set-failure-title-invalid = Language `{ $language }` not found.
data-storage-toggle-audio-btn = ஆடியோ சேமிப்பிடத்தை மாற்றவும்
data-storage-toggle-msgs-btn = செய்தி சேமிப்பகத்தை மாற்றவும்
cmds_automod = automod
    .description = Manage Scripty's automod
automod-setup-embed-not-setup-title = ச்கிரிப்ட்டின் பணி விதிமுறைகள் மற்றும் தனியுரிமைக் கொள்கைக்கு நீங்கள் இன்னும் ஒப்புக் கொள்ளவில்லை.
automod-add-rule-embed-failure-description-premium-limit = Premium tier { $tier } servers are limited to { $maxRules } rules. If you upgrade to tier { $nextTier }, you can add { $nextTierMaxRules } rules.
voice-connection-error-proto-violation = நூலகம் மற்றும் கருத்து வேறுபாடு நெறிமுறையில் உடன்படவில்லை
general-error-user-missing-perms-description-unknown = நீங்கள் என்ன அனுமதிகளைக் காணவில்லை என்று எனக்குத் தெரியவில்லை.
cmds_leave = leave
    .description = Leave any current voice call.
premium-removed = நீங்கள் பிரீமியத்தை கோரிய பயனராக இருந்தால், இந்த சேவையகத்திலிருந்து உங்கள் பிரீமியத்தை இப்போது வெற்றிகரமாக அகற்றிவிட்டீர்கள். நீங்கள் மேம்படுத்த விரும்பினால் அல்லது அதிக இடங்களை வாங்க விரும்பினால், <https://dash.scripty.org/premium> க்குச் செல்லுங்கள்.
config-verbose-enabled = டிரான்ச்கிரிப்சன்களின் போது ச்கிரிப்டி இப்போது வாய்மொழியாக இருக்கும்.
config-verbose-disabled = டிரான்ச்கிரிப்சனின் போது ச்கிரிப்ட் இனி வாய்மொழியாக இருக்காது.
config-transcribe-voice-messages-enabled = ச்கிரிப்டி இப்போது குரல் செய்திகளை படியெடுக்கும்.
config-transcribe-voice-messages-disabled = ச்கிரிப்டி இனி குரல் செய்திகளை படியெடுக்காது.
config-transcribe-video-enabled = ச்கிரிப்டி இப்போது வீடியோ கோப்புகளை படியெடுக்கும்.
config-transcribe-video-disabled = ச்கிரிப்டி இனி வீடியோ கோப்புகளை படியெடுக்காது.
config-transcribe-video-requires-premium =
    வீடியோ கோப்புகளை படியெடுப்பது காப்பீடு அடுக்கு 2 அம்சமாகும், ஏனெனில் இது வீடியோ கோப்புகளை டிரான்ச்கோட் செய்வது மிகவும் கணக்கீட்டு ரீதியாக விலை உயர்ந்தது.
     நீங்கள் காப்பீடு அடுக்கு 2 க்கு மேம்படுத்த விரும்பினால், https://dash.scripty.org/premium க்குச் செல்லுங்கள்.
     இந்த நற்பொருத்தம் இதற்கு முன்பு இயக்கப்பட்டிருந்தால், அது இப்போது முடக்கப்பட்டுள்ளது.
config-auto-detect-lang-enabled = ச்கிரிப்டி இப்போது தானாகவே பேசப்படும் மொழியைக் கண்டறியும்.
config-auto-detect-lang-disabled = பேசப்படும் மொழியை ச்கிரிப்டி தானாகவே கண்டறியாது.
config_transcribe_only_role = transcribe_only_role
    .description = Limit Scripty's transcriptions to only users with this role in a voice chat.
    .transcribe_only_role = transcribe_only_role
    .transcribe_only_role-description = Role to limit to: set empty to disable.
config-translate-not-english = You must set your language to English to enable translation. Do so with `{ $contextPrefix }config language en`.
command-not-found = No command with name `{ $commandName }` found.
context-menu-command-title =
    { "" }
     சூழல் பட்டியல் கட்டளைகள்:
     { "" }
context-menu-command-user =
    { "" }
    { $commandName } (on user)
    { "" }
cmds_user_language = user
    .description = Set your user language to one of the available languages.
    .language = language
    .language-description = The language you want to set your user language to.
cmds_config_server_language = guild
    .description = Set this server's language to one of the available languages.
    .language = language
    .language-description = The language you want to set your guild language to.
language-set-failure-title-db = தரவுத்தள பிழை.
data-storage-opted-in-msgs = மதிப்பெண் பயிற்சிக்காக உங்கள் செய்திகளை சேமிக்க இப்போது நீங்கள் தேர்வு செய்துள்ளீர்கள்.
automod-add-rule-embed-failure-description-premium-limit-hard-cap = You've reached the absolute maximum number of rules ({ $hardCap }). This limit exists to ensure we don't add too much latency in a single message.
automod-remove-rule-embed-success-title = விதி நீக்கப்பட்டது!
automod-remove-rule-embed-failure-description-not-setup = You must run `{ $contextPrefix }automod setup` before removing rules.
voice-connection-error-ws-closed-no-reason = முரண்பாடு காரணமின்றி மூடிய இணைப்பு
more-info-on-command =
    For more information on a specific command, type `{ $contextPrefix }help <name>`
    ```
root-command-invoked-title = இது ஒரு ரூட் கட்டளை!
automod-add-rule-embed-success-description = { $rulesLeft } rules left out of { $maxRules }. { $extraDetails }
automod-add-rule-embed-extra-details-free-limit = இலவச சேவையகங்கள் 25 வழக்கமான விதிகளுக்கு மட்டுமே. இந்த வரம்பை அதிகரிக்க நீங்கள் விரும்பினால், எங்கள் பிரீமியத்தை https://scripty.org/premium இல் பாருங்கள்.
automod-remove-rule-embed-success-description = { $rulesLeft } rules left out of { $maxRules }.
automod-list-rules-footer = Page { $page } of { $maxPage }
automod-list-rules-no-rules = உங்களிடம் எந்த விதிகளும் இல்லை!
vote-reminders-disabled = வாக்களிக்கும் நினைவூட்டல்கள் முடக்கப்பட்டன.
blocked-entity-no-reason-given = தொகுதிக்கு எந்த காரணமும் வழங்கப்படவில்லை.
general-error-cooldown-hit-title = Cooldown hit on { $command }
general-error-user-missing-perms-title = You are missing perms to run { $command }.
cmds_delete_all_data = delete_all_data
    .description = Delete all your data.
automod-add-rule-embed-success-title = Rule { $ruleId } added!
automod-remove-rule-embed-failure-title = விதியை அகற்றுவதில் தோல்வி!
automod-list-rules-embed-title = தன்னியக்க விதிகளை ஆட்ட்கோட் செய்யுங்கள்
automod-list-rules-embed-description = { $rulesLeft } rules left out of { $maxRules }.
blocked-entity-reason-given = Reason given for the block: { $reason }.
blocked-entity-user = You are blocked from using Scripty. { $reason } You may attempt to appeal this block in the support server: { $supportServerInvite }.
voice-connection-error-ws-closed-invalid-payload = தவறான பேலோட் காரணமாக முரண்பாடு மூடிய இணைப்பு
general-error-command-process-description =
    ```
    { $errorFmt }
    ```
    This has been automatically reported. Please do not attempt to repeatedly use this command.
general-error-cooldown-hit-description = { $time } seconds left on cooldown.
delete-data-confirm = ஆம், எல்லா தரவையும் நீக்கு
delete-data-confirm-banned = ஆம், எல்லா தரவையும் நீக்கிவிட்டு என்னை தடைசெய்க
join-no-one-in-channel = There's no one in { $targetMention }. I'm not joining if there's no one there, as that's a waste of limited resources.
join-failed-dropped = முரண்பாடு சிக்கல்களைக் கொண்டிருப்பதாகத் தோன்றுகிறது, இதைப் பற்றி எங்களால் எதுவும் செய்ய முடியாது. தயவுசெய்து பின்னர் மீண்டும் முயற்சிக்கவும்.
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
join-create-thread-in-thread = I can't create a thread while in a thread. Please run this command in a normal channel, likely { $parentChannelMention }.
config-translate-enabled = ச்கிரிப்டி இப்போது டிரான்ச்கிரிப்சன்களை ஆங்கிலத்திற்கு மொழிபெயர்க்கும்.
join-thread-title = Transcription from { $timestamp }
config_transcribe_audio = transcribe_audio
    .description = Toggle whether Scripty transcribes arbitrary audio files. Requires premium.
    .transcribe_audio = transcribe_audio
    .transcribe_audio-description = Defaults to false
guild-language-set-success = Guild language set to `{ $language }`.
cmds_vote_reminder = vote_reminder
    .description = Toggle whether Scripty will remind you to vote for the bot after the time limit has passed.
    .enabled = enabled
    .enabled-description = Enable vote reminders?
blocked-entity-guild = This guild is blocked from using Scripty. { $reason } You may attempt to appeal this block in the support server: { $supportServerInvite }.
voice-connection-error-ws-closed-authentication-failed = அங்கீகார தோல்வி காரணமாக முரண்பாடு மூடிய இணைப்பு
voice-connection-error-ws-closed-server-not-found = குரல் சேவையகத்தைக் கண்டுபிடிக்க முடியவில்லை
general-error-invalid-args-title = Invalid arguments while parsing { $command }.
general-error-command-check-failed-title = A precondition for { $command } failed.
join-forum-thread-content-auto = An automatic transcript was started { $timestamp }.
premium-info-embed-max-duration = அதிகபட்ச அமர்வு காலம் (விநாடிகள்)
config-auto-detect-lang-requires-premium =
    மொழியைக் கண்டறிவது ஒரு காப்பீடு அம்சமாகும், ஏனெனில் மொழியைக் கண்டுபிடிக்க மாதிரியை இரண்டு முறை மீண்டும் இயக்குவது மிகவும் கணக்கீட்டு ரீதியாக விலை உயர்ந்தது.
     நீங்கள் பிரீமியத்திற்கு மேம்படுத்த விரும்பினால், https://dash.scripty.org/premium க்குச் செல்லுங்கள். போட் டிமிங் செய்வதன் மூலம் பிரீமியத்தின் இலவச சோதனையையும் நீங்கள் கோரலாம்.
     இந்த நற்பொருத்தம் இதற்கு முன்பு இயக்கப்பட்டிருந்தால், அது இப்போது முடக்கப்பட்டுள்ளது.
cmds_config_auto_join = auto_join
    .description = Should Scripty automatically join a voice channel when someone joins it?
    .auto_join = auto_join
    .auto_join-description = Defaults to false
config-auto-join-enabled = ஒரு பயனர் செய்யும் போது ச்கிரிப்டி இப்போது தானாகவே வி.சி.எச்சில் சேரும்.
config-auto-join-disabled = ஒரு பயன்பாடு செய்யும்போது ச்கிரிப்டி இனி தானாகவே பகஅ இல் சேராது.
config-auto-join-needs-target-channel = Enabling auto-join requires a default target channel be set. Do that with `{ $contextPrefix }config default target_channel`.
config_translate = translate
    .description = Automatically translate transcriptions to English?
    .translate = translate
    .translate-description = Defaults to false
debug-not-in-call = ச்கிரிப்டி ஒரு வி.சி.யில் இல்லாவிட்டால் இந்த கட்டளை பயனற்றது.
config_enable_kiai = enable_kiai
    .description = Enable Scripty's Kiai integration. Run this command with no arguments to get info on Kiai.
    .enable_kiai = enable_kiai
    .enable_kiai-description = Defaults to false
config-kiai-enabled = ச்கிரிப்டி இப்போது கியாய்க்கு எக்ச்பி பெறும் எந்த குரலையும் அனுப்பும். பயனர்கள் இரட்டை எக்ச்பி பெறுவதைத் தடுக்க கியாயின் குரல் எக்ச்பி சமநிலையை முடக்கு.
config-kiai-missing-perms = இந்த சேவையகத்தில் வேலை செய்ய ச்கிரிப்டி அனுமதிகளைக் காணவில்லை. `811652199100317726` இன் பயன்பாட்டு ஐடியைப் பயன்படுத்தி`/பயன்பாட்டு அங்கீகாரம்` கட்டளையுடன் இதை அங்கீகரிக்கவும், மேலும் ச்கிரிப்ட்டை "அனைத்து நிலைகளையும் எக்ச்பி மற்றும் எக்ச்பி" அனுமதியைக் காணவும்.
voice-connection-error-unknown = அறியப்படாத காரணத்திற்காக துண்டிக்கப்பட்டது
general-error-invalid-args-description = Failed to parse `{ $input }` because `{ $error }`
delete-data-cancel = இல்லை, ரத்துசெய்
leave-success = வி.சி. வெற்றிகரமாக இடது.
premium-info-embed-max-file-length = அதிகபட்ச கோப்பு நீளம் (விநாடிகள்)
config-transcribe-audio-disabled = ச்கிரிப்டி இனி ஆடியோ கோப்புகளை படியெடுக்காது.
config-translate-disabled = ச்கிரிப்டி இப்போது ஆங்கில சொற்களுடன் பேசப்படும் சொற்றொடர்களுடன் பொருந்த முயற்சிக்கும், ஆனால் மொழிபெயர்க்காது.
voice-connection-error-ws-closed-server-crashed = முரண்பாடு குரல் சேவையகம் செயலிழந்தது
voice-connection-error-msg-reconnect = I had an issue ({ $reason }) and disconnected from the voice chat. I'll try reconnecting in 30 seconds.
general-error-user-missing-perms-description-known = Permissions missing: { $perms }
transcription-info-transcription-confidence = நம்பிக்கை
data-storage-command-timed-out = நேரம் முடிந்தது. நீங்கள் இன்னும் அமைப்புகளை நிர்வகிக்க விரும்பினால் இந்த கட்டளையை மீண்டும் இயக்கவும்.
automod-setup-embed-complete-title = ஆட்டோமோட் அமைவு முடிந்தது!
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
data-storage-opted-out-msgs = மதிப்பெண் பயிற்சிக்காக உங்கள் செய்திகளை சேமிப்பதில் இருந்து நீங்கள் இப்போது விலகிவிட்டீர்கள்.
no-channel-specified = You're not in a voice chat, nor did you tell me a channel to join. Try `{ $contextPrefix }join <channel>` to specify a voice chat, or join a voice chat yourself and re-run this command.
join-forum-requires-tags = நீங்கள் என்னைப் பயன்படுத்த முயற்சித்த மன்ற சேனலுக்கு குறிச்சொற்கள் தேவை. என்ன குறிச்சொற்களைப் பயன்படுத்த வேண்டும் என்பதை அறிய எனக்கு வழி இல்லை, எனவே என்னால் அந்த சேனலில் சேர முடியாது. தயவுசெய்து வேறு சேனலைப் பயன்படுத்தவும், அல்லது குறிச்சொல் தேவையை அகற்ற நிர்வாகி கேட்கவும்.
join-target-not-text-based = The channel you told me to send transcripts to ({ $targetMention }) is not a text-based channel. Please use a text-based channel, or pick a different channel in the `target_channel` argument.
cmds_help = help
    .description = Show this help menu
    .command = command
    .command-description = Specific command to show help about
cmds_premium_claim = claim
    .description = Claim your premium within the server where this command is executed.
premium-not-premium = நீங்கள் காப்பீடு சந்தாதாரர் அல்ல. Https://scripty.org/premium இல் குழுசேரவும். நீங்கள் ஒருவர் என்று உங்களுக்குத் தெரிந்தால், தயவுசெய்து உங்கள் பிரீமியத்தை நாங்கள் மீண்டும் நிலைநிறுத்த முடியும்.
premium-too-many-guilds = You have claimed { $totalServers } premium keys. You cannot add any more, unless you upgrade your premium subscription at <https://dash.scripty.org/premium>, or remove some with the `{ $commandPrefix }premium remove` command.
premium-claimed = You have successfully claimed premium on this server. If you would like to upgrade, or purchase more slots, head to <https://dash.scripty.org/premium>. If you would like to remove your premium from this guild, run `{ $commandPrefix }premium remove`.
premium-info-embed-trial-available-title = பிரீமியத்தின் இலவச சோதனை வேண்டுமா?
premium-info-embed-trial-available-description = பிரீமியத்தின் 3 நாள் சோதனையை அமைப்பதில் தொடங்குவதற்கு டி.எம்.
config-transcribe-audio-requires-premium =
    ஆடியோ கோப்புகளை டிரான்ச்கோட் செய்வதற்கு கணக்கீட்டு ரீதியாக விலை உயர்ந்தது என்பதால் ஆடியோ கோப்புகளை படியெடுப்பது காப்பீடு அம்சமாகும்.
     நீங்கள் பிரீமியத்திற்கு மேம்படுத்த விரும்பினால், https://dash.scripty.org/premium க்குச் செல்லுங்கள். போட் டிமிங் செய்வதன் மூலம் பிரீமியத்தின் இலவச சோதனையையும் நீங்கள் கோரலாம்.
     இந்த நற்பொருத்தம் இதற்கு முன்பு இயக்கப்பட்டிருந்தால், அது இப்போது முடக்கப்பட்டுள்ளது.
config_transcribe_video = transcribe_video
    .description = Toggle whether Scripty transcribes arbitrary video files. Requires T2 premium.
    .transcribe_video = transcribe_video
    .transcribe_video-description = Defaults to false
config_auto_detect_lang = auto_detect_lang
    .description = Try to automatically detect the language being spoken? Very inaccurate vs setting a language.
    .auto_detect_lang = auto_detect_lang
    .auto_detect_lang-description = Defaults to false
config-kiai-disabled = கியாயின் ஏபிஐக்கு எக்ச்பி பெறும் எந்த குரலையும் ச்கிரிப்டி இனி அனுப்பாது.
config-kiai-info =
    KIAI பற்றிய கூடுதல் தகவலை [kiai.app] (https://www.kiai.app/?utm_source=scripty_info) இல் காணலாம்.
     { "" }
     இந்த ஒருங்கிணைப்பைப் பயன்படுத்தினால், கியாயின் குரல் எக்ச்பி தொகுதியை முடக்குவதை உறுதிப்படுத்திக் கொள்ளுங்கள்.
cmds_config_prefix = prefix
    .description = Set this server's language to one of the available languages.
    .language = language
    .language-description = The language you want to set your guild language to.
config-prefix-too-long = முன்னொட்டுகள் அதிகபட்சம் எட்டு எழுத்துக்களாக இருக்க வேண்டும். குறுகியதாக மீண்டும் முயற்சிக்கவும்.
config-prefix-unset = The custom prefix for this guild has been cleared. The default prefix (`{ $updatedPrefix }`) will now be used.
config-prefix-updated = Scripty will no longer respond to the default prefix in this guild, but only to `{ $updatedPrefix }`.
language-set-partially-translated-help = ச்கிரிப்ட்டை உங்கள் மொழியில் மொழிபெயர்க்க உதவ விரும்புகிறீர்களா? மொழிபெயர்ப்பு திட்டத்தை https://hosted.weblate.org/engage/scripty-bot/ இல் பாருங்கள்.
cmds_config_default_settings_ephemeral = ephemeral
    .description = Should Scripty, by default, create ephemeral transcripts that disappear when the last user has left?
    .ephemeral = ephemeral
    .ephemeral-description = Default value for ephemeral on the join command
config-default-ephemeral-cant-target-thread = ஒரு நூலை குறிவைக்கும் போது இடைக்கால அமைப்பது டிரான்ச்கிரிப்சன் முடிந்தவுடன் அதை நீக்கும், இது தவறான இயல்புநிலை சேனலை விட்டுவிடும். இயல்புநிலை இலக்கு சேனலை நூல்கள் உருவாக்கக்கூடிய இடத்திற்கு மாற்றவும் அல்லது இடைக்காலத்தைப் பயன்படுத்த வேண்டாம்.
config-default-ephemeral-cant-use-voice-channels = குரல் சேனல்கள் நூல்களை ஆதரிக்காது, எனவே இடைக்கால டிரான்ச்கிரிப்டுகள் சாத்தியமற்றது. இயல்புநிலை இலக்கு சேனலை மாற்றவும், அல்லது இடைக்காலத்தைப் பயன்படுத்த வேண்டாம்.
config-default-ephemeral-enabled = ச்கிரிப்டி இப்போது அனைத்து டிரான்ச்கிரிப்டுகளையும் இடைக்காலமாக்கும்.
config-default-ephemeral-disabled = ச்கிரிப்டி இனி அனைத்து டிரான்ச்கிரிப்டுகளையும் இடைக்காலமாக்காது.
cmds_config_default_settings_new_thread = new_thread
    .description = Should Scripty, by default, create a new thread for all transcriptions?
    .new_thread = new_thread
    .new_thread-description = Default value for new_thread on the join command
config-default-new-thread-cant-make-thread-in-thread = நீங்கள் ஒரு நூலில் ஒரு நூலை உருவாக்க முடியாது. வேறு இயல்புநிலை இலக்கு சேனலைத் தேர்ந்தெடுங்கள் அல்லது புதிய_திரெடியை இயக்க வேண்டாம்.
config-default-new-thread-cant-make-thread-in-vc = குரல் சேனல்களில் நூல்கள் இருக்க முடியாது. வேறு இயல்புநிலை இலக்கு சேனலைத் தேர்ந்தெடுங்கள் அல்லது புதிய_திரெடியை இயக்க வேண்டாம்.
config-default-new-thread-enabled = ச்கிரிப்டி இப்போது அனைத்து டிரான்ச்கிரிப்சன்களுக்கும் ஒரு புதிய நூலை உருவாக்கும்.
config-default-new-thread-disabled = அனைத்து டிரான்ச்கிரிப்சன்களுக்கும் ச்கிரிப்டி இனி ஒரு புதிய நூலை உருவாக்காது.
language-set-failure-description-unsupported = If you'd like to help with adding support for this language, please join the support server at { $supportServerInvite }.
cmds_config_default_settings_record_transcriptions = record_transcriptions
    .description = Should Scripty, by default, record all transcriptions to a text file?
    .record_transcriptions = record_transcriptions
    .record_transcriptions-description = Default value for record_transcriptions on the join command
config-default-record-transcriptions-enabled = ச்கிரிப்டி இப்போது அனைத்து டிரான்ச்கிரிப்சன்களையும் ஒரு உரை கோப்பில் பதிவு செய்யும்.
config-default-record-transcriptions-disabled = ச்கிரிப்டி இனி அனைத்து டிரான்ச்கிரிப்சன்களையும் ஒரு உரை கோப்பில் பதிவு செய்யாது.
cmds_config_default_settings_target_channel = target_channel
    .description = Set the default target channel where Scripty will output transcripts if none are specified.
    .target_channel = target_channel
    .target_channel-description = Default value for target_channel on the join command
config-default-target-channel-enabled = Scripty will now, by default, send all transcripts to { $targetChannelMention }.
config-default-target-channel-disabled = ச்கிரிப்டி இப்போது, இயல்பாக, அனைத்து டிரான்ச்கிரிப்டுகளையும் `/சேர 'செயல்படுத்தப்படும் சேனலுக்கு அனுப்பும்.
config-default-target-channel-cant-disable-with-auto-join = ஆட்டோ-சேர இயக்கப்பட்டிருந்தால் எந்த இயல்புநிலை இலக்கு சேனலையும் அகற்ற முடியாது. ஆட்டோ-சேர முடக்கு அல்லது இலக்கு சேனலை அகற்றுவதற்கு பதிலாக அதை மாற்றவும்.
config-default-target-channel-need-permissions = ச்கிரிப்டி தேவைகள் செய்திகளை அனுப்பவும், இலக்கு சேனலில் வெப்ஊக்குகளை நிர்வகிக்கவும். அதற்கு அந்த அனுமதிகளைக் கொடுத்து மீண்டும் முயற்சிக்கவும்.
cmds_debug = debug
    .description = Output debugging information about Scripty internal state.
debug-info-message = ச்கிரிப்டி உதவி சேவையகத்தில் யாரிடம் இந்த செய்தியை அனுப்பவும்.
default-category-name = கட்டளைகள்
user-language-set-success = User language set to `{ $language }`.
guild-language-set-success-description = To return to English, type `{ $contextPrefix }language guild_language en`.
latency-description =
    WebSocket latency: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    HTTP latency: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Database latency: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)

    Note: if any latency is equal to 0ms, it means that specific latency could not be calculated right now.
    Try again later.
cmds_data_storage = data_storage
    .description = Configure storage settings for your data
data-storage-opted-in-audio = மாதிரி பயிற்சிக்காக உங்கள் ஆடியோவை சேமிக்க இப்போது நீங்கள் தேர்வு செய்துள்ளீர்கள்.
data-storage-opted-out-audio = மாதிரி பயிற்சிக்காக உங்கள் ஆடியோவை சேமிப்பதில் இருந்து இப்போது நீங்கள் விலகிவிட்டீர்கள்.
cmds_setup = setup
    .description = Get started with Scripty's automod.
    .target_channel = target_channel
    .target_channel-description = The channel to send automod logs to.
    .log_recording = log_recording
    .log_recording-description = Should a recording of offending speech be sent to the target channel? Defaults to false.
    .auto_join = auto_join
    .auto_join-description = Should the bot automatically join voice if a user joins? Defaults to true.
automod-setup-embed-complete-description = You can now use `{ $contextPrefix }automod rule add` to add an automod rule. { $extraDetails }
automod-setup-embed-complete-free-limit = இலவச சேவையகங்கள் 25 விதிகளுக்கு மட்டுமே என்பதை நினைவில் கொள்க. இந்த வரம்பை நீங்கள் அகற்ற விரும்பினால், எங்கள் பிரீமியத்தை https://scripty.org/premium இல் பாருங்கள்.
automod-setup-embed-not-setup-description = Do so first by running `{ $contextPrefix } terms_of_service`.
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
automod-add-rule-embed-failure-title = விதியைச் சேர்க்கத் தவறிவிட்டது!
transcription-info-transcription-error =
    internal error: running stt algorithm failed with error: { $error }
    SSRC: { $ssrc }
    This has been logged and will be fixed as soon as possible.
    If possible, please contact the core devs in the support server: { $supportServerInvite }.
    Thanks!
delete-data-description =
    இது உங்கள் எல்லா தரவையும் நீக்கிவிடும். இந்த நடவடிக்கை நிரந்தரமானது, மீளமுடியாதது, செயல்தவிர்க்க முடியாது.

     "உங்கள் தரவு அனைத்தும்" என்று நாங்கள் கூறும்போது, அதையெல்லாம் * என்று அர்த்தப்படுத்துகிறோம். இது உங்கள் குரல் தரவு மற்றும் தரவுத்தளத்தில் உங்கள் பயனரை உள்ளடக்கியது.
     எவ்வாறாயினும், நீங்கள் அதைத் தேர்ந்தெடுத்தால் உங்களிடமிருந்து நாங்கள் சேமித்து வைத்திருக்கும் எந்த செய்திகளையும் * சேர்க்கவில்லை. அந்த செய்திகளை எங்களால் நீக்க முடியாது, ஏனென்றால் பயனர் என்ன செய்தியை அனுப்பினார் என்பது எங்களுக்குத் தெரியாது.

     இந்த செயலுக்குப் பிறகு போட் பயன்படுத்த தடை விதிக்க விரும்பினால், அந்த வகையில் நீங்கள் தற்செயலாக உங்களை நீங்கள் படிக்கவில்லை, கீழே உள்ள பொருத்தமான பொத்தானைக் சொடுக்கு செய்யலாம்.
     அவ்வாறு செய்வது தடைசெய்யப்பட்ட பயனர்களின் பதிவை வைத்திருக்க உங்கள் பயனர் ஐடியை சேமிக்க வேண்டும் என்பதை நினைவில் கொள்க.
     இந்த செயலுக்குப் பிறகு எந்த நேரத்திலும் நீங்கள் தடைசெய்யப்பட விரும்பினால், நீங்கள் உதவி சேவையகத்தைத் தொடர்புகொண்டு ஒரு கையேடு கட்டுப்பாடைக் கேட்கலாம்.

     உங்கள் எல்லா தரவையும் நீக்க விரும்புகிறீர்களா?
free-trial-upsell = ச்கிரிப்டி பிரீமியத்தின் 3 நாள் சோதனைகளை நாங்கள் வழங்கினோம், நீங்கள் அதை முயற்சித்து, அது உங்களுக்கு சரியானதா என்று பார்க்க விரும்பினால். இலவச சோதனையுடன் தொடங்க போட் ஒரு டி.எம்.
