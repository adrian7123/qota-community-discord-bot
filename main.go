package main

import (
	bot "github.com/adrian7123/qota-community-discord-bot/bot"
	"github.com/adrian7123/qota-community-discord-bot/configs"
)

func main() {

	discordToken, err := configs.Env("DISCORD_BOT")

	if err != nil {
		panic("DISCORD_BOT is required")
	}

	bot.BotToken = discordToken
	bot.Run() // call the run function of bot/bot.go
}
