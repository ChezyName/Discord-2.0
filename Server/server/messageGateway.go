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
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 14},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 15},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 16},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 17},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 18},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 19},

		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 20},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 21},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 22},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 23},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 24},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 25},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 26},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 27},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 28},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 29},

		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 30},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 31},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 32},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 33},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 34},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 35},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 36},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 37},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 38},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 39},

		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 30},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 31},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 32},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 33},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 34},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 35},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 36},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 37},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 38},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 39},

		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 30},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 31},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 32},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 33},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 34},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 35},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 36},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 37},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 38},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 39},

		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 30},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 31},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 32},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 33},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 34},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 35},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 36},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 37},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 38},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 39},

		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 30},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 31},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 32},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 33},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 34},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 35},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 36},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 37},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 38},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 39},

		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 30},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 31},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 32},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 33},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 34},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 35},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 36},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 37},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 38},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 39},

		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 30},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 31},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 32},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 33},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 34},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 35},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 36},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 37},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 38},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 39},

		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 30},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 31},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 32},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 33},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 34},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 35},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 36},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 37},
		{Message: "naw", DisplayName: "Name of Cheese", TimeStamp: 38},
		{Message: "yuh", DisplayName: "ChezyName", TimeStamp: 39},
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
