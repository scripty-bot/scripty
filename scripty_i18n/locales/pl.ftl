# ToS command
# This and all attributes show up exclusively in the slash command picker when `terms_of_service` is selected.
cmds_terms_of_service = terms_of_service
    .description = Wyświetl i zaakceptuj Regulamin oraz Politykę Prywatności Scripty.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = premium
    .description = Komendy premium
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = help
    .description = Pokaż to menu pomocy.
    .command = polecenie
    .command-description = Konkretne polecenie, dla którego chcesz zobaczyć pomoc
# Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = leave
    .description = Opuść bieżące połączenie głosowe.
# Data deletion command
delete-data-confirm-banned = Tak, usuń wszystkie dane i zbanuj mnie
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = claim
    .description = Zgłoś roszczenie do swojego premium na serwerze, na którym wykonujesz tę komendę.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = remove
    .description = Usuń swoje premium z serwera, na którym wykonujesz tę komendę.
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = Nie jesteś subskrybentem premium. Zasubskrybuj na stronie https://scripty.org/premium. Jeśli wiesz, że jesteś subskrybentem, proszę wyślij prywatną wiadomość do bota, abyśmy mogli przywrócić twoje premium.
# premium command
# This is shown when the guild the user is running this command in has not yet agreed to the ToS.
premium-server-not-set-up = Ten serwer nie zaakceptował jeszcze Regulaminu i Polityki Prywatności Scripty. Zrób to najpierw za pomocą komendy `{ $commandPrefix }terms_of_service`.
# premium command
# This is shown to the user when they have too many used servers to add more.
premium-too-many-guilds = Zgłosiłeś { $totalServers } kluczy premium. Nie możesz dodać więcej, chyba że zaktualizujesz subskrypcję premium na stronie <https://dash.scripty.org/premium> lub usuniesz niektóre za pomocą komendy `{ $commandPrefix }premium remove`.
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language` is selected.
cmds_language = język
    .description = Zmienia twoje preferencje językowe.
# premium command
# This is shown when the user successfully claims one of their premium subscriptions.
premium-claimed = Pomyślnie zgłoszono premium na tym serwerze. Jeśli chcesz dokonać aktualizacji lub zakupić więcej slotów, udaj się pod adres <https://dash.scripty.org/premium>. Jeśli chcesz usunąć swoje premium z tego serwera, uruchom komendę `{ $commandPrefix }premium remove`.
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language user_language` is selected.
cmds_user_language = użytkownik
    .description = Ustaw swój język użytkownika na jeden z dostępnych języków.
    .language = język
    .language-description = Język, na który chcesz ustawić swój język użytkownika.
# premium command
# This is shown when the user successfully removes their premium from this guild.
premium-removed = Jeśli jesteś użytkownikiem, który zgłosił roszczenie do Premium, pomyślnie usunąłeś swoje premium z tego serwera. Jeśli chcesz dokonać aktualizacji lub zakupić więcej slotów, udaj się pod adres <https://dash.scripty.org/premium>.
# Help menu translation strings
command-not-found = Nie znaleziono komendy o nazwie `{ $commandName }`.
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language guild_language` is selected.
cmds_guild_language = serwer
    .description = Ustaw język tego serwera na jeden z dostępnych języków.
    .language = język
    .language-description = Język, na który chcesz ustawić język tego serwera.
# Help menu translation strings
command-not-found-suggestions = Czy chodziło Ci o `{ $suggestion }`?
# Help menu translation strings
no-help-found = Nie znaleziono pomocy dla komendy `{ $commandName }`.
# Help menu translation strings
default-category-name = Komendy
# Context menu command translation strings
context-menu-command-title =
    { "" }
    Komendy menu kontekstowego:
    { "" }
# Context menu command translation strings
context-menu-command-user =
    { "" }
    { $commandName } (na użytkowniku)
    { "" }
# Context menu command translation strings
context-menu-command-message =
    { "" }
    { $commandName } (na wiadomości)
    { "" }
# Context menu command translation strings
more-info-on-command =
    Aby uzyskać więcej informacji o konkretnej komendzie, wpisz `{ $contextPrefix }help <nazwa>`
    ```
