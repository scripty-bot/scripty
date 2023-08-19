# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = Het forumkanaal dat je hebt geprobeerd me te laten gebruiken, vereist tags. Ik heb geen manier om te weten welke tags ik moet gebruiken, dus ik kan dat kanaal niet betreden. Gebruik alsjeblieft een ander kanaal of vraag een beheerder om de tagvereiste te verwijderen.
# Language configuration strings
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = De database heeft een error ondervonden om jouw taal in te stellen. Deze error is al gerapporteerd, we zullen er heen kijken. Asjeblieft spam dit commando niet. (Als je nieuwsgierig bent, hier is de error: { $error })
# Command invocation contexts
# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = Dit is een hoofdcommando!
# Command invocation contexts
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = Roep alleen de subcommando's van deze opdracht aan om deze te gebruiken. Bekijk `{ $contextPrefix }help { $commandName }` voor meer informatie.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to select a button in time.
tos-agree-timed-out = Time-out. Voer deze opdracht opnieuw uit als je nog steeds akkoord wilt gaan met de Gebruiksvoorwaarden.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user agrees to the ToS.
tos-agree-success = Je hebt succesvol ingestemd met de Gebruiksvoorwaarden en het Privacybeleid van Scripty. Je kunt nu Scripty gebruiken.
# data_storage command
data-storage-embed-title = Data opslag
# general errors
general-error-command-check-failed-description-no-reason = Geen reden opgegeven
# ToS command
# This and all attributes show up exclusively in the slash command picker when `terms_of_service` is selected.
cmds_terms_of_service = tems_of_service
    .description = Bekijk en ga akkoord met de Gebruiksvoorwaarden van Scripty.
# general errors
general-error-user-missing-perms-description-known = Missende rechten: { $perms }
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to agree to the ToS, usually by explicitly clicking the "No" button.
disagreed-to-tos = Je hebt niet ingestemd met de Gebruiksvoorwaarden en het Privacybeleid van Scripty. Als je Scripty wilt gebruiken, moet je akkoord gaan met deze documenten. Dit kun je doen door deze opdracht opnieuw uit te voeren.
# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = join
    .description = Join een spraakchat. Transcripties worden vastgelegd in het kanaal waarin je het commando uitvoert.
    .voice_channel = voice_channel
    .voice_channel-description = Spraak kanaal om te koppelen.
    .record_transcriptions = record_transcriptions
    .record_transcriptions-description = Sla alle Scripty transcripties op, dit staat standaard uit.
    .target_channel = target_channel
    .target_channel-description = Stuur transcripties hier, in plaats van het huidige kanaal. Doel naar een forum om een nieuwe post te creëren.
    .create_thread = create_thread
    .create_thread-description = Een nieuwe thread aanmaken voor deze transcriptie? Dit staat standaard uit.
# Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = leave
    .description = Verlaat het huidige spraak kanaal.
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = Succesvol het spraak kanaal verlaten.
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = help
    .description = Toon het help menu
    .command = opdracht
    .command-description = Specifieke opdracht om help over te tonen
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = premium
    .description = Premium opdrachten
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = claim
    .description = Haal je premiumvoordelen op in de server waarin dit wordt uitgevoerd.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = remove
    .description = Verwijder je premiumvoordelen uit de server waarin dit wordt uitgevoerd.
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = Je bent geen premium-abonnee. Abonneer je op https://scripty.org/premium. Als je zeker weet dat je er een bent, stuur dan een privébericht naar de bot zodat we je premium kunnen herstellen.
# premium command
# This is shown to the user when they have too many used servers to add more.
premium-too-many-guilds = Je hebt { $totalServers } premiumcodes geclaimd. Je kunt er geen meer toevoegen, tenzij je je premiumabonnement upgrade op <https://dash.scripty.org/premium>, of enkele verwijdert met het commando { $commandPrefix }premium remove.
# premium command
# This is shown when the guild the user is running this command in has not yet agreed to the ToS.
premium-server-not-set-up = Deze server heeft nog niet ingestemd met de Gebruiksvoorwaarden en het Privacybeleid van Scripty. Doe dit eerst met het commando `{ $commandPrefix }terms_of_service`.
# premium command
# This is shown when the user successfully claims one of their premium subscriptions.
premium-claimed = Je hebt met succes premium geclaimd op deze server. Als je wilt upgraden of meer slots wilt kopen, ga naar <https://dash.scripty.org/premium>. Als je je premium wilt verwijderen van deze server, voer `{ $commandPrefix }premium remove` uit.
# premium command
# This is shown when the user successfully removes their premium from this guild.
premium-removed = Als jij degene bent die Premium had geclaimd, heb je nu succesvol je premium van deze server verwijderd. Als je wilt upgraden of meer slots wilt kopen, ga naar <https://dash.scripty.org/premium>.
# Help menu translation strings
command-not-found-suggestions = Bedoelde je `{ $suggestion }`?
# Help menu translation strings
no-help-found = Geen hulp gevonden voor commando `{ $commandName }`.
# Help menu translation strings
command-not-found = Geen commando gevonden met de naam `{ $commandName }`.
# Help menu translation strings
default-category-name = Commandos
# Context menu command translation strings
context-menu-command-title =
    { "" }
    Context menu commandos:
    { "" }
