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
join-forum-thread-content = { $authorMention } начал транскрипцию { $timestamp }.
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
    .command = Команда
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
context-menu-command-user =
    { "" }
    { $commandName } (на пользователя)
    { "" }
automod-add-rule-embed-failure-description-premium-limit-hard-cap = Вы достигли абсолютного максимума правил ({ $hardCap }). Это ограничение существует для того, чтобы гарантировать, что мы не добавим слишком много задержки в одно сообщение.
blocked-entity-guild = Этой гильдии запрещено использовать скрипты. { $reason } вы можете попытаться обжаловать эту блокировку на сервере поддержки: { $supportServerInvite }.
voice-connection-error-ws-closed-no-reason = Discord закрыл соединение без причины
context-menu-command-message =
    { "" }
    { $commandName } (на сообщение)
    { "" }
context-menu-command-title =
    { "" }
    Команды контекстного меню:
    { "" }
automod-setup-embed-not-setup-description = Для этого сначала запустите `{ $contextPrefix } terms_of_service`.
voice-connection-error-ws-closed-unknown-protocol = Discord не распознал протокол
default-category-name = Команды
join-success-help-title = Нужна помощь?
join-success-footer-free-trial-upsell = Этот сервер имеет право на бесплатную пробную версию Premium. Напишите боту личное сообщение, чтобы запросить ее.
join-ephemeral-not-thread = Чтобы использовать эфемерный параметр, необходимо выбрать поток в качестве цели, либо установив `create_thread` в значение true, либо указав поток с помощью `target_channel`.
premium-info-embed-title = Премиум статус
cmds_premium_info = info
    .description = Получите информацию о статусе Scripty Premium этого сервера.
premium-info-embed-description-has-subscription = Вы можете управлять своей подпиской на <https://dash.scripty.org/premium>. Спасибо за поддержку Scripty!
premium-info-embed-current-tier = Текущий уровень
premium-info-embed-max-users = Максимальное количество одновременных пользователей
premium-info-embed-max-duration = Максимальная продолжительность сеанса (секунды)
premium-info-embed-trial-available-title = Хотите бесплатную пробную версию Premium?
premium-info-embed-trial-available-description = Напишите боту личное сообщение, чтобы начать настройку 3-дневной пробной версии Premium.
premium-info-embed-manage-subscription-user-has-unclaimed-title = Вы приобрели Premium!
premium-info-embed-manage-subscription-user-has-unclaimed-description = Чтобы получить его на этом сервере, запустите { $claimCommand }.
more-info-on-command =
    Для получения дополнительной информации о конкретной команде введите `{ $contextPrefix }help <name>`
    ```
context-menu-command-unknown =
    { "" }
    { $commandName } (на неизвестном)
    { "" }
user-language-set-success-description = Чтобы вернуться на английский, введите `{ $contextPrefix } язык user_language en`.
root-command-invoked-title = Это команда root!
language-set-partially-translated-help = Хотите помочь перевести Scripty (Скрипти) на свой язык? ознакомьтесь с проектом перевода по адресу https://hosted.weblate.org/engage/scripty-bot/.
automod-add-rule-embed-failure-description-free-limit = Бесплатные серверы ограничены 25 обычными правилами. Если вы хотите увеличить этот лимит, ознакомьтесь с нашей премиум-версией по адресу https://scripty.org/premium.
voice-connection-error-ws-closed-server-not-found = Голосовой сервер не найден
automod-list-rules-embed-field-value =
    Тип: { $ruleType }
    Содержимое: { $ruleContent }
    Действие: { $ruleAction }
premium-too-many-guilds = вы запросили { $totalservers } премиум-ключей. Вы не сможете добавить больше, если не обновите свою премиум-подписку на <https://dash.scripty.org/premium> или не удалите некоторые из них с помощью команды `{ $commandPrefix }premium remove`.
config_transcribe_voice_messages = transcribe_voice_messages
    .description = Включите или выключите функцию Scripty для транскрибирования голосовых сообщений.
    .transcribe_voice_messages = transcribe_voice_messages
    .transcribe_voice_messages-description = Значение по умолчанию true.
config_transcribe_audio = transcribe_audio
    .description = Переключить, будет ли Scripty транскрибировать произвольные аудиофайлы. Требуется премиум.
    .transcribe_audio = transcribe_audio
    .transcribe_audio-description = По умолчанию ложно
config_transcribe_video = transcribe_video
    .description = Переключить, будет ли Scripty транскрибировать произвольные видеофайлы. Требуется T2 premium.
    .transcribe_video = transcribe_video
    .transcribe_video-description = По умолчанию ложно
cmds_user_language = user
    .description = Установите язык пользователя на один из доступных языков
    .language = language
    .language-description = Язык, который вы хотите установить для своего пользователя.
user-language-set-success = Язык пользователя установлен на { $language }`.
language-set-failure-description-unsupported = Если вы хотите помочь с добавлением поддержки этого языка, присоединяйтесь к серверу поддержки по адресу { $supportServerInvite }.
language-set-failure-description-invalid = Указанный вами язык является недопустимым идентификатором языка. Уричина: { $error }
language-set-failure-title-db = Ошибка базы данных.
data-storage-opted-in-audio = Теперь вы выбрали сохранение аудио для обучения модели.
cmds_setup = setup
    .description = Начните работу с автомодом Scripty.
    .target_channel = target_channel
    .target_channel-description = Канал для отправки журналов автомода.
    .log_recording = log_recording
    .log_recording-description = Следует ли отправлять запись оскорбительной речи на целевой канал? По умолчанию false.
    .auto_join = auto_join
    .auto_join-description = Должен ли бот автоматически присоединяться к голосовому чату, если к нему присоединяется пользователь? Значение по умолчанию — true.
