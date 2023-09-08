# Scripty Speech Commands Docs

## Introduction
If you're reading this, you're probably interested in integrating Scripty into your Discord bot,
to allow your users to interact with your bot using their voice.
This document will give a general guideline of Scripty's speech commands, and how to use them.

## Speech Commands
You've probably used something like this before. You say "Hey Google" or "Hey Siri" and then
ask it a question. Scripty's speech commands work in a similar way. You say "Hey Scripty" and
then tell it to do something.
For example, you could say "Hey Scripty, play Never Gonna Give You Up" and Scripty will fire a
webhook to your bot, telling it to play that song.

This document specifically goes over that webhook part, and how to handle it.
If you want to get access to Speech Commands, you'll need to join the Scripty Discord server
and request access.
Even once it's out of beta, you'll still need to request access, as we have to manually add
some things to our models on the server side.

## Webhook guidelines
The webhook will be sent to the URL you give us when you request access.
As with all webhooks, it will be a POST request, and will have a JSON body,
with `Content-Type: application/json`.

### Body structure
| Key         | Type            | Description                                                                                                                                                    |
|-------------|-----------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `command`   | String          | One of the commands you gave us when you requested access.                                                                                                     |
| `remainder` | Option\<String> | The rest of the message after the command.<br/> For example, if you said "Hey Scripty, play Never Gonna Give You Up", this would be "Never Gonna Give You Up". |
| `user`      | u64             | The Discord ID of the user who spoke the command                                                                                                               |
| `guild`     | u64             | The Discord ID of the guild the command was spoken in.                                                                                                         |

### Response
You have a few options for how to respond to the webhook.

Note all responses must be sent within 5 seconds of receiving it, or Scripty will assume
an error occurred and will respond with a message saying so.
Defer any long-running tasks to a background thread, and respond immediately.
We don't offer something like Discord where you can defer for up to 15 minutes, as users expect
a response immediately when they speak.

#### 1) Acknowledge but do nothing (HTTP 204 No Content)
If you did the action requested, and it's already noticeable in the voice channel,
you can just respond with a 204 No Content response.
This causes Scripty to play a small "ding" sound to let the user know that the command was received.
Unless your bot is in the voice channel as well (i.e. playing music), you probably don't need to use this.

#### 2) Respond with a message (HTTP 200 OK)
Respond with 200 OK and a JSON body with the following structure:

| Key             | Type   | Description                                                                                             |
|-----------------|--------|---------------------------------------------------------------------------------------------------------|
| `text`          | String | The text to respond with. Will be spoken by the bot via the TTS model you pick when you request access. |
| `high_priority` | bool   | Whether or not to prioritize this message over other active TTS messages.                               |

##### High priority messages
Do not set `high_priority` to true unless you have a good reason to.
This overrides all active user messages and will mix them in, so may cause it to be difficult to understand any message.
Only use this is you absolutely cannot wait for the user messages to finish.

If you don't set `high_priority`, the message will be played after all active user messages finish.

#### 3) Give the user an error message (HTTP 400 Bad Request)
If the user didn't have permissions or something of the sort to do the action they requested,
you can respond with a 400 Bad Request and a JSON body with the following structure:

| Key    | Type   | Description                                                                                             |
|--------|--------|---------------------------------------------------------------------------------------------------------|
| `text` | String | The text to respond with. Will be spoken by the bot via the TTS model you pick when you request access. |

Note this response will always be low priority, as it's an error message. If it takes longer than
five seconds to play, Scripty will DM the user as well with the error message.

#### 4) Respond with an error message (HTTP 500 Internal Server Error)
Should only be used if you hit an unrecoverable error on your end.
Scripty will both DM the user and speak the error message, as a low priority message.

| Key    | Type   | Description                                                                                             |
|--------|--------|---------------------------------------------------------------------------------------------------------|
| `text` | String | The text to respond with. Will be spoken by the bot via the TTS model you pick when you request access. |


## Example

### Pause song
Here's an example of a webhook body and response, for a user who said "Hey Scripty, pause the current song."

#### Request
```json
{
  "command": "pause",
  "remainder": "the current song",
  "user": 123456789012345678,
  "guild": 123456789012345678
}
```

#### Response (204 No Content)
Note a 204 response is selected as the user can immediately hear the song pause, so there's no need to respond with a message.

### Ask a question
Here's an example of a webhook body and response, for a user who said "Hey Scripty, what's the weather like in New York?"

#### Request
```json
{
  "command": "weather",
  "remainder": "New York",
  "user": 123456789012345678,
  "guild": 123456789012345678
}
```

#### Response (200 OK)
```json
{
  "text": "It's currently 23 degrees and sunny in New York.",
  "high_priority": false
}
```