# Language configuration strings
# This message is shown as the embed title when the user sets their language successfully.
user-language-set-success = Język użytkownika ustawiono na `{ $language }`.
# Data deletion command
delete-data-title = Usuń dane
# Language configuration strings
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = Błąd bazy danych.
# Command invocation contexts
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = Proszę użyj tylko podkomendy tej komendy, aby jej użyć. Zobacz `{ $contextPrefix }help { $commandName }` dla więcej informacji.
# ping command
# This and all attributes show up exclusively in the slash command picker when `ping` is selected.
cmds_ping = ping
    .description = Sprawdź opóźnienie bota.
# Language configuration strings
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = Baza danych napotkała błąd podczas próby ustawienia twojego języka. Ten błąd został zgłoszony i zostanie zbadany. Proszę nie spamuj tej komendy. (Jeśli jesteś ciekawy, oto błąd: { $error })
# data_storage command
# This and all attributes show up exclusively in the slash command picker when `data_storage` is selected.
cmds_data_storage = data_storage
    .description = Skonfiguruj ustawienia przechowywania danych
# transcription info - verbose mode
# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.
transcription-info-transcription-ssrc = SSRC { $ssrc }
# data_storage command
data-storage-toggle-audio-btn = Przełącz Przechowywanie Dźwięku
# data_storage command
data-storage-toggle-msgs-btn = Przełącz Przechowywanie Wiadomości
# automod add rule command
automod-add-rule-embed-failure-description-invalid-type = Niepoprawny typ reguły. Zobacz `{ $contextPrefix }automod rule_help` dla więcej informacji.
# data_storage command
data-storage-opted-in-audio = Zdecydowałeś się teraz na przechowywanie swojego audio w celu treningu modelu.
# data_storage command
data-storage-opted-out-audio = Zdecydowałeś się teraz nie przechowywać swojego audio w celu treningu modelu.
# automod add rule command
automod-add-rule-embed-failure-description-free-locked-type = Darmowe serwery mogą używać tylko zwykłych reguł. Jeśli chcesz używać innych typów reguł, sprawdź nasze Premium na stronie https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-failure-description-not-setup = Musisz najpierw uruchomić `{ $contextPrefix }automod setup`, zanim dodasz reguły.
# automod remove rule command
# This and all attributes show up exclusively in the slash command picker when `automod remove rule` is selected.
cmds_remove_rule = remove_rule
    .description = Usuń regułę automatycznego moderowania.
    .rule_id = id_reguły
    .rule_id-description = ID reguły do usunięcia.
# automod remove rule command
automod-remove-rule-embed-success-title = Reguła usunięta!
# automod remove rule command
automod-remove-rule-embed-success-description = Pozostało { $rulesLeft } reguł z { $maxRules }.
# data_storage command
data-storage-opted-in-msgs = Zdecydowałeś się teraz na przechowywanie swoich wiadomości w celu treningu oceniacza.
# automod remove rule command
automod-remove-rule-embed-failure-title = Nie udało się usunąć reguły!
# data_storage command
data-storage-opted-out-msgs = Zdecydowałeś się teraz nie przechowywać swoich wiadomości w celu treningu oceniacza.
# automod remove rule command
automod-remove-rule-embed-failure-description-invalid-id = Niepoprawne ID reguły. Zobacz `{ $contextPrefix }automod list` dla więcej informacji.
# data_storage command
data-storage-command-timed-out = Przekroczono limit czasu. Uruchom tę komendę ponownie, jeśli nadal chcesz zarządzać ustawieniami.
# automod remove rule command
automod-remove-rule-embed-failure-description-not-setup = Musisz najpierw uruchomić `{ $contextPrefix }automod setup`, zanim będziesz mógł usuwać reguły.
# automod root command
# This and all attributes show up exclusively in the slash command picker when `automod` is selected.
cmds_automod = automod
    .description = Zarządzaj automatycznym moderowaniem Scripty
