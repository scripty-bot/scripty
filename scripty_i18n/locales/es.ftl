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
cmds_leave = leave
    .description = Abandonar cualquier llamada de voz en curso.
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = help
    .description = Mostrar este menú de ayuda
    .command = comando
    .command-description = Comando específico para mostrar ayuda
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = claim
    .description = Reclamar tu premium dentro del servidor donde se ejecuta este comando.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = remove
    .description = Eliminar tu premium del servidor donde se ejecuta este comando.
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = No eres un suscriptor premium. Suscríbete en https://scripty.org/premium. Si sabes que eres uno de ellos, envía un mensaje directo al bot de manera que podamos restablecer su premium.
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = Canal de voz abandonado con éxito.
# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = El canal del foro que intentaste que usara requiere etiquetas. No tengo forma de saber qué etiquetas usar, por lo que no puedo unirme a ese canal. Utiliza un canal diferente, o pídele a un administrador que elimine el requisito de etiqueta.
# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = join
    .description = Unirse a un chat de voz. Las transcripciones se registrarán en el canal en el que ejecute este comando.
    .voice_channel = canal_de_voz
    .voice_channel-description = Chat de voz al que vincularse.
    .record_transcriptions = registrar_transcripciones
    .record_transcriptions-description = ¿Registrar todas las transcripciones? Los usuarios recibirán un mensaje directo cuando Scripty abandone el canal. El valor predeterminado es falso.
    .target_channel = canal_objetivo
    .target_channel-description = Envíe transcripciones aquí, en lugar del canal actual. Apuntar a un foro para crear una nueva publicación.
    .create_thread = crear_hilo
    .create_thread-description = ¿Crear un nuevo hilo para esta transcripción? El valor predeterminado es falso.
# join command
# This message is shown when the user has told the bot to send transcripts to a non-text-based channel (ie category). `target_channel` should be translated, as slash command arguments are localized.
join-target-not-text-based = El canal al que me dijiste que enviara las transcripciones a ({ $targetMention }) no es un canal de texto. Por favor, utiliza un canal de texto o elige un canal diferente en el argumento `target_channel`.
premium-removed = Si tú eres el usuario que ha reclamado Premium, has eliminado tu premium de este servido con éxito. Si quieres actualizarlo, u obtener más espacio, dirígete <https://dash.scripty.org/premium>.
cmds_config_verbose = verbose
    .description = Cambiar si Scripty es locuaz durante las transcripciones.
    .verbose = locuaz
    .verbose-description = El valor predeterminado es falso
config-verbose-disabled = Scripty ya no será locuaz durante las transcripciones.
config_translate = translate
    .description = ¿Traducir transcripciones automáticamente a inglés?
    .translate = traducir
    .translate-description = El valor predeterminado es falso
no-help-found = No se ha encontrado ayuda para el comando `{ $commandName }`.
language-set-failure-title-unsupported = El idioma que has indicado no es compatible con el bot.
language-set-failure-description-db = Se produjo un error en la base de datos al intentar establecer tu idioma. Se ha informado de este error y lo revisaremos. Por favor, no envíe spam con este comando. (Si tienes curiosidad, este es el error: { $error })
language-set-failure-title-invalid = Idioma `{ $language }` no encontrado.
join-success-description = Unido a { $voiceTargetMention } con éxito, y enviando el resultado de la transcripción a { $outputChannelMention }.
join-create-thread-in-thread = No puedo crear un hilo mientras estoy en un hilo. Por favor, ejecuta este comando en un canal normal, probablemente { $parentChannelMention }.
premium-info-embed-current-tier = Nivel actual
premium-info-embed-manage-subscription-user-has-unclaimed-description = Para reclamarlo en este servidor, ejecuta { $claimCommand }.
config-transcribe-audio-enabled = Ahora Scripty transcribirá archivos de audio.
command-not-found-suggestions = ¿Querías decir `{ $suggestion }`?
data-storage-opted-out-audio = Ahora ya no puedes almacenar tu audio para el entrenamiento del modelo.
data-storage-opted-in-msgs = Ahora has optado por almacenar tus mensajes para el entrenamiento de evaluación.
config-transcribe-voice-messages-enabled = Ahora Scripty transcribirá mensajes de voz.
config-transcribe-voice-messages-disabled = Scripty ya no transcribirá mensajes de voz.
config-transcribe-video-requires-premium =
    Transcribir archivos de video es una función premium de nivel 2, ya que transcodificar archivos de video es costoso computacionalmente.
    Si quieres mejorar a nivel 2 de Premium, dirígete a https://dash.scripty.org/premium.
    Si esta función estaba activada antes, ahora está desactivada.
