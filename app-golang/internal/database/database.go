package database

import (
	"bufio"
	"database/sql"
	"fmt"
	"log"
	"math/rand"
	"os"
	"strings"
	"time"

	_ "github.com/go-sql-driver/mysql"
)

var (
	DB              *sql.DB
	portugueseWords []string
)

func InitDB() {
	dsn := "root:123123123@tcp(mysql:3306)/prod?parseTime=true"

	var err error
	DB, err = sql.Open("mysql", dsn)
	if err != nil {
		log.Fatalf("could not connect to database: %v", err)
	}

	for range 5 {
		err = DB.Ping()
		if err == nil {
			break
		}
		log.Printf("database ping failed, retrying in 5 seconds: %v", err)
		time.Sleep(5 * time.Second)
	}

	if err != nil {
		log.Fatalf("could not connect to database after retries: %v", err)
	}

	log.Println("database connection successful")
	if err := createSchema(); err != nil {
		log.Fatalf("could not create schema: %v", err)
	}

	rand.Seed(time.Now().UnixNano()) // Seed the random number generator
	if err := loadWords(); err != nil {
		log.Fatalf("could not load portuguese words: %v", err)
	}
}

func createSchema() error {
	_, err := DB.Exec(`
        CREATE TABLE IF NOT EXISTS events (
            id INT NOT NULL,
            value VARCHAR(255) NULL
        );
    `)
	if err != nil {
		return fmt.Errorf("could not create events table: %w", err)
	}

	_, err = DB.Exec(`
        CREATE TABLE IF NOT EXISTS status (
            id INT NOT NULL
        );
    `)
	if err != nil {
		return fmt.Errorf("could not create status table: %w", err)
	}

	var count int
	err = DB.QueryRow("SELECT COUNT(*) FROM status").Scan(&count)
	if err != nil {
		return fmt.Errorf("could not query status count: %w", err)
	}
	if count == 0 {
		_, err = DB.Exec("INSERT INTO status (id) VALUES (0)")
		if err != nil {
			return fmt.Errorf("could not initialize status: %w", err)
		}
	}

	log.Println("database schema verified")
	return nil
}

func GetNewID(tx *sql.Tx) (int, error) {
	var id int
	row := tx.QueryRow("SELECT id FROM status FOR UPDATE")
	if err := row.Scan(&id); err != nil {
		return 0, fmt.Errorf("could not scan status id: %w", err)
	}

	newID := id + 1
	_, err := tx.Exec("UPDATE status SET id = ?", newID)
	if err != nil {
		return 0, fmt.Errorf("could not update status id: %w", err)
	}
	return newID, nil
}

type Event struct {
	ID    int    `json:"id"`
	Value string `json:"value,omitempty"`
}

func CreateSyncEvent() (*Event, error) {
	event, err := CreateAsyncEvent()
	if err != nil {
		return nil, fmt.Errorf("could not create async event for sync processing: %w", err)
	}

	timeout := time.After(30 * time.Second)
	ticker := time.NewTicker(200 * time.Millisecond)
	defer ticker.Stop()

	for {
		select {
		case <-timeout:
			return nil, fmt.Errorf("timeout waiting for event %d to be processed", event.ID)
		case <-ticker.C:
			processedEvent, err := GetEventByID(event.ID)
			if err != nil {
				return nil, fmt.Errorf("error getting event %d during polling: %w", event.ID, err)
			}
			if processedEvent != nil && processedEvent.Value != "" {
				return processedEvent, nil
			}
		}
	}
}

func CreateAsyncEvent() (*Event, error) {
	tx, err := DB.Begin()
	if err != nil {
		return nil, fmt.Errorf("could not begin transaction: %w", err)
	}
	defer tx.Rollback()

	id, err := GetNewID(tx)
	if err != nil {
		return nil, err
	}

	_, err = tx.Exec("INSERT INTO events (id, value) VALUES (?, NULL)", id)
	if err != nil {
		return nil, fmt.Errorf("could not insert async event: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("could not commit transaction: %w", err)
	}

	return &Event{ID: id}, nil
}

func GetEventCount() (int, error) {
	var count int
	// only works if there is no deletes on the application
	err := DB.QueryRow("SELECT id FROM status").Scan(&count)
	return count, err
}

func GetEventByID(id int) (*Event, error) {
	var event Event
	var value sql.NullString
	err := DB.QueryRow("SELECT id, value FROM events WHERE id = ?", id).Scan(&event.ID, &value)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}
	if value.Valid {
		event.Value = value.String
	}
	return &event, nil
}

func ProcessEvent() (bool, error) {
	tx, err := DB.Begin()
	if err != nil {
		return false, fmt.Errorf("could not begin transaction: %w", err)
	}
	defer tx.Rollback()

	var id int
	err = tx.QueryRow("SELECT id FROM events WHERE value IS NULL LIMIT 1 FOR UPDATE SKIP LOCKED").Scan(&id)
	if err != nil {
		if err == sql.ErrNoRows {
			return false, nil
		}
		return false, fmt.Errorf("could not select event to process: %w", err)
	}

	// Simulate work
	time.Sleep(100 * time.Millisecond)

	var value string
	foundUniqueWord := false
	for range 10 {
		value = getRandomWord()
		var placeholder int
		err := tx.QueryRow("SELECT 1 FROM events WHERE value = ?", value).Scan(&placeholder)
		if err == sql.ErrNoRows {
			// The word doesn't exist, we found a unique one.
			foundUniqueWord = true
			break
		}
		if err != nil {
			// A real error occurred during the check.
			return false, fmt.Errorf("could not check if word exists: %w", err)
		}
		// Word exists, loop will continue.
	}

	if !foundUniqueWord {
		log.Printf("Could not find a unique word for event ID: %d after 10 attempts. Skipping for now.", id)
		// The transaction will be rolled back by defer.
		return false, nil
	}

	_, err = tx.Exec("UPDATE events SET value = ? WHERE id = ?", value, id)
	if err != nil {
		return false, fmt.Errorf("could not update event: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return false, fmt.Errorf("could not commit transaction: %w", err)
	}

	log.Printf("processed event ID: %d", id)
	return true, nil
}

func loadWords() error {
	file, err := os.Open("data/words.txt")
	if err != nil {
		return fmt.Errorf("could not open words file: %w", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		word := strings.TrimSpace(scanner.Text())
		if word != "" {
			portugueseWords = append(portugueseWords, word)
		}
	}

	if err := scanner.Err(); err != nil {
		return fmt.Errorf("error reading words file: %w", err)
	}

	if len(portugueseWords) == 0 {
		return fmt.Errorf("no words found in words.txt")
	}
	log.Printf("loaded %d words", len(portugueseWords))
	return nil
}

func getRandomWord() string {
	if len(portugueseWords) == 0 {
		return "fallback_word"
	}
	return portugueseWords[rand.Intn(len(portugueseWords))]
}
