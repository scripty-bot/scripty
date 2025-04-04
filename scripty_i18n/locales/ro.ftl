# Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = Ieși
    .description = Ieși din orice apel vocal curent.
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = Nu sunteți abonat premium. Abonați-vă la: https://scripty.org/premium. Dacă știți că sunteți unul, vă rugăm să trimiteți un DM botului astfel încât să vă putem restabili statutul premium.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = premium
    .description = comenzi premium
# data_storage command
data-storage-toggle-msgs-btn = Comutați stocarea mesajelor
# automod add rule command
automod-add-rule-embed-failure-description-invalid-type = Tip de regulă nevalid. Consultați `{ $contextPrefix }automod rule_help` pentru mai multe informații.
# config - auto detect language command
config_auto_detect_lang = auto_detectare_limba
    .description = Încercați să detectați automat limba vorbită? Foarte inexact comparativ cu setarea unei limbi.
    .auto_detect_lang = auto_detectare_limba
    .auto_detect_lang-description = Setarea implicit setată la fals
# config - transcribe only role command
config-transcribe-only-role-enabled = Scripty va transcrie acum numai mesajele de la utilizatorii din { $roleId }.
# Help menu translation strings
default-category-name = Comenzi
# Context menu command translation strings
context-menu-command-title =
    { "" }
    Comenzi din meniul contextual:
    { "" }
# data_storage command
data-storage-embed-title = Stocare a datelor
# automod add rule command
automod-add-rule-embed-failure-title = Nu s-a putut adăuga regula!
# automod remove rule command
automod-remove-rule-embed-failure-description-not-setup = Trebuie să rulați `{ $contextPrefix }automod setup` înainte de a elimina regulile.
# general errors
general-error-command-process-title = A apărut o eroare la procesarea { $command }.
# general errors
general-error-command-check-failed-description-no-reason = nici un motiv furnizat
# data_storage command
data-storage-opted-out-audio = Acum ați renunțat la stocarea sunetului pentru antrenamentul modelului.
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit-hard-cap = Ați atins numărul maxim absolut de reguli ({ $hardCap }). Această limită există pentru a ne asigura că nu adăugăm prea multă latență într-un singur mesaj.
# automod list rules command
automod-list-rules-embed-description = { $rulesLeft } reguli rămase din { $maxRules }.
# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = Alăturăte
    .description = Alăturați-vă unui chat vocal. Transcrierile vor fi conectate pe canalul în care rulați această comandă.
    .voice_channel = canal_voce
    .voice_channel-description = Chat vocal la care puteți să vă legați.
    .record_transcriptions = înregistreaza_transcrierile
    .record_transcriptions-description = Înregistrați toate transcrierile? Utilizatorii vor primi DM când Scripty părăsește canalul. Implicit este setat la fals.
    .target_channel = canal_țintă
    .target_channel-description = Trimiteți transcrieri aici, în loc de canalul curent. Vizați un forum pentru a crea o postare nouă.
    .create_thread = creați_thread
    .create_thread-description = Creați un fir nou pentru această transcriere? Implicit este setat la fals.
# join command
# This message is shown when the user requests the bot create a new thread in a channel, but the channel doesn't support threads being created (usually voice channels)
join-create-thread-in-unsupported = Discord nu acceptă threads în { $targetMention }. Vă rugăm să utilizați un alt canal sau să nu creați un thread.
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = Ai părăsit canalul de voce cu succes.
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = Ajutor
    .description = Afișează acest meniu de ajutor
    .command = comanda
    .command-description = Comandă specifică pentru a afișa ajutor despre comanda
# config - verbose command
cmds_config_verbose = detaliat
    .description = Comută dacă Scripty este detaliat în timpul transcripțiilor.
    .verbose = detaliat
    .verbose-description = Setarea implicita setata la false
