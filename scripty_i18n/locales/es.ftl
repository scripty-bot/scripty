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
cmds_leave = abandonar
    .description = Abandonar cualquier llamada de voz en curso.
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = ayuda
    .description = Mostrar este menú de ayuda
    .command = comando
    .command-description = Comando específico para mostrar ayuda
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = reclamar
    .description = Reclama tu premium dentro del servidor donde se ejecuta este comando.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = eliminar
    .description = Elimina tu premium del servidor donde se ejecuta este comando.
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
cmds_join = unirse
    .description = Únete a un chat de voz. Las transcripciones se registrarán en el canal en el que ejecute este comando.
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
cmds_config_verbose = locuaz
    .description = Alterna si Scripty es locuaz durante las transcripciones.
    .verbose = locuaz
    .verbose-description = El valor predeterminado es falso
config-verbose-disabled = Scripty ya no será locuaz durante las transcripciones.
config_translate = traducir
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
cmds_transcribe_message = transcribir_mensaje
    .description = Transcribe un mensaje. Responde a un mensaje para transcribirlo.
cmds_premium_info = info
    .description = Consigue información sobre el estado Premium de Scripty de este servidor.
premium-info-embed-title = Estado Premium
premium-info-embed-description-has-subscription = Puedes gestionar tu suscripción en <https://dash.scripty.org/premium>. ¡Gracias por apoyar a Scripty!
premium-info-embed-max-users = Máximo de usuarios simultáneos
premium-info-embed-max-duration = Duración (en segundos) máxima de sesión
premium-info-embed-max-audio-length = Longitud (en segundos) máxima de archivo de audio
premium-info-embed-max-video-length = Longitud (en segundos) máxima de archivo de video
premium-info-embed-trial-available-title = ¿Quieres una prueba gratuita de Premium?
premium-info-embed-trial-available-description = Envía un mensaje directo al bot para comenzar a configurar una prueba gratuita de Premium de 3 días.
premium-info-embed-manage-subscription-user-has-unclaimed-title = ¡Parece que compraste Premium!
config-verbose-enabled = Ahora Scripty será locuaz durante las transcripciones.
premium-info-embed-description-no-subscription = Puedes suscribirte a Premium en <https://dash.scripty.org/premium>. Además de las ventajas que obtienes, también puedes ayudarnos en nuestro objetivo de hacer Scripty el mejor bot de conversión de voz a texto que existe :)
config_transcribe_audio = transcribir_audio
    .description = Alterna si Scripty transcribe archivos de audio arbitrarios. Premium obligatorio.
    .transcribe_audio = transcribir_audio
    .transcribe_audio-description = El valor predeterminado es falso
config-transcribe-audio-requires-premium =
    Transcribir archivos de audio es una función Premium, ya que transcodificar archivos de audio es costoso computacionalmente.
    Si quieres mejorar a Premium, dirígete a https://dash.scripty.org/premium. También puedes solicitar una prueba gratuita de Premium enviándole un mensaje directo al bot.
    Si esta función estaba activada antes, ahora está desactivada.
config-transcribe-video-disabled = Scripty ya no transcribirá archivos de video.
config_auto_detect_lang = detectar_idioma_automático
    .description = ¿Intenta detectar automáticamente el idioma hablado? Muy impreciso comparado a establecer un idioma.
    .auto_detect_lang = detectar_idioma_automático
    .auto_detect_lang-description = El valor predeterminado es falso
config-transcribe-only-role-disabled = Ahora Scripty transcribirá a todos los usuarios, independientemente del rol.
join-ephemeral-not-thread = Para usar el parámetro efímero, debes seleccionar un hilo como destino, o estableciendo `create_thread` a verdadero o dirigiendo a un hilo con `target_channel`.
premium-too-many-guilds = Has reclamado { $totalServers } claves premium. No puedes añadir ninguna más, salvo que mejores tu suscripción premium en <https://dash.scripty.org/premium>, o elimines algunas con el comando `{ $commandPrefix }premium remove`.
config_transcribe_video = transcribir_video
    .description = Alterna si Scripty transcribe archivos de video arbitrarios. Requiere premium de nivel 2.
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
automod-setup-embed-complete-title = ¡Configuración de automoderación completada!
automod-setup-embed-complete-description = Ahora puedes usar `{ $contextPrefix }automod rule add` para añadir una regla de automoderación. { $extraDetails }
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
cmds_user_language = usuario
    .description = Establece tu idioma de usuario con uno de los idiomas disponibles.
    .language = idioma
    .language-description = El idioma que quieres establecer para tu usuario.
