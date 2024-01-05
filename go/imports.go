package main

import (
	"encoding/binary"
	"unsafe"
)

// https://github.com/WebAssembly/WASI/blob/a2b96e81c0586125cc4dc79a5be0b78d9a059925/legacy/preview1/docs.md#-size-u32
type size = uint32

// https://github.com/WebAssembly/WASI/blob/a2b96e81c0586125cc4dc79a5be0b78d9a059925/legacy/preview1/docs.md#-errno-variant
type errno = uint32

//go:wasmimport fl_imps __console_log
//go:noescape
func __console_log(buf unsafe.Pointer, bufLen size)

//go:wasmimport fl_imps __get_input_data
//go:noescape
func __get_input_data(buf unsafe.Pointer)

//go:wasmimport fl_imps __insert_error
//go:noescape
func __insert_error(ptr unsafe.Pointer, len size)

//go:wasmimport fl_imps __insert_response
//go:noescape
func __insert_response(ptr unsafe.Pointer, len size)

//go:wasmimport fl_imps __http_request
//go:noescape
func __http_request(
	response_ptr unsafe.Pointer, response_len_ptr unsafe.Pointer,
	status_ptr unsafe.Pointer,
	method size,
	uri_ptr unsafe.Pointer, uri_len size,
	header_ptr unsafe.Pointer, header_len size,
	body_ptr unsafe.Pointer, body_len size,
)

func ConsoleLog(s string) {
	buf := []byte(s)
	ptr := unsafe.Pointer(&buf[0])

	__console_log(ptr, size(len(s)))
}

func insertError(s string) {
	buf := []byte(s)
	ptr := unsafe.Pointer(&buf[0])
	__insert_error(ptr, size(len(s)))
}

func insertResponse(s string) {
	buf := []byte(s)
	ptr := unsafe.Pointer(&buf[0])
	__insert_response(ptr, size(len(s)))
}

func getInputData(reqLen size) []byte {
	buf := make([]byte, reqLen)

	// Get a pointer to the underlying array of the slice.
	reqPtr := unsafe.Pointer(&buf[0])

	// Call the imported function to fill the buffer.
	__get_input_data(reqPtr)

	// Create a new slice based on the original buffer and its length.
	slice := buf[:reqLen]

	// Create a new byte slice from the slice.
	return append([]byte(nil), slice...)
}

func HttpRequest(
	uri string,
	method string,
	header string,
	body string,
) ([]byte, uint16) {
	// Create a buffer for the uri
	uriBuf := []byte(uri)
	var uriPtr unsafe.Pointer
	if len(uriBuf) > 0 {
		uriPtr = unsafe.Pointer(&uriBuf[0])
	} else {
		uriPtr = unsafe.Pointer(&uriBuf)
	}

	// Create a buffer for the header
	headerBuf := []byte(header)
	var headerPtr unsafe.Pointer
	if len(headerBuf) > 0 {
		headerPtr = unsafe.Pointer(&headerBuf[0])
	} else {
		headerPtr = unsafe.Pointer(&headerBuf)
	}

	// Create a buffer for the body
	bodyBuf := []byte(body)
	var bodyPtr unsafe.Pointer
	if len(bodyBuf) > 0 {
		bodyPtr = unsafe.Pointer(&bodyBuf[0])
	} else {
		bodyPtr = unsafe.Pointer(&bodyBuf)
	}

	// turn the method into a size (GET = 0, POST = 1, PUT = 2, DELETE = 3)
	var methodSize size
	switch method {
	case "GET":
		methodSize = 0
	case "POST":
		methodSize = 1
	case "PUT":
		methodSize = 2
	case "DELETE":
		methodSize = 3
	default:
		methodSize = 0
	}

	// Create a buffer for the response
	var responseLen size
	responseLenPtr := unsafe.Pointer(&responseLen)
	responseBuf := make([]byte, 1024)
	responsePtr := unsafe.Pointer(&responseBuf[0])

	// Create a buffer for the status
	statusBuf := make([]byte, 2)
	statusPtr := unsafe.Pointer(&statusBuf[0])

	// Call the imported function to fill the buffer.
	__http_request(
		responsePtr, responseLenPtr,
		statusPtr,
		methodSize,
		uriPtr, size(len(uri)),
		headerPtr, size(len(header)),
		bodyPtr, size(len(body)),
	)

	// Create a new slice based on the original buffer and its length.
	responseSlice := responseBuf[:responseLen]

	// Create a new byte slice from the slice.
	response := append([]byte(nil), responseSlice...)

	// Create a new slice based on the original buffer and its length.
	statusSlice := statusBuf[:2]

	// Create a new byte slice from the slice.
	status := append([]byte(nil), statusSlice...)

	return response, binary.LittleEndian.Uint16(status)
}