# Context menu command translation strings
context-menu-command-user =
    { "" }
    { $commandName } (op gebruiker)
    { "" }
# Context menu command translation strings
more-info-on-command =
    Voor meer informatie over een specifieke commando, type `{ $contextPrefix }help <name>`
    ```
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language` is selected.
cmds_language = language
    .description = Verander jouw taal voorkeur.
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language user_language` is selected.
cmds_user_language = user
    .description = Zet jouw gebruikers taal naar een van de beschikbare talen.
    .language = taal
    .language-description = De taal waar jij je gebruikers taal heen wilt veranderen.
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language guild_language` is selected.
cmds_guild_language = guild
    .description = Zet deze server taal naar een van de beschikbare talen.
    .language = taal
    .language-description = De taal waar je de server heen wilt veranderen.
# Language configuration strings
# This message is shown as the embed title when the user sets their language successfully.
user-language-set-success = Gebruikers taal is veranderd naar `{ $language }`.
# Language configuration strings
# This message is shown as the embed description when the user sets their language successfully.
user-language-set-success-description = Om weer naar Engels te gaan, type `{ $contextPrefix }taal gebruikers_taal en`.
# Language configuration strings
# This message is shown as the embed title when the guild sets their language successfully.
guild-language-set-success = Server taal is nu `{ $language }`.
# Language configuration strings
# This message is shown as the embed description when the guild sets their language successfully.
guild-language-set-success-description = Om weer naar engels terug te keren, type `{ $contextPrefix }taal server_taal en`.
# Language configuration strings
# This message is shown as the embed title when an entity tries to set their language to an unsupported language.
language-set-failure-title-unsupported = De taal die je verzoekt is niet ondersteunt door de bot.
# Language configuration strings
# This message is shown as the embed description when an entity tries to set their language to an unsupported language.
language-set-failure-description-unsupported = Als je graag wilt helpen met ondersteuning toevoegen voor deze taail, join de support server { $supportServerInvite }.
# Language configuration strings
# This message is shown as the embed title when an entity tries to set their language to an invalid language.
language-set-failure-title-invalid = Taal `{ $language }` is niet gevonden.
# Context menu command translation strings
context-menu-command-message =
    { "" }
    { $commandName } (op bericht)
    { "" }
# Language configuration strings
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = De opgegeven taal is een ongeldige taalidentificatie. Reden: { $error }
# Language configuration strings
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = Database error.
# ping command
# This and all attributes show up exclusively in the slash command picker when `ping` is selected.
cmds_ping = ping
    .description = Ontvang de latentie van de bot.
# data_storage command
# This and all attributes show up exclusively in the slash command picker when `data_storage` is selected.
cmds_data_storage = data_storage
    .description = Configureer opslaginstellingen voor jouw gegevens