# premium command
# This is shown when the user successfully claims one of their premium subscriptions.
premium-claimed = Ați revendicat cu succes statutul premium pe acest server. Dacă doriți să faceți upgrade sau să cumpărați mai multe sloturi, accesați <https://dash.scripty.org/premium>. Dacă doriți să eliminați premium din această breaslă, rulați comanda `{ $commandPrefix }premium remove`.
# voice connection errors
voice-connection-error-ws-closed-not-authenticated = discord a închis conexiunea din cauza neautentificării
# config - transcribe only role command
config_transcribe_only_role = transcriere_doar_rol
    .description = Limitați transcripțiile lui Scripty doar la utilizatorii cu acest rol într-un chat vocal.
    .transcribe_only_role = transcriere_doar_rol
    .transcribe_only_role-description = Rol limitat: este setat gol pentru al dezactiva.
# config - transcribe only role command
config-transcribe-only-role-disabled = Scripty va transcrie acum pentru toți utilizatorii, indiferent de rol.
# config - translate command
config-translate-not-english = Trebuie să setați limba engleză pentru a activa traducerea. Faceți acest lucru cu `{ $contextPrefix }config language en`.
# Help menu translation strings
command-not-found-suggestions = Ați vrut să spuneți `{ $suggestion }`?
# Help menu translation strings
no-help-found = Nu a fost găsit niciun ajutor pentru comanda `{ $commandName }`.
# Context menu command translation strings
context-menu-command-message =
    { "" }
    { $commandName } (pentru mesaj)
    { "" }
# Context menu command translation strings
context-menu-command-unknown =
    { "" }
    { $commandName } (pentru necunoscut)
    { "" }
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `user_language` is selected.
cmds_user_language = utilizator
    .description = Setați limba utilizatorului la una dintre limbile disponibile.
    .language = limbaj
    .language-description = Limba în care doriți să setați limba utilizatorului.
# Language configuration strings
# This message is shown as the embed title when the user sets their language successfully.
user-language-set-success = Limba utilizatorului setată la `{ $language }`.
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `guild_language` is selected.
cmds_config_server_language = breasla
    .description = Setați limba acestui server la una dintre limbile disponibile.
    .language = limbă
    .language-description = Limba în care doriți să setați limba breslei.
# Language configuration strings
# This message is shown as the embed description when the user sets their language successfully.
user-language-set-success-description = Pentru a reveni la engleză, tastați `{ $contextPrefix }language user_language en`.
# Language configuration strings
# This message is shown as the embed description when the guild sets their language successfully.
guild-language-set-success-description = Pentru a reveni la engleză, tastați `{ $contextPrefix }language guild_language en`.
# Language configuration strings
# This message is shown as the embed title when an entity tries to set their language to an invalid language.
language-set-failure-title-invalid = Limba `{ $language }` nu a fost găsită.
# Language configuration strings
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = Eroare la baza de date.
# Language configuration strings
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = Baza de date a întâmpinat o eroare la încercarea de a vă seta limba. Această eroare a fost raportată și o vom analiza. Vă rugăm să nu trimiteți spam la această comandă. (Dacă ești curios, iată eroarea: { $error })
# Command invocation contexts
# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = Aceasta este o comandă root!
# ping command
# This and all attributes show up exclusively in the slash command picker when `ping` is selected.
cmds_ping = ping
    .description = Obține latența botului.
# data_storage command
data-storage-opted-in-audio = Acum ați optat pentru stocarea audio pentru antrenamentul modelului.
# data_storage command
data-storage-opted-in-msgs = Acum ați optat pentru stocarea mesajelor pentru antrenamentul marcatorilor.
# data_storage command
data-storage-opted-out-msgs = Acum ați renunțat la stocarea mesajelor pentru antrenamentul marcatorilor.
# voice connection errors
voice-connection-error-ws-closed-authentication-failed = conexiune discord a fost închisă din cauza eșecului de autentificare
# automod setup command
# This and all attributes show up exclusively in the slash command picker when `automod setup` is selected.
cmds_setup = setup
    .description = Începeți cu automodul lui Scripty.
    .target_channel = canal_țintă
    .target_channel-description = Canalul către care se trimit jurnalele automod.
    .log_recording = log_înregistrări
    .log_recording-description = Ar trebui să fie trimisă o înregistrare a discursului ofensator către canalul țintă? Setarea implicit setată la fals.
    .auto_join = auto_join
    .auto_join-description = Ar trebui botul să se alăture automat vocea dacă un utilizator se alătură? Setarea implicit setată la adevărat.
