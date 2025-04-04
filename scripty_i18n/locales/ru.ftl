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
no-channel-specified = Вы не в голосовом чате и не указали мне канал, к которому можно присоединиться. Попробуйте { $contextPrefix }  войти в канал, чтобы указать голосовой чат, или присоединитесь к голосовому чату самостоятельно и повторите эту команду.
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
    Транскрибирование видеофайлов — это премиум-функция уровня 2, поскольку перекодирование видеофайлов требует очень больших вычислительных затрат.
    Если вы хотите перейти на премиум-уровень 2, перейдите на страницу https://dash.scripty.org/premium.
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
    .description = Добавьте правило автомода.
    .rule_type = rule_type
    .rule_type-description = Тип правила для добавления. См. `/automod rule_help` для получения дополнительной информации.
    .rule_type-choice-Regular = Regular
    .content = content
    .content-description = Содержание правила для добавления.
    .action = action
    .action-description = Действие, которое следует предпринять при срабатывании правила.
    .action-choice-SilentDelete = Silent delete
    .action-choice-DeleteAndLog = Delete and log
    .action-choice-DeleteLogAndKick = Delete, log, and remove user from voice
    .action-choice-DeleteLogAndSilence = Delete, log, and mute user
cmds_ping = ping
    .description = Получите задержку бота.