automod-add-rule-embed-success-description = { $rulesLeft } правила, оставленные без внимания { $maxRules }. { $extraDetails }
automod-add-rule-embed-extra-details-free-limit = Бесплатные серверы ограничены 25 обычными правилами. Если вы хотите увеличить этот лимит, ознакомьтесь с нашей премиум-версией по адресу https://scripty.org/premium.
automod-add-rule-embed-failure-title = Не удалось добавить правило!
automod-list-rules-no-rules = У вас нет никаких правил!
automod-list-rules-embed-title = Правила автомода
vote-reminders-disabled = Напоминания о голосовании отключены.
blocked-entity-no-reason-given = Причина блокировки не указана.
cmds_vote_reminder = vote_reminder
    .description = Включите или выключите функцию, которая позволит Scripty напомнить вам о необходимости проголосовать за бота после истечения установленного срока.
    .enabled = enabled
    .enabled-description = Включить напоминания о голосовании?
voice-connection-error-host-io-error = ошибка ввода-вывода хоста
vote-reminders-enabled = Напоминания о голосовании включены.
voice-connection-error-proto-violation = Библиотека и Дискорд не согласились с протоколом
voice-connection-error-ws-closed-invalid-payload = Discord закрыл соединение из-за недопустимой полезной нагрузки
voice-connection-error-ws-closed-not-authenticated = Discord закрыл соединение из-за отсутствия аутентификации
voice-connection-error-ws-closed-session-invalid = Дискорд аннулировал сессию
voice-connection-error-ws-closed-authentication-failed = Discord закрыл соединение из-за сбоя аутентификации
data-storage-embed-title = Хранение данных
data-storage-opted-out-msgs = Теперь вы отказались от сохранения сообщений для обучения подсчету очков.
automod-add-rule-embed-success-title = Правило { $ruleId } добавлено!
automod-remove-rule-embed-success-title = Правило удалено!
automod-remove-rule-embed-failure-title = Не удалось удалить правило!
blocked-entity-reason-given = Причина блокировки: { $reason }.
voice-connection-error-internal-lib-error = Внутренняя ошибка библиотеки
voice-connection-error-ws-closed-unknown-encryption-mode = Discord не распознал схему шифрования
language-set-failure-title-unsupported = Указанный вами язык не поддерживается ботом.
guild-language-set-success = Язык гильдии установлен на `{ $language }`.
guild-language-set-success-description = Чтобы вернуться на английский, введите `{ $contextPrefix }language guild_language en`.
no-channel-specified = Вы не находитесь в голосовом чате и не указали мне канал, к которому нужно присоединиться. Попробуйте `{ $contextPrefix }join <channel>`, чтобы указать голосовой чат, или присоединитесь к голосовому чату сами и повторно выполните эту команду.
automod-setup-embed-complete-description = You can now use `{ $contextPrefix }automod rule add` to add an automod rule. { $extraDetails }
automod-list-rules-footer = Страница { $page } из { $maxPage }
voice-connection-error-ws-closed-session-timeout = сеанс истек
join-success-help-description = Вы можете либо присоединиться к серверу поддержки по адресу { $supportServerInvite }, либо написать боту личное сообщение.
config-auto-detect-lang-requires-premium =
    Автоматическое определение языка — это премиум-функция, поскольку повторное выполнение модели дважды для определения языка требует чрезвычайно больших вычислительных затрат.
    если вы хотите перейти на премиум-версию, перейдите на https://dash.scripty.org/premium. Вы также можете запросить бесплатную пробную версию Premium, отправив сообщение боту.
    если эта функция была включена ранее, то теперь она отключена.
