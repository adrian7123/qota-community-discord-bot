package configs

import (
	"errors"
	"log"
	"os"

	"github.com/joho/godotenv"
)

func Env(key string) (string, error) {
	err := godotenv.Load(".env")

	if err != nil {
		log.Print("Error loading .env file")
	}

	val := os.Getenv(key)

	if val == "" {
		return "", errors.New("key not found")
	}

	return val, nil
}