automod-root-response = Este es el comando raíz, debido a limitaciones de Discord no hace nada. Para más información, echa un vistazo a `{ $contextPrefix }help automod`.
no-channel-specified = No te encuentras en un chat de voz, ni me has dicho un canal al que unirme. Prueba `{ $contextPrefix }join <channel>` para especificar un chat de voz, o únete tú mismo a un canal de voz y vuelve a ejecutar este comando.
user-language-set-success = Idioma de usuario establecido a `{ $language }`.
join-no-permission = No tengo permiso para unirme a { $targetMention }. Por favor, concédeme los permisos Ver canal y Conectar, o únete a un chat de voz distinto donde yo tenga permisos.
config_transcribe_voice_messages = transcribir_mensajes_voz
    .description = Alterna si Scripty transcribe mensajes de voz.
    .transcribe_voice_messages = transcribir_mensajes_voz
    .transcribe_voice_messages-description = El valor predeterminado es verdadero
config-auto-detect-lang-requires-premium =
    Detectar automáticamente el idioma es una función Premium, ya que volver a ejecutar el modelo dos veces para adivinar el idioma es extremadamente costoso computacionalmente.
    Si quieres mejorar a Premium, dirígete a https://dash.scripty.org/premium. También puedes solicitar una prueba gratuita de Premium enviándole un mensaje directo al bot.
    Si esta función estaba activada antes, ahora está desactivada.
config-transcribe-only-role-enabled = Ahora Scripty sólo transcribirá mensajes de usuarios en { $roleId }.
cmds_config_server_language = clan
    .description = Establece el idioma de este servidor con uno de los idiomas disponibles.
    .language = idioma
    .language-description = El idioma que quieres establecer para tu clan.
guild-language-set-success-description = Para volver a inglés, escribe `{ $contextPrefix }language guild_language en`.
latency-description =
    Latencia de WebSocket: { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    Latencia de HTTP: { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Latencia de base de datos: { $pgLatencyMs }ms ({ $pgLatencyNs }ns)

    Nota: si cualquier latencia es igual a 0ms, significa que esa latencia en particular no se pudo calcular en este momento.
    Inténtalo de nuevo más tarde.
data-storage-toggle-audio-btn = Alterna almacenamiento de audio
data-storage-toggle-msgs-btn = Alterna almacenamiento de mensajes
data-storage-opted-in-audio = Ahora has optado por almacenar tu audio para el entrenamiento del modelo.
data-storage-command-timed-out = Se acabó el tiempo. Vuelve a ejecutar este comando si todavía quieres gestionar tus ajustes.
cmds_automod = automoderacion
    .description = Gestiona la automoderación de Scripty
cmds_ping = ping
    .description = Obten la latencia del bot.
config-auto-detect-lang-enabled = Ahora Scripty detectará automáticamente el idioma hablado.
data-storage-opted-out-msgs = Ahora ya no puedes almacenar tus mensajes para el entrenamiento de evaluación.
config-auto-detect-lang-disabled = Scripty ya no detectará automáticamente el idioma hablado.
config_transcribe_only_role = transcribir_solo_rol
    .description = Limitar las transcripciones de Scripty solamente a usuarios en una canal de voz con este rol.
    .transcribe_only_role = transcribir_solo_rol
    .transcribe_only_role-description = Rol al que limitar: dejar vacío para desactivar.
automod-setup-embed-not-setup-title = Todavía no has aceptado los Términos de Servicio de Scripty y la Política de Privacidad