# data_storage command
data-storage-embed-description =
    { "**" }OPMERKING**: alles wat volgt is **volledig optioneel**, en ervoor kiezen om dit niet te doen, **zal op geen enkele manier** invloed hebben op je ervaring met Scripty.
    Dat gezegd hebbende, hier gaan we.

    Scripty heeft veel audio- en tekstdatabestanden nodig om een degelijk spraak-naar-tekst model te trainen. Niet iedereen kan doneren of premium kopen om ons te helpen, dus een grote manier waarop je kunt bijdragen is door ons toe te staan jouw gegevens zoals audio en berichten op te slaan voor het trainen van een model.
    We begrijpen dat deze gegevens zeer persoonlijk kunnen zijn, dus dit is volledig optioneel en zal je ervaring op geen enkele manier beïnvloeden.

    Dit is wat we ermee zouden doen:
    { "*" } Met opgeslagen berichten zouden we ze door een scorende tool sturen die gericht is op jouw taal. Deze scorende tool zou het algoritme in staat stellen om de meest waarschijnlijke woorden te selecteren voor een gegeven reeks geluiden. Hoewel enorm nuttig, is dit niet zo belangrijk als audio. Let op dat deze berichtgegevens versleuteld zijn met AES 256-bit encryptie.
    { "*" } Met opgeslagen audio zouden we deze samen met de transcriptie ervan aan een model voeden om de nauwkeurigheid van het spraak-naar-tekst model te vergroten. Dit is ontzettend nuttig, zelfs als je een slechte microfoon hebt en veel achtergrondgeluid: eigenlijk geldt, hoe meer lawaai, hoe beter, zolang een mens nog steeds kan horen wat je zegt.

    Als je bent ingeschreven en later besluit om je uit te schrijven, worden je gegevens nog steeds bewaard, maar je kunt verzoeken om verwijdering van je spraakgegevens door `{ $contextPrefix }delete_all_data` uit te voeren. Het is echter onmogelijk om je berichtgegevens te verwijderen. Dit komt doordat we geen link opslaan van welke gebruiker welk bericht heeft verzonden.
    Je gegevens worden opgeslagen op streng beveiligde servers. Het zou uiterst moeilijk zijn voor iedereen die probeert toegang te krijgen om dit met succes te doen.

    Je kunt je keuzes aanpassen met behulp van de onderstaande knoppen.
# data_storage command
data-storage-toggle-audio-btn = Audio-opslag in- of uitschakelen
# data_storage command
data-storage-toggle-msgs-btn = Berichtenopslag in- of uitschakelen
# data_storage command
data-storage-opted-in-audio = Je bent nu ingeschreven voor het opslaan van je audio voor modeltraining.
# data_storage command
data-storage-opted-out-audio = Je bent nu uitgeschreven voor het opslaan van je audio voor modeltraining.
# data_storage command
data-storage-opted-in-msgs = Je bent nu ingeschreven voor het opslaan van je berichten voor scorerafname.
# data_storage command
data-storage-opted-out-msgs = Je bent nu uitgeschreven voor het opslaan van je berichten voor scorerafname.
# data_storage command
data-storage-command-timed-out = Time-out. Voer deze opdracht opnieuw uit als je nog steeds de instellingen wilt beheren.
# automod root command
# This and all attributes show up exclusively in the slash command picker when `automod` is selected.
cmds_automod = automod
    .description = Beheer de automatische moderatie van Scripty
# ping command
# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    WebSocket latentie: { $wsLatencyMs } ms ({ $wsLatencyNs } ns)
    HTTP latentie: { $httpLatencyMs } ms ({ $httpLatencyNs } ns)
    Database latentie: { $pgLatencyMs } ms ({ $pgLatencyNs } ns)

    Opmerking: als een latentie gelijk is aan 0 ms, betekent dit dat die specifieke latentie op dit moment niet berekend kan worden.
    Probeer later opnieuw.
# automod root command
automod-root-response = Dit is het hoofdcommando, vanwege Discord-beperkingen doet het niets. Bekijk `{ $contextPrefix }help automod` voor meer informatie.
# automod setup command
# This and all attributes show up exclusively in the slash command picker when `automod setup` is selected.
cmds_setup = setup
    .description = Aan de slag met Scripty's automatische moderatie.
    .target_channel = doelkanaal
    .target_channel-description = Het kanaal om automod logs naartoe te sturen.
    .log_recording = log_opname
    .log_recording-description = Moet er een opname van de huidige taal gestuurd worden? Standaard staat dit uit.
    .auto_join = auto_join
    .auto_join-description = Scripty automatisch joinen als een gebruiker joint? Standaard is dit ingeschakeld.