# automod setup command
automod-setup-embed-complete-free-limit = Rețineți că serverele gratuite sunt limitate la 25 de reguli. Dacă doriți să eliminați această limită, consultați Premium la https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-success-title = S-a adăugat regula { $ruleId }!
# automod add rule command
automod-add-rule-embed-success-description = { $rulesLeft } reguli rămase din { $maxRules }. { $extraDetails }
# automod add rule command
automod-add-rule-embed-extra-details-free-limit = Serverele gratuite sunt limitate la 25 de reguli obișnuite. Dacă doriți să creșteți această limită, consultați Premium la https://scripty.org/premium.
# automod remove rule command
automod-remove-rule-embed-success-title = Regula eliminată!
# automod remove rule command
automod-remove-rule-embed-failure-description-invalid-id = ID regulă nevalid. Consultați `{ $contextPrefix }automod list` pentru mai multe informații.
# automod list rules command
automod-list-rules-embed-title = Reguli automod
# automod list rules command
automod-list-rules-embed-field-name = Regula { $ruleId }
# blocked entities description
blocked-entity-no-reason-given = Nu s-a dat niciun motiv pentru blocaj.
# voice connection errors
voice-connection-error-ws-closed-invalid-payload = conexiune discord a fost închisă din cauza unei sarcini utile nevalide
# voice connection errors
voice-connection-error-ws-closed-unknown-protocol = discord nu a recunoscut protocolul
# general errors
general-error-invalid-args-title = Argumente nevalide în timpul analizării { $command }.
# general errors
general-error-invalid-args-description = Nu s-a putut analiza `{ $input }` deoarece `{ $error }`
# general errors
general-error-user-missing-perms-title = Îți lipsesc permisiunile pentru a rula { $command }.
# transcription info - verbose mode
# This is shown as the number of transcriptions the algorithm has discovered.
transcription-info-transcript-count = Transcrierea 1 din { $count }.
# Data deletion command
delete-data-confirm = Da, șterge toate datele
# Data deletion command
delete-data-cancel = Nu, anulează
# generic strings
# Message shown if a guild has not claimed their free trial of premium. Always appears on its own standalone line in the surrounding message.
free-trial-upsell = Oferim probe de 3 zile pentru Scripty Premium dacă doriți să îl încercați și să vedeți dacă este potrivit pentru dvs. Trimiteți botului un DM pentru a începe un trial gratuit.
# join command
# If the user specifies they would like to create a forum post, this is the contents of the initial message. { $timestamp } is the current timestamp, in ISO format, and { $authorMention } is the mention of the user who ran the command.
join-forum-thread-content = { $authorMention } a început o transcriere la { $timestamp }.
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit = Serverele de nivel premium { $tier } sunt limitate la regulile { $maxRules }. Dacă treceți la nivelul { $nextTier }, puteți adăuga reguli { $nextTierMaxRules }.
# automod list rules command
# This and all attributes show up exclusively in the slash command picker when `automod list rules` is selected.
cmds_list_rules = lista_regulilor
    .description = Listează toate regulile automod.
    .filter_by = filter_by
    .filter_by-description = Filtrați regulile după conținut. Lăsați gol pentru a afișa toate regulile.
# voice connection errors
voice-connection-error-msg-reconnect = Am avut o problemă ({ $reason }) și m-am deconectat de la chatul vocal. Voi încerca să mă reconectez în 30 de secunde.
# general errors
general-error-command-process-description =
    ```
    { $errorFmt }
    ```
    Acest lucru a fost raportat automat. Vă rugăm să nu încercați să utilizați în mod repetat această comandă.
