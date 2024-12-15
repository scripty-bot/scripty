# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = entra
    .description = Entra nel canale vocale. Le trascrizioni vengono inviate nel canale in cui invii questo comando.
    .voice_channel = canale_vocale
    .voice_channel-description = Canale vocale da collegare.
    .record_transcriptions = registra_trascrizioni
    .record_transcriptions-description = Salvare le trascrizioni? Gli utenti saranno notificati nei Messaggi Diretti quando Scripty lascia il canale. Default su falso.
    .target_channel = canale_destinato
    .target_channel-description = Manda le trascrizioni qui, al posto del canale corrente. Scegli come target un forum per creare un nuovo post.
    .create_thread = crea_thread
    .create_thread-description = Creare un nuovo thread per questa trascrizione? Defaults su falso.
join-success-description = Unito con successo { $voiceTargetMention }, e inviando l'output della trascrizione a { $outputChannelMention }.
join-success-premium = Puoi controllare il Premium status di questo server con `/premium info`.
join-success-help-title = Hai bisogno di aiuto?
join-success-help-description = Puoi sia entrare nel server del supporto a { $supportServerInvite }, o inviare un messaggio privato al bot.
join-success-footer-free-trial-upsell = Questo server è idoneo per una prova gratuita del Premium. Contatta nei messaggi privati il bot per richiederne una.
