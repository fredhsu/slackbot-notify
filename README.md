# slackbot-notify
Listens on a given NATS subject and will post whatever messages are posted there to the slack channel configured

Uses a json config file for slackbot information of the form:

```
{
	"token": "xoxb-<token>",
	"webhook_url": "https://hooks.slack.com/services/<URL>",
	"channel": "C<channel>"
}
```

