# This message is shown when the user tries to invite the bot to a voice channel, but the bot has not been set up.


# This message is shown on successfuly joining a voice channel.
# {$targetMention} is the mention of the channel the bot joined.


# This message is shown as the embed description when a user tries to invoke the root command of a group.


# This message is shown as the embed title when a user tries to invoke the root command of a group.


# This message is shown when the user is not in a voice channel, nor was a voice channel specified.


# This message is shown as the embed description when an entity tries to set their language to an unsupported language.


# This message is shown as the embed title when the user sets their language successfully.


# This message is shown as the embed description when the user sets their language successfully.


# This message is shown as the embed title when the guild sets their language successfully.


# This message is shown as the embed description when the guild sets their language successfully.


# This message is shown as the embed title when an entity tries to set their language to an unsupported language.


# This message is shown as the embed title when an entity tries to set their language to an invalid language.


# This message is shown as the embed description when an entity tries to set their language to an invalid language.


# This message is shown as the embed title when the database returns an error when setting the language for an entity.


# This message is shown as the embed description when the database returns an error when setting the language for an entity.


# This message is shown when the user tries to invite the bot to a voice channel,
# but the webhook used by the bot has been deleted.


# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.


# This is shown as the number of transcriptions the algorithm has discovered.


# This is shown as the title of the transcript


# This is shown as the percent accuracy of the transcription (roughly)


# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.


# This is shown when the algorithm encounters an error

# generic strings
# Message shown if a guild has not claimed their free trial of premium. Always appears on its own standalone line in the surrounding message.
free-trial-upsell = Nous proposons des essais de Scripty Premium pour trois jours, si vous souhaitez essayer et voir si ça vous plaît. Envoyez un message privé au bot pour commencer votre essai gratuit.
# Language configuration strings
# This message is shown as the embed description when an entity tries to set their language to an invalid language.
language-set-failure-description-invalid = La langue que vous avez sélectionnée n'est pas un identifiant de langue valide. Raison : { $error }
# join command
# This message is shown when the user is not in a voice channel, nor was a voice channel specified.
no-channel-specified = Vous n'êtes pas dans un salon vocal et vous ne m'avez pas indiqué un salon à rejoindre. Essayez `{ $contextPrefix }join <channel>` pour préciser un salon vocal ou rejoignez un salon vocal avant de re-lancer cette commande.
# automod setup command
automod-setup-embed-not-setup-title = Vous n'avez pas encore accepté les Conditions d'Utilisations et la Politique de Confidentialité de Scripty.
# automod add rule command
automod-add-rule-embed-failure-title = La règle n'a pas pu être ajoutée !
# automod add rule command
automod-add-rule-embed-failure-description-free-limit = Les serveurs gratuits sont limités à 25 règles, si vous souhaitez retirer cette limite, découvrez notre version Premium sur https://scripty.org/premium.
# automod list rules command
automod-list-rules-embed-field-name = Règle { $ruleId }
# automod list rules command
automod-list-rules-no-rules = Vous n'avez aucune règle !
# general errors
general-error-user-missing-perms-description-unknown = Je ne suis pas sûr de la permission qu'il vous manque.
# transcription info - verbose mode
# This is shown as the number of transcriptions the algorithm has discovered.
transcription-info-transcript-count = Transcript 1 sur { $count }.
# transcription info - verbose mode
# This is shown as the title of the transcript
transcription-info-transcription-title = Transcription
# transcription info - verbose mode
# This is shown as the percent accuracy of the transcription (roughly)
transcription-info-transcription-confidence = Confiance
# Data deletion command
delete-data-title = Supprimer les données
# Language configuration strings
# This message is shown as the embed title when the user sets their language successfully.
user-language-set-success = Votre langue est définie comme `{ $language }`.
# Language configuration strings
# This message is shown as the embed title when an entity tries to set their language to an unsupported language.
language-set-failure-title-unsupported = La langue choisie n'est pas supportée par le bot.
# Command invocation contexts
# This message is shown as the embed title when a user tries to invoke the root command of a group.
root-command-invoked-title = Cette commande est une commande racine !
# Command invocation contexts
# This message is shown as the embed description when a user tries to invoke the root command of a group.
root-command-invoked-description = Cette commande ne s'utilise qu'avec des sous-commandes. Pour plus d'informations tapez `{ $contextPrefix }help { $commandName }`.
# data_storage command
data-storage-embed-title = Stockage de données
# blocked entities description
blocked-entity-no-reason-given = Aucune raison n'a été donnée pour le bloquage.
# blocked entities description
blocked-entity-reason-given = Raison donnée pour le blocage : { $reason }.
# voice connection errors
voice-connection-error-ws-closed-no-reason = Discord a terminé la connexion sans raison
# voice connection errors
voice-connection-error-unknown = Déconnecté pour une raison inconnue
# general errors
general-error-invalid-args-title = Arguments invalides en analysant { $command }.
# general errors
general-error-user-missing-perms-description-not-owner = Vous n'êtes pas un propriétaire de ce bot.
# general errors
general-error-command-check-failed-description-no-reason = Aucune raison fournie
# ping command
# This message is shown when the user requests latency information.
# Note: the numbers here will be formatted according to the language set for the context.
latency-description =
    Latence du WebSocket : { $wsLatencyMs }ms ({ $wsLatencyNs }ns)
    Latence HTTP : { $httpLatencyMs }ms ({ $httpLatencyNs }ns)
    Latence de la Base de Données : { $pgLatencyMs }ms ({ $pgLatencyNs }ns)

    Note : Si une latence est égale à 0ms, cela indique qu'elle n'a pas pu être calculée maintenant.
    Re-essayez plus tard.
