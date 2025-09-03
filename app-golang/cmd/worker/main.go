
package main

import (
	"dist-systems/app-golang/internal/database"
	"log"
	"time"
)

func main() {
	database.InitDB()

	log.Println("starting worker")

	for {
		processed, err := database.ProcessEvent()
		if err != nil {
			log.Printf("error processing event: %v", err)
		}

		if !processed {
			// Wait before polling again if no event was processed
			time.Sleep(100 * time.Millisecond)
		}
	}
}
