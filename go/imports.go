package fl_wasm

import (
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
