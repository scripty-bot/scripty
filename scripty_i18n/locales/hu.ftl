# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to select a button in time.
tos-agree-timed-out = Időtúllépés. Futtasd még egyszer ezt a parancsot, ha mégis el szeretnéd fogadni a nyilatkozatot.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user agrees to the ToS.
tos-agree-success = Sikeresen elfogadtad a Scripty szolgáltatási feltételeit és adatvédelmi nyilatkozatát. Mostmár használhatod a Scripty-t.
# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = A fórumcsatorna, amit megkértél, hogy használjak, kötelezi címkék használatát. Nem tudhatom, hogy mely címkéket használjam, így nem tudok csatlakozni a csatornához. Kérlek, használj egy másik csatornát, vagy kérj meg egy rendszergazdát, hogy távolítsa el a címkék használatának követelményét.
# ToS command
# This is sent when the user has not yet agreed to the ToS and must do so.
agreeing-to-tos = A Scripty szolgáltatási feltételeit és adatvédelmi nyilatkozatát a https://scripty.org/terms, illetve https://scripty.org/privacy címen tekintheted meg. Az alábbi gombot megnyomva elfogadhatod mindkettőt, és használhatod a Scripty-t.
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = Sikeresen kiléptem a hangcsatornából.
# ToS command
# This and all attributes show up exclusively in the slash command picker when `terms_of_service` is selected.
cmds_terms_of_service = szolgáltatási_feltételek
    .description = Tekintsd meg és fogadd el a Scripty Szolgáltatási feltételeit és adatvédelmi szabályzatát.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to agree to the ToS, usually by explicitly clicking the "No" button.
disagreed-to-tos = Elutasítottad a Scripty szolgáltatási feltételeit és adatvédelmi nyilatkozatát. Ha szeretnéd használni a Scripty-t, akkor el kell fogadnod ezeket a dokumentumokat. Megteheted a parancs ismételt futtatásával.
# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = csatlakozás
    .description = Csatlakozás hangcsatornához. Az átiratok a jelenlegi csatornába lesznek naplózva.
    .voice_channel = hangcsatorna
    .voice_channel-description = Voice chat to bind to.
    .record_transcriptions = record_transcriptions
    .record_transcriptions-description = Összes átirat naplózása. A résztvevők üzenetet fognak kapni Scripty kilépésekor. Alapért.: hamis.
    .target_channel = célcsatorna
    .target_channel-description = Átiratok máshova küldése. Ha fórumcsatornát jelölsz ki, akkor poszt lesz létrehozva.
    .create_thread = gondolatmenet_létrehozása
    .create_thread-description = Létre legyen hozva egy új gondolatmenet az átirathoz? Alapértelmezetten hamis.
# Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = kilépés
    .description = Kilépés bármilyen jelenlegi hanghívásból.
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = segítség
    .description = Megjeleníti e segítség menüt
    .command = parancs
    .command-description = Adott parancs segítségének megjelenítése
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = törlés
    .description = Prémiumod eltávolítása azon a szerveren, ahol e parancsot futtatod.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = igénylés
    .description = Prémiumod igénylése azon a szerveren, ahol e parancsot futtatod.
# join command
# If the user specifies they would like to create a forum post, this is the contents of the initial message. { $timestamp } is the current timestamp, in ISO format, and { $authorMention } is the mention of the user who ran the command.
join-forum-thread-content = { $authorMention } elindított egy átiratot ekkor: { $timestamp }.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = prémium
    .description = Prémium parancsok
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = Nem vagy prémium előfizető. Fizess elő a https://scripty.org/premium oldalon. Ha tudod, hogy előfizető vagy, küldj egy privát üzenetet a botnak, hogy visszaállíthassuk a prémiumod.
# Help menu translation strings
command-not-found-suggestions = Erre gondoltál: `{ $suggestion }`?
# Context menu command translation strings
context-menu-command-user =
    { "" }
    { $commandName } (felhasználón)
    { "" }
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language` is selected.
cmds_language = nyelvezet
    .description = Módosítsd nyelvi beállításaid.
# Language configuration strings
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = Adatbázis hiba.
# Help menu translation strings
no-help-found = Nem található segítség a(z) `{ $commandName }` parancshoz.
# Context menu command translation strings
context-menu-command-message =
    { "" }
    { $commandName } (üzeneten)
    { "" }
# Help menu translation strings
default-category-name = Parancsok
# Help menu translation strings
command-not-found = Nem található `{ $commandName }` nevű parancs.
