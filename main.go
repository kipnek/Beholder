package main

import (
	"fmt"
	"math/rand"
	"net"
	"sync"
	"time"
)

var (
	connections = make(map[string]chan string)
	mutex       = sync.Mutex{}
)

func handleConnection(conn net.Conn) {
	defer conn.Close()

	// Generate a unique ID for this connection
	clientID := randomString(6)
	messageChannel := make(chan string)
	mutex.Lock()
	connections[clientID] = messageChannel
	mutex.Unlock()

	go func() {
		for {
			message := <-messageChannel
			fmt.Printf("Received from %s: %s\n", clientID, message)
		}
	}()

	buffer := make([]byte, 1024)
	for {
		n, err := conn.Read(buffer)
		if err != nil {
			fmt.Println("Connection closed")
			break
		}
		//get target for the connection
		message := string(buffer[:n])
		target := "some_client_id"
		if targetChannel, found := connections[target]; found {
			targetChannel <- message
		}
	}

	mutex.Lock()
	delete(connections, clientID)
	mutex.Unlock()
}

func monitorConnections() {
	for {
		time.Sleep(5 * time.Second) // Adjust the sleep duration as needed

		fmt.Println("Current connections:")
		mutex.Lock()
		for key, value := range connections {
			fmt.Printf("Key: %s, Value: %v\n", key, value)
		}
		mutex.Unlock()
	}
}

func main() {
	listener, err := net.Listen("tcp", ":8080")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	defer listener.Close()
	go monitorConnections()

	for {
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("Error:", err)
			continue
		}
		go handleConnection(conn)
	}
}

func randomString(length int) string {
	const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
	seededRand := rand.New(rand.NewSource(time.Now().UnixNano()))

	b := make([]byte, length)
	for i := range b {
		b[i] = charset[seededRand.Intn(len(charset))]
	}
	return string(b)
}