# automod list rules command
# This and all attributes show up exclusively in the slash command picker when `automod list rules` is selected.
cmds_list_rules = list_rules
    .description = Wyświetl wszystkie reguły automatycznego moderowania.
    .filter_by = filtruj_przez
    .filter_by-description = Filtruj reguły według ich treści. Pozostaw puste, aby pokazać wszystkie reguły.
# automod list rules command
automod-list-rules-embed-title = Reguły automatycznego moderowania
# automod root command
automod-root-response = To jest komenda główna, z powodu ograniczeń Discorda nie wykonuje żadnych działań. Zobacz `{ $contextPrefix }help automod` dla więcej informacji.
# automod list rules command
automod-list-rules-embed-description = Pozostało { $rulesLeft } reguł z { $maxRules }.
# automod setup command
automod-setup-embed-complete-title = Konfiguracja automatycznego moderowania zakończona pomyślnie!
# automod list rules command
automod-list-rules-embed-field-name = Reguła { $ruleId }
# automod list rules command
automod-list-rules-embed-field-value =
    Typ: { $ruleType }
    Treść: { $ruleContent }
    Działanie: { $ruleAction }
# automod setup command
automod-setup-embed-complete-description = Teraz możesz użyć `{ $contextPrefix }automod rule add`, aby dodać regułę automatycznego moderowania. { $extraDetails }
# automod setup command
automod-setup-embed-complete-free-limit = Należy pamiętać, że darmowe serwery mają limit 25 reguł. Jeśli chcesz usunąć ten limit, zajrzyj do naszego Premium na stronie https://scripty.org/premium.
# automod setup command
automod-setup-embed-not-setup-title = Nie zgodziłeś się jeszcze na Regulamin i Politykę Prywatności Scripty.
# automod setup command
automod-setup-embed-not-setup-description = Zrób to najpierw, uruchamiając `{ $contextPrefix } terms_of_service`.
# automod setup command
automod-setup-auto-join-premium-only = Automatyczne dołączanie jest funkcją premium. Sprawdź nasze Premium na stronie https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-success-title = Reguła { $ruleId } dodana!
# automod add rule command
automod-add-rule-embed-success-description = Pozostało { $rulesLeft } reguł z { $maxRules }. { $extraDetails }
# automod add rule command
automod-add-rule-embed-extra-details-free-limit = Darmowe serwery mają limit 25 zwykłych reguł. Jeśli chcesz zwiększyć ten limit, sprawdź nasze Premium na stronie https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-failure-title = Nie udało się dodać reguły!
# automod add rule command
automod-add-rule-embed-failure-description-free-limit = Darmowe serwery mają limit 25 zwykłych reguł. Jeśli chcesz zwiększyć ten limit, sprawdź nasze Premium na stronie https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit = Serwery w warstwie premium { $tier } mają limit { $maxRules } reguł. Jeśli zaktualizujesz do warstwy { $nextTier }, będziesz mógł dodać { $nextTierMaxRules } reguł.
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit-hard-cap = Osiągnąłeś absolutny maksymalny limit reguł ({ $hardCap }). Ten limit istnieje, aby zapewnić, że nie dodamy zbyt dużego opóźnienia w pojedynczej wiadomości.
# automod list rules command
automod-list-rules-footer = Strona { $page } z { $maxPage }
# automod list rules command
automod-list-rules-no-rules = Nie masz żadnych reguł!
# blocked entities description
blocked-entity-no-reason-given = Nie podano powodu blokady.
# voice connection errors
voice-connection-error-ws-closed-server-crashed = Serwer głosowy Discorda uległ awarii
# voice connection errors
voice-connection-error-msg-no-reconnect = Wystąpił problem ({ $reason }) i zostałem rozłączony z czatu głosowego.
# voice connection errors
voice-connection-error-msg-reconnect = Wystąpił problem ({ $reason }) i zostałem rozłączony z czatu głosowego. Spróbuję ponownie połączyć się za 30 sekund.
# general errors
general-error-command-process-title = Wystąpił błąd podczas przetwarzania { $command }.
# general errors
general-error-invalid-structure-title = Niepoprawna struktura od Discorda podczas analizy { $command }.
# general errors
# Note $time will be a decimal with two digits of accuracy.
general-error-cooldown-hit-description = Pozostało { $time } sekund do zakończenia czasu odnowienia.
# general errors
general-error-user-missing-perms-title = Brakuje ci uprawnień do uruchomienia komendy { $command }.
# general errors
general-error-cooldown-hit-title = Limit czasu nałożony na { $command } został osiągnięty
# Language configuration strings
# This message is shown as the embed description when the user sets their language successfully.
user-language-set-success-description = Aby wrócić do języka angielskiego, wpisz `{ $contextPrefix }language user_language en`.
# Language configuration strings
# This message is shown as the embed title when the guild sets their language successfully.
guild-language-set-success = Język serwera ustawiono na `{ $language }`.
# Language configuration strings
# This message is shown as the embed description when the guild sets their language successfully.
guild-language-set-success-description = Aby wrócić do języka angielskiego, wpisz `{ $contextPrefix }language guild_language en`.
# Language configuration strings
# This message is shown as the embed title when an entity tries to set their language to an unsupported language.
language-set-failure-title-unsupported = Podany przez ciebie język nie jest obsługiwany przez bota.
# Language configuration strings
# This message is shown as the embed description when an entity tries to set their language to an unsupported language.
language-set-failure-description-unsupported = Jeśli chcesz pomóc w dodaniu obsługi tego języka, dołącz do serwera wsparcia pod adresem { $supportServerInvite }.
# Language configuration strings
# This message is shown as the embed title when an entity tries to set their language to an invalid language.
language-set-failure-title-invalid = Nie znaleziono języka `{ $language }`.
# Language configuration strings
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = Podany przez ciebie język to niepoprawny identyfikator języka. Powód: { $error }
# Command invocation contexts
# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = To jest komenda główna!
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to select a button in time.
tos-agree-timed-out = Przekroczono limit czasu. Uruchom ponownie to polecenie, jeśli wciąż chcesz zaakceptować Regulamin.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user agrees to the ToS.
tos-agree-success = Pomyślnie zaakceptowałeś Regulamin i Politykę Prywatności Scripty. Możesz teraz z niej korzystać.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to agree to the ToS, usually by explicitly clicking the "No" button.
disagreed-to-tos = Nie zgodziłeś się na Warunki korzystania z usługi Scripty i Politykę prywatności. Aby korzystać z Scripty, musisz zaakceptować te dokumenty. Możesz to zrobić, uruchamiając ponownie to polecenie.
# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = Kanał forum, który próbowałeś użyć, wymaga tagów. Nie mam sposobu, aby poznać jakie tagi użyć, więc nie mogę dołączyć do tego kanału. Proszę użyj innego kanału lub poproś administratora o usunięcie wymogu tagów.
# ping command
# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    Opóźnienie WebSocket: { $wsLatencyMs } ms ({ $wsLatencyNs } ns)
    Opóźnienie HTTP: { $httpLatencyMs } ms ({ $httpLatencyNs } ns)
    Opóźnienie bazy danych: { $pgLatencyMs } ms ({ $pgLatencyNs } ns)

    Uwaga: jeśli jakiekolwiek opóźnienie wynosi 0 ms, oznacza to, że obecnie nie można obliczyć tego konkretnego opóźnienia.
    Spróbuj ponownie później.