# automod setup command
automod-setup-embed-complete-title = Automatische moderatie setup voltooid!
# automod setup command
automod-setup-embed-complete-description = Je kunt nu `{ $contextPrefix }automod rule add` gebruiken om een automod-regel toe te voegen. { $extraDetails }
# automod setup command
automod-setup-embed-complete-free-limit = Let op: gratis servers hebben een limiet van 25 regels. Als je deze limiet wilt verwijderen, bekijk dan onze Premium-optie op https://scripty.org/premium.
# automod setup command
automod-setup-embed-not-setup-title = Je hebt nog niet ingestemd met de Algemene Voorwaarden en het Privacybeleid van Scripty.
# automod setup command
automod-setup-embed-not-setup-description = Doe dit eerst door `{ $contextPrefix } terms_of_service` uit te voeren.
# automod setup command
automod-setup-auto-join-premium-only = Automatisch aansluiten is een premium functie. Bekijk onze Premium-optie op https://scripty.org/premium.
# automod add rule command
# This and all attributes show up exclusively in the slash command picker when `automod add rule` is selected.
cmds_add_rule = add_rule
    .description = Voeg een automatische moderatie regel toe.
    .rule_type = regel_type
    .rule_type-description = Het type regel dat moet worden toegevoegd. Zie /automod rule_help voor meer informatie.
    .rule_type-choice-Regular = Standaard
    .content = inhoud
    .content-description = De regelinhoud die moet worden toegevoegd.
    .action = actie
    .action-description = De actie die moet worden ondernomen wanneer de regel wordt geactiveerd.
    .action-choice-SilentDelete = Stil verwijderen
    .action-choice-DeleteAndLog = Verwijderen en loggen
    .action-choice-DeleteLogAndKick = Verwijderen, loggen en gebruiker uit spraak verwijderen
    .action-choice-DeleteLogAndSilence = Verwijderen, loggen en gebruiker dempen
# automod add rule command
automod-add-rule-embed-success-title = Regel { $ruleId } toegevoegd!
# automod add rule command
automod-add-rule-embed-success-description = Nog { $rulesLeft } regels over van de { $maxRules }. { $extraDetails }
# automod add rule command
automod-add-rule-embed-extra-details-free-limit = Gratis servers hebben een limiet van 25 standaardregels. Als je deze limiet wilt verhogen, bekijk dan onze Premium-optie op https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-failure-title = Het toevoegen van de regel is mislukt!
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit = Servers van Premium niveau { $tier } hebben een limiet van { $maxRules } regels. Als je upgrade naar niveau { $nextTier }, kun je { $nextTierMaxRules } regels toevoegen.
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit-hard-cap = Je hebt het absolute maximum aantal regels bereikt ({ $hardCap }). Deze limiet is ingesteld om ervoor te zorgen dat we niet te veel latentie toevoegen aan één bericht.
# automod add rule command
automod-add-rule-embed-failure-description-invalid-type = Ongeldig regeltype. Zie `{ $contextPrefix }automod rule_help` voor meer informatie.
# automod add rule command
automod-add-rule-embed-failure-description-free-locked-type = Gratis servers kunnen alleen standaardregels gebruiken. Als je andere regeltypes wilt gebruiken, bekijk dan onze Premium-optie op https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-failure-description-not-setup = Je moet `{ $contextPrefix }automod setup` uitvoeren voordat je regels kunt toevoegen.
# automod add rule command
automod-add-rule-embed-failure-description-free-limit = Gratis servers hebben een limiet van 25 standaardregels. Als je deze limiet wilt verhogen, bekijk dan onze Premium-optie op https://scripty.org/premium.
# automod remove rule command
# This and all attributes show up exclusively in the slash command picker when `automod remove rule` is selected.
cmds_remove_rule = remove_rule
    .description = Verwijder een automatische moderatie regel.
    .rule_id = regel_id
    .rule_id-description = Het regel-ID die verwijderd moet worden.