user-language-set-success-description = Para volver a inglés, escribe `{ $contextPrefix }language user_language en`.
language-set-partially-translated-help = ¿Quieres ayudar a traducir Scripty a tu idioma? Consulta el proyecto de traducción en https://hosted.weblate.org/engage/scripty-bot/.
language-set-failure-description-unsupported = Si quieres ayudar a añadir soporte a este idioma, por favor únete al servidor de soporte en { $supportServerInvite }.
language-set-failure-description-invalid = El idioma que has indicado no es un identificador de idioma válido. Reason: { $error }
guild-language-set-failure-translate-enabled = Tu servidor tiene activada la traducción automática. Esto sólo es compatible cuando se traduce al inglés. Desactiva esta función si quieres establecer tu idioma.
root-command-invoked-description = Por favor, ejecuta este subcomando del comando sólo para usarlo. Para más información, echa un vistazo a `{ $contextPrefix }help { $commandName }`.
data-storage-embed-title = Almacenamiento de datos
automod-setup-embed-complete-free-limit = Ten en cuenta que para los servidores gratuitos está limitado a 25 reglas. Si quieres eliminar este límite, echa un vistazo a nuestro Premium en https://scripty.org/premium.
config-transcribe-audio-disabled = Scripty ya no transcribirá archivos de audio.
config-transcribe-video-enabled = Ahora Scripty transcribirá archivos de video.
join-success-premium = Puedes consultar el estado Premium del servidor con `/premium info`.
join-success-help-title = ¿Necesitas ayuda?
join-success-footer-free-trial-upsell = Este servidor es apto para una prueba gratuita de Premium. Envía un mensaje directo al bot para solicitar una.
join-no-one-in-channel = No hay nadie en { $targetMention }. No me uno si no hay nadie allí, ya que es un desperdicio de recursos limitados.
join-success-help-description = Puedes unirte al servidor de soporte en { $supportServerInvite }, o puedes enviar un mensaje directo al bot.
join-thread-title = Transcripción desde { $timestamp }
join-forum-thread-content = { $authorMention } inició una transcripción a las { $timestamp }.
cmds_transcribe_message = transcribe_message
    .description = Transcribir un mensaje. Responde a un mensaje para transcribirlo.
cmds_premium_info = info
    .description = Conseguir información sobre el estado Premium de Scripty de este servidor.
premium-info-embed-title = Estado Premium
premium-info-embed-description-has-subscription = Puedes gestionar tu suscripción en <https://dash.scripty.org/premium>. ¡Gracias por apoyar a Scripty!
premium-info-embed-max-users = Máximo de usuarios simultáneos
premium-info-embed-max-duration = Duración (en segundos) máxima de sesión
premium-info-embed-trial-available-title = ¿Quieres una prueba gratuita de Premium?
premium-info-embed-trial-available-description = Envía un mensaje directo al bot para comenzar a configurar una prueba gratuita de Premium de 3 días.
premium-info-embed-manage-subscription-user-has-unclaimed-title = ¡Parece que compraste Premium!
config-verbose-enabled = Ahora Scripty será locuaz durante las transcripciones.
premium-info-embed-description-no-subscription = Puedes suscribirte a Premium en <https://dash.scripty.org/premium>. Además de las ventajas que obtienes, también puedes ayudarnos en nuestro objetivo de hacer Scripty el mejor bot de conversión de voz a texto que existe :)
config_transcribe_audio = transcribe_audio
    .description = Cambiar si Scripty transcribe archivos de audio arbitrarios. Premium obligatorio.
    .transcribe_audio = transcribir_audio
    .transcribe_audio-description = El valor predeterminado es falso
config-transcribe-video-disabled = Scripty ya no transcribirá archivos de video.
config_auto_detect_lang = auto_detect_lang
    .description = ¿Detectar automáticamente el idioma hablado? Muy impreciso comparado a establecer un idioma.
    .auto_detect_lang = detectar_idioma_automático
    .auto_detect_lang-description = El valor predeterminado es falso
config-transcribe-only-role-disabled = Ahora Scripty transcribirá a todos los usuarios, independientemente del rol.
join-ephemeral-not-thread = Para usar el parámetro efímero, debes seleccionar un hilo como destino, o estableciendo `create_thread` a verdadero o dirigiendo a un hilo con `target_channel`.
premium-too-many-guilds = Has reclamado { $totalServers } claves premium. No puedes añadir ninguna más, salvo que mejores tu suscripción premium en <https://dash.scripty.org/premium>, o elimines algunas con el comando `{ $commandPrefix }premium remove`.
config_transcribe_video = transcribe_video
    .description = Cambiar si Scripty transcribe archivos de video arbitrarios. Requiere premium de nivel 2.
    .transcribe_video = transcribir_video
    .transcribe_video-description = El valor predeterminado es falso