# data_storage command
data-storage-embed-title = Przechowywanie Danych
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = Pomyślnie opuszczono VC.
# ToS command
# This is sent when the user has not yet agreed to the ToS and must do so.
agreeing-to-tos = Możesz zobaczyć Regulamin i Politykę Prywatności Scripty pod adresem https://scripty.org/terms i https://scripty.org/privacy odpowiednio. Możesz kliknąć poniższy przycisk, aby zaakceptować oba te dokumenty i korzystać z Scripty.
# ToS command
# This is sent when the user has already agreed to the ToS and does not need to do so again.
already-agreed-to-tos = Już zaakceptowałeś Regulamin i Politykę Prywatności Scripty. Jeśli chcesz je ponownie zobaczyć, możesz to zrobić pod adresem https://scripty.org/terms i https://scripty.org/privacy odpowiednio.
# join command
# This message is shown when the user tries to tell the bot to join, but they have not agreed to the ToS.
must-agree-to-tos = Musisz zaakceptować Regulamin i Politykę Prywatności, aby korzystać z Scripty. Zobacz `{ $contextPrefix }terms_of_service` dla więcej informacji.
# join command
# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = Nie jesteś w czacie głosowym, ani nie podałeś mi kanału do dołączenia. Spróbuj `{ $contextPrefix }join <kanał>` aby określić czat głosowy lub sam dołącz do czatu głosowego i ponownie uruchom tę komendę.
# join command
# This message is shown when the user attempts to make Scripty join a voice channel, but there is no one in the channel.
join-no-one-in-channel = Nie ma nikogo w { $targetMention }. Nie będę dołączać, jeśli nikogo tam nie ma, ponieważ to marnowanie ograniczonych zasobów.
# join command
# This message is shown when Discord tosses a Dropped or TimedOut error when trying to join a voice channel.
join-failed-dropped = Discord wydaje się mieć problemy, nie możemy na to wpłynąć. Proszę spróbować ponownie później.
# join command
# This message is shown when the bot does not have permissions for the voice channel it is trying to join.
join-no-permission = Nie mam uprawnień do dołączenia do { $targetMention }. Proszę przyznać mi uprawnienia "Wyświetl kanał" i "Dołącz" lub dołączyć do innego czatu głosowego, w którym mam te uprawnienia.
# join command
# This message is shown when the user has told the bot to create a thread while in a thread.
join-create-thread-in-thread = Nie mogę utworzyć wątku będąc wewnątrz wątku. Proszę uruchomić tę komendę na zwykłym kanale, prawdopodobnie { $parentChannelMention }.
# join command
# If the user specifies they would like to create a thread, this is set as the thread name. { $timestamp } is the current timestamp, in ISO format.
join-thread-title = Transkrypcja z { $timestamp }
# join command
# If the user specifies they would like to create a forum post, this is the contents of the initial message. { $timestamp } is the current timestamp, in ISO format, and { $authorMention } is the mention of the user who ran the command.
join-forum-thread-content = { $authorMention } rozpoczął transkrypcję o { $timestamp }.
# blocked entities description
blocked-entity-reason-given = Podany powód blokady: { $reason }.
# blocked entities description
blocked-entity-guild = Ten serwer jest zablokowany przed korzystaniem z Scripty. { $reason } Możesz próbować odwołać się od tej blokady na serwerze wsparcia: { $supportServerInvite }.
# blocked entities description
blocked-entity-user = Jesteś zablokowany przed korzystaniem z Scripty. { $reason } Możesz próbować odwołać się od tej blokady na serwerze wsparcia: { $supportServerInvite }.
# voice connection errors
voice-connection-error-internal-lib-error = Wewnętrzny błąd biblioteki
# voice connection errors
voice-connection-error-host-io-error = Błąd wejścia/wyjścia hosta
# voice connection errors
voice-connection-error-proto-violation = Biblioteka i Discord były niezgodne co do protokołu
# voice connection errors
voice-connection-error-timed-out = Przekroczono limit czasu oczekiwania na połączenie
# voice connection errors
voice-connection-error-ws-closed-no-reason = Discord zamknął połączenie bez podania powodu
# voice connection errors
voice-connection-error-ws-closed-unknown-opcode = Discord zamknął połączenie z powodu nieznanej operacji
# voice connection errors
voice-connection-error-ws-closed-invalid-payload = Discord zamknął połączenie z powodu nieprawidłowego ładunku
# voice connection errors
voice-connection-error-ws-closed-not-authenticated = Discord zamknął połączenie z powodu braku uwierzytelnienia
# voice connection errors
voice-connection-error-ws-closed-authentication-failed = Discord zamknął połączenie z powodu błędu uwierzytelniania
# voice connection errors
voice-connection-error-ws-closed-already-authenticated = Discord zamknął połączenie, ponieważ jest już uwierzytelniony
# voice connection errors
voice-connection-error-ws-closed-session-invalid = Discord unieważnił sesję
# voice connection errors
voice-connection-error-ws-closed-session-timeout = Sesja przekroczyła limit czasu
# voice connection errors
voice-connection-error-ws-closed-server-not-found = Nie można znaleźć serwera głosowego
# voice connection errors
voice-connection-error-ws-closed-unknown-protocol = Discord nie rozpoznał protokołu
# voice connection errors
voice-connection-error-ws-closed-unknown-encryption-mode = Discord nie rozpoznał schematu szyfrowania
# voice connection errors
voice-connection-error-unknown = Rozłączono z nieznanego powodu
# general errors
general-error-command-process-description =
    ```
    { $errorFmt }
    ```
    To zostało automatycznie zgłoszone. Proszę nie próbuj wielokrotnie używać tej komendy.