# automod remove rule command
automod-remove-rule-embed-success-title = Regel verwijderd!
# automod remove rule command
automod-remove-rule-embed-success-description = { $rulesLeft } regels over van de { $maxRules }.
# automod remove rule command
automod-remove-rule-embed-failure-description-invalid-id = Ongeldige regel-ID. Zie `{ $contextPrefix }automod list` voor meer informatie.
# automod remove rule command
automod-remove-rule-embed-failure-title = Het verwijderen van de regel is mislukt!
# automod remove rule command
automod-remove-rule-embed-failure-description-not-setup = Je moet `{ $contextPrefix }automod setup` uitvoeren voordat je regels kunt verwijderen.
# automod list rules command
# This and all attributes show up exclusively in the slash command picker when `automod list rules` is selected.
cmds_list_rules = list_rules
    .description = Lijst van alle automatische moderatie regels.
    .filter_by = filter_op_basis_van
    .filter_by-description = Regels filteren op basis van hun inhoud. Laat leeg om alle regels weer te geven.
# automod list rules command
automod-list-rules-embed-title = Automatische moderatie regels
# automod list rules command
automod-list-rules-embed-description = { $rulesLeft } regels over van de { $maxRules }.
# automod list rules command
automod-list-rules-embed-field-name = Regel { $ruleId }
# automod list rules command
automod-list-rules-embed-field-value =
    Type: { $ruleType }
    Inhoud: { $ruleContent }
    Actie: { $ruleAction }
# automod list rules command
automod-list-rules-footer = Pagina { $page } van { $maxPage }
# automod list rules command
automod-list-rules-no-rules = Je hebt geen regels!
# blocked entities description
blocked-entity-no-reason-given = Er is geen reden opgegeven voor de blokkering.
# blocked entities description
blocked-entity-reason-given = Reden opgegeven voor de blokkering: { $reason }.
# blocked entities description
blocked-entity-guild = Deze server is geblokkeerd voor het gebruik van Scripty. { $reason } Je kunt proberen deze blokkade aan te vechten in de ondersteuningsserver: { $supportServerInvite }.
# blocked entities description
blocked-entity-user = Je bent geblokkeerd voor het gebruik van Scripty. { $reason } Je kunt proberen deze blokkade aan te vechten in de ondersteuningsserver: { $supportServerInvite }.
# voice connection errors
voice-connection-error-internal-lib-error = Interne fout in de bibliotheek
# voice connection errors
voice-connection-error-host-io-error = Host I/O-fout
# voice connection errors
voice-connection-error-proto-violation = De bibliotheek en Discord waren het oneens over het protocol
# voice connection errors
voice-connection-error-timed-out = Time-out bij het wachten op verbinding
# voice connection errors
voice-connection-error-ws-closed-no-reason = Discord heeft de verbinding zonder reden gesloten
# voice connection errors
voice-connection-error-ws-closed-unknown-opcode = Discord heeft de verbinding gesloten vanwege een onbekende opcode
# voice connection errors
voice-connection-error-ws-closed-invalid-payload = Discord heeft de verbinding gesloten vanwege een ongeldige payload
# voice connection errors
voice-connection-error-ws-closed-not-authenticated = Discord heeft de verbinding gesloten omdat er geen authenticatie was
# voice connection errors
voice-connection-error-ws-closed-already-authenticated = Discord heeft de verbinding gesloten omdat al een authenticatie had plaatsgevonden
# voice connection errors
voice-connection-error-ws-closed-session-invalid = Discord heeft de sessie ongeldig verklaard
# voice connection errors
voice-connection-error-ws-closed-authentication-failed = Discord heeft de verbinding gesloten vanwege een authenticatiefout
# voice connection errors
voice-connection-error-ws-closed-session-timeout = Sessie is verlopen
# voice connection errors
voice-connection-error-ws-closed-server-not-found = De spraakserver kon niet worden gevonden
# voice connection errors
voice-connection-error-ws-closed-unknown-protocol = Discord herkende het protocol niet
# voice connection errors
voice-connection-error-ws-closed-server-crashed = De spraakserver van Discord is gecrasht
# voice connection errors
voice-connection-error-msg-no-reconnect = Ik had een probleem ({ $reason }) en ben losgekoppeld van de spraakchat.
# voice connection errors
voice-connection-error-msg-reconnect = Ik had een probleem ({ $reason }) en ben losgekoppeld van de spraakchat. Ik zal over 30 seconden proberen opnieuw verbinding te maken.
# general errors
general-error-command-process-title = Er is een fout opgetreden bij het verwerken van { $command }.
# general errors
general-error-command-process-description =
    ```
    { $errorFmt }
    ```
    Dit is automatisch gemeld. Probeer dit commando niet herhaaldelijk te gebruiken.
