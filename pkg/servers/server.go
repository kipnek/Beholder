package servers

import (
	"fmt"
	"net"

	"github.com/google/uuid"
)

type Server struct {
	id        uuid.UUID
	iface     string
	port      int
	protocol  int
	transport interface{}
	listener  net.Listener
	urls      []string
}

func (s *Server) Listen() (err error) {
	s.listener, err = net.Listen("tcp", "127.0.0.1:8080")
	if err != nil {
		err = fmt.Errorf("there was an error creating a listener for the %s server: %s", s, err)
		return
	}
	return
}