automod-setup-embed-complete-free-limit = Обратите внимание, что бесплатные серверы ограничены 25 правилами. Если вы хотите снять это ограничение, ознакомьтесь с нашей премиум-версией по адресу https://scripty.org/premium.
join-success-description = Успешное присоединение к { $voiceTargetMention } и отправка транскрипционного вывода в { $outputChannelMention }.
premium-info-embed-description-no-subscription = Можете подписаться на премиум по адресу <https://dash.scripty.org/premium>. В дополнение к бонусам, которые вы получаете, вы также помогаете нам в нашей цели сделать Scripty лучшим ботом для преобразования речи в текст :)
config-transcribe-video-requires-premium =
    Транскрипция видеофайлов доступна только в версии Premium, так как требует больших вычислительных мощностей
    Если вы хотите получить доступ к этой функции: 👉 Перейти на https://dash.scripty.org/premium.
    Если эта функция была включена ранее, то теперь она отключена.
config_transcribe_only_role = transcribe_only_role
    .description = Ограничить транскрипции Scripty только для пользователей с этой ролью в голосовом чате.
    .transcribe_only_role = transcribe_only_role
    .transcribe_only_role-description = роль для ограничения: установите пустое значение, чтобы отключить.
voice-connection-error-ws-closed-already-authenticated = Discord закрыл соединение, так как уже был аутентифицирован
config-translate-disabled = Теперь Scripty будет пытаться сопоставить произносимые фразы с английскими словами, но не будет переводить.
join-thread-title = Транскрипция из { $timestamp }
cmds_config_verbose = verbose
    .description = Включите или выключите функцию Scripty для подробного вывода текста во время транскрипции.
    .verbose = verbose
    .verbose-description = Defaults to false
premium-claimed = вы успешно получили премиум на этом сервере. если вы хотите обновиться или купить больше слотов, перейдите на <https://dash.scripty.org/premium>. Если вы хотите удалить свой премиум из этой гильдии, запустите `{ $commandPrefix }premium remove`.
premium-removed = Если вы являетесь пользователем, который запросил премиум, вы успешно удалили свой премиум с этого сервера. Если вы хотите обновиться или приобрести больше слотов, перейдите на страницу <https://dash.scripty.org/premium>.
config_auto_detect_lang = auto_detect_lang
    .description = Попробуйте автоматически определить язык, на котором говорят? Очень неточно по сравнению с настройкой языка.
    .auto_detect_lang = auto_detect_lang
    .auto_detect_lang-description = По умолчанию ложно
config-translate-not-english = Чтобы включить перевод, необходимо установить английский язык. Сделайте это с помощью `{ $contextPrefix }config language en`.
no-help-found = Справка по команде `{ $commandName }` не найдена.
data-storage-toggle-msgs-btn = Переключить хранилище сообщений
cmds_automod = automod
    .description = Управление автомодом Scripty
