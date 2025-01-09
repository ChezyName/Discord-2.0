package server

import (
	"fmt"
	"net/http"

	"github.com/zishang520/engine.io/v2/types"
	"github.com/zishang520/socket.io/v2/socket"
)

/**
* Messaging Server that sends and recieves messages from user to user
* Additionally, functions as Event Serevr which can send live events to users
* 	Events include the following:
*		- Message Pings
*		- User Joined Call
*		- Other Notifications
 */

type Message struct {
	Message     string `json:"message"`
	DisplayName string `json:"user"`
	TimeStamp   int64  `json:"TimeStamp"`
}

type Connection struct {
	IP          string
	DisplayName string
}

func getAllMessages() []Message {
	return []Message{
		{Message: "Hello", DisplayName: "ChezyName", TimeStamp: 1},
		{Message: "Hey", DisplayName: "Name of Cheese", TimeStamp: 2},
		{Message: "So, you gon tell me about why you be doin it?", DisplayName: "ChezyName", TimeStamp: 3},
		{Message: "???", DisplayName: "Name of Cheese", TimeStamp: 4},
		{Message: "I saw you dawg", DisplayName: "ChezyName", TimeStamp: 5},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 6},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 7},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 8},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 9},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 10},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 11},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 12},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 13},
	}
}

var MessageUsers []Connection

// Function to find the index of a Connection by IP
func FindConnectionByIP(ip string) int {
	for index, user := range MessageUsers {
		if user.IP == ip {
			return index
		}
	}
	return -1
}

func launchMessageGateway(server *Server) *socket.Server {
	options := socket.DefaultServerOptions()
	options.ServerOptions.SetCors(&types.Cors{
		Origin: "*",
	})

	io := socket.NewServer(nil, options)

	http.Handle("/socket.io/", io.ServeHandler(nil))

	io.On("connection", func(clients ...any) {
		client := clients[0].(*socket.Socket)

		//init function is when the user handshakes and is preped
		client.On("init", func(data ...any) {
			if len(data) <= 0 {
				fmt.Println("Client Not Allowed: " + client.Handshake().Address + " Used Invalid Display Name on Init")
				return
			}

			displayName, ok := data[0].(string)

			if !ok {
				fmt.Println("Client Not Allowed: " + client.Handshake().Address + " Used Invalid Display Name on Init")
				return
			} else {
				fmt.Println("Client Joined: " + client.Handshake().Address + " // " + displayName)

				//search for IP if already is inside list
				index := FindConnectionByIP(client.Handshake().Address)

				if index == -1 {
					//Add to list
					MessageUsers = append(MessageUsers, Connection{
						DisplayName: displayName,
						IP:          client.Handshake().Address,
					})
				} else {
					//User Has Changed Thier Name - Perhaps Change All Messages By Them?
					MessageUsers[index].DisplayName = displayName
				}

				//return the current message list
				client.Emit("init", getAllMessages())
			}
		})

		//when user leaves the (by choice or disconnected via internet issues)
		client.On("disconnect", func(...any) {
			fmt.Println("Client Disconnected....")
		})
	})

	return io
}