# general errors
general-error-invalid-args-title = Nieprawidłowe argumenty podczas analizy { $command }.
# general errors
general-error-invalid-args-description = Nie udało się przeanalizować `{ $input }`, ponieważ `{ $error }`
# general errors
general-error-invalid-structure-description =
    { $description }

    { "**" }Uwaga**: to jest błąd Discorda.
    Jedynym sposobem na naprawę tego jest oczekiwanie, aż polecenia slash zostaną przekazane przez Discorda, co może potrwać do godziny.
    Jeśli nie chcesz czekać tej godziny, powinieneś użyć komend z prefiksem: uruchom tę komendę za pomocą `~{ $qualifiedName } { $args }`.
# general errors
general-error-user-missing-perms-description-known = Brakujące uprawnienia: { $perms }
# general errors
general-error-user-missing-perms-description-unknown = Nie jestem pewien, jakich uprawnień ci brakuje.
# general errors
general-error-user-missing-perms-description-not-owner = Nie jesteś właścicielem tego bota.
# general errors
general-error-command-check-failed-title = Warunek wstępny dla { $command } nie został spełniony.
# general errors
general-error-command-check-failed-description-no-reason = nie podano powodu
# transcription info - verbose mode
# This is shown as the number of transcriptions the algorithm has discovered.
transcription-info-transcript-count = Transkrypt 1 z { $count }.
# transcription info - verbose mode
# This is shown as the title of the transcript
transcription-info-transcription-title = Transkrypt
# transcription info - verbose mode
# This is shown as the percent accuracy of the transcription (roughly)
transcription-info-transcription-confidence = Pewność
# transcription info - verbose mode
# This is shown when the algorithm encounters an error
transcription-info-transcription-error =
    błąd wewnętrzny: uruchamianie algorytmu STT zakończyło się niepowodzeniem z błędem: { $error }
    SSRC: { $ssrc }
    To zostało zalogowane i zostanie naprawione tak szybko, jak to możliwe.
    Jeśli to możliwe, skontaktuj się z deweloperami rdzenia na serwerze wsparcia: { $supportServerInvite }.
    Dziękujemy!
