package fl_wasm

import "encoding/json"

//export __invoke
func __invoke(reqLen size) errno {
	// get input data (it will be json)
	input := getInputData(reqLen)

	jsonIn := make(map[string]interface{})
	err := json.Unmarshal(input, &jsonIn)
	if err != nil {
		insertError(err.Error())
		return 1
	}

	res, err := Fl.Fn(jsonIn)
	if err != nil {
		insertError(err.Error())
		return 1
	}
	if res == nil {
		insertResponse("{}")
		return 0
	}

	// turn map[string]interface{} into string
	jsonOut, err := json.Marshal(res)
	if err != nil {
		insertError(err.Error())
		return 1
	}

	insertResponse(string(jsonOut))
	return 0
}