data-storage-command-timed-out = Истек тайм-аут. Повторите эту команду, если вы все еще хотите управлять настройками.
data-storage-opted-out-audio = Теперь вы отказались от сохранения аудиоданных для обучения модели.
data-storage-opted-in-msgs = Теперь вы включили сохранение своих сообщений для обучения подсчету очков.
automod-setup-embed-complete-title = Настройка Automod завершена!
automod-setup-embed-not-setup-title = Вы еще не согласились с Условиями обслуживания и Политикой конфиденциальности Scripty.
automod-add-rule-embed-failure-description-free-locked-type = Бесплатные серверы могут использовать только обычные правила. Если вы хотите использовать другие типы правил, ознакомьтесь с нашей премиум-версией по адресу https://scripty.org/premium.
automod-list-rules-embed-field-name = Правило { $ruleId }
free-trial-upsell = Мы предлагаем 3-дневные пробные версии Scripty Premium, если вы хотите попробовать и посмотреть, подходит ли она вам. Отправьте боту DM, чтобы начать с бесплатной пробной версии.
cmds_config_server_language = guild
    .description = Установите язык этого сервера на один из доступных языков.
    .language = language
    .language-description = Язык, который вы хотите установить в своей гильдии (сервере).
guild-language-set-failure-translate-enabled = На вашем сервере включен автоматический перевод. Поддерживается только при переводе на английский язык. Отключите эту функцию, если хотите установить свой язык.
cmds_data_storage = data_storage
    .description = Настройте параметры хранения ваших данных
voice-connection-error-ws-closed-unknown-opcode = Discord закрыл соединение из-за неизвестного кода операции
root-command-invoked-description = Пожалуйста, вызовите только подкоманды этой команды, чтобы использовать ее. Смотрите `{ $contextPrefix }help { $commandName }` для получения дополнительной информации.
latency-description =
    WebSocket latency: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    HTTP latency: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Database latency: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)

    Примечание: если какая-либо задержка равна 0 мс, это означает, что конкретная задержка не может быть рассчитана прямо сейчас.
    Повторите попытку позже.
language-set-failure-title-invalid = Язык `{ $language }` не поддеривается.
language-set-failure-description-db = база данных обнаружила ошибку при попытке установить ваш язык. Эта ошибка была сообщена, и мы рассмотрим ее. Пожалуйста, не спамьте этой командой. (Если вам интересно, вот ошибка: { $error })
data-storage-toggle-audio-btn = Переключить аудиохранилище
automod-add-rule-embed-failure-description-premium-limit = Премиум-серверы { $tier } ограничены правилами { $maxRules }. Если вы переходите на уровень { $nextTier }, вы можете добавить правила { $nextTierMaxRules }.
blocked-entity-user = Вам запрещено использовать scripty. { $reason } вы можете попытаться обжаловать эту блокировку на сервере поддержки: { $supportServerInvite }.
config_translate = translate
    .description = Автоматически переводить транскрипции на английский язык?
    .translate = translate
    .translate-description = По умолчанию ложно
voice-connection-error-timed-out = истекло время ожидания соединения
join-success-premium = Вы можете проверить Премиум-статус этого сервера с помощью `/premium info`.
cmds_join = join
    .description = Присоединяйтесь к голосовому чату. Транскрипты будут записываться в канал, в котором вы запустите эту команду.
    .voice_channel = voice_channel
    .voice_channel-description = Голосовой чат для привязки.
    .record_transcriptions = record_transcriptions
    .record_transcriptions-description = LogЗаписывать все транскрипты? Пользователи получат DM, когда Scripty покинет канал. По умолчанию false.
    .target_channel = target_channel
    .target_channel-description = Отправляйте транскрипты сюда, а не на текущий канал. Выберите форум, чтобы создать новый пост.
    .create_thread = create_thread
    .create_thread-description = Создать новую ветку для этой транскрипции? По умолчанию false.
