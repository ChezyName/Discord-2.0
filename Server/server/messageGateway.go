package server

import (
	"fmt"
	"net/http"
	"strings"
	"time"

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

type MessageReturn struct {
	Message      Message
	MessageCount int
}

var Messages = []Message{}

func getAllMessages(conn *Connection, server *Server) []Message {
	var bytes uint64

	for _, msg := range Messages {
		bytes += uint64(len(msg.Message))
	}

	for i, user := range server.Connections {
		if user.Name == conn.DisplayName {
			server.Connections[i].TotalReceivedBytes += bytes
			break
		}
	}

	return Messages
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
				fmt.Println("[MSG SERVER] Client Not Allowed: " + client.Handshake().Address + " Used Invalid Display Name on Init")
				return
			}

			displayName, ok := data[0].(string)

			if !ok {
				fmt.Println("[MSG SERVER] Client Not Allowed: " + client.Handshake().Address + " Used Invalid Display Name on Init")
				return
			} else {
				fmt.Println("[MSG SERVER] Client Joined: " + client.Handshake().Address + " // " + displayName)

				//search for IP if already is inside list
				connIndex := FindConnectionByIP(client.Handshake().Address)

				var userConn Connection
				if connIndex == -1 {
					//Add to list
					userConn = Connection{
						DisplayName: displayName,
						IP:          client.Handshake().Address,
					}
					MessageUsers = append(MessageUsers, userConn)
				} else {
					//User Has Changed Thier Name - Perhaps Change All Messages By Them?
					MessageUsers[connIndex].DisplayName = displayName
					userConn = MessageUsers[connIndex]
				}

				//return the current message list
				client.Emit("init", getAllMessages(&userConn, server))

				// Check if user already exists, if not store it as part of the list.
				var index = -1
				for i, item := range server.Connections {
					if strings.Compare(item.Address, client.Handshake().Address) == 0 {
						index = i
						break
					}
				}

				var NewVC VoiceConnection = VoiceConnection{
					Address:           client.Handshake().Address,
					Name:              displayName,
					LastSeen:          time.Now().Unix(),
					CanAutoDisconnect: false,
				}

				if index == -1 {
					// Create new User
					server.Connections = append(server.Connections, NewVC)
				} else {
					server.Connections[index] = NewVC
				}
			}
		})

		//Reloading Messages Since User's Message Count is Invalid
		client.On("msg-reload", func(data ...any) {

			for _, item := range MessageUsers {
				if strings.Compare(item.IP, client.Handshake().Address) == 0 {
					client.Emit("msg-reload", getAllMessages(&item, server))
					break
				}
			}
		})

		client.On("msg", func(data ...any) {
			if len(data) <= 0 {
				fmt.Println("[MSG SERVER] Client Not Allowed: " + client.Handshake().Address + " Send Invalid Message")
				return
			}

			msg, ok := data[0].(string)
			if !ok {
				fmt.Println("[MSG SERVER] Client Not Allowed: " + client.Handshake().Address + " Send Invalid Message")
				return
			} else {
				index := FindConnectionByIP(client.Handshake().Address)
				if index == -1 {
					//cannot send messages if not connected
					fmt.Println("[MSG SERVER] Client Not Allowed: " + client.Handshake().Address + " Not Connected but Sending Messages")
					return
				}

				displayName := MessageUsers[index].DisplayName

				//append to message list
				Messages = append(Messages, Message{
					Message:     msg,
					DisplayName: displayName,
					TimeStamp:   time.Now().Unix(),
				})

				fmt.Println("[MSG SERVER] New Message from " + displayName + ".")

				io.Emit("msg", MessageReturn{
					MessageCount: len(Messages),
					Message: Message{
						Message:     msg,
						DisplayName: displayName,
						TimeStamp:   time.Now().Unix(),
					},
				})

				//Get User and Update Message Counter
				for i, user := range server.Connections {
					if user.Name == displayName {
						server.Connections[i].MessagesSent++
						server.Connections[i].TotalSentBytes += uint64(len(msg))
						break
					}
				}
			}
		})

		//when user leaves the (by choice or disconnected via internet issues)
		client.On("disconnect", func(...any) {
			fmt.Println("[MSG SERVER] Client Disconnected from Message Server")
		})
	})

	return io
}