# transcription info - verbose mode
# This is shown when the algorithm encounters an error
transcription-info-transcription-error =
    eroare internă: rularea algoritmului stt a eșuat cu eroare: { $error }
    SSRC: { $ssrc }
    Acest lucru a fost înregistrat și va fi remediat cât mai curând posibil.
    Dacă este posibil, contactați dezvoltatorii de bază din serverul de asistență: { $supportServerInvite }.
    Mulțumim!
# join command
# This message is shown when the user has told the bot to create a thread while in a thread.
join-create-thread-in-thread = Nu pot crea un thread în timp ce sunt într-un thread. Rulați această comandă pe un canal normal, probabil { $parentChannelMention }.
# automod add rule command
# This and all attributes show up exclusively in the slash command picker when `automod add rule` is selected.
cmds_add_rule = add_rule
    .description = Adăugați o regulă automod.
    .rule_type = tip_regulă
    .rule_type-description = Tipul de regulă de adăugat. Consultați `/automod rule_help` pentru mai multe informații.
    .rule_type-choice-Regular = Regular
    .content = conținut
    .content-description = Conținutul regulii de adăugat.
    .action = acțiune
    .action-description = Acțiunea de întreprins atunci când regula este declanșată.
    .action-choice-SilentDelete = Ștergere silențioasă
    .action-choice-DeleteAndLog = Şterge şi log
    .action-choice-DeleteLogAndKick = Ștergeți, înregistrați și eliminați utilizatorul din voce
    .action-choice-DeleteLogAndSilence = Şterge, înregistrează şi amuțește utilizatorul
# Data deletion command
delete-data-description =
    Acest lucru va șterge toate datele dvs. Această acțiune este permanentă, ireversibilă și nu poate fi anulată.

    Când spunem „toate datele tale” ne referim la *toate*. Acestea includ datele dvs. vocale și utilizatorul din baza de date.
    Totuși, *nu* include mesajele pe care le-am stocat de la dvs. dacă ați optat pentru aceasta. Nu putem șterge acele mesaje, pur și simplu pentru că nu știm ce utilizator a trimis ce mesaj.

    Dacă doriți să vi se interzică și utilizarea botului după această acțiune, astfel încât să nu vă readăugați pe dvs. înapoi accidental, puteți face clic pe butonul corespunzător de mai jos.
    Rețineți că acest lucru va necesita să vă stocăm ID-ul de utilizator pentru a păstra o evidență a utilizatorilor banați.
    Dacă în orice moment după această acțiune ați dori să fiți debanat, puteți contacta serverul de asistență și puteți solicita o debanare manuală.

    Sigur doriți să ștergeți toate datele dvs.?
