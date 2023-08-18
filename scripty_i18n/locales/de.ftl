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
# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.
bot-not-set-up = Scheint, als hättest du den Bot noch nicht eingerichtet. Erledige das zunächst, mit `{ $contextPrefix }setup`.
# This message is shown on successfuly joining a voice channel.
# { $targetMention } is the mention of the channel the bot joined.
join-success =
    Erfolgreich { $voiceTargetMention } beigetreten und Transkriptionsausgabe an { $outputChannelMention } gesendet.

    Hinweis: Deine aktuelle Premiumstufe ist { $tier }. Damit können { $maxUsers } Benutzer auf einmal transkribiert werden. Außerdem verlässt der Bot den Kanal automatisch nach { $leaveDuration } Sekunden, unabhängig davon, wie viele Nutzer sich in dem Kanal befinden. Damit soll ein Missbrauch unseres Systems verhindert werden.
    Wenn du mehr Nutzer und eine längere Nutzungsdauer möchtest und den Bot unterstützen willst, solltest du ein Premium-Abonnement abschließen: <https://dash.scripty.org/premium>
    Wenn du weißt, dass du bereits ein Premium-Abonnent bist, schreibe dem Bot bitte eine DM, damit wir dein Premium-Abonnement wieder aktivieren können.
    { $freeTrialUpsell }
# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.
webhook-deleted = Du hast den Webhook gelöscht, den ich benutze! *bonk* Wiederhole `{ $contextPrefix }setup` um das Problem zu lösen.
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
# This is shown as the description of the join command (eg what shows up in the slash command picker)
join-command-description =
    Tritt einem Voice-Chat bei.

    Argument 1 ist ein Voice-Chat, dem du beitreten willst.
    Wenn du keinen Sprachkanal angibst, dem du beitreten möchtest, wählt der Bot standardmäßig denselben, in dem du dich gerade befindest.
delete-data-title = Daten löschen
data-storage-toggle-audio-btn = Audiospeicherung umschalten
voice-connection-error-ws-closed-already-authenticated = Discord hat die Verbindung aufgrund von bereits bestehender Authentifizierung beendet
# This is shown as the description of the data_storage command (eg what shows up in the slash command picker)
delete-data-command-description =
    Alle deine Daten löschen.

    Mit diesem Befehl werden alle deine Daten unwiderruflich und dauerhaft gelöscht. Diese Aktion kann nicht rückgängig gemacht werden.
blocked-entity-reason-given = Folgender Grund wurde fürs Blockieren gegeben: { $reason }.
data-storage-opted-out-audio = Du hast nun deine Zustimmung widerrufen, dass deine Audiodaten gespeichert werden um das Modell zu trainieren.
data-storage-command-timed-out = Zeitüberschreitung. Führe den Befehl erneut aus, wenn du die Einstellungen weiterhin verwalten möchten.
delete-data-confirm = Ja, lösche alle Daten
# This is shown as the description of the root language command (eg what shows up in the slash command picker)
language-root-command-description =
    Ändere deine Spracheinstellungen.

    Basisbefehl dieser Gruppe. Siehe Unterbefehle für weitere Informationen.
# This is shown as the description of the guild language command (eg what shows up in the slash command picker)
language-guild-command-description =
    Setze deine Gildensprache auf eine der verfügbaren Sprachen.

    Hinweis: Dies ändert nur die Sprache deiner Gilde, nicht die Sprache deines Benutzers. Siehe dazu `user_language`.
# Embed title for the credits command
credits-title = Anerkennungen
credits-field1-title = Hauptentwickler
blocked-entity-guild = Dieser Server wurde von der Verwendung von Scripty gesperrt. { $reason } Du kannst diese Sperre auf dem Support Server anfechten: { $supportServerInvite }.
voice-connection-error-host-io-error = Host IO Fehler
# This is shown as the description of the first argument to the user language command
language-user-argument1-description = Die Sprache die du für deinen Benutzer einstellen willst.
# This is shown as the description of the first argument to the guild language command
language-guild-argument1-description = The Sprache auf die deine Server Sprache gesetzt werden soll.
# This is shown as the title of the transcript
transcription-info-transcription-title = Transkript
# This is shown as the description of the data_storage command. (eg what shows up in the slash command picker).
data-storage-command-description = Konfiguriere deine Datenspeichereinstellungen
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
credits-description =
    Dies ist eine Liste von Leuten, die zu Scripty beigetragen haben.
    Ein großes Dankeschön geht an alle hier <3
