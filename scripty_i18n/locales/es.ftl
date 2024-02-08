# This message is shown as the embed description when the guild sets their language successfully.


# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.


# This message is shown on successfuly joining a voice channel.
# {$targetMention} is the mention of the channel the bot joined.


# This message is shown as the embed description when an entity tries to set their language to an invalid language.


# This message is shown as the embed title when an entity tries to set their language to an unsupported language.


# This message is shown as the embed title when an entity tries to set their language to an invalid language.


# This message is shown as the embed description when an entity tries to set their language to an unsupported language.


# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.


# This message is shown when the user is not in a voice channel, nor was a voice channel specified.


# This message is shown as the embed title when the database returns an error when setting the language for an entity.


# This message is shown as the embed title when the user sets their language successfully.


# This message is shown as the embed description when the user sets their language successfully.


# This message is shown as the embed title when the guild sets their language successfully.


# This message is shown as the embed description when the database returns an error when setting the language for an entity.


# This message is shown as the embed description when a user tries to invoke the root command of a group.


# This message is shown as the embed title when a user tries to invoke the root command of a group.


# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.


# This is shown as the number of transcriptions the algorithm has discovered.


# This is shown as the title of the transcript


# This is shown as the percent accuracy of the transcription (roughly)


# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.


# This is shown when the algorithm encounters an error

# join command
# This message is shown when the user requests the bot create a new thread in a channel, but the channel doesn't support threads being created (usually voice channels)
join-create-thread-in-unsupported = Discord no admite hilos en { $targetMention }. Utilice un canal diferente o no cree un hilo.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = premium
    .description = Comandos premium
# Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = dejar
    .description = Dejar cualquier llamada de voz actual.
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = ayuda
    .description = Mostrar este menú de ayuda
    .command = comando
    .command-description = Comando específico para mostrar ayuda sobre
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = reclamar
    .description = Reclama tu prima dentro del servidor donde se ejecuta este comando.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = eliminar
    .description = Elimina tu premium del servidor donde se ejecuta este comando.
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = No eres un suscriptor premium. Suscríbase en https://scripty.org/premium. Si sabe que es uno de ellos, envíe un mensaje directo al bot para que podamos restablecer su prima.
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = Canal de voz izquierdo con éxito.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to select a button in time.
tos-agree-timed-out = Caducado. Vuelva a ejecutar este comando si aún desea aceptar los términos de servicio.
# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = El canal del foro que intentaste hacerme usar requiere etiquetas. No tengo forma de saber qué etiquetas usar, por lo que no puedo unirme a ese canal. Utilice un canal diferente o pídale a un administrador que elimine el requisito de etiqueta.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user agrees to the ToS.
tos-agree-success = Ha aceptado exitosamente los términos de servicio y la política de privacidad de Scripty. Ahora puedes usar Scripty.
# ToS command
# This replaces the original content of the message (key agreeing-to-tos) when the user fails to agree to the ToS, usually by explicitly clicking the "No" button.
disagreed-to-tos = No has estado de acuerdo con los términos de servicio y la política de privacidad de Scripty. Si desea utilizar Scripty, debe aceptar estos documentos. Puede hacerlo ejecutando este comando nuevamente.
# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = join
    .description = Únase a un chat de voz. Las transcripciones se registrarán en el canal en el que ejecute este comando.
    .voice_channel = canel_de_voz
    .voice_channel-description = Chat de voz al que vincularse
    .record_transcriptions = registrar_transcripciones
    .record_transcriptions-description = ¿Registrar todas las transcripciones? Los usuarios recibirán un mensaje directo cuando Scripty abandone el canal. El valor predeterminado es falso.
    .target_channel = canal_objetivo
    .target_channel-description = Envíe transcripciones aquí, en lugar del canal actual. Apuntar a un foro para crear una nueva publicación.
    .create_thread = crear_hilo
    .create_thread-description = ¿Crear un nuevo hilo para esta transcripción? El valor predeterminado es falso.
# join command
# This message is shown when the user has told the bot to send transcripts to a non-text-based channel (ie category). `target_channel` should be translated, as slash command arguments are localized.
join-target-not-text-based = El canal al que me dijo que enviara transcripciones ({ $targetMention }) no es un canal basado en texto. Utilice un canal basado en texto o elija un canal diferente en el argumento `canal_objetivo`.
# ToS command
# This and all attributes show up exclusively in the slash command picker when `terms_of_service` is selected.
cmds_terms_of_service = términos_de_servicio
    .description = Vea y acepte los Términos de servicio y la Política de privacidad de Scripty.
# ToS command
# This is sent when the user has not yet agreed to the ToS and must do so.
agreeing-to-tos = Podrá acceder a Los Términos de servicio y Política de Privacidad aquí https://scripty.org/terms y https://scripty.org/privacy respectivamente. Debe hacer clic en el botón de abajo para aceptar ambos documentos y usar Scripty.