# Help menu translation strings
command-not-found = Nu a fost găsită nicio comandă cu numele `{ $commandName }`.
# Context menu command translation strings
more-info-on-command =
    Pentru mai multe informații despre o anumită comandă, tastați `{ $contextPrefix }help <name>`
    ```
# Language configuration strings
# This message is shown as the embed title when the guild sets their language successfully.
guild-language-set-success = Limba guild setată la `{ $language }`.
# Language configuration strings
# This message is shown as the embed title when an entity tries to set their language to an unsupported language.
language-set-failure-title-unsupported = Limba pe care ați specificat-o nu este acceptată de bot.
# Language configuration strings
# This message is shown as the embed description when an entity tries to set their language to an unsupported language.
language-set-failure-description-unsupported = Dacă doriți să ajutați cu adăugarea asistenței pentru această limbă, vă rugăm să vă alăturați serverului de asistență la { $supportServerInvite }.
# Language configuration strings
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = Limba pe care ați specificat-o este un identificator de limbă nevalid. Motiv: { $error }
# Command invocation contexts
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = Vă rugăm să invocați numai subcomenzile acestei comenzi pentru a o utiliza. Consultați `{ $contextPrefix }help { $commandName }` pentru mai multe informații.
# ping command
# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    Latență WebSocket: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    Latență HTTP: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Latența bazei de date: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)

    Notă: dacă orice latență este egală cu 0 ms, înseamnă că latența specifică nu a putut fi calculată în acest moment.
    Încercați mai târziu.
# data_storage command
# This and all attributes show up exclusively in the slash command picker when `data_storage` is selected.
cmds_data_storage = stocarea_datelor
    .description = Configurați setările de stocare pentru datele dvs
# data_storage command
data-storage-toggle-audio-btn = Comutați stocarea audio
# automod setup command
automod-setup-embed-complete-title = Configurare automod completă!
# automod setup command
automod-setup-embed-complete-description = Acum puteți folosi `{ $contextPrefix }automod rule add` pentru a adăuga o regulă automod. { $extraDetails }
# automod add rule command
automod-add-rule-embed-failure-description-free-limit = Serverele gratuite sunt limitate la 25 de reguli obișnuite. Dacă doriți să creșteți această limită, consultați Premium la https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-failure-description-free-locked-type = Serverele gratuite pot folosi doar reguli obișnuite. Dacă doriți să utilizați alte tipuri de reguli, consultați Premium la https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-failure-description-not-setup = Trebuie să rulați `{ $contextPrefix }automod setup` înainte de a adăuga reguli.
# automod remove rule command
# This and all attributes show up exclusively in the slash command picker when `automod remove rule` is selected.
cmds_remove_rule = Eliminați_regula
    .description = Eliminați o regulă automod.
    .rule_id = rule_id
    .rule_id-description = ID-ul regulii de eliminat.
# automod remove rule command
automod-remove-rule-embed-success-description = { $rulesLeft } reguli rămase din { $maxRules }.
# automod remove rule command
automod-remove-rule-embed-failure-title = Nu s-a putut elimina regula!
# automod list rules command
automod-list-rules-embed-field-value =
    Tip: { $ruleType }
    Conținut: { $ruleContent }
    Acțiune: { $ruleAction }
# automod list rules command
automod-list-rules-footer = Pagina { $page } din { $maxPage }
# automod list rules command
automod-list-rules-no-rules = Nu ai reguli!
# vote reminder command
cmds_vote_reminder = vot_reminder
    .description = Comutați dacă Scripty să vă va reamintească să votați pentru bot după ce limita de timp a trecut.
    .enabled = activat
    .enabled-description = Activați mementourile de vot?
# vote reminder command
vote-reminders-enabled = Mementourile de vot au fost activate.
# vote reminder command
vote-reminders-disabled = Mementourile de vot au fost dezactivate.
# blocked entities description
blocked-entity-reason-given = Motivul dat pentru bloc: { $reason }.
# blocked entities description
blocked-entity-user = Sunteți blocat de la utilizarea Scripty. { $reason } Puteți încerca să contestați acest blocaj pe serverul de asistență: { $supportServerInvite }.
# blocked entities description
blocked-entity-guild = Această breaslă este blocată să folosească Scripty. { $reason } Puteți încerca să contestați acest blocaj pe serverul de asistență: { $supportServerInvite }.
# voice connection errors
voice-connection-error-internal-lib-error = eroare internă a bibliotecii
# voice connection errors
voice-connection-error-host-io-error = eroare IO al gazdei
# voice connection errors
voice-connection-error-proto-violation = biblioteca și discord nu au fost de acord cu protocolul
# voice connection errors
voice-connection-error-timed-out = a expirat în așteptarea conexiunii
# voice connection errors
voice-connection-error-ws-closed-no-reason = discord a închis conexiunea fără motiv
# voice connection errors
voice-connection-error-ws-closed-unknown-opcode = conexiunea discord a fost închisă din cauza unui cod operațional necunoscut
# voice connection errors
voice-connection-error-ws-closed-already-authenticated = conexiune discord a fost închisă din cauza autentificării deja
# voice connection errors
voice-connection-error-ws-closed-session-invalid = sesiune discord invalidată
# voice connection errors
voice-connection-error-ws-closed-session-timeout = sesiunea a expirat
# voice connection errors
voice-connection-error-ws-closed-server-not-found = serverul de voce nu a putut fi găsit
# voice connection errors
voice-connection-error-ws-closed-unknown-encryption-mode = discord nu a recunoscut schema de criptare
# voice connection errors
voice-connection-error-unknown = deconectat dintr-un motiv necunoscut
# voice connection errors
voice-connection-error-msg-no-reconnect = Am avut o problemă ({ $reason }) și m-am deconectat de la chatul vocal.
# general errors
general-error-invalid-structure-title = Structură nevalidă din Discord în timpul analizei { $command }.
# general errors
general-error-invalid-structure-description =
    { $description }

    { "**" }Notă**: aceasta este o eroare al Discord-ului.
    Singura soluție pentru aceasta este să așteptați ca Discord să propage comenzile oblice, ceea ce poate dura până la o oră.
    Dacă nu doriți să așteptați această oră, ar trebui să utilizați comenzile de prefix: rulați această comandă cu `~{ $qualifiedName } { $args }`.
# general errors
general-error-cooldown-hit-title = Cooldown pe { $command }
# general errors
general-error-user-missing-perms-description-known = Permisiuni lipsă: { $perms }
# general errors
general-error-user-missing-perms-description-unknown = Nu sunt sigur ce permisiuni îți lipsesc.
# general errors
general-error-user-missing-perms-description-not-owner = Nu este proprietarul acestui bot.
# transcription info - verbose mode
# This is shown as the title of the transcript
transcription-info-transcription-title = Transcriere
# transcription info - verbose mode
# This is shown as the percent accuracy of the transcription (roughly)
transcription-info-transcription-confidence = Încredere
# transcription info - verbose mode
# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.
transcription-info-transcription-ssrc = SSRC { $ssrc }
# Data deletion command
# This and all attributes show up exclusively in the slash command picker when `delete_all_data` is selected.
cmds_delete_all_data = șterge_toate_datele
    .description = Ștergeți toate datele dvs.
# Data deletion command
delete-data-confirm-banned = Da, șterge toate datele și banează-mă
# join command
# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = Nu ești într-un chat vocal și nici nu mi-ai spus un canal la care să mă alătur. Încercați `{ $contextPrefix }join <channel>` pentru a specifica un chat vocal sau alăturați-vă singur la un chat vocal și reluați această comandă.
# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = Canalul de forum pe care ai încercat să mă faci să-l folosesc îmi necesită etichete. Nu am de unde să știu ce etichete să folosesc, așa că nu mă pot alătura canalului respectiv. Utilizați un alt canal sau cereți unui administrator să elimine cerința de etichetă.
# join command
# This message is shown when Discord tosses a Dropped or TimedOut error when trying to join a voice channel.
join-failed-dropped = Discord pare să aibă probleme, nu putem face nimic în acest sens. Vă rugăm să încercați din nou mai târziu.
# join command
# If the user specifies they would like to create a thread, this is set as the thread name. { $timestamp } is the current timestamp, in ISO format.
join-thread-title = Transcriere din { $timestamp }
# join command
# This message is shown when the user attempts to make Scripty join a voice channel, but there is no one in the channel.
join-no-one-in-channel = Nu este nimeni în { $targetMention }. Nu mă alătur dacă nu este nimeni acolo, deoarece este o risipă de resurse limitate.
# join command
# This message is shown when the bot does not have permissions for the voice channel it is trying to join.
join-no-permission = Nu am permisiunea de a mă alătura { $targetMention }. Vă rog să-mi acordați permisiunile de vizualizare a canalul și de alăturare sau alăturați-vă unui alt chat vocal unde am permisiuni.
# Data deletion command
delete-data-title = Ștergeți datele
# config - translate command
config-translate-enabled = Scripty va traduce acum transcripțiile în engleză.
# config - translate command
config-translate-disabled = Scripty va încerca acum să potrivească expresiile rostite cu cuvintele în limba engleză, dar nu se va traduce.
# Context menu command translation strings
context-menu-command-user =
    { "" }
    { $commandName } (pentru utilizator)
    { "" }
# config - translate command
config_translate = Traduceți
    .description = Traduceți automat transcripțiile în engleză?
    .translate = traduceți
    .translate-description = Setarea implicit setată la fals
# Language configuration strings
guild-language-set-failure-translate-enabled = Serverul dvs. are traducerea automată activată. Acest lucru este acceptat numai atunci când traduceți în engleză. Dezactivați această funcție dacă doriți să vă setați limba.
# voice connection errors
voice-connection-error-ws-closed-server-crashed = serverul de voce discord s-a prăbușit
# general errors
# Note $time will be a decimal with two digits of accuracy.
general-error-cooldown-hit-description = Au mai rămas { $time } secunde pentru cooldown.
# general errors
general-error-command-check-failed-title = O precondiție pentru { $command } a eșuat.
# join command
# This message is shown when the user has told the bot to send transcripts to a non-text-based channel (ie category). `target_channel` should be translated, as slash command arguments are localized.
join-target-not-text-based = Canalul către care mi-ai spus să trimit transcrieri ({ $targetMention }) nu este un canal bazat pe text. Utilizați un canal bazat pe text sau alegeți un alt canal în argumentul `canal_țintă`.
# transcribe_message command
# This and all attributes show up exclusively in the slash command picker when `transcribe_message` is selected.
cmds_transcribe_message = transcrie_mesaje
    .description = Transcrie un mesaj. Răspundeți la un mesaj pentru a-l transcrie.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = Revendica
    .description = Revendicați-vă statutul premium în serverul pe care este executată această comandă.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = elimina
    .description = Eliminați premium de pe serverul pe care este executată această comandă.
# premium command
# This is shown to the user when they have too many used servers to add more.
premium-too-many-guilds =
    Ați revendicat { $totalServers } chei premium. Nu puteți adăuga mai multe, decât dacă vă actualizați abonamentul premium la <https://dash.scripty.org/premium>, sau eliminați unele cu comanda
    `{ $commandPrefix }premium ștergere`.
# premium command
# This is shown when the user successfully removes their premium from this guild.
premium-removed = Dacă sunteți utilizatorul care a revendicat statutul Premium, acum ați eliminat cu succes statutul premium de pe acest server. Dacă doriți să faceți upgrade sau să cumpărați mai multe sloturi, accesați <https://dash.scripty.org/premium>.
# config - verbose command
config-verbose-disabled = Scripty nu va mai fi detaliat în timpul transcripțiilor.
# config - verbose command
config-verbose-enabled = Scripty va fi acum detaliat în timpul transcripțiilor.
# config - transcribe voice messages command
config-transcribe-voice-messages-enabled = Scripty va transcrie acum mesajele vocale.
# config - transcribe voice messages command
config_transcribe_voice_messages = transcrie_mesaje_vocale
    .description = Comută dacă Scripty transcrie mesajele vocale.
    .transcribe_voice_messages = transcrie_mesaje_vocale
    .transcribe_voice_messages-description = Setarea implicit setata la Adevărat
# config - transcribe voice messages command
config-transcribe-voice-messages-disabled = Scripty nu va mai transcrie mesajele vocale.
# config - transcribe audio command
config_transcribe_audio = transcrie_audio
    .description = Comută dacă Scripty transcrie fișiere audio arbitrare. Necesită statut premium.
    .transcribe_audio = transcrie_audio
    .transcribe_audio-description = Setarea implicit setata la fals
# config - transcribe audio command
config-transcribe-audio-enabled = Scripty va transcrie acum fișierele audio.
# config - transcribe audio command
config-transcribe-audio-disabled = Scripty nu va mai transcrie fișierele audio.
# config - transcribe video command
config_transcribe_video = transcrie_video
    .description = Comută dacă Scripty transcrie fișiere video arbitrare. Necesită T2 premium.
    .transcribe_video = transcrie_video
    .transcribe_video-description = Setarea implicit setata la fals
# config - transcribe video command
config-transcribe-video-enabled = Scripty va transcrie acum fișierele video.
# config - transcribe video command
config-transcribe-video-disabled = Scripty nu va mai transcrie fișierele video.
# config - transcribe video command
config-transcribe-video-requires-premium =
    Transcrierea fișierelor video este o caracteristică Premium Tier 2, deoarece este foarte costisitoare din punct de vedere computațional să transcodăm fișierele video.
    Dacă doriți să faceți upgrade la Premium Tier 2, accesați https://dash.scripty.org/premium.
    Dacă această caracteristică a fost activată înainte, acum este dezactivată.
# config - auto detect language command
config-auto-detect-lang-enabled = Scripty va detecta acum automat limba vorbită.
# config - auto detect language command
config-auto-detect-lang-disabled = Scripty nu va mai detecta automat limba vorbită.
# config - auto detect language command
config-auto-detect-lang-requires-premium =
    Detectarea automată a limbii este o caracteristică Premium, deoarece este extrem de costisitoare din punct de vedere computațional să reporniți modelul de două ori pentru a afla limba.
    Dacă doriți să faceți upgrade la Premium, accesați https://dash.scripty.org/premium. De asemenea, puteți solicita o încercare gratuită a Premium prin DM la bot.
    Dacă această caracteristică a fost activată înainte, acum este dezactivată.
# data_storage command
data-storage-command-timed-out = A expirat. Reluați această comandă dacă tot doriți să gestionați setările.
# automod root command
# This and all attributes show up exclusively in the slash command picker when `automod` is selected.
cmds_automod = automod
    .description = Gestionați automodul lui Scripty
# automod setup command
automod-setup-embed-not-setup-title = Încă nu ați fost de acord cu Termenii și condițiile și Politica de confidențialitate Scripty.
# automod setup command
automod-setup-embed-not-setup-description = Faceți acest lucru mai întâi rulând `{ $contextPrefix } terms_of_service`.
# data_storage command
data-storage-embed-description =
    { "**" }NOTĂ**: tot ceea ce urmează este **complet opțional**, iar renunțarea **nu va**, în niciun fel, să vă afecteze experiența cu Scripty.
    Acestea fiind spuse, iată.

    Scripty necesită o mulțime de date audio și text pentru a antrena un model corect de vorbire în text. Nu toată lumea poate să doneze sau să cumpere premium pentru a ne ajuta, așa că un mod important în care ne puteți ajuta este permițându-ne să stocăm datele dvs., cum ar fi sunetul și mesajele pentru antrenarea unui model.
    Înțelegem că aceste date pot fi extrem de personale, așa că aceasta este în totalitate opt-in și nu va afecta experiența dumneavoastră în niciun fel.

    Iată ce am face cu el:
    { "*" } Cu mesajele stocate, le-am introduce într-un marcator care vizează limba dvs. Acest scorer ar permite algoritmului să selecteze cuvintele cele mai probabile pentru un anumit set de sunete. Deși extrem de util, acest lucru nu este la fel de important ca audio. Rețineți că datele acestui mesaj sunt criptate cu criptare AES pe 256 de biți.
    { "*" } Cu sunetul stocat, l-am alimenta pe acesta și transcrierea acestuia într-un model pentru a crește acuratețea modelului vorbire în text.
    Acest lucru este nebunește de util, chiar dacă aveți un microfon slab și mult zgomot de fundal: de fapt, cu cât mai mult zgomot, cu atât mai bine, atâta timp cât un om poate înțelege tot ce spui.
    Dacă sunteți înscris și decideți ulterior să renunțați, datele dvs. sunt în continuare stocate, dar puteți solicita ștergerea datelor dvs. vocale rulând `{ $contextPrefix }delete_all_data`. Cu toate acestea, este imposibil să ștergeți datele mesajului dvs. Acest lucru se datorează faptului că nu stocăm un link cu ce utilizator a trimis ce mesaj.
    Datele dvs. sunt stocate pe servere care sunt blocate strâns. Ar fi extrem de dificil pentru oricine încearcă să obțină acces să o facă cu succes.

    Puteți comuta opțiunile folosind butoanele de mai jos.
# Language configuration strings
language-set-partially-translated-help = Doriți să ajutați la traducerea Scripty în limba dvs.? Consultă proiectul de traducere la https://hosted.weblate.org/engage/scripty-bot/.