# Data deletion command
delete-data-confirm = Tak, usuń wszystkie dane
# Data deletion command
delete-data-cancel = Nie, anuluj
# generic strings
# Message shown if a guild has not claimed their free trial of premium. Always appears on its own standalone line in the surrounding message.
free-trial-upsell = Oferujemy 3-dniowe wersje próbne Scripty Premium, jeśli chciałbyś ją wypróbować i zobaczyć, czy to jest odpowiednie dla ciebie. Wyślij bota wiadomość prywatną, aby rozpocząć bezpłatny okres próbny.
# Data deletion command
# This and all attributes show up exclusively in the slash command picker when `delete_all_data` is selected.
cmds_delete_all_data = delete_all_data
    .description = Usuń wszystkie swoje dane.
# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = join
    .description = Dołącz do czatu głosowego. Protokoły będą rejestrowane na kanale, na którym wykonujesz tę komendę.
    .voice_channel = kanał_głosowy
    .voice_channel-description = Kanał głosowy do przypisania.
    .record_transcriptions = zapisz_protokoły
    .record_transcriptions-description = Protokoły dziennika? Użytkownicy otrzymają prywatną wiadomość, gdy Scripty opuści kanał. False
    .target_channel = kanał_docelowy
    .target_channel-description = Wysyłaj tutaj protokoły zamiast bieżącego kanału. Celuj w forum, aby utworzyć nowy post.
    .create_thread = utwórz_wątek
    .create_thread-description = Utworzyć nowy wątek dla tej transkrypcji? Wartość domyślna: fałsz.