# data_storage command
data-storage-toggle-audio-btn = Stockage Audio
# data_storage command
data-storage-toggle-msgs-btn = Stockage de Message
# data_storage command
data-storage-opted-in-audio = Vous avez choisi de nous autoriser à stocker vos données audio pour l'entrainement de modèles.
# automod setup command
automod-setup-embed-complete-description = Vous pouvez maintenant utiliser `{ $contextPrefix }automod rule add` pour ajouter une règle automod. { $extraDetails }
# automod setup command
automod-setup-embed-complete-free-limit = Veuillez noter que les serveurs gratuits sont limités à 25 règles. Si vous souhaitez supprimer cette limite, consultez notre Premium sur https://scripty.org/premium.
# automod setup command
automod-setup-embed-complete-title = Configuration de l'automod terminée !
# automod setup command
automod-setup-embed-not-setup-description = Faites-le d'abord en utilisant `{ $contextPrefix } terms_of_service`.
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit = Les serveurs Premium tier { $tier } sont limitées à { $maxRules } règles. Si vous souscrivez au tier { $nextTier }, vous pourrez ajouter { $nextTierMaxRules } règles.
# automod add rule command
automod-add-rule-embed-failure-description-invalid-type = Type de règle invalide. Pour plus d'informations, voyez `{ $contextPrefix }automod rule_help`.
# automod remove rule command
automod-remove-rule-embed-success-description = { $rulesLeft } règles restantes sur { $maxRules }.
# automod remove rule command
automod-remove-rule-embed-failure-title = La règle n'a pas pu être retirée !
# automod remove rule command
automod-remove-rule-embed-failure-description-invalid-id = Identifiant de règle invalide. Pour plus d'informations, utilisez `{ $contextPrefix }automod list`.
# automod list rules command
automod-list-rules-embed-field-value =
    Type : { $ruleType }
    Contenu : { $ruleContent }
    Action : { $ruleAction }
# automod list rules command
automod-list-rules-footer = Page { $page } sur { $maxPage }
# blocked entities description
blocked-entity-guild = Ce serveur n'est pas autorisé à utiliser Scripty. { $reason } Vous pouvez faire appel de cette décision sur le serveur de support : { $supportServerInvite }.
# voice connection errors
voice-connection-error-internal-lib-error = Erreur interne de bibliothèque (library internal error)
# voice connection errors
voice-connection-error-ws-closed-session-invalid = Discord a rendu la session invalide
# voice connection errors
voice-connection-error-ws-closed-session-timeout = La session a time out
# general errors
general-error-command-process-description =
    ```
    { $errorFmt }
    ```
    Cette erreur a été signalée automatiquement, merci de ne pas essayer d'utiliser cette commande en boucle.
