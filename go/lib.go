package fl_wasm

type FLFunc struct{}

type FLMain interface {
	Fn(json map[string]interface{}) (map[string]interface{}, error)
}

var Fl FLMain = &FLFunc{}

func (fl *FLFunc) Fn(json map[string]interface{}) (map[string]interface{}, error) {
	// User Function Here
	return nil, nil
}