data-storage-embed-description =
    { "**" }примечание**: все, что следует далее, **совершенно необязательно**, и отказ **никаким образом** не повлияет на ваш опыт работы со Scripty.
    Итак, вот.

    Scripty требует много аудио и текстовых данных для обучения правильной модели преобразования речи в текст. Не все могут пожертвовать или купить премиум, чтобы помочь нам, поэтому большой способ помочь — это позволить нам хранить ваши данные, такие как аудио и сообщения, для обучения модели.
    Мы понимаем, что эти данные могут быть очень личными, поэтому это полностью добровольно и никак не повлияет на ваш опыт.

    Вот что мы с этим сделаем:
    { "*" } Сохраненные сообщения мы бы передали в оценщик, ориентированный на ваш язык. Этот оценщик позволил бы алгоритму выбирать наиболее вероятные слова для заданного набора звуков. Хотя это чрезвычайно полезно, это не так важно, как аудио. Обратите внимание, что данные этого сообщения зашифрованы с помощью 256-битного шифрования AES.
    { "*" } Сохраненный аудиофайл мы бы передали его и его расшифровку в модель, чтобы повысить точность модели преобразования речи в текст. Это безумно полезно, даже если у вас плохой микрофон и много фонового шума: на самом деле, чем больше шума, тем лучше, пока человек все еще может разобрать, что вы говорите.

    Если вы согласились, а позже решили отказаться, ваши данные все равно будут храниться, но вы можете запросить удаление ваших голосовых данных, запустив `{ $contextPrefix }delete_all_data`. Однако удалить данные ваших сообщений невозможно. Это связано с тем, что мы не храним ссылку на то, какой пользователь отправил какое сообщение.
    Ваши данные хранятся на серверах, которые надежно защищены. Любому, кто попытается получить доступ, будет крайне сложно сделать это успешно.

    Вы можете переключать свой выбор с помощью кнопок ниже.
cmds_add_rule = add_rule
    .description = Добавить правило автомода.
    .rule_type = rule_type
    .rule_type-description = Тип добавляемого правила. Дополнительную информацию см. в `/automod rule_help`.
    .rule_type-choice-Regular = Регулярный
    .content = содержимое
    .content-description = Содержание правила, которое нужно добавить.
    .action = действие
    .action-description = Действие, которое нужно предпринять при срабатывании правила.
    .action-choice-SilentDelete = Бесшумное удаление
    .action-choice-DeleteAndLog = Удалять журнал
    .action-choice-DeleteLogAndKick = Удалить, журнал и удалить пользователя из голоса
    .action-choice-DeleteLogAndSilence = Удалить, журнал и отключить звук
cmds_ping = ping
    .description = Получите задержку бота.
transcribe-message-initial-reply = Загрузка...
transcribe-message-not-slash-command = Из-за ограничений Discord эта команда недоступна в виде косой черты. Используйте варианты префикса или контекстного меню.
transcribe-message-timed-out-after-reply = Возникла серьезная ошибка в работе системы. Пожалуйста, сообщите разработчикам: Таймаут ожидания данных (ID { $msgId })
transcribe-message-transcoding-file = Преобразование файла { $filename }... (длительность { $fileLength } секунд)
transcribe-message-transcribing-file = Выполняется транскрибация файла { $filename } (длительность: { $fileLength } секунд)
transcribe-message-inline-header = Расшифровка:
transcribe-message-time-taken-single-file = Потребовалось { $timeTaken } секунд, чтобы расшифровать { $fileLength } второй файл.
transcribe-message-time-taken-named-file = Для расшифровки файла "{ $filename }" потребовалось { $timeTaken } секунд (длина файла { $fileLength } секунд)
transcribe-message-unusually-long = Это не займет много времени. Если вы хотите поделиться с нами содержимым, пожалуйста, сообщите нам об этом на нашем сервере поддержки.
transcribe-message-no-transcript = Не возвращена расшифровка для файла `{ $filename }` в { $took } ({ $fileLength } второй файл)
transcribe-message-too-long = С вашим текущим статусом Premium вы можете транскрибировать файлы длиной не более { $maxFileLength } секунд. Значение "{ $filename }" равно { $fileLength } секундам и было проигнорировано.
transcribe-message-result-error = При обработке `{ $filename }` была обнаружена ошибка: `{ $error }`. Пожалуйста, сообщите об этом на наш сервер поддержки.
transcribe-message-malformed-input = При обработке "{ $filename }" был обнаружен неверный ввод. Исправьте это и повторите попытку. `{ $error }`
transcribe-message-needs-reply = Вы должны ответить на сообщение, которое хотите расшифровать.
config_enable_kiai = enable_kiai
    .description = Включить интеграцию Scripty с Kiai. Запустите команду без аргументов для получения информации о Kiai.
    .enable_kiai = enable_kiai
    .enable_kiai-description = По умолчанию: false (отключено)