# general errors
general-error-invalid-args-description = Mislukt om `{ $input }` te parseren omdat `{ $error }`
# general errors
general-error-invalid-structure-title = Ongeldige structuur van Discord tijdens het verwerken van { $command }.
# general errors
general-error-invalid-structure-description =
    { $description }

    { "" }Opmerking: dit is een Discord-fout.
    De enige oplossing hiervoor is wachten tot Discord de slash-commando's heeft verspreid, wat tot een uur kan duren.
    Als je niet dit uur wilt wachten, kun je de prefix-commando's gebruiken: voer dit commando uit met ~{ $qualifiedName } { $args }.
# general errors
general-error-cooldown-hit-title = Cooldown bereikt voor { $command }
# general errors
# Note $time will be a decimal with two digits of accuracy.
general-error-cooldown-hit-description = Nog { $time } seconden over op de cooldown.
# general errors
general-error-user-missing-perms-title = Je hebt geen rechten om { $command } uit te voeren.
# general errors
general-error-user-missing-perms-description-unknown = Ik ben niet zeker welke rechten je mist.
# general errors
general-error-user-missing-perms-description-not-owner = Geen eigenaar van deze bot.
# general errors
general-error-command-check-failed-title = Een voorwaarde voor { $command } is niet voldaan.
# transcription info - verbose mode
# This is shown as the number of transcriptions the algorithm has discovered.
transcription-info-transcript-count = Transcriptie 1 van { $count }.
# transcription info - verbose mode
# This is shown as the title of the transcript
transcription-info-transcription-title = Transcriptie
# voice connection errors
voice-connection-error-ws-closed-unknown-encryption-mode = Discord herkende het versleutelingsschema niet
# voice connection errors
voice-connection-error-unknown = Verbinding verbroken om onbekende reden
# transcription info - verbose mode
# This is shown as the percent accuracy of the transcription (roughly)
transcription-info-transcription-confidence = Vertrouwen
# general errors
general-error-invalid-args-title = Ongeldige argumenten tijdens het verwerken van { $command }.
# transcription info - verbose mode
# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.
transcription-info-transcription-ssrc = SSRC { $ssrc }
# join command
# This message is shown when the user tries to tell the bot to join, but they have not agreed to the ToS.
must-agree-to-tos = Je moet akkoord gaan met de Algemene Voorwaarden en het Privacybeleid om Scripty te gebruiken. Zie `{ $contextPrefix }terms_of_service` voor meer informatie.
# join command
# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = Je zit niet in een spraakchat en je hebt me ook geen kanaal opgegeven om naartoe te gaan. Probeer `{ $contextPrefix }join <kanaal>` om een spraakchat op te geven, of ga zelf naar een spraakchat en voer deze opdracht opnieuw uit.
# join command
# This message is shown on successfuly joining a voice channel.
# { $targetMention } is the mention of the channel the bot joined.
join-success =
    Succesvol verbonden met { $voiceTargetMention }, en de transcriptie-uitvoer wordt verzonden naar { $outputChannelMention }.
    { "" }
    Opmerking: je huidige premiumniveau is { $tier }. Dit staat { $maxUsers } gebruikers toe om tegelijkertijd getranscribeerd te worden. Daarnaast zal de bot automatisch vertrekken na { $leaveDuration } seconden, ongeacht hoeveel gebruikers er in het kanaal zijn. Dit is om misbruik van onze systemen te voorkomen.
    Als je meer gebruikers wilt, een langere gebruiksduur en de bot wilt ondersteunen, overweeg dan om je te abonneren op onze Premium: <https://dash.scripty.org/premium>
    Als je al een Premium-abonnee bent, stuur dan een privébericht naar de bot zodat we je Premium kunnen herstellen.
    { $freeTrialUpsell }
# join command
# This message is shown when the user attempts to make Scripty join a voice channel, but there is no one in the channel.
join-no-one-in-channel = Er is niemand in { $targetMention }. Ik sluit me niet aan als er niemand is, omdat dat verspilling is van beperkte middelen.
# Data deletion command
delete-data-title = Gegevens verwijderen
# transcription info - verbose mode
# This is shown when the algorithm encounters an error
transcription-info-transcription-error =
    Interne fout: uitvoeren van stt-algoritme mislukt met fout: { $error }
    SSRC: { $ssrc }
    Dit is gelogd en zal zo snel mogelijk worden opgelost.
    Indien mogelijk, neem contact op met de kernontwikkelaars in de ondersteuningsserver: { $supportServerInvite }.
    Bedankt!
