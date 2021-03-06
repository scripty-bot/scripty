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
language-set-failure-description-unsupported = Wenn du mithelfen m??chtest, den Bot in diese Sprache zu ??bersetzen, trete dem Support-Server bei: { $supportServerInvite }.
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = Es gab einen Fehler in der Datenbank beim Speichern deiner Sprache. Der Fehler ist gemeldet und wir werden ihn uns anschauen. Bitte spamme diesen Befehl nicht. (F??r die Neugierigen ist hier der Fehler: { $error })
donation-title = Spenden
command-not-found = Kein Befehl mit Name `{ $commandName }` gefunden.
command-not-found-suggestions = Meintest du `{ $suggestion }`?
no-help-found = Keine Hilfe gefunden f??r Befehl `{ $commandName }`.
default-category-name = Befehle
context-menu-command-title =
    { "" }
    Kontextmen??befehle:
    { "" }
context-menu-command-user =
    { "" }
    { $commandName } (f??r Nutzer)
    { "" }
context-menu-command-message =
    { "" }
    { $commandName } (f??r Nachrichten)
    { "" }
more-info-on-command =
    F??r weitere Information ??ber einen Befehl, verwende `{ $contextPrefix }help <name>`
    ```
# This message is shown as the embed title when the user sets their language successfully.
user-language-set-success = Benutzersprache ist nun `{ $language }`.
# This message is shown as the embed description when the user sets their language successfully.
user-language-set-success-description = Um auf Englisch zur??ckzuschalten, verwende `{ $contextPrefix }language user_language en`.
# This message is shown as the embed title when the guild sets their language successfully.
guild-language-set-success = Sprache f??r diesen Server ist nun `{ $language }`.
# This message is shown as the embed description when the guild sets their language successfully.
guild-language-set-success-description = Um auf Englisch zur??ckzuschalten, verwende `{ $contextPrefix }language guild_language en`.
# This message is shown as the embed title when an entity tries to set their language to an unsupported language.
language-set-failure-title-unsupported = Die angegebene Sprache wird nicht vom Bot unterst??tzt.
# This message is shown as the embed title when an entity tries to set their language to an invalid language.
language-set-failure-title-invalid = Sprache `{ $language }` nicht gefunden.
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = Die angegebene Sprache ist eine ung??ltige Sprachkennzeichnung. Grund: { $error }
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = Datenbankfehler.
# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = Das ist ein Wurzelbefehl!
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = Bitte verwende nur die Unterbefehle dieses Befehls. Siehe `{ $contextPrefix }help { $commandName }` f??r weitere Information.
# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = Du bist weder in einem Sprachkanal, noch hast du mich einem Kanal hinzugef??gt. Verwende `{ $contextPrefix }join <channel>` um mich einem Sprachkanal hinzuzuf??gen, oder tritt selbst einem Sprachkanal bei und wiederhole diesen Befehl.
# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.
bot-not-set-up = Scheint, als h??ttest du den Bot noch nicht eingerichtet. Erledige das zun??chst, mit `{ $contextPrefix }setup`.
# This message is shown on successfuly joining a voice channel.
# { $targetMention } is the mention of the channel the bot joined.
join-success = Erfolgreich { $targetMention } beigetreten.
# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.
webhook-deleted = Du hast den Webhook gel??scht, den ich benutze! *bonk* Wiederhole `{ $contextPrefix }setup` um das Problem zu l??sen.
# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    WebSocket-Latenz: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    HTTP-Latenz: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Datenbanklatenz: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)
    
    Hinweis: eine Latenzzeit von 0ms bedeutet, dass diese Latenzzeit gerade nicht verf??gbar ist.
    Versuche es sp??ter noch einmal.
setup-tos-agree =
    Durch das Einrichten von Scripty stimmst du der Datenschutzerkl??rung und den Allgemeinen Gesch??ftsbedingungen zu.
    Datenschutzerkl??rung: https://scripty.org/privacy
    Allgemeine Gesch??ftsbedingungen: https://scripty.org/terms
setup-tos-agree-failure = Du musst den Allgemeinen Gesch??ftsbedingungen und der Datenschutzerkl??rung zustimmen, um Scripty zu benutzen. Einrichtung abgebrochen.
setup-success-title = Einrichtung erfolgreich!
voice-connection-error-ws-closed-server-not-found = Voiceserver konnte nicht gefunden werden
voice-connection-error-ws-closed-session-timeout = Zeit??berschreitung der Session
# This is shown as the title of the donate command (eg what shows up in the slash command picker)
donations-command-name = spenden
# This is shown as the description of the donate command (eg what shows up in the slash command picker)
donations-command-description = Erhalte Informationen, wie du Scripty mit Spenden unterst??tzen kannst.
# This is shown as the description of the join command (eg what shows up in the slash command picker)
join-command-description = Einem Sprachchat beitreten.
delete-data-title = Daten l??schen
data-storage-toggle-audio-btn = Audiospeicherung umschalten
voice-connection-error-ws-closed-already-authenticated = Discord hat die Verbindung aufgrund von bereits bestehender Authentifizierung beendet
# This is shown as the description of the data_storage command (eg what shows up in the slash command picker)
delete-data-command-description = L??sche alle deine Daten.
blocked-entity-reason-given = Folgender Grund wurde f??rs Blockieren gegeben: { $reason }.
data-storage-opted-out-audio = Du hast nun deine Zustimmung widerrufen, dass deine Audiodaten gespeichert werden um das Modell zu trainieren.
data-storage-command-timed-out = Zeit??berschreitung. F??hre den Befehl erneut aus, wenn du die Einstellungen weiterhin verwalten m??chten.
delete-data-confirm = Ja, l??sche alle Daten
# This is shown as the description of the root language command (eg what shows up in the slash command picker)
language-root-command-description = Modifiziere deine Spracheinstellungen.
# This is shown as the description of the guild language command (eg what shows up in the slash command picker)
language-guild-command-description = Setze deine Server Sprache auf eine der verf??gbaren Sprachen.
# This is shown as the description of the setup command (eg what shows up in the slash command picker)
setup-command-description = Setze den Bot auf.
# Embed title for the credits command
credits-title = Anerkennungen
credits-field1-title = Hauptentwickler
blocked-entity-guild = Dieser Server wurde von der Verwendung von Scripty gesperrt. Du kannst diese Sperre auf dem Support Server anfechten: { $supportServerInvite }.
voice-connection-error-host-io-error = Host IO Fehler
# This is shown as the description of the first argument to the user language command
language-user-argument1-description = Die Sprache die du f??r deinen Benutzer einstellen willst.
# This is shown as the description of the first argument to the guild language command
language-guild-argument1-description = The Sprache auf die deine Server Sprache gesetzt werden soll.
# This is shown as the title of the setup command (eg what shows up in the slash command picker)
setup-command-name = Einrichtung
setup-command-argument1-description = Channel, in den Transkriptionen gesendet werden sollen. (Erforderlich)
setup-command-argument2-description = Zielsprache des STT Algorithmus. (Optional, Standardsprache ist Englisch)
# This is shown as the title of the transcript
transcription-info-transcription-title = Transkript
setup-command-argument3-description = Bei Transkriptionen wortreich sein? Dies f??gt keinen zus??tzlichen Overhead hinzu. (Optional, Standardwert: false)
# This is shown as the description of the data_storage command. (eg what shows up in the slash command picker).
data-storage-command-description = Konfiguriere deine Datenspeichereinstellungen
data-storage-toggle-msgs-btn = Nachrichtenspeicherung umschalten
data-storage-opted-in-audio = Du hast nun Zugestimmt dass deine Audiodaten gespeichert werden um das Modell zu trainieren.
data-storage-opted-in-msgs = Du hast nun Zugestimmt dass deine Nachrichten gespeichert werden um den Scorer zu trainieren.
data-storage-opted-out-msgs = Du hast nun deine Zustimmung widerrufen, dass deine Nachrichten gespeichert werden um den Scorer zu trainieren.
blocked-entity-no-reason-given = Kein Grund wurde f??r das Blockieren gegeben.
voice-connection-error-timed-out = Zeit??berschreitung beim Warten auf eine Verbindung
voice-connection-error-ws-closed-no-reason = Discord hat die Verbindung ohne Angabe von Gr??nden beendet
voice-connection-error-ws-closed-unknown-opcode = Discord hat die Verbindung mit einem unbekannten Opcode beendet
voice-connection-error-ws-closed-invalid-payload = Discord hat die Verbindung aufgrund eines ung??ltigen Payloads beendet
voice-connection-error-ws-closed-not-authenticated = Discord hat die Verbindung aufgrund von fehlender Authentifizierung beendet
voice-connection-error-ws-closed-authentication-failed = Discord hat die Verbindung aufgrund eines Problems bei der Authentifizierung beendet
voice-connection-error-ws-closed-session-invalid = Die Session wurde von Discord f??r ung??ltig erkl??rt
delete-data-confirm-banned = Ja, l??sche alle Daten und sperre mich
delete-data-cancel = Nein, abbrechen
credits-description = Das ist eine Liste an Leuten, die bei Scripty geholfen haben.
credits-field1-description = 0/0 and valkyrie_pilot
credits-field2-title = ??bersetzer
credits-field3-title = Hosting Anbieter
# This is shown as the description of the user language command (eg what shows up in the slash command picker)
language-user-command-description = Setze deine Sprache auf eine der verf??gbaren Sprachen.
# This is shown as the description of the credits command (eg what shows up in the slash command picker)
credits-command-description = Eine Liste an allen Dingen, die Scripty m??glich machen.
# This is shown as the title of the root language command (eg what shows up in the slash command picker)
language-root-command-name = Sprache
# This is shown as the description of the ping command (eg what shows up in the slash command picker)
ping-command-description = Zeigt die Latenz des Bots an.
# This is shown as the percent accuracy of the transcription (roughly)
transcription-info-transcription-confidence = Sicher
blocked-entity-user = Du wurdest davon blockiert, Scripty zu benutzen. Du kannst diese Sperre im Support Server anfechten: { $supportServerInvite }.
voice-connection-error-internal-lib-error = Bibliothekeninterner Fehler
voice-connection-error-proto-violation = Bibliothek und Discord sind zu keiner ??bereinstimmung beim Protokoll gekommen
data-storage-embed-title = Datenspeicher
# This is shown as the description of the first argument to the join command
join-command-argument1-description = Zu bindender Sprachchat.
voice-connection-error-ws-closed-unknown-protocol = Discord konnte das Protokoll nicht erkennen
credits-field2-description = Viele Leute haben bei den ??bersetzungen f??r den Bot geholfen. Du kannst mit dem `{ $contextPrefix }translators` eine Liste an ??bersetzern sehen.
credits-field3-description = Droplet Development hat Scripty gro??z??gigerweise einen leistungsstarken Server zur Verf??gung gestellt, mit genug Leistung um auf ca. 2.500 Discord Server zu skalieren. Du kannst ihre Seite hier besuchen: https://droplet.gg/
credits-field4-title = Alle unsere Nutzer
credits-field4-description = Ja, das beinhaltet dich! Wir freuen uns dass du Scripty benutzt und hoffen dass du ihn gerne benutzt.
voice-connection-error-msg-reconnect = Bei mir ist ein Problem aufgetreten ({ $reason }) und habe die Verbindung zum Sprachchat getrennt. Ich werde versuchen mich in 30 Sekunden erneut zu verbinden.
# This is shown as the number of transcriptions the algorithm has discovered.
transcription-info-transcript-count = Transkript 1 von { $count }.
voice-connection-error-ws-closed-server-crashed = Discord Sprachserver ist abgest??tzt
voice-connection-error-ws-closed-unknown-encryption-mode = Discord hat das Verschl??sselungsschema nicht erkannt
voice-connection-error-unknown = Verbindung wurde aus unbekanntem Grund getrennt
voice-connection-error-msg-no-reconnect = Bei mir ist ein Problem aufgetreten ({ $reason }) und habe die Verbindung zum Sprachchat getrennt.
# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.
transcription-info-transcription-ssrc = SSRC { $ssrc }
# This is shown when the algorithm encounters an error
transcription-info-transcription-error = Interner Fehler: Ausf??hren des STT Algorithmus ist fehlgeschlagen mit dem Fehler: { $error }
delete-data-description = Dies wird alle deine Daten l??schen. Diese Aktion ist permanent, irreversibel und kann nicht r??ckg??ngig gemacht werden.
donation-description = Es ist nicht sehr g??nstig Scripty zu betreiben. Scripty l??uft momentan auf einem 1.500$ Server mit einer AMD Ryzen 9 3900 CPU und 128 GB RAM. Selbst mit dieser Hardware, sch??tzen wir, dass Scripty h??chstens etwa 100 Zeitgleiche Transkriptionen durchf??hren kann. Spenden w??rden uns helfen unsere Hardware Kapazit??t zu erweitern um viel mehr Transkriptionen zeitgleich durchzuf??hren, vielleicht sogar Zehntausende eines Tages, mit genug Spenden.
