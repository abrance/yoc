package main

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHelloHandlerWritesGreeting(t *testing.T) {
	request := httptest.NewRequest(http.MethodGet, "/", nil)
	recorder := httptest.NewRecorder()

	helloHandler(recorder, request)

	if recorder.Code != http.StatusOK {
		t.Fatalf("status code = %d, want %d", recorder.Code, http.StatusOK)
	}

	if got := recorder.Body.String(); got != "hello from go\n" {
		t.Fatalf("body = %q, want %q", got, "hello from go\n")
	}
}
