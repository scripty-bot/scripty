# This message is shown as the embed description when a user tries to invoke the root command of a group.


# This message is shown when the user is not in a voice channel, nor was a voice channel specified.


# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.


# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.


# This message is shown as the embed title when the user sets their language successfully.


# This message is shown as the embed description when the user sets their language successfully.


# This message is shown as the embed title when the guild sets their language successfully.


# This message is shown as the embed description when the guild sets their language successfully.


# This message is shown as the embed title when an entity tries to set their language to an unsupported language.


# This message is shown as the embed title when a user tries to invoke the root command of a group.


# This message is shown on successfuly joining a voice channel.
# {$targetMention} is the mention of the channel the bot joined.


# This message is shown as the embed description when an entity tries to set their language to an unsupported language.


# This message is shown as the embed description when the database returns an error when setting the language for an entity.


# This message is shown as the embed title when an entity tries to set their language to an invalid language.


# This message is shown as the embed description when an entity tries to set their language to an invalid language.


# This message is shown as the embed title when the database returns an error when setting the language for an entity.


# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.


# This is shown as the number of transcriptions the algorithm has discovered.


# This is shown as the title of the transcript


# This is shown as the percent accuracy of the transcription (roughly)


# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.


# This is shown when the algorithm encounters an error

# This message is shown as the embed description when an entity tries to set their language to an unsupported language.
language-set-failure-description-unsupported = Wenn du mithelfen möchtest, den Bot in diese Sprache zu übersetzen, trete dem Support-Server bei: { $supportServerInvite }.
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = Es gab einen Fehler in der Datenbank beim Speichern deiner Sprache. Der Fehler ist gemeldet und wir werden ihn uns anschauen. Bitte spamme diesen Befehl nicht. (Für die Neugierigen ist hier der Fehler: { $error })
command-not-found = Kein Befehl mit Name `{ $commandName }` gefunden.
command-not-found-suggestions = Meintest du `{ $suggestion }`?
no-help-found = Keine Hilfe gefunden für Befehl `{ $commandName }`.
default-category-name = Befehle
context-menu-command-title =
    { "" }
    Kontextmenübefehle:
    { "" }
context-menu-command-user =
    { "" }
    { $commandName } (für Nutzer)
    { "" }
context-menu-command-message =
    { "" }
    { $commandName } (für Nachrichten)
    { "" }