# data_storage command
data-storage-embed-description =
    { "**" } UWAGA: wszystko, co następuje, jest **całkowicie opcjonalne**, a rezygnacja **nie wpłynie w żaden sposób** na twoje doświadczenie z Scripty.
    Mając to na uwadze, oto idziemy dalej.

    Scripty wymaga dużej ilości danych audio i tekstowych, aby trenować odpowiedni model mowy-na-tekst. Nie każdy jest w stanie przekazać darowiznę lub zakupić premium, aby nam pomóc, więc dużo możesz nam pomóc, pozwalając nam przechowywać twoje dane, takie jak audio i wiadomości, w celu trenowania modelu.
    Rozumiemy, że te dane mogą być bardzo osobiste, dlatego jest to całkowicie opcjonalne i nie wpłynie w żaden sposób na twoje doświadczenie.

    Oto, co byśmy z tym zrobili:
    { "*" } Z przechowywanych wiadomości nakarmilibyśmy je do oceniacza skierowanego na twój język. Ten oceniacz pozwoliłby algorytmowi wybrać najbardziej prawdopodobne słowa dla danego zestawu dźwięków. Chociaż ogromnie pomocne, to nie jest tak istotne jak audio. Zauważ, że dane wiadomości są szyfrowane przy użyciu szyfru AES 256-bit.
    { "*" } Z przechowywanego audio nakarmilibyśmy je i jego transkrypcję modelem, aby zwiększyć dokładność modelu mowy-na-tekst. Jest to niezwykle pomocne, nawet jeśli masz słaby mikrofon i dużo hałasu w tle: w rzeczywistości, im więcej hałasu, tym lepiej, pod warunkiem że człowiek nadal może usłyszeć, co mówisz.

    Jeśli jesteś zapisany, a później zdecydujesz się zrezygnować, twoje dane nadal są przechowywane, ale możesz poprosić o usunięcie swoich danych głosowych, uruchamiając `{ $contextPrefix }delete_all_data`. Niemożliwe jest jednak usunięcie danych wiadomości. Wynika to z faktu, że nie przechowujemy łącza, której użytkownik wysłał jaką wiadomość.
    Twoje dane są przechowywane na serwerach, które są ściśle zabezpieczone. Byłoby niezwykle trudne dla każdej próby uzyskania dostępu do nich.

    Możesz przełączać swoje wybory za pomocą poniższych przycisków.