# Data deletion command
# This and all attributes show up exclusively in the slash command picker when `delete_all_data` is selected.
cmds_delete_all_data = delete_all_data
    .description = Verwijder al je gegevens.
# Data deletion command
delete-data-description =
    Dit zal al je gegevens verwijderen. Deze actie is permanent, onomkeerbaar en kan niet ongedaan worden gemaakt.

    Met "al je gegevens" bedoelen we *alle* gegevens. Dit omvat je stemgegevens en je gebruikersgegevens in de database.
    Dit omvat echter *niet* eventuele berichten die we van je hebben opgeslagen als je hiervoor hebt gekozen. We kunnen die berichten niet verwijderen, omdat we niet weten welke gebruiker welk bericht heeft verzonden.

    Als je na deze actie ook verbannen wilt worden van het gebruik van de bot, zodat je jezelf niet per ongeluk weer kunt toevoegen, kun je op de juiste knop hieronder klikken.
    Houd er rekening mee dat dit vereist dat we je gebruikers-ID opslaan om een lijst van verbannen gebruikers bij te houden.
    Als je op enig moment na deze actie wilt worden vrijgegeven, kun je contact opnemen met de ondersteuningsserver en vragen om een handmatige opheffing van de ban.

    Weet je zeker dat je al je gegevens wilt verwijderen?
# Data deletion command
delete-data-cancel = Nee, annuleren
# Data deletion command
delete-data-confirm = Ja, verwijder alle gegevens
# Data deletion command
delete-data-confirm-banned = Ja, verwijder alle gegevens en verbied mezelf
# generic strings
# Message shown if a guild has not claimed their free trial of premium. Always appears on its own standalone line in the surrounding message.
free-trial-upsell = We bieden 3-daagse proefperiodes van Scripty Premium aan als je het wilt uitproberen en wilt zien of het iets voor jou is. Stuur de bot een privébericht om te beginnen met een gratis proefperiode.
# ToS command
# This is sent when the user has not yet agreed to the ToS and must do so.
agreeing-to-tos = Je kunt de Algemene Voorwaarden en het Privacybeleid van Scripty bekijken op respectievelijk https://scripty.org/terms en https://scripty.org/privacy. Je kunt op de onderstaande knop klikken om akkoord te gaan met beide documenten en Scripty te gebruiken.
# ToS command
# This is sent when the user has already agreed to the ToS and does not need to do so again.
already-agreed-to-tos = Je hebt al ingestemd met de Algemene Voorwaarden en het Privacybeleid van Scripty. Als je ze opnieuw wilt bekijken, kun je dat doen op respectievelijk https://scripty.org/terms en https://scripty.org/privacy.
# join command
# This message is shown when Discord tosses a Dropped or TimedOut error when trying to join a voice channel.
join-failed-dropped = Discord lijkt problemen te hebben, hier kunnen we niets aan doen. Probeer het later opnieuw.
# join command
# This message is shown when the bot does not have permissions for the voice channel it is trying to join.
join-no-permission = Ik heb geen toestemming om { $targetMention } binnen te gaan. Geef me alstublieft de toestemmingen om het kanaal te bekijken en toe te treden, of ga naar een andere spraakchat waar ik wel toestemming heb.
# join command
# This message is shown when the user has told the bot to create a thread while in a thread.
join-create-thread-in-thread = Ik kan geen thread maken terwijl ik me in een thread bevind. Voer deze opdracht uit in een normaal kanaal, waarschijnlijk { $parentChannelMention }.
# join command
# If the user specifies they would like to create a thread, this is set as the thread name. { $timestamp } is the current timestamp, in ISO format.
join-thread-title = Transcriptie vanaf { $timestamp }
# join command
# If the user specifies they would like to create a forum post, this is the contents of the initial message. { $timestamp } is the current timestamp, in ISO format, and { $authorMention } is the mention of the user who ran the command.
join-forum-thread-content = { $authorMention } is begonnen met een transcriptie om { $timestamp }.
