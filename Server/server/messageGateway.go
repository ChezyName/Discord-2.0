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

func launchMessageGateway() *socket.Server {
	options := socket.DefaultServerOptions()
	options.ServerOptions.SetCors(&types.Cors{
		Origin: "*",
	})

	io := socket.NewServer(nil, options)

	http.Handle("/socket.io/", io.ServeHandler(nil))

	io.On("connection", func(clients ...any) {
		client := clients[0].(*socket.Socket)

		fmt.Println("Client Joined: " + client.Handshake().Address)

		client.On("debug", func(datas ...any) {
			client.Emit("debug", "Data")
		})

		client.On("disconnect", func(...any) {
			fmt.Println("Client Disconnected....")
		})
	})

	return io
}