# automod setup command
# This and all attributes show up exclusively in the slash command picker when `automod setup` is selected.
cmds_setup = setup
    .description = Rozpocznij korzystanie z automatycznego moderowania Scripty.
    .target_channel = kanał_docelowy
    .target_channel-description = Kanał do wysyłania logów automatycznego moderowania.
    .log_recording = zapis_logów
    .log_recording-description = Czy zapis nagrania obraźliwego tekstu ma być wysyłany na kanał docelowy? Wartość domyślna: fałsz.
    .auto_join = automatyczny_join
    .auto_join-description = Czy bot ma automatycznie dołączać do rozmowy głosowej, jeśli użytkownik dołączy? Wartość domyślna: prawda.
# automod add rule command
# This and all attributes show up exclusively in the slash command picker when `automod add rule` is selected.
cmds_add_rule = add_rule
    .description = Dodaj regułę automatycznego moderowania.
    .rule_type = typ_reguły
    .rule_type-description = Typ reguły do dodania. Zobacz `/automod rule_help` dla więcej informacji.
    .rule_type-choice-Regular = Zwykła
    .content = treść
    .content-description = Treść reguły do dodania.
    .action = działanie
    .action-description = Działanie do podjęcia, gdy reguła zostanie aktywowana.
    .action-choice-SilentDelete = Ciche usunięcie
    .action-choice-DeleteAndLog = Usunięcie i zapis do logów
    .action-choice-DeleteLogAndKick = Usunięcie, zapis do logów i wyrzucenie użytkownika z kanału głosowego
    .action-choice-DeleteLogAndSilence = Usunięcie, zapis do logów i wyciszenie użytkownika
# Data deletion command
delete-data-description =
    To spowoduje usunięcie wszystkich twoich danych. Ta czynność jest trwała, nieodwracalna i nie może zostać cofnięta.

    Kiedy mówimy "wszystkie twoje dane", mamy na myśli *wszystkie*. Obejmuje to twoje dane głosowe i twojego użytkownika w bazie danych.
    Nie obejmuje to jednak żadnych wiadomości, które mogliśmy od ciebie przechować, jeśli się na to zgodziłeś. Nie możemy usunąć tych wiadomości, po prostu dlatego, że nie wiemy, jak użytkownik wysłał jaką wiadomość.

    Jeśli chcesz również zostać zbanowany i nie chcesz przypadkowo dodać się ponownie, możesz kliknąć odpowiedni przycisk poniżej.
    Należy jednak zauważyć, że takie działanie wymaga przechowywania twojego identyfikatora użytkownika, aby zachować rejestr zbanowanych użytkowników.
    Jeśli w dowolnym momencie po tej czynności chciałbyś być odbanowany, możesz skontaktować się z serwerem wsparcia i poprosić o ręczne odbanowanie.

    Czy na pewno chcesz usunąć wszystkie swoje dane?
# join command
# This message is shown on successfuly joining a voice channel.
# { $targetMention } is the mention of the channel the bot joined.
join-success =
    Pomyślnie dołączono do { $voiceTargetMention } i przesyłanie transkrypcji do { $outputChannelMention }.
    { "" }
    Uwaga: twój obecny poziom Premium to { $tier }. Pozwala to na przeprowadzenie transkrypcji jednocześnie dla { $maxUsers } użytkowników. Wraz z tym bot automatycznie opuści kanał po { $leaveDuration } sekundach, niezależnie od liczby użytkowników na kanale. Ma to na celu zapobieżenie nadużyciom naszych systemów.
    Jeśli chcesz więcej użytkowników, dłuższy czas użytkowania i chcesz również wesprzeć bota, rozważ subskrypcję naszego Premium: <https://dash.scripty.org/premium>
    Jeśli wiesz, że jesteś już subskrybentem Premium, proszę wyślij botowi wiadomość prywatną, abyśmy mogli przywrócić twoje Premium.
    { $freeTrialUpsell }