# general errors
general-error-cooldown-hit-title = Limite de temps atteinte sur { $command }
# general errors
general-error-command-check-failed-title = Une condition pour { $command } a échouée.
# Data deletion command
delete-data-confirm = Oui, supprimer toutes mes données
# Data deletion command
delete-data-confirm-banned = Oui, supprimer toutes mes données et être bannis
# Data deletion command
delete-data-cancel = No, annuler
# data_storage command
data-storage-opted-out-audio = Vous avez choisi de ne plus nous autoriser à stocker vos données audio pour l'entrainement de modèles.
# automod add rule command
automod-add-rule-embed-success-title = Règle { $ruleId } ajoutée !
# automod add rule command
automod-add-rule-embed-failure-description-premium-limit-hard-cap = Vous avez atteint le nombre maximum de règles possible ({ $hardCap }). Cette limite existe pour s'assurer que nous n'ajoutons pas trop de latence à un message.
# automod add rule command
automod-add-rule-embed-failure-description-free-locked-type = Les serveurs gratuits peuvent uniquement utiliser les règles basiques. Si vous souhaitez utiliser plus de types de règles, découvrez notre version Premium sur https://scripty.org/premium.
# automod add rule command
automod-add-rule-embed-failure-description-not-setup = Vous devez utiliser `{ $contextPrefix }automod setup` avant d'ajouter une règle.
# automod remove rule command
automod-remove-rule-embed-success-title = Règle retirée !
# automod remove rule command
automod-remove-rule-embed-failure-description-not-setup = Vous devez utiliser `{ $contextPrefix }automod setup` avant de retirer des règles.
# voice connection errors
voice-connection-error-ws-closed-invalid-payload = Discord a fermé la connection à cause d'un payload invalide
# voice connection errors
voice-connection-error-ws-closed-not-authenticated = Discord a fermé la connection car non-enregistrée
# voice connection errors
voice-connection-error-ws-closed-authentication-failed = Discord a fermé la connection à cause d'une erreur d'authentification
# voice connection errors
voice-connection-error-ws-closed-already-authenticated = Discord a fermé la connection à cause d'une authentification déjà complétée
# voice connection errors
voice-connection-error-ws-closed-server-crashed = Le serveur vocal de discord a crash
# general errors
general-error-invalid-structure-title = Structure invalide de Discord en analysant { $command }.
# general errors
general-error-user-missing-perms-title = Vous n'avez pas la permissions d'utiliser { $command }.
# general errors
# Note $time will be a decimal with two digits of accuracy.
general-error-cooldown-hit-description = { $time } secondes restantes.
# general errors
general-error-user-missing-perms-description-known = Permissions manquantes : { $perms }
# Context menu command translation strings
more-info-on-command =
    Pour plus d'informations à propos d'une commande, tapez `{ $contextPrefix }help <nom>`
    ```
# Language configuration strings
# This and all attributes show up exclusively in the slash command picker when `language user_language` is selected.
cmds_user_language = user
    .description = Sélectionnez votre langue parmi les langues disponibles
    .language = langue
    .language-description = La langue que vous souhaitez séléctionner.
# Language configuration strings
# This message is shown as the embed description when the user sets their language successfully.
user-language-set-success-description = Pour revenir en Anglais, tapez `{ $contextPrefix }language user_language en`.
# Language configuration strings
# This message is shown as the embed title when the guild sets their language successfully.
guild-language-set-success = Langue du serveur définie : `{ $language }`.
# Language configuration strings
# This message is shown as the embed description when the guild sets their language successfully.
guild-language-set-success-description = Pour revenir en Anglais, tapez `{ $contextPrefix }language guild_language en`.
# Language configuration strings
# This message is shown as the embed description when an entity tries to set their language to an unsupported language.
language-set-failure-description-unsupported = Si vous souhaitez aider à mettre en place cette langue, rejoignez le serveur de support : { $supportServerInvite }.
# Language configuration strings
# This message is shown as the embed title when an entity tries to set their language to an invalid language.
language-set-failure-title-invalid = Langue `{ $language }` introuvable.
# Language configuration strings
# This message is shown as the embed title when the database returns an error when setting the language for an entity.
language-set-failure-title-db = Erreur de base de donnée.
# join command
# If the user specifies they would like to create a thread, this is set as the thread name. { $timestamp } is the current timestamp, in ISO format.
join-thread-title = Transcription depuis { $timestamp }
# join command
# If the user specifies they would like to create a forum post, this is the contents of the initial message. { $timestamp } is the current timestamp, in ISO format, and { $authorMention } is the mention of the user who ran the command.
join-forum-thread-content = { $authorMention } a commencé une transcription à { $timestamp }.
# ping command
# This and all attributes show up exclusively in the slash command picker when `ping` is selected.
cmds_ping = ping
    .description = Obtenez la latence du bot.
# data_storage command
data-storage-command-timed-out = Expiré, Relancez cette commande si vous souhaitez toujours gérer ces paramètres.
# automod root command
# This and all attributes show up exclusively in the slash command picker when `automod` is selected.
cmds_automod = automod
    .description = Gérez l'automod de Scripty
# automod add rule command
# This and all attributes show up exclusively in the slash command picker when `automod add rule` is selected.
cmds_add_rule = add_rule
    .description = Ajoutez une règle d'automod
    .rule_type = rule_type
    .rule_type-description = Le type de règle à ajouter. Utilisez `/automod rule_help`pour plus d'informations.
    .rule_type-choice-Regular = Regular
    .content = content
    .content-description = Le contenu de la règle à ajouter.
    .action = action
    .action-description = L'action à prendre quand la règle est activée.
    .action-choice-SilentDelete = Suppression silencieuse
    .action-choice-DeleteAndLog = Supprimer et enregistrer un log
    .action-choice-DeleteLogAndKick = Supprimer, enregistrer un log et exclure
    .action-choice-DeleteLogAndSilence = Supprimer, enregistrer un log et rendre muet
# automod remove rule command
# This and all attributes show up exclusively in the slash command picker when `automod remove rule` is selected.
cmds_remove_rule = remove_rule
    .description = Retire une règle d'automod.
    .rule_id = rule_id
    .rule_id-description = L'ID de la règle à retirer.
# voice connection errors
voice-connection-error-ws-closed-server-not-found = Le serveur vocal n'a pas pu être trouvé
# Language configuration strings
# This message is shown as the embed description when the database returns an error when setting the language for an entity.
language-set-failure-description-db = La base de données à rencontrée une erreur en changeant votre langue. Cette erreur à été signalée et nous investiguons le problème. Merci de ne pas répéter cette commande. (Si vous êtes curieux, voici l'erreur : { $error })
# data_storage command
# This and all attributes show up exclusively in the slash command picker when `data_storage` is selected.
cmds_data_storage = data_storage
    .description = Sélectionnez un réglage de stockage pour vos données
# data_storage command
data-storage-embed-description =
    { "**" }NOTE** : Tout ce qui suis est **entièrement optionnel**, et ne pas accepter **ne va pas**, en aucun cas, affecter votre experience avec Scripty.
    Ceci-dit.

    Scripty nécessite beaucoup de données audio et texte pour entrainer un modèle de reconnaissance vocale. Tout le monde n'a pas la possibilité de donner ou de souscrire à l'abonnement premium pour nous aider, donc un bon moyen de nous aider est de nous autoriser à stocker vos données audio et textuelles pour entrainer notre model.
    Nous comprenons que ces données peuvent être extrêmement personnelles, pour cette raison, ceci est totalement volontaire et n'affectera en aucun cas votre experience.

    Voici ce que nous ferions avec :
    { "*" } Avec les messages stockés, nous les utiliserions pour alimenter un outil de notation adapté à votre langue, cet outil permettrais à l'algorithme de sélectionner les mots les plus probablement associés à certains sons. Bien qu'extrêmement utile, ce n'est pas aussi important que l'audio. Notez que les données de messages sont encroûtées avec une encryption AES 256-bit.
    { "*" } Avec l'audio stocké et son transcript, nous pourrons nourrir un modèle qui augmente la précision du modèle de reconnaissance vocale. C'est extrêmement utile, même si vous avez un mauvais microphone ou beaucoup de bruits de fond : en fait, plus il y a de bruits, mieux c'est. Tant qu'un humain peut comprendre ce que vous dites.

    Si vous décidez d'autoriser la collecte de ces données, et que vous changez d'avis plus tard, vos données restent stockées mais vous pouvez nous envoyer une demande de suppression de vos données vocales en utilisant la commande `{ $contextPrefix }delete_all_data`. Néanmoins, il est impossible de supprimer vos données de messages car nous ne stockons que des données sans lien direct avec l'utilisateur qui a envoyé le message.
    Vos données sont stockées sur des serveurs sécurisés, il serait extrêmement compliqué pour quelqu'un d'avoir accès à ces données. 

    Vous pouvez définir vos choix en utilisant les boutons ci-dessous.
# automod setup command
# This and all attributes show up exclusively in the slash command picker when `automod setup` is selected.
cmds_setup = setup
    .description = Commencez à utiliser l'automod de Scripty.
    .target_channel = target_channel
    .target_channel-description = Le salon auquel envoyer les logs d'automod.
    .log_recording = log_recording
    .log_recording-description = Est-ce qu'un enregistrement d'un discours offensant doit être envoyé à dans le salon de log ? Defaut : non.
    .auto_join = auto_join
    .auto_join-description = Est-ce que le bot doit automatiquement rejoindre un salon vocal quand un utilisateur se connecte ? Defaut : oui.
# Data deletion command
delete-data-description =
    Ceci supprimera toutes vos données. Cette action est permanente, irreversible et ne peut pas être annulée.

    Quand nous disons, "toutes vos données" nous voulons dire *toutes* vos données. Celles-ci incluent vos données vocales, et votre utilisateur dans la base de données.
    Cela n'inclut pas en revanche les messages envoyés que nous pourrions avoir stockés si vous nous avez permis de le faire. Nous ne pouvons pas supprimer ces messages, car nous n'enregistrons pas l'auteur de ces messages.

    Si vous souhaitez aussi être banni de ce bot après cette action, pour ne pas accidentellement vous ajouter à nouveau, vous pouvez cliquer sur le bouton approprié ci-dessous.
    Notez que cela nous autorisera à stocker votre ID d'utilisateur pour garder une liste des utilisateurs bannis.
    Si dans le futur vous souhaitez être débannis, vous pourrez contacter le support et demande un dé bannissement manuel.

    Êtes vous sûr de vouloir supprimer toutes vos données ?
# automod list rules command
# This and all attributes show up exclusively in the slash command picker when `automod list rules` is selected.
cmds_list_rules = list_rules
    .description = Liste toutes les règles d'automod.
    .filter_by = filter_by
    .filter_by-description = Filtrer les règles par leur contenu, laisser vide pour afficher toutes les règles.
# automod list rules command
automod-list-rules-embed-title = Règles d'automod
# automod list rules command
automod-list-rules-embed-description = { $rulesLeft } règles restantes sur { $maxRules }.
# blocked entities description
blocked-entity-user = Vous n'êtes pas autorisé à utiliser Scripty. { $reason } Vous pouvez faire appel de cette décision sur le serveur de support : { $supportServerInvite }.
# voice connection errors
voice-connection-error-host-io-error = host IO error
# voice connection errors
voice-connection-error-ws-closed-unknown-opcode = Discord a fermé la connection à cause d'un opcode inconnu
# voice connection errors
voice-connection-error-ws-closed-unknown-protocol = Discord n'a pas reconnu le protocole
# voice connection errors
voice-connection-error-proto-violation = la bibliothèque et Discord n'étaient pas d'accord sur le protocole
# voice connection errors
voice-connection-error-ws-closed-unknown-encryption-mode = Discord n'a pas reconnu le schema d'encryption
# voice connection errors
voice-connection-error-msg-reconnect = J'ai eu un problème ({ $reason }) et j'ai déconnecté du salon vocal. J'essaierais de me reconnecter dans 30 secondes.
# general errors
general-error-invalid-args-description = Impossible d'analyser `{ $input }` à cause de `{ $error }`
# voice connection errors
voice-connection-error-msg-no-reconnect = J'ai eu un problème ({ $reason }) et j'ai déconnecté du salon vocal.
# general errors
general-error-invalid-structure-description =
    { $description }

    { "**" }Note** : Ceci est une erreur de Discord.
    La seule solution à ce problème est d'attendre que Discord diffuse la commande slash, ce qui peut prendre jusqu'à une heure..
    Si vous ne voulez pas attendre cette heure, vous devriez utiliser une commande texte : utilisez cette commande avec `~{ $qualifiedName } { $args }`.
# general errors
general-error-command-process-title = Une erreur est survenue pendant l'execution de { $command }.
# transcription info - verbose mode
# This is shown when the algorithm encounters an error
transcription-info-transcription-error =
    Erreur interne : l'execution de l'algorithme de reconnaissance de voix a échouée avec l'erreur : { $error }
    SSRC : { $ssrc }
    Ceci a été enregistré et sera réglé dès que possible.
    Si possible, contactez les développeurs sur le serveur de support : { $supportServerInvite }.
    Merci !
# Data deletion command
# This and all attributes show up exclusively in the slash command picker when `delete_all_data` is selected.
cmds_delete_all_data = delete_all_data
    .description = Supprimer toutes vos données.
# join command
# This message is shown when the user has told the bot to create a thread while in a thread.
join-create-thread-in-thread = Je ne peux pas créer un fil dans un fil. Merci d'utiliser cette commande dans un salon normal, probablement { $parentChannelMention }.
# join command
# This message is shown when Discord tosses a Dropped or TimedOut error when trying to join a voice channel.
join-failed-dropped = Discord semble avoir des problèmes, nous ne pouvons rien faire pour le moment, re-essayez plus tard.
# data_storage command
data-storage-opted-in-msgs = Vous avez choisi de nous autoriser à stocker vos messages pour l'entrainement d'un outil de notation.
# data_storage command
data-storage-opted-out-msgs = Vous avez choisi de ne pas nous autoriser à stocker vos messages pour l'entrainement d'un outil de notation.
# join command
# This message is shown when the user attempts to make Scripty join a voice channel, but there is no one in the channel.
join-no-one-in-channel = Il n'y a personne dans { $targetMention }. Je ne rejoindrais pas si il n'y a personne pour éviter de gaspiller des ressources.
# automod add rule command
automod-add-rule-embed-success-description = { $rulesLeft } règles restantes sur { $maxRules }. { $extraDetails }
# automod add rule command
automod-add-rule-embed-extra-details-free-limit = Les serveurs gratuits sont limités à 25 règles, si vous souhaitez retirer cette limite, découvrez notre version Premium sur https://scripty.org/premium.
# voice connection errors
voice-connection-error-timed-out = Connexion perdue, en attendre de connexion
# transcription info - verbose mode
# This is shown as the user's SSRC (Synchonization Source)
# You do not need to translate this, but it is here if your language can provide a more accurate translation.
transcription-info-transcription-ssrc = SSRC { $ssrc }
# join command
# This message is shown when the bot does not have permissions for the voice channel it is trying to join.
join-no-permission = Je n'ai pas la permission de rejoindre { $targetMention }. Merci de me donner la permission de Voir le salon ainsi que la permission de me Connecter, ou rejoignez un autre salon vocal ou j'ai la permission.
# premium command
# This is shown to the user when they have too many used servers to add more.
premium-too-many-guilds = Vous avez réclamé { $totalServers } clés premium. Vous ne pouvez plus en ajouter, sauf si vous mettez à niveau votre abonnement premium sur <https://dash.scripty.org/premium>, ou en supprimez certaines avec la commande `{ $commandPrefix }premium remove`.
# Help menu translation strings
no-help-found = Aucune aide trouvée pour la commande `{ $commandName }`.
# Leave command
# This is shown when the bot successfully leaves a voice call
leave-success = A quitté le salon vocal avec succès.
# Leave command
# This and all attributes show up exclusively in the slash command picker when `leave` is selected.
cmds_leave = partir
    .description = Quitter tout appel en cours.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium` is selected.