config-kiai-info =
    Дополнительную информацию о Kiai можно найти на [kiai.app] (https://www.kiai.app/?utm_source=scripty_info)."
    { "" }
    "При использовании этой интеграции обязательно отключите модуль голосового XP в Kiai, чтобы избежать конфликтов.
transcribe-message-probing-file = Проверяем файл "{ $filename }`...
transcribe-message-no-results = Расшифровки не возвращены. Либо сообщение, на которое дана ссылка, не содержит вложений, либо вложения не поддерживаются (только видео с премиум-версией 1 или выше)
transcribe-message-video-needs-premium = Перекодирование видео требует больших вычислительных затрат и требует дополнительной платы. { $filename } был обнаружен как видео и был проигнорирован.
join-forum-thread-content-auto = Была запущена автоматическая расшифровка { $timestamp }.
config-kiai-disabled = Прекращена передача очков опыта (XP), полученных в голосовых каналах, в API системы Kiai.
transcribe-message-discord-error = Discord сломался при обработке "{ $filename }"! Скорее всего, мы ничего не сможем с этим поделать, прочитайте сообщение об ошибке и при необходимости проверьте права доступа. `{ $error }`
premium-info-embed-max-file-length = Максимальная длина файла (секунды)
cmds_config_auto_join = auto_join
    .description = Должен ли Scripty автоматически подключаться к голосовому каналу при входе пользователя?
    .auto_join = auto_join
    .auto_join-description = По умолчанию: false (отключено)
config-auto-join-enabled = Scripty теперь будет автоматически подключаться к голосовым каналам, когда это делает пользователь.
config-auto-join-disabled = Scripty больше не будет автоматически подключаться к голосовым каналам при входе пользователя.
config-auto-join-needs-target-channel = Для включения автоподключения необходимо задать канал по умолчанию. Сделайте это командой `{ $contextPrefix }config default target_channel`.
config-kiai-enabled = Scripty теперь будет отправлять весь полученный голосовой XP в Kiai. Отключите систему уровней Kiai для голосовых каналов, чтобы пользователи не получали двойной XP.
transcribe-message-downloading-file = Загружаемый файл `{ $filename }`... (размер { $fileSize } байт)
config-default-ephemeral-cant-target-thread = Установка параметра ephemeral для треда приведёт к его удалению сразу после завершения транскрипции, оставляя недействительный канал по умолчанию. Либо измените целевой канал по умолчанию на место, где можно создавать треды, либо не используйте ephemeral.
config-prefix-updated = Scripty больше не будет реагировать на префикс по умолчанию на этом сервере, а только на` { $updatedPrefix }`.
config-kiai-missing-perms = У Scripty отсутствуют разрешения для работы на этом сервере. Авторизуйте его с помощью команды` /application authorize`, используя ID приложения `811652199100317726` и предоставив Scripty право «просматривать и редактировать все уровни и XP.
cmds_config_default_settings_ephemeral = ephemeral
    .description = Должен ли Scripty по умолчанию создавать временные транскрипты, которые исчезают, когда последний пользователь выходит?
    .ephemeral = временный
    .ephemeral-description = Значение по умолчанию для параметра ephemeral в команде join
config-default-ephemeral-enabled = Теперь Scripty сделает все расшифровки ephemeral.
config-default-ephemeral-cant-use-voice-channels = Голосовые каналы не поддерживают потоки, поэтому ephemeral расшифровка невозможна. Либо измените целевой канал по умолчанию, либо не используйте ephemeral.
cmds_config_prefix = префикс
    .description = Установить язык сервера из списка доступных языков.
    .language = язык
    .language-description = Язык, который вы хотите установить для этого сервера.
config-prefix-too-long = Префиксы должны содержать не более 8 символов. Попробуйте еще раз, используя более короткий префикс.
config-prefix-unset = Пользовательский префикс для этого сервера был удалён. Теперь будет применяться стандартный префикс (`{ $updatedPrefix }`).
automod-remove-rule-embed-failure-description-invalid-id = Неверный идентификатор правила. Дополнительную информацию см. в разделе `{ $contextPrefix }automod list`.
general-error-cooldown-hit-title = Время восстановления по { $command }
config-default-ephemeral-disabled = Scripty больше не будет делать все транскрипты эфемерными.
cmds_config_default_settings_new_thread = new_thread
    .description = Должен ли Scripty по умолчанию создавать новую тему для всех транскрипций?
    .new_thread = new_thread
    .new_thread-description = Значение по умолчанию для new_thread в команде join
cmds_config_default_settings_record_transcriptions = record_transcriptions
    .description = Должен ли Scripty по умолчанию записывать все транскрипции в текстовый файл?
    .record_transcriptions = record_transcriptions
    .record_transcriptions-description = Значение по умолчанию для record_transcriptions в команде join
debug-info-message = Перешлите это сообщение тому, кто на сервере поддержки Scripty просит вас об этом.
general-error-command-process-title = При обработке { $command } произошла ошибка.
voice-connection-error-msg-no-reconnect = У меня возникла проблема ({ $reason }), и я отключился от голосового чата.
transcription-info-transcription-error =
    Внутренняя ошибка: запуск алгоритма stt завершился с ошибкой: { $error }
    SSRC: { $ssrc }
    Эта ошибка была зарегистрирована и будет исправлена как можно скорее.
    Если это возможно, пожалуйста, свяжитесь с разработчиками ядра на сервере поддержки: { $supportServerInvite }.
    Спасибо!
delete-data-title = Удалить данные
general-error-command-check-failed-description-no-reason = причина не указана
transcription-info-transcription-title = Транскрипт
cmds_delete_all_data = delete_all_data
    .description = Удалить все ваши данные.
automod-remove-rule-embed-success-description = { $rulesLeft } правила, оставшиеся после { $maxRules }.
automod-list-rules-embed-description = { $rulesLeft } правила, оставшиеся после { $maxRules }.
cmds_debug = debug
    .description = Вывод отладочной информации о внутреннем состоянии Scripty.
join-failed-dropped = Похоже, что в Discord возникли проблемы, мы не можем ничего с этим поделать. Пожалуйста, повторите попытку позже.
cmds_remove_rule = remove_rule
    .description = Удалить правило automod.
    .rule_id = rule_id
    .rule_id-description = Идентификатор правила для удаления.
general-error-invalid-structure-description =
    { $description }

    { "**" }Note**: this is a Discord error.
    The only fix for this is to wait for Discord to propagate slash commands, which can take up to one hour.
    If you do not want to wait this hour, you should use the prefix commands: run this command with `~{ $qualifiedName } { $args }`.
config-default-new-thread-cant-make-thread-in-vc = Голосовые каналы не могут иметь потоков. Либо выберите другой целевой канал по умолчанию, либо не включайте new_thread.
config-default-new-thread-enabled = Scripty теперь будет создавать новую тему для всех транскрипций.
delete-data-confirm-banned = Да, удалить все данные и забанить себя
debug-not-in-call = Эта команда бесполезна, если Scripty не находится в VC.
automod-add-rule-embed-failure-description-invalid-type = Неверный тип правила. Дополнительную информацию см. в разделе `{ $contextPrefix }automod rule_help`.
delete-data-confirm = Да, удалите все данные
config-default-target-channel-need-permissions = Scripty нужны Send Messages и Manage Webhooks в целевом канале. Дайте ему эти разрешения и попробуйте снова.
general-error-invalid-args-description = Не удалось разобрать `{ $input }`, потому что `{ $error }`
general-error-command-process-description =
    ```
    { $errorFmt }
    ```
    Сообщение об ошибке было получено автоматически. Пожалуйста, не пытайтесь повторно использовать эту команду.
general-error-command-check-failed-title = Предварительное условие для { $command } не выполнено.
transcription-info-transcription-confidence = Уверенность
transcription-info-transcript-count = Транскрипт 1 из { $count }.
transcription-info-transcription-ssrc = SSRC { $ssrc }
join-no-one-in-channel = В { $targetMention } никого нет. Я не присоединяюсь, если там никого нет, так как это пустая трата ограниченных ресурсов.
general-error-invalid-structure-title = Неверная структура из Discord при разборе { $command }.
config-default-new-thread-disabled = Scripty больше не будет создавать новую тему для всех транскрипций.
join-no-permission = У меня нет разрешения присоединиться к { $targetMention }. Пожалуйста, предоставьте мне права на просмотр канала и присоединение или присоединитесь к другому голосовому чату, где у меня есть права.
general-error-user-missing-perms-description-unknown = Я не уверен, каких разрешений вам не хватает.
delete-data-description =
    Это приведет к удалению всех ваших данных. Это действие является постоянным, необратимым и не может быть отменено.

    Когда мы говорим «все ваши данные», мы имеем в виду *все* данные. Сюда входят ваши голосовые данные и ваш пользователь в базе данных.
    Однако сюда не входят сообщения, которые мы могли сохранить, если вы согласились на это. Мы не можем удалить эти сообщения, просто потому что не знаем, какой пользователь отправил то или иное сообщение.

    Если вы хотите, чтобы после этого действия вам также запретили пользоваться ботом, чтобы вы случайно не прочитали себя, вы можете нажать соответствующую кнопку ниже.
    Обратите внимание, что для этого нам потребуется сохранить ваш идентификатор пользователя, чтобы вести учет забаненных пользователей.
    Если в любой момент после этого действия вы захотите снять с себя запрет, вы можете связаться с сервером поддержки и попросить снять запрет вручную.

    Вы уверены, что хотите удалить все свои данные?
join-create-thread-in-thread = Я не могу создать поток, находясь в потоке. Пожалуйста, выполните эту команду в обычном канале, скорее всего { $parentChannelMention }.
config-default-new-thread-cant-make-thread-in-thread = Вы не можете создать поток в потоке. Либо выберите другой целевой канал по умолчанию, либо не включайте new_thread.
config-default-record-transcriptions-enabled = Scripty теперь будет записывать все транскрипции в текстовый файл.
config-default-record-transcriptions-disabled = Scripty больше не будет записывать все транскрипции в текстовый файл.
cmds_config_default_settings_target_channel = target_channel
    .description = Установите целевой канал по умолчанию, на который Scripty будет выводить транскрипты, если ни один из них не указан.
    .target_channel = target_channel
    .target_channel-description = Значение по умолчанию для target_channel в команде join
config-default-target-channel-enabled = Теперь Scripty по умолчанию будет отправлять все транскрипты на { $targetChannelMention }.
config-default-target-channel-disabled = Теперь Scripty по умолчанию будет отправлять все транскрипты на канал, на котором выполняется `/join`.
config-default-target-channel-cant-disable-with-auto-join = Вы не можете удалить целевой канал по умолчанию, если включено авто соединение. Либо отключите авто соединение, либо измените целевой канал вместо его удаления.
automod-add-rule-embed-failure-description-not-setup = Перед добавлением правил необходимо выполнить команду `{ $contextPrefix }automod setup`.
automod-remove-rule-embed-failure-description-not-setup = Перед удалением правил необходимо выполнить команду `{ $contextPrefix }automod setup`.
cmds_list_rules = list_rules
    .description = Список всех правил автомода.
    .filter_by = filter_by
    .filter_by-description = Фильтровать правила по их содержанию. Оставьте пустым, чтобы показать все правила.
voice-connection-error-ws-closed-server-crashed = Голосовой сервер discord потерпел крах
voice-connection-error-unknown = отключен по неизвестной причине
voice-connection-error-msg-reconnect = У меня возникла проблема ({ $reason }), и я отключился от голосового чата. Я попробую подключиться снова через 30 секунд.
general-error-invalid-args-title = Недопустимые аргументы при разборе { $command }.
general-error-user-missing-perms-title = Вам не хватает разрешений на выполнение { $command }.
general-error-user-missing-perms-description-known = Разрешения отсутствуют: { $perms }
general-error-cooldown-hit-description = Осталось { $time } секунд до окончания действия.
general-error-user-missing-perms-description-not-owner = Не являюсь владельцем этого бота.
delete-data-cancel = Нет, отменить