more-info-on-command =
    Für weitere Information über einen Befehl, verwende `{ $contextPrefix }help <name>`
    ```
# This message is shown as the embed title when the user sets their language successfully.
user-language-set-success = Benutzersprache ist nun `{ $language }`.
# This message is shown as the embed description when the user sets their language successfully.
user-language-set-success-description = Um auf Englisch zurückzuschalten, verwende `{ $contextPrefix }language user_language en`.
# This message is shown as the embed title when the guild sets their language successfully.
guild-language-set-success = Sprache für diesen Server ist nun `{ $language }`.
# This message is shown as the embed description when the guild sets their language successfully.
guild-language-set-success-description = Um auf Englisch zurückzuschalten, verwende `{ $contextPrefix }language guild_language en`.
# This message is shown as the embed title when an entity tries to set their language to an unsupported language.
language-set-failure-title-unsupported = Die angegebene Sprache wird nicht vom Bot unterstützt.
# This message is shown as the embed title when an entity tries to set their language to an invalid language.
language-set-failure-title-invalid = Sprache `{ $language }` nicht gefunden.
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = Die angegebene Sprache ist eine ungültige Sprachkennzeichnung. Grund: { $error }
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = Datenbankfehler.
# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = Das ist ein Wurzelbefehl!
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = Bitte verwende nur die Unterbefehle dieses Befehls. Siehe `{ $contextPrefix }help { $commandName }` für weitere Information.
# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = Du bist weder in einem Sprachkanal, noch hast du mich einem Kanal hinzugefügt. Verwende `{ $contextPrefix }join <channel>` um mich einem Sprachkanal hinzuzufügen, oder tritt selbst einem Sprachkanal bei und wiederhole diesen Befehl.
# This message is shown on successfuly joining a voice channel.
# { $targetMention } is the mention of the channel the bot joined.
join-success =
    Erfolgreich { $voiceTargetMention } beigetreten und Transkriptionsausgabe an { $outputChannelMention } gesendet.
    { "" }
    Hinweis: Deine aktuelle Premiumstufe ist { $tier }. Damit können { $maxUsers } Benutzer auf einmal transkribiert werden. Außerdem verlässt der Bot den Kanal automatisch nach { $leaveDuration } Sekunden, unabhängig davon, wie viele Nutzer sich in dem Kanal befinden. Damit soll ein Missbrauch unseres Systems verhindert werden.
    Wenn du mehr Nutzer und eine längere Nutzungsdauer möchtest und den Bot unterstützen willst, solltest du ein Premium-Abonnement abschließen: <https://dash.scripty.org/premium>
    Wenn du weißt, dass du bereits ein Premium-Abonnent bist, schreibe dem Bot bitte eine DM, damit wir dein Premium-Abonnement wieder aktivieren können.
    { $freeTrialUpsell }
# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    WebSocket-Latenz: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    HTTP-Latenz: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Datenbanklatenz: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)

    Hinweis: eine Latenzzeit von 0ms bedeutet, dass diese Latenzzeit gerade nicht verfügbar ist.
    Versuche es später noch einmal.
voice-connection-error-ws-closed-server-not-found = Voiceserver konnte nicht gefunden werden
voice-connection-error-ws-closed-session-timeout = Zeitüberschreitung der Session
delete-data-title = Daten löschen
data-storage-toggle-audio-btn = Audiospeicherung umschalten
voice-connection-error-ws-closed-already-authenticated = Discord hat die Verbindung aufgrund von bereits bestehender Authentifizierung beendet
blocked-entity-reason-given = Folgender Grund wurde fürs Blockieren gegeben: { $reason }.
data-storage-opted-out-audio = Du hast nun deine Zustimmung widerrufen, dass deine Audiodaten gespeichert werden um das Modell zu trainieren.
data-storage-command-timed-out = Zeitüberschreitung. Führe den Befehl erneut aus, wenn du die Einstellungen weiterhin verwalten möchten.
delete-data-confirm = Ja, lösche alle Daten
blocked-entity-guild = Dieser Server wurde von der Verwendung von Scripty gesperrt. { $reason } Du kannst diese Sperre auf dem Support Server anfechten: { $supportServerInvite }.
voice-connection-error-host-io-error = Host IO Fehler
# This is shown as the title of the transcript
transcription-info-transcription-title = Transkript
data-storage-toggle-msgs-btn = Nachrichtenspeicherung umschalten
data-storage-opted-in-audio = Du hast nun Zugestimmt dass deine Audiodaten gespeichert werden um das Modell zu trainieren.
data-storage-opted-in-msgs = Du hast nun Zugestimmt dass deine Nachrichten gespeichert werden um den Scorer zu trainieren.
data-storage-opted-out-msgs = Du hast nun deine Zustimmung widerrufen, dass deine Nachrichten gespeichert werden um den Scorer zu trainieren.
blocked-entity-no-reason-given = Kein Grund wurde für das Blockieren gegeben.
voice-connection-error-timed-out = Zeitüberschreitung beim Warten auf eine Verbindung
voice-connection-error-ws-closed-no-reason = Discord hat die Verbindung ohne Angabe von Gründen beendet
voice-connection-error-ws-closed-unknown-opcode = Discord hat die Verbindung mit einem unbekannten Opcode beendet
voice-connection-error-ws-closed-invalid-payload = Discord hat die Verbindung aufgrund eines ungültigen Payloads beendet
voice-connection-error-ws-closed-not-authenticated = Discord hat die Verbindung aufgrund von fehlender Authentifizierung beendet
voice-connection-error-ws-closed-authentication-failed = Discord hat die Verbindung aufgrund eines Problems bei der Authentifizierung beendet
voice-connection-error-ws-closed-session-invalid = Die Session wurde von Discord für ungültig erklärt
delete-data-confirm-banned = Ja, lösche alle Daten und sperre mich
delete-data-cancel = Nein, abbrechen
# This is shown as the percent accuracy of the transcription (roughly)
transcription-info-transcription-confidence = Sicher
blocked-entity-user = Du wurdest davon blockiert, Scripty zu benutzen. { $reason } Du kannst diese Sperre im Support Server anfechten: { $supportServerInvite }.
voice-connection-error-internal-lib-error = Interner Fehler der Bibliothek
voice-connection-error-proto-violation = Bibliothek und Discord sind zu keiner Übereinstimmung beim Protokoll gekommen
data-storage-embed-title = Datenspeicher
voice-connection-error-ws-closed-unknown-protocol = Discord konnte das Protokoll nicht erkennen
voice-connection-error-msg-reconnect = Bei mir ist ein Problem aufgetreten ({ $reason }) und habe die Verbindung zum Sprachchat getrennt. Ich werde versuchen mich in 30 Sekunden erneut zu verbinden.
# This is shown as the number of transcriptions the algorithm has discovered.
transcription-info-transcript-count = Transkript 1 von { $count }.
voice-connection-error-ws-closed-server-crashed = Discord Sprachserver ist abgestützt
voice-connection-error-ws-closed-unknown-encryption-mode = Discord hat das Verschlüsselungsschema nicht erkannt
voice-connection-error-unknown = Verbindung wurde aus unbekanntem Grund getrennt
voice-connection-error-msg-no-reconnect = Bei mir ist ein Problem aufgetreten ({ $reason }) und habe die Verbindung zum Sprachchat getrennt.
# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.
transcription-info-transcription-ssrc = SSRC { $ssrc }
# This is shown when the algorithm encounters an error
transcription-info-transcription-error =
    Interner Fehler: Die Ausführung des stt-Algorithmus ist mit einem Fehler fehlgeschlagen: { $error }
    SSRC: { $ssrc }
    Dieser Fehler wurde protokolliert und wird so schnell wie möglich behoben.
    Wenn möglich, kontaktiere bitte die Core-Entwickler im Support-Server: { $supportServerInvite }.
    Vielen Dank!
delete-data-description =
    Dadurch werden alle deine Daten gelöscht. Diese Aktion ist dauerhaft, unumkehrbar und kann nicht rückgängig gemacht werden.

    Wenn wir "alle deine Daten" sagen, meinen wir *alle* Daten. Dazu gehören deine Sprachdaten und dein Benutzer in der Datenbank.
    Das schließt jedoch *nicht* alle Nachrichten ein, die wir von dir gespeichert haben, wenn du dich dafür entschieden hast. Wir können diese Nachrichten nicht löschen, weil wir nicht wissen, welcher Nutzer welche Nachricht geschickt hat.

    Wenn du nach dieser Aktion auch für die Nutzung des Bots gesperrt werden möchtest, damit du dich nicht versehentlich selbst liest, kannst du unten auf die entsprechende Schaltfläche klicken.
    Beachte, dass wir in diesem Fall deine Benutzer-ID speichern müssen, um eine Liste der gesperrten Benutzer zu führen.
    Wenn du zu irgendeinem Zeitpunkt nach dieser Aktion wieder gebannt werden möchtest, kannst du den Support-Server kontaktieren und um eine manuelle Aufhebung der Sperre bitten.

    Bist du dir sicher, dass du alle deine Daten löschen willst?
# automod add rule command
automod-add-rule-embed-failure-description-free-locked-type = Kostenlose Server können nur reguläre Regeln verwenden. Wenn du andere Regeltypen verwenden möchtest, schau dir unser Premium-Angebot auf https://scripty.org/premium an.
# premium command
# This is shown when the user successfully removes their premium from this guild.
premium-removed = Wenn du der Benutzer bist, der Premium in Anspruch genommen hat, hast du jetzt erfolgreich deine Premium von diesem Server entfernt. Wenn du upgraden oder mehr Slots kaufen möchtest, gehe zu <https://dash.scripty.org/premium>.
# automod setup command
automod-setup-embed-complete-title = Automod-Setup abgeschlossen!
# automod setup command
automod-setup-embed-not-setup-title = Du hast den Nutzungsbedingungen und der Datenschutzrichtlinie von Scripty noch nicht zugestimmt.
# automod remove rule command
automod-remove-rule-embed-failure-title = Regel wurde nicht entfernt!
# automod list rules command
automod-list-rules-no-rules = Du hast keine Regeln!
# general errors
general-error-user-missing-perms-description-unknown = Ich bin mir nicht sicher, welche Berechtigungen dir fehlen.
# general errors
general-error-user-missing-perms-description-not-owner = Kein Besitzer dieses Bot.
# premium command
# This is shown to the user when they have too many used servers to add more.
premium-too-many-guilds = Du hast { $totalServers } Premium-Keys beansprucht. Du kannst keine weiteren mehr hinzufügen, es sei denn, du aktualisierst dein Premium-Abonnement unter <https://dash.scripty.org/premium> oder entfernst einige mit dem Befehl`{ $commandPrefix }premium remove`.
# automod remove rule command
automod-remove-rule-embed-success-description = { $rulesLeft } Regeln, die außerhalb von { $maxRules } liegen.
# premium command
# This is shown when the user successfully claims one of their premium subscriptions.
premium-claimed = Du hast erfolgreich Premium auf diesem Server aktiviert. Wenn du upgraden oder mehr Slots kaufen möchtest, gehe zu <https://dash.scripty.org/premium>. Wenn du dein premium von diesem Server entfernen möchtest, führe `{ $commandPrefix }premium remove` aus.
# join command
# This message is shown when the user attempts to make Scripty join a voice channel, but there is no one in the channel.
join-no-one-in-channel = Es gibt niemanden in { $targetMention }. Ich trete nicht bei, wenn es dort niemanden gibt, denn das ist eine Verschwendung von begrenzten Ressourcen.
# automod add rule command
automod-add-rule-embed-success-description = { $rulesLeft } Regeln, die außerhalb von { $maxRules } liegen. { $extraDetails }
# automod add rule command
automod-add-rule-embed-failure-description-invalid-type = Ungültiger Regeltyp. Siehe `{ $contextPrefix }automod rule_help` für weitere Informationen.
# automod remove rule command
automod-remove-rule-embed-success-title = Regel entfernt!
# automod remove rule command
automod-remove-rule-embed-failure-description-invalid-id = Ungültige Regel-ID. Siehe `{ $contextPrefix }automod list` für weitere Informationen.
# automod list rules command
automod-list-rules-embed-title = Automod Regeln
# general errors
general-error-command-process-title = Bei der Verarbeitung von { $command } ist ein Fehler aufgetreten.
# general errors
general-error-invalid-args-title = Ungültige Argumente beim Parsen von { $command }.
# general errors
general-error-invalid-args-description = Das Parsen von `{ $input }` ist fehlgeschlagen, weil `{ $error }`
# general errors
general-error-command-check-failed-title = Eine Vorbedingung für { $command } ist fehlgeschlagen.
# general errors
general-error-command-check-failed-description-no-reason = kein Grund angegeben
# join command
# This message is shown when the user has told the bot to create a thread while in a thread.
join-create-thread-in-thread = Ich kann keinen Thread erstellen, während ich mich in einem Thread befinde. Bitte führe diesen Befehl in einem normalen Channel aus, wahrscheinlich { $parentChannelMention }.
# automod add rule command
automod-add-rule-embed-success-title = Regel { $ruleId } hinzugefügt!
# automod remove rule command
automod-remove-rule-embed-failure-description-not-setup = Du musst `{ $contextPrefix }automod setup` ausführen, bevor du Regeln entfernst.
# general errors
general-error-invalid-structure-title = Ungültige Struktur von Discord beim Parsen von { $command }.
# general errors
general-error-cooldown-hit-title = Cooldown getroffen bei { $command }
# general errors
# Note $time will be a decimal with two digits of accuracy.
general-error-cooldown-hit-description = { $time } Sekunden Cooldown verbleibend.
# general errors
general-error-user-missing-perms-title = Dir fehlen berechtigungen, um { $command } auszuführen.
# general errors
general-error-user-missing-perms-description-known = Berechtigungen fehlen: { $perms }
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = VC erfolgreich verlassen.
# generic strings
# Message shown if a guild has not claimed their free trial of premium. Always appears on its own standalone line in the surrounding message.
free-trial-upsell = Wir bieten eine 3-tägige Testversion von Scripty Premium an, wenn du es ausprobieren und sehen möchtest, ob es das Richtige für dich ist. Schicke dem Bot eine DM, um mit einer kostenlosen Testversion zu beginnen.
# join command
# This message is shown when Discord tosses a Dropped or TimedOut error when trying to join a voice channel.
join-failed-dropped = Discord scheint Probleme zu haben, wir können nichts dagegen tun. Bitte versuche es später noch einmal.
# join command
# If the user specifies they would like to create a thread, this is set as the thread name. { $timestamp } is the current timestamp, in ISO format.
join-thread-title = Transkription von { $timestamp }
# join command
# If the user specifies they would like to create a forum post, this is the contents of the initial message. { $timestamp } is the current timestamp, in ISO format, and { $authorMention } is the mention of the user who ran the command.
join-forum-thread-content = { $authorMention } hat eine Transkription um { $timestamp } begonnen.
# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = Der Forumskanal, den ich benutzen soll, erfordert Tags. Da ich nicht weiß, welche Tags ich verwenden soll, kann ich diesem Channel nicht beitreten. Bitte verwende einen anderen Kanal oder bitte einen Administrator, die Tag-Anforderung zu entfernen.
# automod add rule command
automod-add-rule-embed-failure-description-free-limit = Kostenlose Server sind auf 25 reguläre Regeln beschränkt. Wenn du dieses Limit erhöhen möchtest, schau dir unser Premium-Angebot auf https://scripty.org/premium an.
# automod add rule command
automod-add-rule-embed-extra-details-free-limit = Kostenlose Server sind auf 25 reguläre Regeln beschränkt. Wenn du dieses Limit erhöhen möchtest, schau dir unser Premium-Angebot auf https://scripty.org/premium an.
# automod add rule command
automod-add-rule-embed-failure-description-not-setup = Du musst `{ $contextPrefix }automod setup` ausführen, bevor du Regeln hinzufügst.
# general errors
general-error-invalid-structure-description =
    { $description }

    { "**" }Hinweis**: Dies ist ein Discord-Fehler.
    Die einzige Lösung ist, darauf zu warten, dass Discord die Slash-Befehle weitergibt, was bis zu einer Stunde dauern kann.
    Wenn du diese Stunde nicht warten willst, solltest du die Präfix-Befehle verwenden: Führe diesen Befehl mit `~{ $qualifiedName } { $args }`.
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = Du bist kein Premium-Abonnent. Abonniere unter https://scripty.org/premium. Wenn du weißt, dass du ein Premium-Abonnent bist, sende bitte eine DM an den Bot, damit wir deinen Premium wiederherstellen können.
# automod root command
automod-root-response = Dies ist der Root-Befehl, der aufgrund von Discord-Einschränkungen nichts bewirkt. Siehe `{ $contextPrefix }help automod` für weitere Informationen.
# automod setup command
automod-setup-embed-complete-description = Du kannst jetzt `{ $contextPrefix }automod rule add` verwenden, um eine Automod-Regel hinzuzufügen. { $extraDetails }
# automod setup command
automod-setup-embed-complete-free-limit = Beachte, dass die kostenlosen Server auf 25 Regeln beschränkt sind. Wenn du diese Begrenzung aufheben möchtest, schau dir unseren Premium-Server auf https://scripty.org/premium an.
# automod setup command
automod-setup-embed-not-setup-description = Dazu musst du zuerst `{ $contextPrefix } terms_of_service` ausführen.
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit-hard-cap = Du hast die absolute Höchstzahl an Regeln erreicht ({ $hardCap }). Dieses Limit soll sicherstellen, dass wir nicht zu viel Latenz in einer einzigen Nachricht hinzufügen.
# automod add rule command
automod-add-rule-embed-failure-title = Die Regel wurde nicht hinzugefügt!
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit = Premium Tier { $tier } Server sind auf { $maxRules } Regeln beschränkt. Wenn du auf Stufe { $nextTier } aufsteigst, kannst du { $nextTierMaxRules } Regeln hinzufügen.
# automod list rules command
automod-list-rules-embed-description = { $rulesLeft } Regeln, die außerhalb von { $maxRules } liegen.
# automod list rules command
automod-list-rules-embed-field-name = Regel { $ruleId }
# automod list rules command
automod-list-rules-embed-field-value =
    Typ: { $ruleType }
    Inhalt: { $ruleContent }
    Action: { $ruleAction }
# automod list rules command
automod-list-rules-footer = Seite { $page } aus { $maxPage }
# general errors
general-error-command-process-description =
    ```
    { $errorFmt }
    ```
    Dies wurde automatisch gemeldet. Bitte versuche nicht, diesen Befehl wiederholt zu verwenden.
# join command
# This message is shown when the bot does not have permissions for the voice channel it is trying to join.
join-no-permission = Ich habe nicht die Berechtigung, { $targetMention } beizutreten. Bitte gib mir die Berechtigung, den Kanal zu sehen und beizutreten, oder tritt einem anderen Voice-Chat bei, für den ich die Berechtigung habe.
# data_storage command
data-storage-embed-description =
    { "**" } HINWEIS**: Alles, was jetzt folgt, ist **ausschließlich optional**. Wenn du dich dagegen entscheidest, wird das deine Erfahrung mit Scripty in keiner Weise beeinflussen.
    Aber jetzt geht's los.

    Scripty benötigt eine Menge Audio- und Textdaten, um ein geeignetes Sprache-zu-Text-Modell zu trainieren. Nicht jeder hat die Möglichkeit, uns zu helfen, indem er spendet oder eine Prämie kauft. Deshalb kannst du uns helfen, indem du uns erlaubst, deine Daten wie Audios und Nachrichten für das Training eines Modells zu speichern.
    Da wir wissen, dass diese Daten sehr persönlich sein können, ist dies eine freiwillige Angelegenheit, die dein Erlebnis in keiner Weise beeinträchtigt.

    So würden wir damit verfahren:
    { "*" } Die gespeicherten Nachrichten würden wir in einen Scorer einspeisen, der auf deine Sprache zugeschnitten ist. Dieser Scorer würde es dem Algorithmus ermöglichen, die wahrscheinlichsten Wörter für einen bestimmten Satz von Lauten auszuwählen. Obwohl dies sehr hilfreich ist, ist es nicht so wichtig wie Audio. Beachte, dass diese Nachrichtendaten mit einer AES 256-Bit-Verschlüsselung verschlüsselt sind.
    { "*" } Bei gespeicherten Audiodaten würden wir sie und die Abschrift davon in ein Modell einspeisen, um die Genauigkeit des Sprache-zu-Text-Modells zu erhöhen. Das ist wahnsinnig hilfreich, selbst wenn du ein schlechtes Mikrofon und viele Hintergrundgeräusche hast: Je mehr Geräusche, desto besser, solange ein Mensch noch verstehen kann, was du sagst.

    Wenn du dich angemeldet hast und dich später dafür entscheidest, dich abzumelden, werden deine Daten weiterhin gespeichert, aber du kannst die Löschung deiner Sprachdaten verlangen, indem du `{ $contextPrefix }delete_all_data` ausführst. Es ist jedoch nicht möglich, deine Nachrichtendaten zu löschen. Das liegt daran, dass wir nicht speichern, welcher Nutzer welche Nachricht gesendet hat.
    Deine Daten werden auf Servern gespeichert, die streng abgeschottet sind. Es wäre für jeden, der versucht, sich Zugang zu verschaffen, äußerst schwierig, dies erfolgreich zu tun.

    Mit den Schaltflächen unten kannst du zwischen den verschiedenen Optionen wechseln.
# data_storage command
# This and all attributes show up exclusively in the slash command picker when `data_storage` is selected.
cmds_data_storage = daten_speicher
    .description = Konfiguriere deine Datenspeichereinstellungen
# Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = verlassen
    .description = Beende einen laufenden Sprachanruf.
# ping command
# This and all attributes show up exclusively in the slash command picker when `ping` is selected.
cmds_ping = ping
    .description = Zeigt die Latenz des Bots an.
# automod root command
# This and all attributes show up exclusively in the slash command picker when `automod` is selected.
cmds_automod = automod
    .description = Automod-Einstellungen konfigurieren
# automod list rules command
# This and all attributes show up exclusively in the slash command picker when `automod list rules` is selected.
cmds_list_rules = regeln_auflisten
    .description = Alle Automod-Regeln auflisten.
    .filter_by = filter_by
    .filter_by-description = Regeln nach ihrem Inhalt filtern. Leer lassen, um alle Regeln anzuzeigen.
# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = beitreten
    .description = Tritt einem Voice-Chat bei. Die Mitschriften werden in dem Kanal protokolliert, in dem du diesen Befehl ausführst.
    .voice_channel = voice_channel
    .voice_channel-description = Voice-Chat, dem du beitreten willst.
    .record_transcriptions = record_transcriptions
    .record_transcriptions-description = Alle Transkripte aufzeichnen? Benutzer werden mitgeschrieben, wenn Scripty den Kanal verlässt. Der Standardwert ist false.
    .target_channel = target_channel
    .target_channel-description = Sende die Transkripte hierher, anstatt in den aktuellen Kanal. Zielt auf ein Forum, um einen neuen Beitrag zu erstellen.
    .create_thread = create_thread
    .create_thread-description = Einen neuen Thread für diese Abschrift erstellen? Der Standardwert ist false.
# automod setup command
# This and all attributes show up exclusively in the slash command picker when `automod setup` is selected.
cmds_setup = einrichten
    .description = Beginne mit dem Automod von Scripty.
    .target_channel = target_channel
    .target_channel-description = Der Kanal, an den Automod-Logs gesendet werden sollen.
    .log_recording = log_recording
    .log_recording-description = Soll eine Aufnahme der beanstandeten Sprache an den Zielkanal gesendet werden? Der Standardwert ist false.
    .auto_join = auto_join
    .auto_join-description = Soll der Bot automatisch der Stimme beitreten, wenn ein Benutzer beitritt? Der Standardwert ist true.
# Data deletion command
# This and all attributes show up exclusively in the slash command picker when `delete_all_data` is selected.
cmds_delete_all_data = alle_daten_löschen
    .description = Alle deine Daten löschen.
# automod add rule command
# This and all attributes show up exclusively in the slash command picker when `automod add rule` is selected.
cmds_add_rule = regel_hinzufügen
    .description = Eine Automod-Regel hinzufügen.
    .rule_type = rule_type
    .rule_type-description = Der Typ der Regel, die hinzugefügt werden soll. Siehe `/automod rule_help` für weitere Informationen.
    .rule_type-choice-Regular = Regelmäßig
    .content = Inhalt
    .content-description = Der Regelinhalt, der hinzugefügt werden soll.
    .action = Aktion
    .action-description = Die Aktion, die ausgeführt werden soll, wenn die Regel ausgelöst wird.
    .action-choice-SilentDelete = Stummes Löschen
    .action-choice-DeleteAndLog = Löschen und protokollieren
    .action-choice-DeleteLogAndKick = Löschen, protokollieren und den Benutzer aus der Stimme entfernen
    .action-choice-DeleteLogAndSilence = Benutzer löschen, protokollieren und stummschalten
# automod remove rule command
# This and all attributes show up exclusively in the slash command picker when `automod remove rule` is selected.
cmds_remove_rule = regel_entfernen
    .description = Entferne eine Automod-Regel.
    .rule_id = rule_id
    .rule_id-description = Die ID der zu entfernenden Regel.
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = Hilfe
    .description = Dieses Hilfemenü anzeigen
    .command = Befehl
    .command-description = Spezifischer Befehl, zu dem Hilfe angezeigt werden soll
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = premium
    .description = Premium-Befehle
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = einfordern
    .description = Fordere deine Prämie auf dem Server ein, auf dem dies ausgeführt wird.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = entfernen
    .description = Entferne deine Prämie von dem Server, auf dem sie ausgeführt wird.
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language user_language` is selected.
cmds_user_language = Nutzer
    .description = Setze deine Benutzersprache auf eine der verfügbaren Sprachen.
    .language = Sprache
    .language-description = Die Sprache, auf die du deine Benutzersprache einstellen willst.
