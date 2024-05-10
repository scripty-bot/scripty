# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = entra
    .description = Entra nel canale vocale. Le trascrizioni vengono inviate nel canale in cui invii questo comando.
    .voice_channel = canale_vocale
    .voice_channel-description = Voice chat to bind to.
    .record_transcriptions = record_transcriptions
    .record_transcriptions-description = Log all transcripts? Users will be DMed when Scripty leaves the channel. Defaults to false.
    .target_channel = target_channel
    .target_channel-description = Send transcripts here, instead of the current channel. Target a forum to create a new post.
    .create_thread = create_thread
    .create_thread-description = Create a new thread for this transcription? Defaults to false.
