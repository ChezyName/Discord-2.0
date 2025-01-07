package server

import (
	"net/http"

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

func launchMessageGateway() *socket.Server {
	io := socket.NewServer(nil, nil)
	http.Handle("/messages/", io.ServeHandler(nil))

	io.On("connection", func(clients ...any) {
		client := clients[0].(*socket.Socket)
		client.On("event", func(datas ...any) {
		})
		client.On("disconnect", func(...any) {
		})
	})

	return io
}