config-translate-enabled = Ahora Scripty traducirá las transcripciones a inglés.
config-translate-disabled = Ahora Scripty intentará que coincidan las frases dichas a inglés, pero no las traducirá.
command-not-found = No se ha encontrado ningún comando llamado `{ $commandName }`.
default-category-name = Comandos
context-menu-command-title =
    { "" }
    Comandos de menú contextual:
    { "" }
context-menu-command-user =
    { "" }
    { $commandName } (sobre un usuario)
    { "" }
guild-language-set-success = Idioma de clan establecido a `{ $language }`.
language-set-failure-title-db = Error de base de datos.
root-command-invoked-title = ¡Es un comando raíz!
automod-setup-embed-complete-title = ¡Configuración de Automod completada!
automod-setup-embed-complete-description = Ahora puedes usar `{ $contextPrefix }automod rule add` para añadir una regla de moderación automática. { $extraDetails }
premium-claimed = Has reclamado con éxito premium en este servidor. Si quieres actualizarlo, u obtener más espacio, dirígete a <https://dash.scripty.org/premium>. Si quieres eliminar tu premium de este clan, ejecuta `{ $commandPrefix }premium remove`.
join-failed-dropped = Parece que Discord está teniendo problemas, no podemos hacer nada al respecto. Por favor, inténtalo más tarde.
config-translate-not-english = Debes establecer tu idioma a inglés para activar la traducción. Hazlo con `{ $contextPrefix }config language en`.
context-menu-command-message =
    { "" }
    { $commandName } (sobre un mensaje)
    { "" }
context-menu-command-unknown =
    { "" }
    { $commandName } (sobre un desconocido)
    { "" }