credits-field1-description = 0/0 and valkyrie_pilot
credits-field2-title = Übersetzer
# This is shown as the description of the user language command (eg what shows up in the slash command picker)
language-user-command-description =
    Setze deine Benutzersprache auf eine der verfügbaren Sprachen.

    Hinweis: Dies ändert nur deine Benutzersprache, nicht die Sprache deiner Gilde. Siehe dazu `guild_language`.
# This is shown as the description of the credits command (eg what shows up in the slash command picker)
credits-command-description = Eine Liste an allen Dingen, die Scripty möglich machen.
# This is shown as the title of the root language command (eg what shows up in the slash command picker)
language-root-command-name = Sprache
# This is shown as the description of the ping command (eg what shows up in the slash command picker)
ping-command-description = Zeigt die Latenz des Bots an.
# This is shown as the percent accuracy of the transcription (roughly)
transcription-info-transcription-confidence = Sicher
blocked-entity-user = Du wurdest davon blockiert, Scripty zu benutzen. { $reason } Du kannst diese Sperre im Support Server anfechten: { $supportServerInvite }.
voice-connection-error-internal-lib-error = Interner Fehler der Bibliothek
voice-connection-error-proto-violation = Bibliothek und Discord sind zu keiner Übereinstimmung beim Protokoll gekommen
data-storage-embed-title = Datenspeicher
# This is shown as the description of the first argument to the join command
join-command-argument1-description = Zu bindender Sprachchat.
voice-connection-error-ws-closed-unknown-protocol = Discord konnte das Protokoll nicht erkennen
credits-field2-description = Viele Leute haben bei den Übersetzungen für den Bot geholfen. Du kannst mit dem `{ $contextPrefix }translators` eine Liste an Übersetzern sehen.
credits-field4-title = Alle unsere Nutzer
credits-field4-description = Ja, das beinhaltet dich! Wir freuen uns dass du Scripty benutzt und hoffen dass du ihn gerne benutzt.
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
# Language configuration strings
# This is shown as the title of the guild language command (eg what shows up in the slash command picker)
language-guild-command-name = Server
# automod add rule command
automod-add-rule-embed-failure-description-free-locked-type = Kostenlose Server können nur reguläre Regeln verwenden. Wenn du andere Regeltypen verwenden möchtest, schau dir unser Premium-Angebot auf https://scripty.org/premium an.
# premium command
# This is shown when the user successfully removes their premium from this guild.
premium-removed = Wenn du der Benutzer bist, der Premium in Anspruch genommen hat, hast du jetzt erfolgreich deine Premium von diesem Server entfernt. Wenn du upgraden oder mehr Slots kaufen möchtest, gehe zu <https://dash.scripty.org/premium>.
# Language configuration strings
# This is shown as the title of the user language command (eg what shows up in the slash command picker)
language-user-command-name = Nutzer
# join command
# This is shown as the title of the join command (eg what shows up in the slash command picker)
join-command-name = beitreten
# automod root command
# This is shown as the description of the automod root command (eg what shows up in the slash command picker)
automod-root-command-description = Automod-Einstellungen konfigurieren
# automod setup command
automod-setup-embed-complete-title = Automod-Setup abgeschlossen!
# automod setup command
automod-setup-embed-not-setup-title = Der Bot wurde noch nicht eingerichtet!
# automod add rule command
automod-add-rule-argument1-description = Die Art der Regel, die hinzugefügt werden soll. Siehe `/automod rule_help` für weitere Informationen.
# automod add rule command
automod-add-rule-argument3-enum-silent-delete = Stilles Löschen
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
# This is shown when the guild the user is running this command in has not finished setup.
premium-server-not-set-up = Dieser Server ist noch nicht eingerichtet worden. Das musst du zuerst mit dem Befehl `{ $commandPrefix }setup` erledigen.
# premium command
# This is shown when the user successfully claims one of their premium subscriptions.
premium-claimed = Du hast erfolgreich Premium auf diesem Server aktiviert. Wenn du upgraden oder mehr Slots kaufen möchtest, gehe zu <https://dash.scripty.org/premium>. Wenn du dein premium von diesem Server entfernen möchtest, führe `{ $commandPrefix }premium remove` aus.
# join command
# This message is shown when the user attempts to make Scripty join a voice channel, but there is no one in the channel.
join-no-one-in-channel = Es gibt niemanden in { $targetMention }. Ich trete nicht bei, wenn es dort niemanden gibt, denn das ist eine Verschwendung von begrenzten Ressourcen.
# ping command
# This is shown as the title of the ping command (eg what shows up in the slash command picker)
ping-command-name = ping
# automod setup command
# This is shown as the title of the automod setup command (eg what shows up in the slash command picker)
automod-setup-command-name = setup
# automod setup command
automod-setup-argument2-description = Soll eine Aufnahme der beleidigend Rede an den Zielkanal gesendet werden? Die Voreinstellung ist false.
# automod add rule command
automod-add-rule-embed-success-description = { $rulesLeft } Regeln, die außerhalb von { $maxRules } liegen. { $extraDetails }
# automod add rule command
automod-add-rule-embed-failure-description-invalid-type = Ungültiger Regeltyp. Siehe `{ $contextPrefix }automod rule_help` für weitere Informationen.
# automod remove rule command
automod-remove-rule-embed-success-title = Regel entfernt!
# automod remove rule command
automod-remove-rule-embed-failure-description-invalid-id = Ungültige Regel-ID. Siehe `{ $contextPrefix }automod list` für weitere Informationen.
# automod list rules command
# This is shown as the title of the automod list rules command (eg what shows up in the slash command picker)
automod-list-rules-command-name = regel_liste
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
# join command
# This message is shown when the user tries to tell the bot to join, but they have not agreed to the ToS.
must-agree-to-tos = Um Scripty nutzen zu können, musst du den Nutzungsbedingungen und der Datenschutzrichtlinie zustimmen. Siehe `{ $contextPrefix }terms_of_service` für weitere Informationen.
# automod add rule command
automod-add-rule-argument2-description = Den Regelinhalt hinzufügen.
# automod add rule command
automod-add-rule-embed-success-title = Regel { $ruleId } hinzugefügt!
# automod remove rule command
automod-remove-rule-argument1-description = Die ID der zu entfernenden Regel.
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
# Data deletion command
# This is shown as the title of the delete_all_data command (eg what shows up in the slash command picker)
delete-data-command-name = alle_daten_löschen
# Credits command
# This is shown as the title of the credits command (eg what shows up in the slash command picker)
credits-command-name = credits
# Leave command
# This is shown as the title of the leave command (eg what shows up in the slash command picker)
leave-command-name = verlassen
# Leave command
# This is shown as the description of the leave command (eg what shows up in the slash command picker)
leave-command-description = Beende einen laufenden Sprachanruf.
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
# ToS command
# This is shown as the title of the ToS command (eg what shows up in the slash command picker)
tos-command-name = Allgemeine_Geschäftsbedingungen
# ToS command
# This is shown as the description of the ToS command (eg what shows up in the slash command picker)
tos-command-description = Sieh dir die Nutzungsbedingungen und Datenschutzrichtlinien von Scripty an und stimme ihnen zu.
# ToS command
# This is sent when the user has not yet agreed to the ToS and must do so.
agreeing-to-tos = Du kannst die Nutzungsbedingungen und die Datenschutzrichtlinie von Scripty unter https://scripty.org/terms bzw. https://scripty.org/privacy einsehen. Du kannst auf die Schaltfläche unten klicken, um diesen beiden Dokumenten zuzustimmen und Scripty zu nutzen.
# ToS command
# This is sent when the user has already agreed to the ToS and does not need to do so again.
already-agreed-to-tos = Du hast den Nutzungsbedingungen und der Datenschutzrichtlinie von Scripty bereits zugestimmt. Wenn du sie noch einmal ansehen möchtest, kannst du das unter https://scripty.org/terms bzw. https://scripty.org/privacy tun.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to select a button in time.
tos-agree-timed-out = Zeitüberschreitung. Führe diesen Befehl noch einmal aus, wenn du den ToS immer noch zustimmen willst.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user agrees to the ToS.
tos-agree-success = Du hast den Nutzungsbedingungen und der Datenschutzrichtlinie von Scripty zugestimmt. Du kannst Scripty jetzt benutzen.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to agree to the ToS, usually by explicitly clicking the "No" button.
disagreed-to-tos = Du hast den Nutzungsbedingungen und der Datenschutzrichtlinie von Scripty nicht zugestimmt. Wenn du Scripty nutzen möchtest, musst du diesen Dokumenten zustimmen. Du kannst dies tun, indem du diesen Befehl erneut ausführst.
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
# automod remove rule command
# This is shown as the title of the automod remove rule command (eg what shows up in the slash command picker)
automod-remove-rule-command-name = regel_entfernen
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = Du bist kein Premium-Abonnent. Abonniere unter https://scripty.org/premium. Wenn du weißt, dass du ein Premium-Abonnent bist, sende bitte eine DM an den Bot, damit wir deinen Premium wiederherstellen können.
# data_storage command
# This is shown as the title of the data_storage command (eg what shows up in the slash command picker).
data-storage-command-name = daten_speicher
# automod root command
# This is shown as the title of the automod root command (eg what shows up in the slash command picker)
automod-root-command-name = automod
# automod root command
automod-root-response = Dies ist der Root-Befehl, der aufgrund von Discord-Einschränkungen nichts bewirkt. Siehe `{ $contextPrefix }help automod` für weitere Informationen.
# automod setup command
automod-setup-auto-join-premium-only = Auto-Join ist eine Premium-Funktion. Schau dir unser Premium auf https://scripty.org/premium an.
# automod add rule command
automod-add-rule-argument3-description = Die Aktion, die ausgeführt werden soll, wenn die Regel ausgelöst wird.
# automod add rule command
automod-add-rule-argument3-enum-delete-and-log = Löschen und Loggen
# automod add rule command
automod-add-rule-argument3-enum-delete-log-and-kick = Löschen, Loggen, und Benutzer aus der Stimme entfernen
# automod add rule command
# This is shown as the title of the automod add rule command (eg what shows up in the slash command picker)
automod-add-rule-command-name = regel_hinzufügen
# automod setup command
# This is shown as the description of the automod setup command (eg what shows up in the slash command picker)
automod-setup-command-description = Starte mit dem Automod von Scripty.
# automod setup command
automod-setup-argument1-description = Der Kanal, an den Automod-Protokolle gesendet werden sollen.
# automod add rule command
# This is shown as the description of the automod add rule command (eg what shows up in the slash command picker)
automod-add-rule-command-description = Füge eine Automod-Regel hinzu.
# automod add rule command
automod-add-rule-argument1-enum-regular-type = Regulär
# automod setup command
automod-setup-argument3-description = Soll der Bot automatisch der Stimme beitreten, wenn ein Benutzer beitritt? Der Standardwert ist true.
# automod setup command
automod-setup-embed-complete-description = Du kannst jetzt `{ $contextPrefix }automod rule add` verwenden, um eine Automod-Regel hinzuzufügen. { $extraDetails }
# automod setup command
automod-setup-embed-complete-free-limit = Beachte, dass die kostenlosen Server auf 25 Regeln beschränkt sind. Wenn du diese Begrenzung aufheben möchtest, schau dir unseren Premium-Server auf https://scripty.org/premium an.
# automod setup command
automod-setup-embed-not-setup-description = Richte ihn zuerst ein, indem du `{ $contextPrefix } setup` ausführst.
# automod remove rule command
# This is shown as the description of the automod remove rule command (eg what shows up in the slash command picker)
automod-remove-rule-command-description = Entferne eine Automod-Regel.
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit-hard-cap = Du hast die absolute Höchstzahl an Regeln erreicht ({ $hardCap }). Dieses Limit soll sicherstellen, dass wir nicht zu viel Latenz in einer einzigen Nachricht hinzufügen.
# automod add rule command
automod-add-rule-embed-failure-title = Die Regel wurde nicht hinzugefügt!
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit = Premium Tier { $tier } Server sind auf { $maxRules } Regeln beschränkt. Wenn du auf Stufe { $nextTier } aufsteigst, kannst du { $nextTierMaxRules } Regeln hinzufügen.
# automod list rules command
# This is shown as the description of the automod list rules command (eg what shows up in the slash command picker)
automod-list-rules-command-description = Liste alle Automod-Regeln auf.
# automod list rules command
automod-list-rules-argument1-description = Filtere Regeln nach ihrem Inhalt. Leer lassen, um alle Regeln anzuzeigen.
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
