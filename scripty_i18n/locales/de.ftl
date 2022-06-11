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
donation-title = Spenden
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
join-success = Erfolgreich { $targetMention } beigetreten.
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
setup-arg3-description = Ob der Bot ausführlich sein sollte.
setup-tos-agree =
    Durch das Einrichten von Scripty stimmst du der Datenschutzerklärung und den Allgemeinen Geschäftsbedingungen zu.
    Datenschutzerklärung: https://scripty.org/privacy
    Allgemeine Geschäftsbedingungen: https://scripty.org/terms
setup-tos-agree-failure = Du musst den Allgemeinen Geschäftsbedingungen und der Datenschutzerklärung zustimmen, um Scripty zu benutzen. Einrichtung abgebrochen.
setup-success-title = Einrichtung erfolgreich!