cmds_premium = premium
    .description = Commandes Premium
# join command
# This message is shown when the user has told a bot to join a forum channel, but the forum requires tags. This is not possible for the bot to work around as it has no way of knowing what tags to use.
join-forum-requires-tags = Le forum que vous avez essayé de me faire utiliser demande des balises obligatoires. Je n'ai aucun moyen de savoir quelles balises utiliser, je ne peux donc pas rejoindre ce canal. Veuillez utiliser un autre canal ou demander à un administrateur de supprimer l'exigence de balise.
# Help command
# This and all attributes show up exclusively in the slash command picker when `help` is selected.
cmds_help = aide
    .description = Afficher ce menu d'aide
    .command = commande
    .command-description = Commande spécifique pour laquelle afficher l'aide
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium claim` is selected.
cmds_premium_claim = claim
    .description = Réclamez votre Premium sur le serveur où cette commande est exécutée.
# join command
# This and all attributes show up exclusively in the slash command picker when `join` is selected.
cmds_join = rejoindre
    .description = Rejoindre un chat vocal. Les transcriptions seront enregistrées sur le canal dans lequel vous exécutez cette commande.
    .voice_channel = chat_vocal
    .voice_channel-description = Chat vocal auquel se lier.
    .record_transcriptions = enregistrer_transcriptions
    .record_transcriptions-description = Enregistrer toutes les transcriptions ? Les utilisateurs seront MP lorsque Scripty quittera le canal. La valeur par défaut est false.
    .target_channel = canal_cible
    .target_channel-description = Envoyez les transcriptions ici, au lieu du canal actuel. Ciblez un forum pour créer un nouveau message.
    .create_thread = créer_thread
    .create_thread-description = Créer un nouveau fil pour cette transcription ? La valeur par défaut est false.
# premium command
# This and all attributes show up exclusively in the slash command picker when `premium remove` is selected.
cmds_premium_remove = Supprimer
    .description = Retirer votre Premium du serveur où cette commande est exécutée.
# premium command
# This is shown when the user successfully claims one of their premium subscriptions.
premium-claimed = Vous avez réclamé avec succès le premium sur ce serveur. Si vous souhaitez mettre à niveau ou acheter plus d'emplacements, rendez-vous sur <https://dash.scripty.org/premium>. Si vous souhaitez supprimer votre premium de ce serveur, exécutez `{ $commandPrefix }premium remove`.
# premium command
# This is shown when the user successfully removes their premium from this guild.
premium-removed = Si vous êtes l'utilisateur qui l'avait réclamé, vous avez maintenant supprimé avec succès votre premium de ce serveur. Si vous souhaitez mettre à niveau ou acheter plus d'emplacements, rendez-vous sur <https://dash.scripty.org/premium>.
# Context menu command translation strings
context-menu-command-user =
    { "" }
    { $commandName } (sur un utilisateur)
    { "" }
# premium command
# This is shown to the user when they are not subscribed to premium.
premium-not-premium = Vous n'êtes pas un abonné premium. Abonnez-vous sur https://scripty.org/premium. Si vous savez que vous en êtes un, veuillez envoyer un DM au bot afin que nous puissions rétablir votre premium.
# Help menu translation strings
command-not-found = Aucune commande avec le nom `{ $commandName }` trouvée.
# Help menu translation strings
command-not-found-suggestions = Vouliez-vous dire `{ $suggestion }` ?
# Help menu translation strings
default-category-name = Commandes
# Context menu command translation strings
context-menu-command-title =
    { "" }
    Commandes du menu contextuel :
    { "" }
# Context menu command translation strings
context-menu-command-message =
    { "" }
    { $commandName } (sur un message)
    { "" }
config-translate-enabled = Scripty traduira désormais les transcriptions en anglais.
premium-info-embed-current-tier = Niveau actuel
join-success-footer-free-trial-upsell = Ce serveur est éligible à un essai gratuit de Premium. Envoyez un DM au bot pour en demander un.
config-kiai-info =
    Vous pouvez trouver plus d'informations sur Kiai sur [kiai.app](https://www.kiai.app/?utm_source=scripty_info).
    { "" }
    Si vous utilisez cette intégration, assurez vous de désactiver le module XP vocal de Kiai car ils entreront en conflit.
config-transcribe-only-role-enabled = Scripty transcrira désormais uniquement les messages des utilisateurs dans { $roleId }.
join-success-help-description = Vous pouvez soit rejoindre le serveur de support à l'adresse { $supportServerInvite }, soit envoyer un DM au bot.
join-success-premium = Vous pouvez vérifier le statut Premium de ce serveur avec `/premium info`.
join-create-thread-in-unsupported = Discord ne prend pas en charge les canaux dans { $targetMention }. Veuillez utiliser un autre canal ou ne pas créer de canal.
cmds_premium_info = info
    .description = Obtenez des informations sur le statut Scripty Premium de ce serveur.
premium-info-embed-title = Statut Premium
premium-info-embed-description-has-subscription = Vous pouvez gérer votre abonnement sur <https://dash.scripty.org/premium>. Merci de soutenir Scripty !
premium-info-embed-max-users = Nombre maximal d'utilisateurs simultanés
premium-info-embed-trial-available-title = Vous souhaitez un essai gratuit de Premium ?
premium-info-embed-trial-available-description = Envoyez un DM au bot pour commencer à configurer un essai de 3 jours de Premium.
premium-info-embed-manage-subscription-user-has-unclaimed-title = Il semblerait que vous ayez acheté Premium !
premium-info-embed-manage-subscription-user-has-unclaimed-description = Pour le réclamer sur ce serveur, exécutez { $claimCommand }.
config-verbose-disabled = Scripty ne sera plus verbeux pendant les transcriptions.
config-transcribe-voice-messages-enabled = Scripty va désormais transcrire les messages vocaux.
config-transcribe-video-requires-premium =
    La transcription de fichiers vidéo est une fonctionnalité Premium Tier 2, car le transcodage de fichiers vidéo est très coûteux en termes de calcul.
    Si vous souhaitez passer au niveau Premium Tier 2, rendez-vous sur https://dash.scripty.org/premium.
    Si cette fonctionnalité était activée auparavant, elle est désormais désactivée.
config-auto-detect-lang-enabled = Scripty détectera désormais automatiquement la langue parlée.
config-auto-detect-lang-disabled = Scripty ne détectera plus automatiquement la langue parlée.
config_translate = traduire
    .description = Traduire automatiquement les transcriptions en anglais ?
    .translate = traduire
    .translate-description = La valeur par défaut est faux
config-translate-disabled = Scripty va maintenant tenter de faire correspondre les phrases prononcées aux mots anglais, mais ne traduira pas.
config-translate-not-english = Vous devez définir votre langue sur l'anglais pour activer la traduction. Faites le avec `{ $contextPrefix }config language en`.
config-transcribe-voice-messages-disabled = Scripty ne transcrira plus les messages vocaux.
config_transcribe_audio = transcribe_audio
    .description = Activer ou désactiver la transcription de fichiers audio arbitraires par Scripty. Nécessite une version premium.
    .transcribe_audio = transcribe_audio
    .transcribe_audio-description = La valeur par défaut est faux
config-transcribe-video-enabled = Scripty va désormais transcrire les fichiers vidéo.
language-set-partially-translated-help = Vous souhaitez aider à traduire Scripty dans votre langue ? Découvrez le projet de traduction sur https://hosted.weblate.org/engage/scripty-bot/.
cmds_config_verbose = verbose
    .description = Activer ou désactiver le mode verbeux de Scripty pendant les transcriptions.
    .verbose = verbeux
    .verbose-description = La valeur par défaut est false
config-transcribe-audio-disabled = Scripty ne transcrira plus les fichiers audio.
config-transcribe-video-disabled = Scripty ne transcrira plus les fichiers vidéo.
guild-language-set-failure-translate-enabled = La traduction automatique est activée sur votre serveur. Cette fonction n'est prise en charge que lors de la traduction vers l'anglais. Désactivez cette fonctionnalité si vous souhaitez définir votre langue.
premium-info-embed-max-duration = Durée maximale de la session (secondes)
premium-info-embed-max-file-length = Longueur maximale du fichier (secondes)
config_transcribe_only_role = transcribe_only_role
    .description = Limitez les transcriptions de Scripty aux seuls utilisateurs ayant ce rôle dans un chat vocal.
    .transcribe_only_role = transcribe_only_role
    .transcribe_only_role-description = Rôle à limiter : laissez vide pour désactiver.
config_transcribe_video = transcribe_video
    .description = Activer ou désactiver la transcription de fichiers vidéo arbitraires par Scripty. Nécessite T2 Premium.
    .transcribe_video = transcribe_video
    .transcribe_video-description = La valeur par défaut est faux
config-verbose-enabled = Scripty sera désormais verbeux pendant les transcriptions.
config_auto_detect_lang = auto_detect_lang
    .description = Essayer de détecter automatiquement la langue parlée ? Très imprécis par rapport à la définition d'une langue.
    .auto_detect_lang = auto_detect_lang
    .auto_detect_lang-description = La valeur par défaut est faux
vote-reminders-enabled = Rappels de vote activés.
vote-reminders-disabled = Rappels de vote désactivés.
cmds_vote_reminder = vote_reminder
    .description = Activer ou désactiver si Scripty vous rappellera de voter pour le bot une fois le délai écoulé.
    .enabled = activé
    .enabled-description = Activer les rappels de vote ?
config-kiai-enabled = Scripty enverra désormais tout XP vocal gagné à Kiai. Désactivez la mise à niveau de l'XP vocale de Kiai pour empêcher les utilisateurs d'obtenir le double d'XP.
config-kiai-disabled = Scripty n'enverra plus aucun XP de voix gagné à l'API de Kiai.
config_enable_kiai = enable_kiai
    .description = Activer l'intégration Kiai de Scripty. Exécutez cette commande sans argument pour obtenir des informations sur Kiai.
    .enable_kiai = enable_kiai
    .enable_kiai-description = La valeur par défaut est faux
cmds_transcribe_message = transcribe_message
    .description = Transcrire un message. Répondre à un message pour le transcrire.
config-transcribe-audio-enabled = Scripty va désormais transcrire les fichiers audio.
config-transcribe-only-role-disabled = Scripty transcrira désormais tous les utilisateurs, quel que soit leur rôle.
cmds_debug = debug
    .description = Affiche des informations de débogage sur l'état interne de Scripty.
debug-info-message = Transférez ce message à la personne du serveur d'assistance Scripty qui vous le demande.
debug-not-in-call = Cette commande est inutile si Scripty n'est pas dans un VC.
premium-info-embed-description-no-subscription = Vous pouvez vous abonner à Premium sur <https://dash.scripty.org/premium>. En plus des avantages dont vous bénéficiez, vous nous aidez également à atteindre notre objectif de faire de Scripty le meilleur bot de conversion de la parole en texte :)
join-ephemeral-not-thread = Pour utiliser le paramètre éphémère, vous devez sélectionner un canal comme cible, soit en définissant `create_thread` sur vrai, soit en ciblant un canal avec `target_channel`.
context-menu-command-unknown =
    { "" }
    { $commandName } (sur inconnu)
    { "" }
join-success-help-title = Besoin d'aide ?
join-target-not-text-based = Le canal auquel vous m'avez demandé d'envoyer des transcriptions ({ $targetMention }) n'est pas un canal textuel. Veuillez utiliser un canal textuel ou choisir un autre canal dans l'argument `target_channel`.
config_transcribe_voice_messages = transcribe_voice_messages
    .description = Activer ou désactiver la transcription des messages vocaux par Scripty.
    .transcribe_voice_messages = transcribe_voice_messages
    .transcribe_voice_messages-description = La valeur par défaut est vrai
config-auto-detect-lang-requires-premium =
    La détection automatique de la langue est une fonctionnalité Premium, car il est extrêmement coûteux en termes de calcul de réexécuter le modèle deux fois pour déterminer la langue.
    Si vous souhaitez passer à Premium, rendez-vous sur https://dash.scripty.org/premium. Vous pouvez également demander un essai gratuit de Premium en envoyant un message privé au bot.
    Si cette fonctionnalité était activée auparavant, elle est désormais désactivée.
cmds_config_server_language = guilde
    .description = Définissez la langue de ce serveur sur l'une des langues disponibles.
    .language = langue
    .language-description = La langue sur laquelle vous souhaitez définir la langue de votre guilde.
config-transcribe-audio-requires-premium =
    La transcription de fichiers audio est une fonctionnalité Premium, car le transcodage de fichiers audio est coûteux en termes de calcul.
    Si vous souhaitez passer à la version Premium, rendez-vous sur https://dash.scripty.org/premium. Vous pouvez également demander un essai gratuit de la version Premium en envoyant un message privé au bot.
    Si cette fonctionnalité était activée auparavant, elle est désormais désactivée.
join-success-description = { $voiceTargetMention } a été rejoint avec succès et la sortie de transcription a été envoyée à { $outputChannelMention }.
config-kiai-missing-perms = Il manque à Scripty les autorisations nécessaires pour fonctionner sur ce serveur. Autorisez le avec la commande `/application authorize`, en utilisant un ID d'application de `811652199100317726` et en donnant à Scripty l'autorisation « afficher et modifier tous les niveaux et XP ».