more-info-on-command =
    Para más información sobre un comando específico, escribe `{ $contextPrefix }help <name>`
    ```
cmds_user_language = user
    .description = Establecer tu idioma de usuario con uno de los idiomas disponibles.
    .language = idioma
    .language-description = El idioma que quieres establecer para tu usuario.
no-channel-specified = No te encuentras en un chat de voz, ni me has dicho un canal al que unirme. Prueba `{ $contextPrefix }join <channel>` para especificar un chat de voz, o únete tú mismo a un canal de voz y vuelve a ejecutar este comando.
user-language-set-success = Idioma de usuario establecido a `{ $language }`.
join-no-permission = No tengo permiso para unirme a { $targetMention }. Por favor, concédeme los permisos Ver canal y Conectar, o únete a un chat de voz distinto donde yo tenga permisos.
config_transcribe_voice_messages = transcribe_voice_messages
    .description = Cambiar si Scripty transcribe mensajes de voz.
    .transcribe_voice_messages = transcribir_mensajes_voz
    .transcribe_voice_messages-description = El valor predeterminado es verdadero
config-auto-detect-lang-requires-premium =
    Detectar automáticamente el idioma es una función Premium, ya que volver a ejecutar el modelo dos veces para adivinar el idioma es extremadamente costoso computacionalmente.
    Si quieres mejorar a Premium, dirígete a https://dash.scripty.org/premium. También puedes solicitar una prueba gratuita de Premium enviándole un mensaje directo al bot.
    Si esta función estaba activada antes, ahora está desactivada.
config-transcribe-only-role-enabled = Ahora Scripty sólo transcribirá mensajes de usuarios en { $roleId }.
cmds_config_server_language = guild
    .description = Establecer el idioma de este servidor con uno de los idiomas disponibles.
    .language = idioma
    .language-description = El idioma que quieres establecer para tu clan.
guild-language-set-success-description = Para volver a inglés, escribe `{ $contextPrefix }language guild_language en`.
latency-description =
    Latencia de WebSocket: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    Latencia de HTTP: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Latencia de base de datos: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)

    Nota: si cualquier latencia es igual a 0ms, significa que esa latencia en particular no se pudo calcular en este momento.
    Inténtalo de nuevo más tarde.
data-storage-toggle-audio-btn = Cambiar almacenamiento de audio
data-storage-toggle-msgs-btn = Cambiar almacenamiento de mensajes
data-storage-opted-in-audio = Ahora has optado por almacenar tu audio para el entrenamiento del modelo.
data-storage-command-timed-out = Se acabó el tiempo. Vuelve a ejecutar este comando si todavía quieres gestionar tus ajustes.
cmds_automod = automod
    .description = Gestionar la moderación automática de Scripty
cmds_ping = ping
    .description = Obtener la latencia del bot.
config-auto-detect-lang-enabled = Ahora Scripty detectará automáticamente el idioma hablado.
data-storage-opted-out-msgs = Ahora ya no puedes almacenar tus mensajes para el entrenamiento de evaluación.
config-auto-detect-lang-disabled = Scripty ya no detectará automáticamente el idioma hablado.
config_transcribe_only_role = transcribe_only_role
    .description = Limitar las transcripciones de Scripty solamente a usuarios en una canal de voz con este rol.
    .transcribe_only_role = transcribir_solo_rol
    .transcribe_only_role-description = Rol al que limitar: dejar vacío para desactivar.
automod-setup-embed-not-setup-title = Todavía no has aceptado los Términos de Servicio de Scripty y la Política de Privacidad.
automod-add-rule-embed-failure-description-not-setup = Debes ejecutar `{ $contextPrefix }automod setup` antes de añadir reglas.
vote-reminders-enabled = Recordatorio de voto activado.
vote-reminders-disabled = Recordatorio de voto desactivado.
blocked-entity-no-reason-given = No se dio ningún motivo para el bloqueo.
blocked-entity-reason-given = Motivo del bloqueo: { $reason }.
voice-connection-error-ws-closed-invalid-payload = discord cerró la conexión debido a una carga útil no válida
voice-connection-error-ws-closed-session-invalid = discord anuló la sesión
voice-connection-error-ws-closed-session-timeout = tiempo de espera de sesión agotado
voice-connection-error-ws-closed-server-crashed = servidor de voz de discord se ha colgado
voice-connection-error-unknown = desconectado sin motivo
voice-connection-error-msg-reconnect = He tenido un problema ({ $reason }) y he desconectado del canal de voz. Intentaré reconectar en 30 segundos.
general-error-command-process-title = Un error ocurrió mientras procesaba { $command }.
general-error-cooldown-hit-description = Quedan { $time } segundos en tiempo de espera.
free-trial-upsell = Ofrecemos pruebas de 3 días de Scripty Premium si quieres probarlo y ver si es lo que buscas. Envía un mensaje directo al bot para comenzar una prueba gratuita.
automod-setup-embed-not-setup-description = Hazlo primero ejecutando `{ $contextPrefix } terms_of_service`.
general-error-invalid-structure-title = Estructura no válida desde Discord al intentar analizar { $command }.
general-error-cooldown-hit-title = Tiempo de espera alcanzado en { $command }
automod-add-rule-embed-extra-details-free-limit = Los servidores gratuitos están limitados a 25 reglas generales. Si quieres aumentar este límite, echa un vistazo a nuestro Premium en https://scripty.org/premium.
automod-list-rules-embed-title = Reglas de moderación automática
automod-list-rules-footer = Página { $page } de { $maxPage }
automod-list-rules-no-rules = ¡No tienes ninguna regla!
voice-connection-error-ws-closed-unknown-encryption-mode = discord no reconoció el esquema de encriptado
voice-connection-error-msg-no-reconnect = He tenido un problema ({ $reason }) y he desconectado del canal de voz.
general-error-invalid-args-title = Argumentos no válidos mientras analizaba { $command }.
general-error-invalid-args-description = Error al analizar `{ $input }` porque `{ $error }`
general-error-user-missing-perms-description-known = Faltan permisos: { $perms }
general-error-user-missing-perms-description-not-owner = No eres propietario de este bot.
transcription-info-transcription-ssrc = SSRC { $ssrc }
general-error-command-process-description =
    ```
    { $errorFmt }
    ```
    Esto se ha informado automáticamente. Por favor, no intentes usar este comando en repetidas ocasiones.
delete-data-confirm = Sí, elimina todos los datos
automod-add-rule-embed-failure-description-invalid-type = Tipo de regla no válido. Revisa `{ $contextPrefix }automod rule_help` para más información.
automod-list-rules-embed-description = Te quedan { $rulesLeft } reglas de { $maxRules }.
voice-connection-error-host-io-error = error de host IO
voice-connection-error-ws-closed-no-reason = discord cerró la conexión sin motivo
voice-connection-error-ws-closed-unknown-opcode = discord cerró la conexión debido a un código de operación desconocido
general-error-user-missing-perms-title = Te faltan permisos para ejecutar { $command }.
general-error-user-missing-perms-description-unknown = No sé que permisos te faltan.
cmds_delete_all_data = delete_all_data
    .description = Eliminar todos tus datos.
delete-data-confirm-banned = Sí, eliminar todos los datos y prohíbeme a mí mismo
automod-remove-rule-embed-success-title = ¡Regla eliminada!
voice-connection-error-ws-closed-already-authenticated = discord cerró la conexión debido a que ya estaba autenticado
general-error-command-check-failed-title = Ha fallado una precondición para { $command }.
automod-add-rule-embed-failure-title = ¡Error al añadir la regla!
automod-remove-rule-embed-failure-description-invalid-id = Identificador de regla no válido. Revisa `{ $contextPrefix }automod list` para más información.
automod-add-rule-embed-success-description = Te quedan { $rulesLeft } reglas de { $maxRules }. { $extraDetails }
automod-add-rule-embed-failure-description-free-limit = Los servidores gratuitos están limitados a 25 reglas generales. Si quieres aumentar este límite, echa un vistazo a nuestro Premium en https://scripty.org/premium.
automod-list-rules-embed-field-name = Regla { $ruleId }
automod-list-rules-embed-field-value =
    Tipo: { $ruleType }
    Contenido: { $ruleContent }
    Acción: { $ruleAction }
voice-connection-error-internal-lib-error = error interno de biblioteca
voice-connection-error-proto-violation = biblioteca y discord no están de acuerdo en el protocolo
voice-connection-error-timed-out = tiempo de espera agotado para conexión
voice-connection-error-ws-closed-not-authenticated = discord cerró la conexión debido a no estar autenticado
voice-connection-error-ws-closed-authentication-failed = discord cerró la conexión debido a un fallo de autenticación
delete-data-cancel = No, cancelar
automod-add-rule-embed-success-title = ¡Regla { $ruleId } añadida!
automod-add-rule-embed-failure-description-premium-limit = Los servidores Premium de nivel { $tier } están limitados a { $maxRules } reglas. Si mejoras al nivel { $nextTier }, puedes añadir { $nextTierMaxRules } reglas.
automod-add-rule-embed-failure-description-premium-limit-hard-cap = Has alcanzado el máximo absoluto de reglas ({ $hardCap }). Este límite existe para asegurar que no añadimos mucha latencia en un solo mensaje.
automod-remove-rule-embed-success-description = Te quedan { $rulesLeft } reglas de { $maxRules }.
automod-remove-rule-embed-failure-title = ¡Error al eliminar regla!
automod-remove-rule-embed-failure-description-not-setup = Debes ejecutar `{ $contextPrefix }automod setup` antes de eliminar reglas.
voice-connection-error-ws-closed-server-not-found = no se encontró servidor de voz
voice-connection-error-ws-closed-unknown-protocol = discord no reconoció el protocolo
general-error-command-check-failed-description-no-reason = no se ha proporcionado ningún motivo
transcription-info-transcript-count = Transcripción 1 de { $count }.
transcription-info-transcription-title = Transcripción
delete-data-title = Eliminar datos
transcription-info-transcription-confidence = Fiabilidad
blocked-entity-guild = Han bloqueado este clan de usar Scripty. { $reason } Puedes intentar recurrir este bloqueo en el servidor de soporte: { $supportServerInvite }.
cmds_vote_reminder = vote_reminder
    .description = Cambiar si Scripty te recordará votar para el voto después de que haya pasado el tiempo límite.
    .enabled = activado
    .enabled-description = ¿Activar recordatorio de votos?
general-error-invalid-structure-description =
    { $description }

    { "**" }Nota**: este es un error de Discord.
    La única solución para esto es esperar a que Discord difunda el uso de los comandos Slash, lo que puede llevar hasta una hora.
    Si no quieres esperar esa hora, deberías usar los comando prefijo: ejecuta este comando con `~{ $qualifiedName } { $args }`.
cmds_setup = setup
    .description = Comenzar con la moderación automática de Scripty.
    .target_channel = canal_objetivo
    .target_channel-description = El canal al que enviar los registros de moderación automática.
    .log_recording = grabar_registro
    .log_recording-description = ¿Debería enviarse una grabación de lenguaje inapropiado al canal objetivo? El valor predeterminado es falso.
    .auto_join = auto_unir
    .auto_join-description = ¿El bot debería unirse automáticamente a una canal de voz si un usuario se une? El valor predeterminado es verdadero.
cmds_add_rule = add_rule
    .description = Añadir una regla de moderación automática.
    .rule_type = tipo_regla
    .rule_type-description = El tipo de regla a añadir. Revisa `/automod rule_help` para más información.
    .rule_type-choice-Regular = General
    .content = contenido
    .content-description = El contenido de la regla a añadir.
    .action = acción
    .action-description = La acción a tomar cuando se desencadene la regla.
    .action-choice-SilentDelete = Eliminar en silencio
    .action-choice-DeleteAndLog = Eliminar y registrar
    .action-choice-DeleteLogAndKick = Eliminar, registrar y desconectar al usuario del canal de voz
    .action-choice-DeleteLogAndSilence = Eliminar, registrar y silenciar al usuario
transcription-info-transcription-error =
    error interno: ejecutar el algoritmo stt ha fallado con este error: { $error }
    SSRC: { $ssrc }
    Esto se ha registrado y se arreglará tan pronto como sea posible.
    Si es posible, por favor contacta con los desarrolladores principales en el servidor de soporte: { $supportServerInvite }.
    ¡Gracias!
delete-data-description =
    Esto eliminará todos tus datos. Esta acción es permanente, irreversible y no se puede deshacer.

    Cuando decimos "todos tus datos" nos referimos a *todo* sobre tí. Esto incluye tus datos de voz, y tu usuario en la base de datos.
    En cambio, esto *no* incluye los mensajes que hayamos guardado de tí si optaste por ello. No podemos eliminar esos mensajes, simplemente porque no sabemos que usuario envió qué mensaje.

    Si también quieres que te prohibamos usar el bot tras esta acción, de manera que no te vuelvas a añadir accidentalmente, puedes hacer clic en el botón apropiado a continuación.
    Ten en cuenta que al hacerlo, nos obligará a guardar tu identificador de usuario para mantener un registro de los usuarios vetados.
    Si en cualquier momento tras esta acción quieres que te eliminemos la prohibición, puedes contactar en el servidor de soporte y solicitar que te eliminen la prohibición de manera manual.

    ¿Estás seguro de que quieres eliminar todos tus datos?
data-storage-embed-description =
    { "**" }NOTA**: todo lo que sigue es **completamente opcional**, y no participando **no** afectará, de ninguna manera, a tu experiencia con Scripty.
    Dicho esto, allá vamos.

    Scripty necesita muchos datos de audio y texto para entrenar un modelo adecuado de conversión de voz a texto. No todos pueden permitirse donar o comprar la suscripción premium para ayudarnos, así que una buena manera de ayudarnos es permitiendo que almacenemos tus datos como audios y mensajes para entrenar un modelo.
    Entendemos que estos datos pueden ser extremadamente personales, así que es totalmente opcional y no afectará tu experiencia de ninguna manera.

    Esto es lo que haríamos con ellos:
    { "*" } Con los mensajes guardados, alimentaríamos un evaluador dirigido a tu idioma. Este evaluador permitiría que el algoritmo seleccione las palabras más parecidas para una colección dada de sonidos. Aunque es inmensamente útil, esto no es tan importante como el audio. Ten en cuenta que estos datos de mensajes están encriptados con un cifrado AES 256-bit.
    { "*" } Con el audio almacenado, lo alimentaríamos junto con su transcripción a un modelo para aumentar la precisión del modelo de conversión de voz a texto. Esto es increiblemente útil, incluso si tienes un micrófono de baja calidad y ruido de fondo: de hecho, contra más ruido, mejor, siempre que una persona pueda entender lo que dices.

    Si participaste, y más tarde decides que ya no quieres participar, tus datos todavia se mantendrán guardados, pero puedes solicitar que eliminemos tus datos de voz ejecutando `{ $contextPrefix }delete_all_data`. Sin embargo, es imposible eliminar los datos de tus mensajes. Esto ocurre porque no guardamos un enlace de qué usuario dijo qué mensaje.
    Tus datos se guardan en servidores protegidos a cal y canto. Sería extremadamente difícil que alguien que intentara acceder a ellos lo consiguiera.

    Puedes cambiar tu elección utilizando los botones de abajo.
cmds_data_storage = data_storage
    .description = Configurar los ajustes de almacenamiento para tus datos
blocked-entity-user = Te han bloqueado para usar Scripty. { $reason } Puedes intentar recurrir este bloqueo en el servidor de soporte: { $supportServerInvite }.
automod-add-rule-embed-failure-description-free-locked-type = Los servidores gratuitos solo pueden utilizar reglas básicas. Si deseas utilizar más tipos de reglas, consulta nuestra versión Premium en https://scripty.org/premium.
cmds_remove_rule = remove_rule
    .description = Eliminar una regla de moderación automática.
    .rule_id = identificador_regla
    .rule_id-description = El identificador de la regla a eliminar.
cmds_list_rules = list_rules
    .description = Enumerar todas las reglas de moderación automática.
    .filter_by = filtrar_por
    .filter_by-description = Filtrar reglas por su contenido. Dejar vacío para mostrar todas las reglas.
debug-not-in-call = Este comando es inútil si Scripty no está en una VC.
debug-info-message = Reenvía este mensaje a quien te lo pida en el servidor de soporte de Scripty.
cmds_debug = depurar
    .description = Salida de depuración de la información sobre el funcionamiento interno de Scripty.
transcribe-message-initial-reply = Cargando...
transcribe-message-needs-reply = Debes responder al mensaje que quieres transcribir.
transcribe-message-downloading-file = Descargando archivo `{ $filename }`... (tamaño { $fileSize } bytes)
transcribe-message-probing-file = Sondeando el archivo `{ $filename }`...
transcribe-message-transcoding-file = Transcodificando el archivo `{ $filename }`... ({ $fileLength } segundos de duración)
transcribe-message-transcribing-file = Transcribiendo archivo `{ $filename }`... ({ $fileLength } segundos de duración)
transcribe-message-inline-header = Transcripción:
transcribe-message-time-taken-named-file = El archivo `{ $filename }` tardó { $timeTaken } segundos en transcribirse ({ $fileLength } segundos de duración)
transcribe-message-unusually-long = Esto no debería tardar mucho. Si desea compartir el contenido con nosotros, por favor, infórmenos en nuestro servidor de soporte.
transcribe-message-no-transcript = No se devolvió ninguna transcripción para el archivo `{ $filename }` tomó { $took } segundos para un archivo de ({ $fileLength } segundos)
transcribe-message-result-error = Se encontró un error al procesar `{ $filename }`: `{ $error }`. Por favor, infórmelo a nuestro servidor de soporte.
transcribe-message-malformed-input = Se detectó una entrada incorrecta al procesar `{ $filename }`. Corríjalo y vuelva a intentarlo. `{ $error }`
premium-info-embed-max-file-length = Longitud máxima del archivo (segundos)
config-prefix-unset = Se ha eliminado el prefijo personalizado de este gremio. Ahora se usará el prefijo predeterminado (`{ $updatedPrefix }`).
config-prefix-updated = Scripty ya no responderá al prefijo predeterminado en este gremio, sino solo a `{ $updatedPrefix }`.
config-default-new-thread-cant-make-thread-in-thread = No puedes crear un hilo dentro de un hilo. Elija un canal de destino predeterminado diferente o no habilite new_thread.
config-default-target-channel-enabled = Scripty ahora, de forma predeterminada, enviará todas las transcripciones a { $targetChannelMention }.
config-default-target-channel-disabled = Scripty ahora, de forma predeterminada, enviará todas las transcripciones al canal donde se ejecuta '/join'.
config-default-target-channel-cant-disable-with-auto-join = No puedes eliminar ningún canal de destino predeterminado si la unión automática está habilitada. Deshabilite la unión automática o cambie el canal de destino en lugar de eliminarlo.
config-default-ephemeral-enabled = Scripty ahora hará que todas las transcripciones sean efímeras.
config-default-record-transcriptions-enabled = Scripty ahora grabará todas las transcripciones en un archivo de texto.
config-default-record-transcriptions-disabled = Scripty ya no grabará todas las transcripciones en un archivo de texto.
join-forum-thread-content-auto = Una transcripción automática fue iniciada { $timestamp }.
config-auto-join-enabled = Scripty ahora se unirá automáticamente a los VC cuando un usuario lo haga.
cmds_config_auto_join = auto_join
    .description = ¿Debería Scripty unirse automáticamente a un canal de voz cuando alguien se une?
    .auto_join = auto_join
    .auto_join-description = El valor predeterminado es falso
config-auto-join-disabled = Scripty ya no se unirá automáticamente a los VC cuando un usuario lo haga.
config-auto-join-needs-target-channel = Para habilitar la unión automática, se requiere configurar un canal de destino predeterminado. Para ello, utilice `{ $contextPrefix }config default target_channel`.
config-kiai-enabled = Scripty ahora enviará a Kiai la XP de voz obtenida. Desactiva la nivelación de XP de voz de Kiai para evitar que los usuarios obtengan el doble de XP.
config_enable_kiai = enable_kiai
    .description = Habilita la integración de Kiai en Scripty. Ejecuta este comando sin argumentos para obtener información sobre Kiai.
    .enable_kiai = enable_kiai
    .enable_kiai-description = El valor predeterminado es falso
config-default-ephemeral-cant-use-voice-channels = Los canales de voz no admiten hilos, por lo que las transcripciones efímeras son imposibles. Cambie el canal de destino predeterminado o no utilice efímero.
cmds_config_default_settings_ephemeral = efímero
    .description = ¿Debería Scripty, por defecto, crear transcripciones efímeras que desaparezcan cuando el último usuario se haya ido?
    .ephemeral = efímero
    .ephemeral-description = Valor predeterminado para efímero en el comando de unión
config-default-ephemeral-cant-target-thread = Si se establece como efímero al apuntar a un hilo, este se eliminará tan pronto como finalice la transcripción, dejando un canal predeterminado no válido. Cambie el canal de destino predeterminado a un lugar donde se puedan crear subprocesos o no utilice efímeros.
config-default-new-thread-enabled = Scripty ahora creará un nuevo hilo para todas las transcripciones.
config-default-new-thread-cant-make-thread-in-vc = Los canales de voz no pueden tener hilos. Elija un canal de destino predeterminado diferente o no habilite new_thread.
config-default-new-thread-disabled = Scripty ya no creará un nuevo hilo para todas las transcripciones.
cmds_config_default_settings_record_transcriptions = record_transcriptions
    .description = ¿Debería Scripty, por defecto, grabar todas las transcripciones en un archivo de texto?
    .record_transcriptions = transcripciones_de_registros
    .record_transcriptions-description = Valor predeterminado para record_transcriptions en el comando de unión
config-kiai-disabled = Scripty ya no enviará ningún XP de voz obtenido a la API de Kiai.
config-kiai-missing-perms = A Scripty le faltan permisos para funcionar en este servidor. Autorícelo con el comando `/application authorize`, usando el ID de aplicación `811652199100317726` y otorgándole el permiso para ver y editar todos los niveles y XP.
config-kiai-info =
    Puedes encontrar más información sobre Kiai en [kiai.app](https://www.kiai.app/?utm_source=scripty_info).
    { "" }
    Si usas esta integración, asegúrate de desactivar el módulo de voz XP de Kiai, ya que podrían entrar en conflicto.
transcribe-message-timed-out-after-reply = Se produjo un error interno fatal. Esto nunca debería ocurrir y es un error. (Se agotó el tiempo de espera del resultado final tras la desaparición del flujo, ID { $msgId })
transcribe-message-not-slash-command = Debido a las limitaciones de Discord, este comando no está disponible como barra diagonal. Usa las versiones de prefijo o del menú contextual.
transcribe-message-no-results = No se devolvieron las transcripciones. El mensaje vinculado no tiene archivos adjuntos o no se admiten (solo para vídeo con Premium 1 o superior)
config-prefix-too-long = Los prefijos deben tener un máximo de ocho caracteres. Inténtalo de nuevo con uno más corto.
transcribe-message-video-needs-premium = Los videos son computacionalmente más costosos de transcodificar y requieren Premium. { $filename } ha sido detectado como un video y ha sido ignorado.
cmds_config_prefix = prefijo
    .description = Establece el idioma de este servidor en uno de los idiomas disponibles.
    .language = idioma
    .language-description = El idioma en el que quieres configurar el idioma de tu gremio.
transcribe-message-too-long = Con su estado Premium actual, puede transcribir archivos de una duración máxima de { $maxFileLength } segundos. `{ $filename }` tiene una duración de { $fileLength } segundos y ha sido ignorado.
transcribe-message-time-taken-single-file = Tomó { $timeTaken } segundos transcribir el archivo de { $fileLength } segundos.
cmds_config_default_settings_new_thread = new_thread
    .description = ¿Debería Scripty, por defecto, crear un nuevo hilo para todas las transcripciones?
    .new_thread = nuevo_hilo
    .new_thread-description = Valor predeterminado para new_thread en el comando de unión
config-default-ephemeral-disabled = Scripty ya no hará que todas las transcripciones sean efímeras.
config-default-target-channel-need-permissions = Scripty necesita enviar mensajes y administrar webhooks en el canal de destino. Dale esos permisos y vuelve a intentarlo.
cmds_config_default_settings_target_channel = target_channel
    .description = Establezca el canal de destino predeterminado donde Scripty emitirá las transcripciones si no se especifica ninguna.
    .target_channel = canal_objetivo
    .target_channel-description = Valor predeterminado para target_channel en el comando de unión
