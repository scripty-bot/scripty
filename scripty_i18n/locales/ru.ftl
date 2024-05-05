command-not-found = не нашел команду с именем `{ $commandName }`.
command-not-found-suggestions = вы имели в виду `{ $suggestion }`?
# Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = leave
    .description = Покинуть все текущие голосовые звонки.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = claim
    .description = Получить премиум на сервере, на котором была выполнена команда.
# config - transcribe voice messages command
config-transcribe-voice-messages-enabled = Теперь Scripty будет расшифровывать голосовые сообщения.
# config - transcribe audio command
config-transcribe-audio-disabled = Scripty больше не будет расшифровывать аудиофайлы.
# join command
# This message is shown when the user has told the bot to send transcripts to a non-text-based channel (ie category). `target_channel` should be translated, as slash command arguments are localized.
join-target-not-text-based = Канал, который ты выбрал для отправки транскрипций, ({ $targetMention }), не текстовый. Пожалуйста, используй текстовый канал или выбери другой канал в аргументе `target_channel`.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = remove
    .description = Удалить премиум с сервера, на котором была выполнена команда.
# config - verbose command
config-verbose-disabled = Scripty больше не будет подробным во время расшифровки.
# config - verbose command
config-verbose-enabled = Scripty теперь будет подробным во время расшифровки.
# config - transcribe voice messages command
config-transcribe-voice-messages-disabled = Теперь Scripty не будет расшифровывать голосовые сообщения.
# config - transcribe audio command
config-transcribe-audio-enabled = Теперь Scripty будет расшифровывать аудиофайлы.
# config - transcribe video command
config-transcribe-video-enabled = Теперь Scripty будет расшифровывать видеофайлы.
# config - transcribe video command
config-transcribe-video-disabled = Scripty больше не будет расшифровывать видеофайлы.
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = Ты не премиум пользователь. Подпишись на https://scripty.org/premium. Если ты уверен, что у тебя есть премиум, пожалуйста, напиши боту в ЛС таким образом, мы сможем восстановить твой премиум.
# join command
# If the user specifies they would like to create a forum post, this is the contents of the initial message. { $timestamp } is the current timestamp, in ISO format, and { $authorMention } is the mention of the user who ran the command.
join-forum-thread-content = { $authorMention } начал расшифровку в { $timestamp }.
# config - translate command
config-translate-enabled = Теперь Scripty будет переводить расшифровки на английский.
# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = Канал, который ты пытался использовать, требует тэги. У меня нет возможности узнать, какие теги использовать, поэтому я не могу присоединиться к этому каналу. Пожалуйста, используй другой канал или попроси администратора убрать требование к тэгам.
# join command
# This message is shown when the user requests the bot create a new thread in a channel, but the channel doesn't support threads being created (usually voice channels)
join-create-thread-in-unsupported = Discord не поддерживает темы в { $targetMention }. Пожалуйста, используй другой канал или не создавай тему.
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = Успешно покинул голосовой канал.
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = help
    .description = Показать меню помощи.
    .command = Команда.
    .command-description = Показать страницу помощи для конкретной команды
# transcribe_message command
# This and all attributes show up exclusively in the slash command picker when `transcribe_message` is selected.
cmds_transcribe_message = transcribe_message
    .description = Расшифруй сообщение. Ответь на сообщение чтобы расшифровать его.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = premium
    .description = Премиум команды
# config - auto detect language command
config-auto-detect-lang-enabled = Теперь Scripty будет автоматически определять язык на котором ведется разговор.
# config - transcribe only role command
config-transcribe-only-role-enabled = Теперь Scripty будет расшифровывать сообщения только от пользователей с ролью { $roleId }.
# config - transcribe only role command
config-transcribe-only-role-disabled = Теперь Scripty будет расшифровывать сообщения всех пользователей, независимо от роли.
# config - auto detect language command
config-auto-detect-lang-disabled = Scripty больше не будет определять язык на котором ведется разговор.
